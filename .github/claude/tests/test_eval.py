"""Tests for eval/eval.py: its GitHub never writes."""

import asyncio
import importlib.util
from pathlib import Path

import pytest

import claude_gh as cg

_spec = importlib.util.spec_from_file_location(
    "claude_eval", Path(__file__).parents[1] / "eval" / "eval.py"
)
assert _spec and _spec.loader
ev = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(ev)

WRITES = {
    "inline_comment": (7, "a.rs", 1, "body", "sha"),
    "reply": (7, 1, "body"),
    "comment": (7, "body"),
    "edit_comment": (cg.Posted(1, False), "body"),
    "resolve_thread": ("T_1",),
    "thread": ("T_1",),
}


class Untouchable:
    def __getattr__(self, name):
        raise AssertionError(f"inner GitHub called: {name}")


class Reads:
    async def pr_view(self, pr):
        return f"view {pr}"


@pytest.mark.parametrize("name", WRITES)
def test_writes_raise_without_touching_inner(name):
    gh = ev.ReadOnlyGitHub(Untouchable())
    with pytest.raises(ev.ReadOnly):
        asyncio.run(getattr(gh, name)(*WRITES[name]))


def test_every_protocol_write_is_covered():
    reads = {"pr_diff", "compare_diff", "pr_view", "issue_view", "head_sha"}
    methods = {m for m in vars(cg.GitHub) if not m.startswith("_")}
    assert methods - reads == set(WRITES)


def test_reads_delegate():
    assert asyncio.run(ev.ReadOnlyGitHub(Reads()).pr_view(7)) == "view 7"
