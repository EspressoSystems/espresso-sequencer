"""Tests for `claude_gh`: tool recording, apply validation, status texts and exit codes, with fake GitHub and agent.

No network or SDK calls. Run with:

    just claude::test
"""

import asyncio
import json
import logging
from collections.abc import AsyncIterator
from dataclasses import dataclass, field
from pathlib import Path
from types import SimpleNamespace
from typing import Any

import pytest
from claude_agent_sdk import (
    AssistantMessage,
    Message,
    ResultMessage,
    SdkMcpTool,
    ToolUseBlock,
)
from githubkit.exception import RequestFailed

import claude_gh as cg

PR = cg.Target("o/r", 7, True)
ISSUE = cg.Target("o/r", 9, False)
SECRETS = {"ANTHROPIC_API_KEY": "sk-secret", "GH_TOKEN": "ghs_secret"}
RUN = "http://run"
STATUS = cg.Posted(99, False)


@dataclass
class FakeGitHub:
    threads: dict[str, dict] = field(default_factory=dict)
    fail_on: set[str] = field(default_factory=set)
    posted: list[tuple[str, tuple]] = field(default_factory=list)
    resolved: list[str] = field(default_factory=list)
    edits: list[tuple[cg.Posted, str]] = field(default_factory=list)
    head_fetches: int = 0

    async def pr_diff(self, pr: int) -> str:
        return f"diff {pr}"

    async def pr_view(self, pr: int) -> str:
        return f"pr {pr}"

    async def pr_meta(self, pr: int) -> str:
        return f"meta {pr}"

    async def issue_view(self, number: int) -> str:
        return f"issue {number}"

    async def head_sha(self, pr: int) -> str:
        self.head_fetches += 1
        return "fetched"

    async def inline_comment(
        self, pr: int, path: str, line: int, body: str, sha: str
    ) -> None:
        if body in self.fail_on:
            raise RuntimeError("422 line outside the diff")
        self.posted.append(("inline", (pr, path, line, body, sha)))

    async def reply(self, pr: int, comment_id: int, body: str) -> cg.Posted:
        if body in self.fail_on:
            raise RuntimeError("422 line outside the diff")
        self.posted.append(("reply", (pr, comment_id, body)))
        return cg.Posted(len(self.posted), True)

    async def comment(self, number: int, body: str) -> cg.Posted:
        self.posted.append(("comment", (number, body)))
        return cg.Posted(len(self.posted), False)

    async def edit_comment(self, comment: cg.Posted, body: str) -> None:
        self.edits.append((comment, body))

    async def thread(self, thread_id: str) -> cg.Thread:
        return cg.parse_thread(self.threads[thread_id])

    async def resolve_thread(self, thread_id: str) -> None:
        self.resolved.append(thread_id)


def make_result(**kw: Any) -> ResultMessage:
    fields: dict[str, Any] = {
        "subtype": "success",
        "duration_ms": 2000,
        "duration_api_ms": 1000,
        "is_error": False,
        "num_turns": 3,
        "session_id": "s",
        "total_cost_usd": 0.5,
        "result": "done",
        "permission_denials": [],
    }
    return ResultMessage(**{**fields, **kw})


@dataclass
class FakeRunner:
    """Calls each (tool, args) through the job's handlers, then yields `result`."""

    calls: list[tuple[str, dict]]
    result: ResultMessage | None
    outputs: list[dict] = field(default_factory=list)

    async def __call__(self, job: Any) -> AsyncIterator[Message]:
        by_name = {t.name: t for t in job.tools}
        for name, args in self.calls:
            block = ToolUseBlock(id=name, name=f"mcp__gh__{name}", input=args)
            yield AssistantMessage(content=[block], model="m")
            self.outputs.append(await by_name[name].handler(args))
        if self.result:
            yield self.result


def tools_by_name(tools: list[SdkMcpTool]) -> dict[str, SdkMcpTool]:
    return {t.name: t for t in tools}


def make(gh: FakeGitHub, target: cg.Target = PR, name: str = "review"):
    rec = cg.Record(name, "opus")
    return cg.make_tools(gh, target, SECRETS, rec), rec


def call(tools: list[SdkMcpTool], name: str, args: dict) -> dict:
    return asyncio.run(tools_by_name(tools)[name].handler(args))  # ty: ignore[invalid-argument-type]


def text(out: dict) -> str:
    return out["content"][0]["text"]


def thread(author: str = "claude", repo: str = "o/r", number: int = 7) -> dict:
    return {
        "node": {
            "pullRequest": {"number": number, "repository": {"nameWithOwner": repo}},
            "comments": {"nodes": [{"author": {"login": author}}]},
        }
    }


def job(tools: list[SdkMcpTool] | None = None, **kw: Any) -> Any:
    fields: dict[str, Any] = {
        "name": "review",
        "prompt": "p",
        "model": "opus",
        "effort": "high",
        "append": None,
        "tools": tools or [],
        "builtin": ["Read", "Grep", "Glob"],
        "settings": ["user"],
        "record": cg.Record("review", "opus"),
    }
    return cg.Job(**{**fields, **kw})


def test_options_no_bash():
    t, _ = make(FakeGitHub())
    o = cg.options(job(t), {"type": "sdk", "name": "gh", "instance": None})  # ty: ignore[invalid-argument-type]
    assert o.tools == ["Read", "Grep", "Glob"]
    assert "Bash" not in o.allowed_tools
    assert "mcp__gh" in o.allowed_tools
    assert o.disallowed_tools == ["Read(//proc/**)"]
    assert o.setting_sources == ["user"]
    assert o.permission_mode == "dontAsk"
    assert o.max_turns == 120
    assert o.env == {"GH_TOKEN": "", "CLAUDE_CODE_DISABLE_BACKGROUND_TASKS": "1"}


def test_options_builtin_and_settings_per_job():
    o = cg.options(job(builtin=["Read", "Agent"], settings=["user", "project"]), {})  # ty: ignore[invalid-argument-type, missing-typed-dict-key]
    assert o.tools == ["Read", "Agent"]
    assert o.allowed_tools == ["Read", "Agent", "mcp__gh"]
    assert o.setting_sources == ["user", "project"]


def test_options_model_effort_and_append():
    o = cg.options(job(model="sonnet", effort="max", append="extra"), {})  # ty: ignore[invalid-argument-type, missing-typed-dict-key]
    assert (o.model, o.effort) == ("sonnet", "max")
    assert o.system_prompt == {
        "type": "preset",
        "preset": "claude_code",
        "append": "extra",
    }


def test_write_tools_record_actions_and_post_nothing():
    gh = FakeGitHub()
    t, rec = make(gh)
    calls = [
        ("inline_comment", {"path": "a.rs", "line": 3, "body": "i"}),
        ("reply", {"comment_id": 5, "body": "r"}),
        ("resolve_thread", {"thread_id": "T_1"}),
        ("comment", {"body": "s"}),
    ]
    for name, args in calls:
        assert text(call(t, name, args)) == "recorded"
    assert gh.posted == [] and gh.resolved == []
    assert cg.to_json(rec) == {
        "job": "review",
        "model": "opus",
        "summary": "s",
        "actions": [
            {"kind": "inline_comment", "path": "a.rs", "line": 3, "body": "i"},
            {"kind": "reply", "comment_id": 5, "body": "r"},
            {"kind": "resolve_thread", "thread_id": "T_1"},
        ],
        "footer": None,
        "error": None,
    }


def test_summary_last_comment_wins():
    t, rec = make(FakeGitHub())
    call(t, "comment", {"body": "first"})
    call(t, "comment", {"body": "last"})
    assert rec.summary == "last" and rec.actions == []


def test_tools_have_no_target_arg():
    t, _ = make(FakeGitHub())
    for tl in t:
        assert not {"pr", "number", "repo"} & set(tl.input_schema)


def test_pr_tools_vs_issue_tools():
    pr = tools_by_name(make(FakeGitHub())[0])
    issue = tools_by_name(make(FakeGitHub(), ISSUE)[0])
    assert {"pr_diff", "pr_view"} <= set(pr) and "issue_view" not in pr
    assert "issue_view" in issue and "pr_diff" not in issue


@pytest.mark.parametrize(
    "name,writes",
    [
        ("mention", {"comment"}),
        ("code-review", {"inline_comment", "reply", "comment"}),
        ("review", {"inline_comment", "reply", "comment", "resolve_thread"}),
    ],
)
def test_job_writes_pinned(name, writes):
    assert cg.JOB_WRITES[name] == writes
    names = set(tools_by_name(cg.select_tools(make(FakeGitHub())[0], name)))
    reads = set() if name == "review" else {"pr_diff", "pr_view"}
    assert names == reads | writes


def test_reads_return_text():
    t, _ = make(FakeGitHub())
    assert text(call(t, "pr_diff", {})) == "diff 7"
    assert text(call(t, "pr_view", {})) == "pr 7"


@pytest.mark.parametrize("secret", ["sk-secret", "ghs_secret"])
@pytest.mark.parametrize(
    "name,args",
    [
        ("inline_comment", {"path": "a", "line": 1}),
        ("reply", {"comment_id": 1}),
        ("comment", {}),
    ],
)
def test_secret_body_refused(name, args, secret):
    t, rec = make(FakeGitHub())
    with pytest.raises(ValueError, match="refusing"):
        call(t, name, {**args, "body": f"leak {secret} here"})
    assert rec.actions == [] and rec.summary is None


def test_load_secrets_api_key_optional(monkeypatch):
    monkeypatch.setenv("GH_TOKEN", "ghs_x")
    monkeypatch.delenv("ANTHROPIC_API_KEY", raising=False)
    assert cg.load_secrets() == {"GH_TOKEN": "ghs_x"}
    monkeypatch.setenv("ANTHROPIC_API_KEY", "sk-x")
    assert cg.load_secrets() == {"GH_TOKEN": "ghs_x", "ANTHROPIC_API_KEY": "sk-x"}


def test_load_secrets_requires_gh_token(monkeypatch):
    monkeypatch.delenv("GH_TOKEN", raising=False)
    with pytest.raises(KeyError):
        cg.load_secrets()


def test_empty_secret_never_blocks():
    cg.check_body("anything", {"EMPTY": ""})
    t, _ = make(FakeGitHub())
    assert "is_error" not in call(t, "comment", {"body": "anything"})


def test_resolve_bad_thread_id_refused():
    t, rec = make(FakeGitHub())
    with pytest.raises(ValueError, match="invalid thread_id"):
        call(t, "resolve_thread", {"thread_id": 'x") { y } #'})
    assert rec.actions == []


@pytest.mark.parametrize("line", [0, -1])
def test_inline_nonpositive_line_refused(line):
    t, rec = make(FakeGitHub())
    with pytest.raises(ValueError, match="positive"):
        call(t, "inline_comment", {"path": "a", "line": line, "body": "x"})
    assert rec.actions == []


def comment_event(**comment: Any) -> dict:
    return {
        "repository": {"full_name": "o/r"},
        "pull_request": {
            "number": 7,
            "title": "T",
            "body": "B",
            "user": {"login": "bob"},
        },
        "comment": {
            "id": 11,
            "body": "@claude hi",
            "user": {"login": "amy"},
            **comment,
        },
    }


ISSUE_COMMENT_EVENT = {
    "repository": {"full_name": "o/r"},
    "issue": {
        "number": 9,
        "title": "T",
        "body": None,
        "user": {"login": "bob"},
        "pull_request": {},
    },
    "comment": {"id": 1, "body": "@claude", "user": {"login": "amy"}},
}


def test_mention_review_comment_replies_to_thread_root():
    m = cg.mention_from_event(
        "pull_request_review_comment", comment_event(in_reply_to_id=4)
    )
    assert m.reply_to == 4 and m.target == PR
    m = cg.mention_from_event("pull_request_review_comment", comment_event())
    assert m.reply_to == 11


def test_mention_issue_comment_is_top_level():
    m = cg.mention_from_event("issue_comment", ISSUE_COMMENT_EVENT)
    assert m.reply_to is None and m.target == cg.Target("o/r", 9, True)
    assert m.context["author"] == "amy" and "body" not in m.context
    assert "number" not in m.context


def test_mention_issue_event_is_not_pr():
    event = {
        "repository": {"full_name": "o/r"},
        "issue": {
            "number": 9,
            "title": "@claude",
            "body": "b",
            "user": {"login": "bob"},
        },
    }
    m = cg.mention_from_event("issues", event)
    assert m.target == ISSUE and m.context["author"] == "bob"


def test_mention_review_event_uses_review_author():
    event = {
        "repository": {"full_name": "o/r"},
        "pull_request": {
            "number": 7,
            "title": "T",
            "body": "B",
            "user": {"login": "bob"},
        },
        "review": {"body": "@claude look", "user": {"login": "amy"}},
    }
    m = cg.mention_from_event("pull_request_review", event)
    assert m.context["author"] == "amy" and m.context["review_body"] == "@claude look"
    assert m.reply_to is None


def test_mention_prompt_embeds_event_as_plain_context():
    m = cg.mention_from_event("pull_request_review_comment", comment_event())
    prompt = cg.mention_prompt(m, "DOC")
    assert prompt.startswith("REPO: o/r\n\nDOC")
    assert '"comment_body": "@claude hi"' in prompt
    assert "untrusted" not in prompt


def test_code_review_prompt_is_one_slash_line():
    assert cg.code_review_prompt(PR, "xhigh") == "/code-review xhigh --comment 7"
    event = {
        "repository": {"full_name": "o/r"},
        "issue": {"number": 7, "pull_request": {}},
    }
    assert cg.target(event) == PR


def test_review_prompt_header_and_order():
    p = cg.review_prompt(PR, "abc123", "http://run", "FOCUS", "COMMON")
    assert p.splitlines()[:4] == [
        "REPO: o/r",
        "PR NUMBER: 7",
        "HEAD SHA: abc123",
        "RUN URL: http://run",
    ]
    assert p.index("FOCUS") < p.index("COMMON")


def test_focus_file_choice():
    assert cg.focus_path(False).name == "review-standard.md"
    assert cg.focus_path(True).name == "review-backport.md"


def test_target_from_pull_request_event():
    event = {"repository": {"full_name": "o/r"}, "pull_request": {"number": 7}}
    assert cg.target(event) == PR


def test_tool_line_has_no_bodies_or_newlines():
    line = cg.tool_line(
        "mcp__gh__inline_comment", {"path": "a.rs", "line": 3, "body": "secret\nbody"}
    )
    assert line == "tool inline_comment path=a.rs line=3 body=<11 chars>"
    assert "\n" not in cg.tool_line("Grep", {"pattern": "a\nb" * 100})


def run_job(tmp_path: Path, calls: list, result: ResultMessage | None, runner=None):
    t, rec = make(FakeGitHub())
    j = job(t, record=rec)
    path = tmp_path / "tmp" / "actions.json"
    runner = runner or FakeRunner(calls, result)
    return j, path, runner


def test_run_ok_logs_final_line_and_writes_record(caplog, tmp_path):
    caplog.set_level(logging.INFO, logger="claude-gh")
    j, path, runner = run_job(
        tmp_path,
        [("comment", {"body": "hi"})],
        make_result(permission_denials=[{"t": 1}]),
    )
    assert asyncio.run(cg.run(j, runner, path)) == 0
    messages = [r.getMessage() for r in caplog.records]
    assert messages[0] == "tool comment body=<2 chars>"
    assert messages[1] == "done"
    assert messages[-1] == (
        'total_cost_usd=0.5 turns=3 duration_s=2 permission_denials=[{"t": 1}]'
    )
    assert caplog.records[-1].levelno == logging.INFO
    assert json.loads(path.read_text()) == {
        "job": "review",
        "model": "opus",
        "summary": "hi",
        "actions": [],
        "footer": cg.footer(make_result(), "opus"),
        "error": None,
    }


def test_run_error_records_subtype_and_partial_actions(tmp_path):
    j, path, runner = run_job(
        tmp_path,
        [("reply", {"comment_id": 1, "body": "r"})],
        make_result(is_error=True, subtype="error_max_turns"),
    )
    assert asyncio.run(cg.run(j, runner, path)) == 1
    record = cg.parse_record(json.loads(path.read_text()), "review")
    assert record.actions == [cg.Reply(1, "r")]
    assert record.error == "error_max_turns"


def test_run_missing_result_fails_and_records(tmp_path):
    j, path, runner = run_job(tmp_path, [("comment", {"body": "s"})], None)
    assert asyncio.run(cg.run(j, runner, path)) == 1
    record = cg.parse_record(json.loads(path.read_text()), "review")
    assert record.footer is None and record.summary == "s"


def test_run_raising_runner_still_writes_record(tmp_path):
    class Boom:
        async def __call__(self, job: Any) -> AsyncIterator[Message]:
            raise RuntimeError("boom")
            yield

    j, path, _ = run_job(tmp_path, [], None)
    j.record.summary = "partial"
    with pytest.raises(RuntimeError, match="boom"):
        asyncio.run(cg.run(j, Boom(), path))
    assert cg.parse_record(json.loads(path.read_text()), "review").summary == "partial"


def test_run_is_error_fails():
    lines, code = cg.result_lines(make_result(is_error=True, subtype="error_max_turns"))
    assert code == 1
    assert "subtype=error_max_turns" in "\n".join(lines)
    assert lines[-1].startswith("total_cost_usd=")


def test_cost_none_prints():
    lines, code = cg.result_lines(make_result(total_cost_usd=None))
    assert (
        code == 0
        and lines[-1]
        == "total_cost_usd=None turns=3 duration_s=2 permission_denials=[]"
    )


def test_run_appends_step_summary(tmp_path):
    out = tmp_path / "summary.md"
    j, path, runner = run_job(tmp_path, [], make_result())
    asyncio.run(cg.run(j, runner, path, out))
    assert out.read_text() == (
        "- claude-gh review: total_cost_usd=0.5 turns=3 duration_s=2 "
        "permission_denials=[]\n"
    )


def test_parse_args():
    args = cg.parse_args(["run", "code-review", "--model", "m", "--effort", "max"])
    assert (args.job, args.model, args.effort) == ("code-review", "m", "max")
    args = cg.parse_args(["run", "review"])
    assert (args.model, args.effort) == ("opus", "high")
    with pytest.raises(SystemExit):
        cg.parse_args(["run", "review", "--effort", "extreme"])
    args = cg.parse_args(
        [
            "apply",
            "mention",
            "--status-id",
            "5",
            "--run-result",
            "failure",
            "--record",
            "r",
        ]
    )
    assert args.status_id == 5 and args.record == Path("r")


def test_parse_thread():
    assert cg.parse_thread(thread()) == cg.Thread("claude", "o/r", 7)


@pytest.mark.parametrize("data", [{"node": {}}, {"node": None}, {"node": {"id": "x"}}])
def test_parse_thread_rejects_wrong_node(data):
    with pytest.raises(ValueError, match="not a review thread"):
        cg.parse_thread(data)


def test_prefetch_writes_pr_files(tmp_path):
    out = tmp_path / "tmp"
    asyncio.run(cg.prefetch(FakeGitHub(), 7, out))
    assert (out / "pr.json").read_text() == "meta 7"
    assert (out / "pr.diff").read_text() == "diff 7"


def test_meta_json_adds_refs():
    user = SimpleNamespace(login="amy")
    pr = SimpleNamespace(
        title="T",
        body="B",
        user=user,
        state="open",
        base=SimpleNamespace(ref="main"),
        head=SimpleNamespace(ref="feat"),
    )
    assert json.loads(cg.meta_json(pr)) == {
        "title": "T",
        "body": "B",
        "author": "amy",
        "state": "open",
        "base": "main",
        "head": "feat",
    }


def test_view_json_is_compact_and_has_no_comments():
    user = SimpleNamespace(login="amy")
    item = SimpleNamespace(title="T", body="B", user=user, state="open")
    out = cg.view_json(item)
    assert "\n" not in out
    assert json.loads(out) == {
        "title": "T",
        "body": "B",
        "author": "amy",
        "state": "open",
    }


def test_view_json_deleted_user_is_ghost():
    item = SimpleNamespace(title="T", body="B", user=None, state="open")
    assert json.loads(cg.view_json(item))["author"] == "ghost"


# Records and apply

FOOT = cg.footer(make_result(), "opus")


def record(
    actions: list | None = None,
    summary: str | None = "sum",
    job_name: str = "review",
    footer: str | None = FOOT,
    error: str | None = None,
) -> cg.Record:
    return cg.Record(job_name, "opus", actions or [], summary, footer, error)


def do_apply(
    gh: FakeGitHub,
    rec: cg.Record | str | None,
    job_name: str = "review",
    run_result: str = "success",
    tgt: cg.Target = PR,
    status: cg.Posted = STATUS,
    head_sha: str | None = "head",
) -> int:
    raw = json.dumps(cg.to_json(rec)) if isinstance(rec, cg.Record) else rec
    return asyncio.run(
        cg.apply(gh, job_name, tgt, raw, status, RUN, run_result, head_sha)
    )


ALL = [
    cg.InlineComment("a.rs", 3, "i"),
    cg.Reply(5, "r"),
    cg.Resolve("T_1"),
]


def test_record_json_roundtrip():
    rec = record(ALL, error="error_max_turns")
    assert cg.parse_record(cg.to_json(rec), "review") == rec
    rec = record(summary=None, footer=None)
    assert cg.parse_record(cg.to_json(rec), "review") == rec


def test_apply_posts_all_in_order_and_edits_status():
    gh = FakeGitHub(threads={"T_1": thread()})
    assert do_apply(gh, record(ALL)) == 0
    assert gh.posted == [
        ("inline", (7, "a.rs", 3, "i", "head")),
        ("reply", (7, 5, "r")),
    ]
    assert gh.resolved == ["T_1"]
    assert gh.edits == [(STATUS, f"sum\n\n{FOOT}")]
    assert gh.head_fetches == 0


def test_apply_fetches_head_once_without_event_sha():
    gh = FakeGitHub()
    actions = [cg.InlineComment("a.rs", 1, "x"), cg.InlineComment("a.rs", 2, "y")]
    assert do_apply(gh, record(actions), head_sha=None) == 0
    assert gh.head_fetches == 1
    assert [p[1][4] for p in gh.posted] == ["fetched", "fetched"]
    do_apply(gh, record([cg.Reply(1, "r")]), head_sha=None)
    assert gh.head_fetches == 1


@pytest.mark.parametrize("author", ["claude", "github-actions"])
def test_resolve_own_thread(author):
    gh = FakeGitHub(threads={"T_1": thread(author)})
    assert do_apply(gh, record([cg.Resolve("T_1")])) == 0
    assert gh.resolved == ["T_1"]


def test_apply_without_summary_says_so():
    gh = FakeGitHub()
    do_apply(gh, record(summary=None))
    assert gh.edits == [(STATUS, f"Review finished without a summary\n\n{FOOT}")]


@pytest.mark.parametrize(
    "bad",
    [
        "{",
        "[]",
        json.dumps({"job": "review"}),
        json.dumps({**cg.to_json(record()), "extra": 1}),
        json.dumps({**cg.to_json(record()), "actions": [{"kind": "comment"}]}),
        json.dumps({**cg.to_json(record()), "actions": [{"kind": "reply", "x": 1}]}),
        json.dumps({**cg.to_json(record()), "actions": [{"comment_id": 1}]}),
    ],
)
def test_unusable_record_refused_whole(bad):
    gh = FakeGitHub()
    with pytest.raises((ValueError, TypeError, KeyError)):
        do_apply(gh, bad)
    assert gh.posted == []
    assert gh.edits == [(STATUS, "Review failed: invalid actions. [run](http://run)")]


def test_job_mismatch_refused_whole():
    gh = FakeGitHub()
    with pytest.raises(ValueError, match="expected 'review'"):
        do_apply(gh, record(job_name="code-review"), job_name="review")
    assert gh.posted == [] and "failed" in gh.edits[-1][1]


@pytest.mark.parametrize(
    "name,action,line",
    [
        ("code-review", cg.Resolve("T_1"), "resolve_thread T_1: resolve_thread not"),
        ("mention", cg.Reply(1, "x"), "reply 1: reply not allowed in mention"),
    ],
)
def test_kind_not_allowed_is_skipped(name, action, line):
    gh = FakeGitHub(threads={"T_1": thread()})
    assert do_apply(gh, record([action], job_name=name), job_name=name) == 1
    assert gh.posted == [] and gh.resolved == []
    assert f"**Not applied**\n- {line}" in gh.edits[-1][1]


@pytest.mark.parametrize(
    "other,match",
    [
        (thread("alice"), "alice"),
        (thread(number=8), "belongs to"),
        (thread(repo="o/x"), "belongs to"),
        ({"node": {}}, "not a review thread"),
    ],
)
def test_bad_thread_is_skipped(other, match):
    gh = FakeGitHub(threads={"T_1": other})
    assert do_apply(gh, record([cg.Resolve("T_1")])) == 1
    assert gh.resolved == []
    assert match in gh.edits[-1][1] and "**Not applied**" in gh.edits[-1][1]


@pytest.mark.parametrize(
    "action",
    [
        cg.InlineComment("a.rs", 0, "x"),
        cg.Reply(0, "x"),
        cg.Resolve('x") { y } #'),
    ],
)
def test_invalid_action_is_skipped(action):
    gh = FakeGitHub()
    assert do_apply(gh, record([action])) == 1
    assert gh.posted == [] and gh.resolved == []


def test_github_error_skips_only_that_action():
    gh = FakeGitHub(fail_on={"bad"})
    actions = [
        cg.Reply(1, "first"),
        cg.InlineComment("a.rs", 9, "bad"),
        cg.Reply(2, "third"),
    ]
    assert do_apply(gh, record(actions)) == 1
    assert gh.posted == [("reply", (7, 1, "first")), ("reply", (7, 2, "third"))]
    assert gh.edits == [
        (
            STATUS,
            "sum\n\n**Not applied**\n"
            + "- inline_comment a.rs:9: 422 line outside the diff"
            + f"\n\n{FOOT}",
        )
    ]


@pytest.mark.parametrize("outcome", ["failure", "cancelled"])
def test_failed_run_posts_partial_record(outcome):
    gh = FakeGitHub()
    rec = record([cg.Reply(5, "r")], footer=None)
    assert do_apply(gh, rec, run_result=outcome) == 1
    assert gh.posted == [("reply", (7, 5, "r"))]
    assert gh.edits == [
        (STATUS, f"sum\n\nReview incomplete: {outcome}. [run](http://run)")
    ]


def test_error_subtype_is_the_incomplete_reason():
    gh = FakeGitHub()
    rec = record([cg.Reply(5, "r")], error="error_max_turns")
    assert do_apply(gh, rec, run_result="failure") == 1
    assert gh.posted == [("reply", (7, 5, "r"))]
    assert gh.edits == [
        (
            STATUS,
            f"sum\n\nReview incomplete: error_max_turns. [run](http://run)\n\n{FOOT}",
        )
    ]


def test_missing_record_marks_status_failed():
    gh = FakeGitHub()
    assert do_apply(gh, None, run_result="cancelled") == 1
    assert gh.edits == [(STATUS, "Review failed: cancelled. [run](http://run)")]
    assert do_apply(gh, None) == 1
    assert gh.edits[-1][1] == "Review failed: no actions file. [run](http://run)"
    assert gh.posted == []


def test_mention_status_uses_answer_wording():
    gh = FakeGitHub()
    status = cg.Posted(5, True)
    rec = record(job_name="mention", summary="answer")
    assert do_apply(gh, rec, "mention", tgt=ISSUE, status=status) == 0
    assert gh.edits == [(status, f"answer\n\n{FOOT}")]
    do_apply(gh, None, "mention")
    assert gh.edits[-1][1].startswith("Answer failed: no actions file")
    do_apply(gh, record(None, None, "mention"), "mention", run_result="failure")
    assert "Answer incomplete: failure" in gh.edits[-1][1]
    do_apply(gh, record(None, None, "mention"), "mention")
    assert "Answer finished without a summary" in gh.edits[-1][1]


def test_start_in_review_thread_replies_to_root():
    gh = FakeGitHub()
    posted = asyncio.run(
        cg.start(
            gh,
            "mention",
            "pull_request_review_comment",
            comment_event(in_reply_to_id=4),
            RUN,
        )
    )
    assert gh.posted == [("reply", (7, 4, f"Working on it… [run]({RUN})"))]
    assert posted.review is True


@pytest.mark.parametrize(
    "name,event", [("issue_comment", ISSUE_COMMENT_EVENT), ("issues", comment_event())]
)
def test_start_top_level_comments(name, event):
    gh = FakeGitHub()
    posted = asyncio.run(cg.start(gh, "mention", name, event, RUN))
    assert [k for k, _ in gh.posted] == ["comment"]
    assert posted.review is False


def test_start_review_names_head_and_code_review_is_generic():
    event = {
        "repository": {"full_name": "o/r"},
        "pull_request": {"number": 7, "head": {"sha": "abcdef123456"}},
    }
    gh = FakeGitHub()
    asyncio.run(cg.start(gh, "review", "pull_request", event, RUN))
    asyncio.run(cg.start(gh, "code-review", "pull_request", event, RUN))
    assert gh.posted == [
        ("comment", (7, "Reviewing abcdef1… [run](http://run)")),
        ("comment", (7, "Working on it… [run](http://run)")),
    ]


def test_start_prints_only_the_comment_id(capsys):
    event = ISSUE_COMMENT_EVENT
    gh = FakeGitHub()
    assert asyncio.run(cg.print_start(gh, "mention", "issue_comment", event, RUN)) == 0
    assert capsys.readouterr().out == "1\n"


def test_footer_formats_cost_and_duration():
    r = make_result(total_cost_usd=0.589, num_turns=27, duration_ms=133_000)
    assert cg.footer(r, "opus") == "<sub>Cost $0.59 · 27 turns · 2m 13s · opus</sub>"
    r = make_result(total_cost_usd=None, duration_ms=45_000)
    assert cg.footer(r, "opus") == "<sub>Cost n/a · 3 turns · 45s · opus</sub>"


DIFF = r"""diff --git a/old.rs b/new.rs
similarity index 90%
rename from old.rs
rename to new.rs
--- a/old.rs
+++ b/new.rs
@@ -1,3 +1,4 @@
 keep
-gone
+--- not a header
+++ not a header
 keep2
@@ -10 +11,2 @@
-x
+y
+z
\ No newline at end of file
diff --git a/fresh.rs b/fresh.rs
new file mode 100644
--- /dev/null
+++ b/fresh.rs
@@ -0,0 +1,2 @@
+a
+b
diff --git a/dead.rs b/dead.rs
deleted file mode 100644
--- a/dead.rs
+++ /dev/null
@@ -1,2 +0,0 @@
-a
-b
diff --git a/pure.rs b/moved.rs
similarity index 100%
rename from pure.rs
rename to moved.rs
"""


def test_diff_lines():
    assert cg.diff_lines(DIFF) == {
        "new.rs": {1, 2, 3, 4, 11, 12},
        "fresh.rs": {1, 2},
    }


def test_diff_lines_empty():
    assert cg.diff_lines("") == {}


def test_inline_comment_outside_diff_is_refused():
    lines = cg.diff_lines(DIFF)
    tools = cg.make_tools(FakeGitHub(), PR, SECRETS, cg.Record("review", "opus"), lines)
    with pytest.raises(ValueError, match="not a file"):
        call(tools, "inline_comment", {"path": "dead.rs", "line": 1, "body": "x"})
    with pytest.raises(ValueError, match="not inside a diff hunk"):
        call(tools, "inline_comment", {"path": "new.rs", "line": 5, "body": "x"})
    call(tools, "inline_comment", {"path": "new.rs", "line": 12, "body": "x"})


def failed(status: int, body: str) -> RequestFailed:
    response = SimpleNamespace(
        raw_request=None, raw_response=None, status_code=status, text=body
    )
    return RequestFailed(response)  # ty: ignore[invalid-argument-type]


def test_github_error_shows_message_and_details():
    body = json.dumps(
        {
            "message": "Unprocessable Entity",
            "errors": [
                {"field": "line", "message": "line must be part of the diff"},
                "oops",
            ],
        }
    )
    assert (
        cg.github_error(failed(422, body))
        == "422 Unprocessable Entity line must be part of the diff oops"
    )


def test_github_error_non_json_and_other_exceptions():
    assert cg.github_error(failed(502, "bad\n gateway")) == "502 bad gateway"
    assert cg.github_error(RuntimeError("a\nb")) == "a b"
