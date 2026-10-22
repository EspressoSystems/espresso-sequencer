"""Tests for `claude-gh`: tool policy, routing, prompts and exit codes, with fake GitHub and agent.

No network or SDK calls. Run with:

    just py::test test_claude_gh.py
"""

import asyncio
import importlib.util
import json
import logging
from collections.abc import AsyncIterator
from dataclasses import dataclass, field
from importlib.machinery import SourceFileLoader
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

SCRIPT = Path(__file__).with_name("claude-gh")
_spec = importlib.util.spec_from_loader(
    "claude_gh", SourceFileLoader("claude_gh", str(SCRIPT))
)
assert _spec is not None
cg = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(cg)

PR = cg.Target("o/r", 7, True)
ISSUE = cg.Target("o/r", 9, False)
SECRETS = {"ANTHROPIC_API_KEY": "sk-secret", "GH_TOKEN": "ghs_secret"}


@dataclass
class FakeGitHub:
    threads: dict[str, Any] = field(default_factory=dict)
    posted: list[tuple[str, tuple]] = field(default_factory=list)
    resolved: list[str] = field(default_factory=list)
    edits: list[tuple[cg.Posted, str]] = field(default_factory=list)

    def _posted(self, review: bool, url: str) -> cg.Posted:
        return cg.Posted(len(self.posted), url, review)

    async def pr_diff(self, pr: int) -> str:
        return f"diff {pr}"

    async def pr_view(self, pr: int) -> str:
        return f"pr {pr}"

    async def issue_view(self, number: int) -> str:
        return f"issue {number}"

    async def inline_comment(
        self, pr: int, path: str, line: int, body: str
    ) -> cg.Posted:
        self.posted.append(("inline", (pr, path, line, body)))
        return self._posted(True, f"url/inline/{pr}")

    async def reply(self, pr: int, comment_id: int, body: str) -> cg.Posted:
        self.posted.append(("reply", (pr, comment_id, body)))
        return self._posted(True, f"url/reply/{comment_id}")

    async def comment(self, number: int, body: str) -> cg.Posted:
        self.posted.append(("comment", (number, body)))
        return self._posted(False, f"url/comment/{number}")

    async def edit_comment(self, comment: cg.Posted, body: str) -> None:
        self.edits.append((comment, body))

    async def thread(self, thread_id: str) -> Any:
        return self.threads[thread_id]

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


def call(tools: list[SdkMcpTool], name: str, args: dict) -> dict:
    return asyncio.run(tools_by_name(tools)[name].handler(args))


def text(out: dict) -> str:
    return out["content"][0]["text"]


def thread(author: str = "claude", repo: str = "o/r", number: int = 7) -> Any:
    return cg.Thread(author, repo, number)


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
    }
    return cg.Job(**{**fields, **kw})


def test_options_no_bash():
    t = cg.make_tools(FakeGitHub(), PR, SECRETS)
    o = cg.options(job(t), {"type": "sdk", "name": "gh", "instance": None})
    assert o.tools == ["Read", "Grep", "Glob"]
    assert "Bash" not in o.allowed_tools
    assert "mcp__gh" in o.allowed_tools
    assert o.disallowed_tools == ["Read(//proc/**)"]
    assert o.setting_sources == ["user"]
    assert o.permission_mode == "dontAsk"
    assert o.max_turns == 120
    assert o.env == {"GH_TOKEN": "", "CLAUDE_CODE_DISABLE_BACKGROUND_TASKS": "1"}


def test_options_builtin_and_settings_per_job():
    o = cg.options(job(builtin=["Read", "Agent"], settings=["user", "project"]), {})
    assert o.tools == ["Read", "Agent"]
    assert o.allowed_tools == ["Read", "Agent", "mcp__gh"]
    assert o.setting_sources == ["user", "project"]


def test_options_model_effort_and_append():
    o = cg.options(job(model="sonnet", effort="max", append="extra"), {})
    assert (o.model, o.effort) == ("sonnet", "max")
    assert o.system_prompt == {
        "type": "preset",
        "preset": "claude_code",
        "append": "extra",
    }


def test_tools_have_no_target_arg_and_hit_bound_target():
    gh = FakeGitHub()
    t = cg.make_tools(gh, PR, SECRETS)
    for tl in t:
        assert not {"pr", "number", "repo"} & set(tl.input_schema)
    call(t, "inline_comment", {"path": "a.rs", "line": 3, "body": "x"})
    call(t, "reply", {"comment_id": 5, "body": "x"})
    call(t, "comment", {"body": "x"})
    assert [(k, v[0]) for k, v in gh.posted] == [
        ("inline", 7),
        ("reply", 7),
        ("comment", 7),
    ]


def test_pr_tools_vs_issue_tools():
    pr = tools_by_name(cg.make_tools(FakeGitHub(), PR, SECRETS))
    issue = tools_by_name(cg.make_tools(FakeGitHub(), ISSUE, SECRETS))
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
    names = set(
        tools_by_name(cg.select_tools(cg.make_tools(FakeGitHub(), PR, SECRETS), name))
    )
    assert names == {"pr_diff", "pr_view"} | writes


def test_reads_return_text():
    t = cg.make_tools(FakeGitHub(), PR, SECRETS)
    assert text(call(t, "pr_diff", {})) == "diff 7"
    assert text(call(t, "pr_view", {})) == "pr 7"


def test_write_returns_url():
    t = cg.make_tools(FakeGitHub(), PR, SECRETS)
    assert text(call(t, "comment", {"body": "hi"})) == "url/comment/7"


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
    gh = FakeGitHub()
    t = cg.make_tools(gh, PR, SECRETS)
    with pytest.raises(ValueError, match="refusing"):
        call(t, name, {**args, "body": f"leak {secret} here"})
    assert gh.posted == []


def test_empty_secret_never_blocks():
    cg.check_body("anything", {"EMPTY": ""})
    gh = FakeGitHub()
    t = cg.make_tools(gh, PR, SECRETS)
    assert "is_error" not in call(t, "comment", {"body": "anything"})


@pytest.mark.parametrize("author", ["claude", "github-actions"])
def test_resolve_own_thread(author):
    gh = FakeGitHub(threads={"T_1": thread(author)})
    t = cg.make_tools(gh, PR, SECRETS)
    assert "is_error" not in call(t, "resolve_thread", {"thread_id": "T_1"})
    assert gh.resolved == ["T_1"]


def test_resolve_others_thread_refused():
    gh = FakeGitHub(threads={"T_1": thread("alice")})
    t = cg.make_tools(gh, PR, SECRETS)
    with pytest.raises(ValueError, match="alice"):
        call(t, "resolve_thread", {"thread_id": "T_1"})
    assert gh.resolved == []


@pytest.mark.parametrize("other", [thread(number=8), thread(repo="o/x")])
def test_resolve_thread_of_other_pr_refused(other):
    gh = FakeGitHub(threads={"T_1": other})
    t = cg.make_tools(gh, PR, SECRETS)
    with pytest.raises(ValueError, match="belongs to"):
        call(t, "resolve_thread", {"thread_id": "T_1"})
    assert gh.resolved == []


def test_resolve_bad_thread_id_refused():
    gh = FakeGitHub()
    t = cg.make_tools(gh, PR, SECRETS)
    with pytest.raises(ValueError, match="invalid thread_id"):
        call(t, "resolve_thread", {"thread_id": 'x") { y } #'})


@pytest.mark.parametrize("line", [0, -1])
def test_inline_nonpositive_line_refused(line):
    gh = FakeGitHub()
    t = cg.make_tools(gh, PR, SECRETS)
    with pytest.raises(ValueError, match="positive"):
        call(t, "inline_comment", {"path": "a", "line": line, "body": "x"})
    assert gh.posted == []


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


def test_mention_review_comment_replies_to_thread_root():
    m = cg.mention_from_event(
        "pull_request_review_comment", comment_event(in_reply_to_id=4)
    )
    assert m.reply_to == 4 and m.target == PR
    m = cg.mention_from_event("pull_request_review_comment", comment_event())
    assert m.reply_to == 11


def test_mention_review_comment_routes_through_reply():
    gh = FakeGitHub()
    m = cg.mention_from_event(
        "pull_request_review_comment", comment_event(in_reply_to_id=4)
    )
    t = cg.make_tools(gh, m.target, SECRETS, m.reply_to)
    call(t, "comment", {"body": "answer"})
    assert gh.posted == [("reply", (7, 4, "answer"))]


def test_mention_issue_comment_is_top_level():
    event = {
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
    m = cg.mention_from_event("issue_comment", event)
    assert m.reply_to is None and m.target == cg.Target("o/r", 9, True)
    assert m.context["author"] == "amy" and "body" not in m.context
    assert "number" not in m.context
    gh = FakeGitHub()
    t = cg.make_tools(gh, m.target, SECRETS, m.reply_to)
    call(t, "comment", {"body": "answer"})
    assert gh.posted == [("comment", (9, "answer"))]


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


def test_run_ok_logs_final_line(caplog):
    caplog.set_level(logging.INFO, logger="claude-gh")
    runner = FakeRunner(
        [("comment", {"body": "hi"})], make_result(permission_denials=[{"t": 1}])
    )
    gh = FakeGitHub()
    t = cg.make_tools(gh, PR, SECRETS)
    assert asyncio.run(cg.run(job(t), runner, gh)) == 0
    messages = [r.getMessage() for r in caplog.records]
    assert messages[0] == "tool comment body=<2 chars>"
    assert messages[1] == "done"
    assert messages[-1] == 'total_cost_usd=0.5 turns=3 permission_denials=[{"t": 1}]'
    assert caplog.records[-1].levelno == logging.INFO


def test_run_is_error_fails():
    lines, code = cg.summary(make_result(is_error=True, subtype="error_max_turns"))
    assert code == 1
    assert "subtype=error_max_turns" in "\n".join(lines)
    assert lines[-1].startswith("total_cost_usd=")


def test_run_missing_result_fails():
    assert asyncio.run(cg.run(job(), FakeRunner([], None), FakeGitHub())) == 1


def test_cost_none_prints():
    lines, code = cg.summary(make_result(total_cost_usd=None))
    assert (
        code == 0 and lines[-1] == "total_cost_usd=None turns=3 permission_denials=[]"
    )


def test_run_appends_step_summary(tmp_path):
    out = tmp_path / "summary.md"
    asyncio.run(
        cg.run(job(name="mention"), FakeRunner([], make_result()), FakeGitHub(), out)
    )
    assert out.read_text() == "- claude-gh mention: cost 0.5 USD, 3 turns, 2 s\n"


def test_dry_run_posts_nothing(caplog):
    caplog.set_level(logging.INFO, logger="claude-gh")
    inner = FakeGitHub(threads={"T_1": thread()})
    dry = cg.DryRun(inner)
    calls = [
        ("pr_diff", {}),
        ("inline_comment", {"path": "a", "line": 2, "body": "one"}),
        ("comment", {"body": "two"}),
        ("resolve_thread", {"thread_id": "T_1"}),
    ]
    runner = FakeRunner(calls, make_result())
    assert asyncio.run(cg.run(job(cg.make_tools(dry, PR, SECRETS)), runner, dry)) == 0
    assert inner.posted == [] and inner.resolved == []
    assert [k for k, _ in dry.posted] == ["inline", "comment", "resolve"]
    assert runner.outputs[0]["content"][0]["text"] == "diff 7"
    assert "would post comment 7: 'two'" in caplog.messages


def test_parse_args_effort_choices():
    assert (
        cg.parse_args(["code-review", "--model", "m", "--effort", "max"]).effort
        == "max"
    )
    with pytest.raises(SystemExit):
        cg.parse_args(["code-review", "--model", "m", "--effort", "extreme"])


def test_footer_format():
    j = job()
    r = make_result(total_cost_usd=0.589, num_turns=27, duration_ms=133_000)
    assert cg.footer(j, r) == "<sub>Cost $0.59 · 27 turns · 2m 13s · opus</sub>"
    r = make_result(total_cost_usd=None, duration_ms=45_000)
    assert cg.footer(j, r) == "<sub>Cost n/a · 3 turns · 45s · opus</sub>"


def test_parse_thread():
    data = {
        "node": {
            "pullRequest": {"number": 7, "repository": {"nameWithOwner": "o/r"}},
            "comments": {"nodes": [{"author": {"login": "claude"}}]},
        }
    }
    assert cg.parse_thread(data) == cg.Thread("claude", "o/r", 7)


def test_view_json_is_compact_and_complete():
    user = SimpleNamespace(login="amy")
    item = SimpleNamespace(title="T", body="B", user=user, state="open")
    comment = SimpleNamespace(user=user, body="hi", created_at="2026-01-01")
    out = cg.view_json(item, [comment])
    assert "\n" not in out
    assert json.loads(out) == {
        "title": "T",
        "body": "B",
        "author": "amy",
        "state": "open",
        "comments": [{"author": "amy", "body": "hi", "created_at": "2026-01-01"}],
    }


@pytest.mark.parametrize("is_error", [False, True])
def test_last_comment_gets_footer(is_error):
    gh = FakeGitHub()
    posted: list = []
    t = cg.make_tools(gh, PR, SECRETS, posted=posted)
    calls = [
        ("comment", {"body": "first"}),
        ("inline_comment", {"path": "a", "line": 1, "body": "inline"}),
        ("comment", {"body": "last"}),
    ]
    runner = FakeRunner(calls, make_result(is_error=is_error))
    asyncio.run(cg.run(job(t, posted=posted), runner, gh))
    [(comment, body)] = gh.edits
    assert comment == cg.Posted(3, "url/comment/7", False)
    assert body == f"last\n\n{cg.footer(job(), make_result(is_error=is_error))}"


def test_thread_reply_edit_keeps_review_kind():
    gh = FakeGitHub()
    posted: list = []
    t = cg.make_tools(gh, PR, SECRETS, reply_to=4, posted=posted)
    runner = FakeRunner([("comment", {"body": "answer"})], make_result())
    asyncio.run(cg.run(job(t, posted=posted), runner, gh))
    assert gh.edits[0][0].review is True


def test_no_footer_without_comment_or_result():
    gh = FakeGitHub()
    asyncio.run(cg.run(job(), FakeRunner([], make_result()), gh))
    posted: list = []
    t = cg.make_tools(gh, PR, SECRETS, posted=posted)
    runner = FakeRunner([("comment", {"body": "x"})], None)
    asyncio.run(cg.run(job(t, posted=posted), runner, gh))
    assert gh.edits == []


def test_dry_run_prints_edit_instead_of_writing(caplog):
    caplog.set_level(logging.INFO, logger="claude-gh")
    inner = FakeGitHub()
    dry = cg.DryRun(inner)
    posted: list = []
    t = cg.make_tools(dry, PR, SECRETS, posted=posted)
    runner = FakeRunner([("comment", {"body": "x"})], make_result())
    asyncio.run(cg.run(job(t, posted=posted), runner, dry))
    assert inner.edits == [] and len(dry.edits) == 1
    assert dry.edits[0][1].startswith("x\n\n<sub>Cost $0.50")
    assert any(m.startswith("would edit comment") for m in caplog.messages)


RUN = "http://run"


def review_job(gh: FakeGitHub, calls: list, result: ResultMessage | None, runner=None):
    status = asyncio.run(gh.comment(7, cg.reviewing_text("abcdef123456", RUN)))
    posted: list = []
    t = cg.make_tools(gh, PR, SECRETS, posted=posted, status=status)
    j = job(t, posted=posted, status=status, run_url=RUN)
    return j, status, runner or FakeRunner(calls, result)


def test_status_text_builders():
    assert (
        cg.reviewing_text("abcdef123456", RUN) == "Reviewing abcdef1… [run](http://run)"
    )
    j = job(run_url=RUN)
    foot = cg.footer(j, make_result())
    assert cg.status_body(j, "sum", make_result()) == f"sum\n\n{foot}"
    assert cg.status_body(j, None, make_result()) == (
        f"Review finished without a summary\n\n{foot}"
    )
    err = make_result(is_error=True, subtype="error_max_turns")
    assert cg.status_body(j, "sum", err) == (
        f"sum\n\nReview failed: error_max_turns. [run](http://run)\n\n"
        f"{cg.footer(j, err)}"
    )
    assert (
        cg.status_body(j, None, None) == "Review failed: no result. [run](http://run)"
    )


def test_review_summary_replaces_status_with_footer():
    gh = FakeGitHub()
    j, status, runner = review_job(
        gh,
        [("comment", {"body": "first"}), ("comment", {"body": "last"})],
        make_result(),
    )
    asyncio.run(cg.run(j, runner, gh))
    assert [k for k, _ in gh.posted] == ["comment"]
    assert [(c, b) for c, b in gh.edits] == [
        (status, "first"),
        (status, "last"),
        (status, f"last\n\n{cg.footer(j, make_result())}"),
    ]


def test_review_without_summary():
    gh = FakeGitHub()
    j, status, runner = review_job(gh, [], make_result())
    asyncio.run(cg.run(j, runner, gh))
    assert gh.edits == [
        (status, f"Review finished without a summary\n\n{cg.footer(j, make_result())}")
    ]


@pytest.mark.parametrize("result", [make_result(is_error=True, subtype="boom"), None])
def test_review_failure_keeps_summary(result):
    gh = FakeGitHub()
    j, status, runner = review_job(gh, [("comment", {"body": "sum"})], result)
    asyncio.run(cg.run(j, runner, gh))
    assert gh.edits[-1] == (status, cg.status_body(j, "sum", result))
    assert "Review failed: " in gh.edits[-1][1]


def test_review_raising_runner_updates_status_then_raises():
    class Boom:
        async def __call__(self, job: Any) -> AsyncIterator[Message]:
            raise RuntimeError("boom")
            yield

    gh = FakeGitHub()
    j, status, runner = review_job(gh, [], None, Boom())
    with pytest.raises(RuntimeError, match="boom"):
        asyncio.run(cg.run(j, runner, gh))
    assert gh.edits == [(status, "Review failed: no result. [run](http://run)")]


def test_review_dry_run_status_goes_through_dry_run():
    inner = FakeGitHub()
    dry = cg.DryRun(inner)
    j, _, runner = review_job(dry, [("comment", {"body": "sum"})], make_result())
    asyncio.run(cg.run(j, runner, dry))
    assert inner.posted == [] and inner.edits == []
    assert len(dry.posted) == 1 and len(dry.edits) == 2
