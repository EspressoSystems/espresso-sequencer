r"""Run Claude on a GitHub PR or issue with a fixed set of tools, in three workflow jobs.

    claude-gh start review          # write token: post the status comment, print its id
    claude-gh run review            # read token, Claude: record actions in tmp/actions.json
    claude-gh run code-review --model opus --effort high
    claude-gh apply review --status-id 1 --run-result success --record tmp/actions/actions.json

Jobs: review (PR opened), mention (@claude mention, answers in the thread), code-review.

`run` gives the model Read/Grep/Glob and an in-process MCP server `gh`. There is no Bash.
Reads go to the PR or issue bound at startup. Writes are only recorded, never sent.

`apply` is best effort: each action is validated and posted independently. An action that
fails validation or that GitHub rejects is skipped and listed in the status comment under
"Not applied", and apply exits non-zero after posting the rest. Only a structurally
unusable record (bad JSON, unknown kind, job mismatch) is refused as a whole.

The record is not a trust boundary on review: the PR author controls the apply workflow
too. The checks catch model mistakes and prompt injection: a thread must be started by a
bot and belong to the same PR, and the target, mention routing and head sha come from the
event, never from the record.

Env: GH_TOKEN, GITHUB_EVENT_PATH, GITHUB_REPOSITORY, GITHUB_SERVER_URL, GITHUB_RUN_ID,
GITHUB_EVENT_NAME, ANTHROPIC_API_KEY (run), GITHUB_STEP_SUMMARY (optional).
"""

import argparse
import asyncio
import json
import logging
import os
import re
import sys
from collections.abc import AsyncIterator, Awaitable, Callable, Mapping
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any, Protocol, get_args

from claude_agent_sdk import (
    AssistantMessage,
    ClaudeAgentOptions,
    Message,
    ResultMessage,
    SdkMcpTool,
    ToolUseBlock,
    create_sdk_mcp_server,
    query,
    tool,
)
from claude_agent_sdk.types import (
    EffortLevel,
    McpSdkServerConfig,
    SettingSource,
    SystemPromptPreset,
)
from githubkit import GitHub as GitHubClient
from githubkit import TokenAuthStrategy
from githubkit.exception import RequestFailed

log = logging.getLogger("claude-gh")

BUILTIN = ["Read", "Grep", "Glob"]
DISALLOWED = ["Read(//proc/**)"]
BOTS = {"github-actions", "claude"}
WRITES = {"inline_comment", "reply", "comment", "resolve_thread"}
JOB_WRITES = {
    "review": WRITES,
    "mention": {"comment"},
    "code-review": WRITES - {"resolve_thread"},
}
# The CLI gets no GitHub token: all GitHub access goes through the `gh` MCP server.
# Background subagents outlive the main result and leave the review unposted.
CLI_ENV = {"GH_TOKEN": "", "CLAUDE_CODE_DISABLE_BACKGROUND_TASKS": "1"}
MAX_TURNS = 120
SERVER = "gh"
THREAD_ID = re.compile(r"[A-Za-z0-9_=-]+")
HUNK = re.compile(r"@@ -\d+(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")
CLAUDE_DIR = Path(__file__).parent
BACKPORT_MARKER = Path("tmp/is-backport")
RECORD = Path("tmp/actions.json")
PREFETCH_DIR = Path("tmp")
READS = {"pr_diff", "pr_view", "issue_view"}
# Review reads the PR from files written by `prefetch`, so it gets no read tools.
JOB_READS = {"review": set(), "mention": READS, "code-review": READS}


@dataclass(frozen=True)
class Target:
    repo: str
    number: int
    is_pr: bool


@dataclass(frozen=True)
class Mention:
    target: Target
    reply_to: int | None  # review-thread root; None means a top-level comment
    context: dict[str, Any]


@dataclass(frozen=True)
class Posted:
    id: int
    review: bool  # review-thread comment (pulls API) rather than issue comment


@dataclass(frozen=True)
class InlineComment:
    path: str
    line: int
    body: str


@dataclass(frozen=True)
class Reply:
    comment_id: int
    body: str


@dataclass(frozen=True)
class Resolve:
    thread_id: str


Action = InlineComment | Reply | Resolve


ACTIONS: dict[str, type[Action]] = {
    "inline_comment": InlineComment,
    "reply": Reply,
    "resolve_thread": Resolve,
}
KINDS = {cls: kind for kind, cls in ACTIONS.items()}


@dataclass
class Record:
    """What `run` writes and `apply` reads."""

    job: str
    model: str
    actions: list[Action] = field(default_factory=list)
    summary: str | None = None  # body of the last `comment` call
    footer: str | None = None  # cost line; None without a result message
    error: str | None = None  # result subtype when the run ended in error


@dataclass(frozen=True)
class Job:
    name: str
    prompt: str
    model: str
    effort: EffortLevel
    append: str | None
    tools: list[SdkMcpTool]
    builtin: list[str]
    settings: list[SettingSource]
    record: Record


@dataclass(frozen=True)
class Thread:
    author: str
    repo: str
    number: int


class GitHub(Protocol):
    async def pr_diff(self, pr: int) -> str: ...
    async def pr_view(self, pr: int) -> str: ...
    async def pr_meta(self, pr: int) -> str: ...
    async def issue_view(self, number: int) -> str: ...
    async def head_sha(self, pr: int) -> str: ...
    async def inline_comment(
        self, pr: int, path: str, line: int, body: str, sha: str
    ) -> None: ...
    async def reply(self, pr: int, comment_id: int, body: str) -> Posted: ...
    async def comment(self, number: int, body: str) -> Posted: ...
    async def edit_comment(self, comment: Posted, body: str) -> None: ...
    async def thread(self, thread_id: str) -> Thread: ...
    async def resolve_thread(self, thread_id: str) -> None: ...


Runner = Callable[[Job], AsyncIterator[Message]]


# Pure core


def target(event: dict) -> Target:
    item = event["issue"] if "issue" in event else event["pull_request"]
    return Target(
        event["repository"]["full_name"],
        item["number"],
        "pull_request" in event or "pull_request" in item,
    )


def mention_from_event(name: str, event: dict) -> Mention:
    item = event["issue"] if "issue" in event else event["pull_request"]
    comment = event.get("comment")
    review = event.get("review")
    reply_to = None
    if name == "pull_request_review_comment":
        root = event["comment"]
        reply_to = root.get("in_reply_to_id") or root["id"]
    context = {
        "event_name": name,
        "title": item["title"],
        "body": item["body"],
        "author": (comment or review or item)["user"]["login"],
        "comment_body": comment and comment["body"],
        "path": comment and comment.get("path"),
        "line": comment and comment.get("line"),
        "diff_hunk": comment and comment.get("diff_hunk"),
        "review_body": review and review["body"],
    }
    context = {k: v for k, v in context.items() if v is not None}
    return Mention(target(event), reply_to, context)


def focus_path(backport: bool) -> Path:
    return CLAUDE_DIR / ("review-backport.md" if backport else "review-standard.md")


def review_prompt(
    target: Target, head_sha: str, run_url: str, focus: str, common: str
) -> str:
    header = (
        f"REPO: {target.repo}\nPR NUMBER: {target.number}\n"
        f"HEAD SHA: {head_sha}\nRUN URL: {run_url}"
    )
    return f"{header}\n\n{focus}\n\n{common}"


def mention_prompt(m: Mention, doc: str) -> str:
    event = json.dumps(m.context, indent=2)
    return f"REPO: {m.target.repo}\n\n{doc}\n\nEvent:\n\n{event}\n"


def code_review_prompt(target: Target, effort: str) -> str:
    return f"/code-review {effort} --comment {target.number}"


def diff_lines(diff: str) -> dict[str, set[int]]:
    """RIGHT-side line numbers inside a hunk (added or context), per file in a unified diff."""
    result: dict[str, set[int]] = {}
    path: str | None = None
    old = new = line_no = 0  # lines left in the current hunk, next RIGHT line number
    for line in diff.split("\n"):
        if old > 0 or new > 0:
            tag = line[:1]
            if tag == "\\":
                continue
            if tag != "+":
                old -= 1
            if tag != "-":
                new -= 1
                if path:
                    result[path].add(line_no)
                line_no += 1
        elif line.startswith("+++ "):
            name = line[4:]
            path = name.removeprefix("b/") if name.startswith("b/") else None
            if path:
                result.setdefault(path, set())
        elif m := HUNK.match(line):
            old = int(m[1] or 1)
            line_no = int(m[2])
            new = int(m[3] or 1)
    return result


def check_in_diff(lines: Mapping[str, set[int]], path: str, line: int) -> None:
    if path not in lines:
        raise ValueError(f"{path} is not a file changed in the PR diff")
    if line not in lines[path]:
        raise ValueError(
            f"line {line} of {path} is not inside a diff hunk; "
            "use a RIGHT-side line shown in the diff"
        )


def github_error(e: Exception) -> str:
    """One line for a failed call: status, GitHub's message and its error details."""
    if not isinstance(e, RequestFailed):
        return " ".join(str(e).split())[:200]
    resp = e.response
    try:
        body = json.loads(resp.text)
    except ValueError:
        return f"{resp.status_code} {' '.join(resp.text.split())[:200]}"
    details = [
        err if isinstance(err, str) else err.get("message") or err.get("code", "")
        for err in body.get("errors", [])
    ]
    parts = [str(resp.status_code), body.get("message", ""), *details]
    return " ".join(p for p in parts if p)[:300]


def check_body(body: str, secrets: Mapping[str, str]) -> None:
    for name, value in secrets.items():
        if value and value in body:
            raise ValueError(f"body contains {name}, refusing")


def options(job: Job, server: McpSdkServerConfig) -> ClaudeAgentOptions:
    prompt: SystemPromptPreset = {"type": "preset", "preset": "claude_code"}
    if job.append:
        prompt["append"] = job.append
    return ClaudeAgentOptions(
        model=job.model,
        effort=job.effort,
        max_turns=MAX_TURNS,
        tools=job.builtin,
        allowed_tools=job.builtin + [f"mcp__{SERVER}"],
        disallowed_tools=DISALLOWED,
        setting_sources=job.settings,
        system_prompt=prompt,
        mcp_servers={SERVER: server},
        permission_mode="dontAsk",
        env=CLI_ENV,
    )


def select_tools(tools: list[SdkMcpTool], job_name: str) -> list[SdkMcpTool]:
    allowed = JOB_WRITES[job_name] | JOB_READS[job_name]
    return [t for t in tools if t.name not in WRITES | READS or t.name in allowed]


def tool_line(name: str, args: Mapping[str, Any]) -> str:
    parts = []
    for key, value in args.items():
        text = f"<{len(value)} chars>" if key == "body" else str(value)
        parts.append(f"{key}={text.replace(chr(10), ' ')[:80]}")
    return f"tool {name.removeprefix(f'mcp__{SERVER}__')} {' '.join(parts)}".rstrip()


def result_lines(result: ResultMessage | None) -> tuple[list[str], int]:
    """Log lines for a run and its exit code; the last line is also the step summary."""
    if result is None:
        return ["no result message"], 1
    lines = [result.result] if result.result else []
    code = 0
    if result.is_error:
        lines.append(f"result is_error, subtype={result.subtype}")
        code = 1
    denials = json.dumps(result.permission_denials)
    lines.append(
        f"total_cost_usd={result.total_cost_usd} turns={result.num_turns} "
        f"duration_s={result.duration_ms / 1000:.0f} permission_denials={denials}"
    )
    return lines, code


def footer(result: ResultMessage, model: str) -> str:
    cost = "n/a" if result.total_cost_usd is None else f"${result.total_cost_usd:.2f}"
    minutes, seconds = divmod(round(result.duration_ms / 1000), 60)
    duration = f"{minutes}m {seconds}s" if minutes else f"{seconds}s"
    return f"<sub>Cost {cost} · {result.num_turns} turns · {duration} · {model}</sub>"


def noun(job: str) -> str:
    return "Answer" if job == "mention" else "Review"


def status_body(
    record: Record, run_result: str, run_url: str, skipped: list[str]
) -> str:
    what = noun(record.job)
    reason = record.error or (run_result if run_result != "success" else None)
    parts = [record.summary] if record.summary else []
    if skipped:
        parts.append("**Not applied**\n" + "\n".join(skipped))
    if reason:
        parts.append(f"{what} incomplete: {reason}. [run]({run_url})")
    elif not record.summary:
        parts.append(f"{what} finished without a summary")
    if record.footer:
        parts.append(record.footer)
    return "\n\n".join(parts)


def to_json(record: Record) -> dict[str, Any]:
    data = asdict(record)
    data["actions"] = [{"kind": KINDS[type(a)], **asdict(a)} for a in record.actions]
    return data


def parse_action(data: Any) -> Action:
    fields = dict(data)
    kind = fields.pop("kind")
    if kind not in ACTIONS:
        raise ValueError(f"unknown action kind {kind!r}")
    return ACTIONS[kind](**fields)


def parse_record(data: Any, job: str) -> Record:
    fields = dict(data)
    actions = [parse_action(a) for a in fields.pop("actions")]
    record = Record(**fields, actions=actions)
    if record.job != job:
        raise ValueError(f"record is for job {record.job!r}, expected {job!r}")
    return record


def check_action(action: Action) -> None:
    match action:
        case InlineComment(line=line) if line <= 0:
            raise ValueError(f"line must be positive, got {line}")
        case Reply(comment_id=comment_id) if comment_id <= 0:
            raise ValueError(f"comment_id must be positive, got {comment_id}")
        case Resolve(thread_id=thread_id) if not THREAD_ID.fullmatch(thread_id):
            raise ValueError(f"invalid thread_id {thread_id!r}")


def label(action: Action) -> str:
    match action:
        case InlineComment(path=path, line=line):
            return f"inline_comment {path}:{line}"
        case Reply(comment_id=comment_id):
            return f"reply {comment_id}"
        case Resolve(thread_id=thread_id):
            return f"resolve_thread {thread_id}"


def parse_thread(data: dict[str, Any]) -> Thread:
    node = data["node"]
    if not node or "pullRequest" not in node:
        raise ValueError("id is not a review thread")
    pr = node["pullRequest"]
    return Thread(
        node["comments"]["nodes"][0]["author"]["login"],
        pr["repository"]["nameWithOwner"],
        pr["number"],
    )


def login(user: Any) -> str:
    return "ghost" if user is None else user.login  # deleted accounts


def view_data(item: Any) -> dict[str, Any]:
    return {
        "title": item.title,
        "body": item.body,
        "author": login(item.user),
        "state": item.state,
    }


def view_json(item: Any) -> str:
    return json.dumps(view_data(item))


def meta_json(pr: Any) -> str:
    refs = {"base": pr.base.ref, "head": pr.head.ref}
    return json.dumps({**view_data(pr), **refs})


# Tools


def make_tools(
    gh: GitHub,
    target: Target,
    secrets: Mapping[str, str],
    rec: Record,
    lines: Mapping[str, set[int]] | None = None,
) -> list[SdkMcpTool]:
    """Reads act on `target`; the model never names a PR, issue or repo.

    Builds every tool; `select_tools` drops the writes a job may not use. Writes append to
    `rec` and post nothing. With `lines` (see `diff_lines`), inline comments outside the
    diff are refused. Handlers raise ValueError on refused calls, which the SDK
    reports to the model as an error result.
    """
    n = target.number

    def define(
        name: str,
        desc: str,
        schema: dict[str, type],
        run: Callable[[Any], Awaitable[str]],
    ):
        async def handler(args: dict[str, Any]) -> dict[str, Any]:
            if "body" in args:
                check_body(args["body"], secrets)
            out = await run(args)
            return {"content": [{"type": "text", "text": out}]}

        return tool(name, desc, schema)(handler)

    async def add(action: Action) -> str:
        check_action(action)
        if lines is not None and isinstance(action, InlineComment):
            check_in_diff(lines, action.path, action.line)
        rec.actions.append(action)
        return "recorded"

    async def comment(a: dict[str, Any]) -> str:
        rec.summary = a["body"]
        return "recorded"

    if target.is_pr:
        reads = [
            define("pr_diff", "Diff of the PR.", {}, lambda a: gh.pr_diff(n)),
            define(
                "pr_view",
                "Title, body and author of the PR.",
                {},
                lambda a: gh.pr_view(n),
            ),
        ]
    else:
        reads = [
            define(
                "issue_view",
                "Title, body and author of the issue.",
                {},
                lambda a: gh.issue_view(n),
            )
        ]
    return reads + [
        define(
            "inline_comment",
            "Review comment on a line of the PR head, RIGHT side.",
            {"path": str, "line": int, "body": str},
            lambda a: add(InlineComment(a["path"], a["line"], a["body"])),
        ),
        define(
            "reply",
            "Reply in a review thread; comment_id is the thread root comment.",
            {"comment_id": int, "body": str},
            lambda a: add(Reply(a["comment_id"], a["body"])),
        ),
        define(
            "comment",
            "Final summary comment; the last call is posted.",
            {"body": str},
            comment,
        ),
        define(
            "resolve_thread",
            "Resolve a review thread started by github-actions or claude.",
            {"thread_id": str},
            lambda a: add(Resolve(a["thread_id"])),
        ),
    ]


# GitHub implementations


class GitHubKit:
    def __init__(self, repo: str, token: str):
        self.owner, self.name = repo.split("/")
        self.gh = GitHubClient(TokenAuthStrategy(token))

    async def pr_diff(self, pr: int) -> str:
        resp = await self.gh.arequest(
            "GET",
            f"/repos/{self.owner}/{self.name}/pulls/{pr}",
            headers={"Accept": "application/vnd.github.diff"},
        )
        return resp.text

    async def pr_view(self, pr: int) -> str:
        resp = await self.gh.rest.pulls.async_get(
            owner=self.owner, repo=self.name, pull_number=pr
        )
        return view_json(resp.parsed_data)

    async def pr_meta(self, pr: int) -> str:
        resp = await self.gh.rest.pulls.async_get(
            owner=self.owner, repo=self.name, pull_number=pr
        )
        return meta_json(resp.parsed_data)

    async def issue_view(self, number: int) -> str:
        resp = await self.gh.rest.issues.async_get(
            owner=self.owner, repo=self.name, issue_number=number
        )
        return view_json(resp.parsed_data)

    async def head_sha(self, pr: int) -> str:
        resp = await self.gh.rest.pulls.async_get(
            owner=self.owner, repo=self.name, pull_number=pr
        )
        return resp.parsed_data.head.sha

    async def inline_comment(
        self, pr: int, path: str, line: int, body: str, sha: str
    ) -> None:
        await self.gh.rest.pulls.async_create_review_comment(
            owner=self.owner,
            repo=self.name,
            pull_number=pr,
            body=body,
            commit_id=sha,
            path=path,
            line=line,
            side="RIGHT",
        )

    async def reply(self, pr: int, comment_id: int, body: str) -> Posted:
        resp = await self.gh.rest.pulls.async_create_reply_for_review_comment(
            owner=self.owner,
            repo=self.name,
            pull_number=pr,
            comment_id=comment_id,
            body=body,
        )
        return Posted(resp.parsed_data.id, review=True)

    async def comment(self, number: int, body: str) -> Posted:
        resp = await self.gh.rest.issues.async_create_comment(
            owner=self.owner, repo=self.name, issue_number=number, body=body
        )
        return Posted(resp.parsed_data.id, review=False)

    async def edit_comment(self, comment: Posted, body: str) -> None:
        if comment.review:
            await self.gh.rest.pulls.async_update_review_comment(
                owner=self.owner, repo=self.name, comment_id=comment.id, body=body
            )
        else:
            await self.gh.rest.issues.async_update_comment(
                owner=self.owner, repo=self.name, comment_id=comment.id, body=body
            )

    async def thread(self, thread_id: str) -> Thread:
        data = await self.gh.async_graphql(THREAD_QUERY, {"id": thread_id})
        return parse_thread(data)

    async def resolve_thread(self, thread_id: str) -> None:
        await self.gh.async_graphql(RESOLVE_MUTATION, {"id": thread_id})


THREAD_QUERY = """
query($id: ID!) {
  node(id: $id) {
    ... on PullRequestReviewThread {
      pullRequest { number repository { nameWithOwner } }
      comments(first: 1) { nodes { author { login } } }
    }
  }
}"""

RESOLVE_MUTATION = """
mutation($id: ID!) {
  resolveReviewThread(input: {threadId: $id}) { thread { isResolved } }
}"""


# Start and apply (write token, no Claude)


async def start(
    gh: GitHub, job: str, event_name: str, event: dict, run_url: str
) -> Posted:
    n = target(event).number
    if job == "review":
        text = f"Reviewing {event['pull_request']['head']['sha'][:7]}… [run]({run_url})"
    else:
        text = f"Working on it… [run]({run_url})"
    root = mention_from_event(event_name, event).reply_to if job == "mention" else None
    if root:
        return await gh.reply(n, root, text)
    return await gh.comment(n, text)


async def print_start(
    gh: GitHub, job: str, event_name: str, event: dict, run_url: str
) -> int:
    """Prints only the status comment id: the workflow captures stdout."""
    print((await start(gh, job, event_name, event, run_url)).id)
    return 0


async def check_thread(gh: GitHub, tgt: Target, thread_id: str) -> None:
    thread = await gh.thread(thread_id)
    if (thread.repo, thread.number) != (tgt.repo, tgt.number):
        raise ValueError(
            f"thread {thread_id} belongs to {thread.repo}#{thread.number}, "
            f"not {tgt.repo}#{tgt.number}"
        )
    if thread.author not in BOTS:
        raise ValueError(
            f"thread {thread_id} was started by {thread.author!r}, not resolving"
        )


async def post(
    gh: GitHub, job: str, tgt: Target, sha: str | None, action: Action
) -> None:
    kind = KINDS[type(action)]
    if kind not in JOB_WRITES[job]:
        raise ValueError(f"{kind} not allowed in {job}")
    check_action(action)
    match action:
        case InlineComment(path, line, body):
            assert sha is not None
            await gh.inline_comment(tgt.number, path, line, body, sha)
        case Reply(comment_id, body):
            await gh.reply(tgt.number, comment_id, body)
        case Resolve(thread_id):
            await check_thread(gh, tgt, thread_id)
            await gh.resolve_thread(thread_id)


async def apply(
    gh: GitHub,
    job: str,
    tgt: Target,
    raw: str | None,
    status: Posted,
    run_url: str,
    run_result: str,
    head_sha: str | None,
) -> int:
    """Post each action independently, then edit the status. Returns the exit code.

    `head_sha` comes from the event when it has one; otherwise it is fetched once.
    """
    what = noun(job)
    if raw is None:
        reason = "no actions file" if run_result == "success" else run_result
        await gh.edit_comment(status, f"{what} failed: {reason}. [run]({run_url})")
        return 1
    try:
        record = parse_record(json.loads(raw), job)
    except (ValueError, TypeError, KeyError) as e:
        log.error("invalid record: %r", e)
        await gh.edit_comment(
            status, f"{what} failed: invalid actions. [run]({run_url})"
        )
        raise
    if head_sha is None and any(isinstance(a, InlineComment) for a in record.actions):
        head_sha = await gh.head_sha(tgt.number)
    skipped = []
    for action in record.actions:
        try:
            await post(gh, job, tgt, head_sha, action)
        except Exception as e:  # noqa: BLE001 best effort: any failure skips this action only
            reason = github_error(e)
            log.error("skipped %s: %s", label(action), reason)
            skipped.append(f"- {label(action)}: {reason}")
    await gh.edit_comment(status, status_body(record, run_result, run_url, skipped))
    return 0 if run_result == "success" and not skipped else 1


# Run (read token, Claude)


async def prefetch(gh: GitHub, pr: int, out: Path) -> str:
    """Writes the PR files for the model and returns the diff."""
    out.mkdir(parents=True, exist_ok=True)
    diff = await gh.pr_diff(pr)
    (out / "pr.json").write_text(await gh.pr_meta(pr))
    (out / "pr.diff").write_text(diff)
    return diff


async def run(
    job: Job, runner: Runner, record_path: Path, step_summary: Path | None = None
) -> int:
    result = None
    try:
        async for message in runner(job):
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, ToolUseBlock):
                        log.info(tool_line(block.name, block.input))
            elif isinstance(message, ResultMessage):
                result = message
    finally:
        if result:
            job.record.footer = footer(result, job.model)
            job.record.error = result.subtype if result.is_error else None
        record_path.parent.mkdir(parents=True, exist_ok=True)
        record_path.write_text(json.dumps(to_json(job.record)))
    lines, code = result_lines(result)
    for line in lines:
        log.info(line)
    if step_summary and result:
        with step_summary.open("a") as f:
            f.write(f"- claude-gh {job.name}: {lines[-1]}\n")
    return code


def sdk_runner(job: Job) -> AsyncIterator[Message]:
    server = create_sdk_mcp_server(SERVER, tools=job.tools)
    return query(prompt=job.prompt, options=options(job, server))


def load_secrets() -> dict[str, str]:
    return {
        "ANTHROPIC_API_KEY": os.environ["ANTHROPIC_API_KEY"],
        "GH_TOKEN": os.environ["GH_TOKEN"],
    }


def build_job(
    args: argparse.Namespace,
    gh: GitHub,
    secrets: Mapping[str, str],
    event: dict,
    run_url: str,
    diff: str | None,
) -> Job:
    append = None
    builtin = BUILTIN
    settings: list[SettingSource] = ["user", "project"]  # base-branch checkout
    if args.job == "review":
        tgt = target(event)
        head_sha = event["pull_request"]["head"]["sha"]
        focus = focus_path(BACKPORT_MARKER.exists()).read_text()
        common = (CLAUDE_DIR / "review-common.md").read_text()
        prompt = review_prompt(tgt, head_sha, run_url, focus, common)
        settings = ["user"]  # PR head checkout: no project settings
    elif args.job == "mention":
        mention = mention_from_event(os.environ["GITHUB_EVENT_NAME"], event)
        tgt = mention.target
        prompt = mention_prompt(mention, (CLAUDE_DIR / "mention.md").read_text())
    else:
        tgt = target(event)
        prompt = code_review_prompt(tgt, args.effort)
        append = (CLAUDE_DIR / "code-review.md").read_text()
        builtin = BUILTIN + ["Agent"]
    record = Record(args.job, args.model)
    lines = None if diff is None else diff_lines(diff)
    tools = make_tools(gh, tgt, secrets, record, lines)
    return Job(
        args.job,
        prompt,
        args.model,
        args.effort,
        append,
        select_tools(tools, args.job),
        builtin,
        settings,
        record,
    )


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    sub = parser.add_subparsers(dest="command", required=True)
    for command in ("start", "run", "apply"):
        p = sub.add_parser(command)
        p.add_argument("job", choices=JOB_WRITES)
        if command == "run":
            p.add_argument("--model", default="opus")
            p.add_argument("--effort", default="high", choices=get_args(EffortLevel))
        if command == "apply":
            p.add_argument("--status-id", type=int, required=True)
            p.add_argument("--run-result", required=True)
            p.add_argument("--record", type=Path, required=True)
    return parser.parse_args(argv)


def main() -> int:
    logging.basicConfig(
        level=logging.INFO,
        format="%(levelname)s %(name)s: %(message)s",
        stream=sys.stderr,
    )
    args = parse_args(sys.argv[1:])
    return asyncio.run(amain(args))


async def amain(args: argparse.Namespace) -> int:
    event = json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text())
    repo = os.environ["GITHUB_REPOSITORY"]
    server, run_id = os.environ["GITHUB_SERVER_URL"], os.environ["GITHUB_RUN_ID"]
    run_url = f"{server}/{repo}/actions/runs/{run_id}"
    gh: GitHub = GitHubKit(repo, os.environ["GH_TOKEN"])
    name = os.environ["GITHUB_EVENT_NAME"]
    if args.command == "start":
        return await print_start(gh, args.job, name, event, run_url)
    if args.command == "apply":
        root = (
            mention_from_event(name, event).reply_to if args.job == "mention" else None
        )
        raw = args.record.read_text() if args.record.exists() else None
        status = Posted(args.status_id, review=root is not None)
        head_sha = (
            event["pull_request"]["head"]["sha"] if args.job == "review" else None
        )
        return await apply(
            gh, args.job, target(event), raw, status, run_url, args.run_result, head_sha
        )
    diff = None
    if args.job == "review":
        diff = await prefetch(gh, target(event).number, PREFETCH_DIR)
    elif args.job == "code-review":
        diff = await gh.pr_diff(target(event).number)
    job = build_job(args, gh, load_secrets(), event, run_url, diff)
    summary_file = os.environ.get("GITHUB_STEP_SUMMARY")
    return await run(
        job, sdk_runner, RECORD, Path(summary_file) if summary_file else None
    )


if __name__ == "__main__":
    sys.exit(main())
