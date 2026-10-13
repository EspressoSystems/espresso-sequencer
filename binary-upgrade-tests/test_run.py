import pytest

import run

# main cut 0.6.3.0 and later 0.7.0.0; 0.6.3.11 was tagged after the 0.7 cut.
TAGS = {
    "20260814",
    "20260918",
    "0.6.3.0",
    "0.6.3.10",
    "0.6.3.11",
    "0.7.0.0",
    "0.7.0.1",
}
MAIN = {"20260814", "0.6.3.0", "0.7.0.0"}


@pytest.mark.parametrize(
    "reachable, containing, base",
    [
        pytest.param(MAIN, set(), "0.7.0.1", id="main"),
        pytest.param(MAIN, {"0.7.0.0", "0.7.0.1"}, "0.6.3.11", id="cut-tag-push"),
        pytest.param(
            {"20260814", "0.6.3.0", "0.6.3.10", "0.6.3.11"},
            {"0.6.3.11"},
            "0.6.3.10",
            id="patch-tag-push",
        ),
        pytest.param(
            {"20260814", "0.6.3.0", "0.6.3.10"}, set(), "0.6.3.11", id="release-branch"
        ),
        pytest.param({"20260814", "20260918"}, set(), "20260918", id="legacy-branch"),
    ],
)
def test_pick_base_tag(reachable: set[str], containing: set[str], base: str) -> None:
    assert run.pick_base_tag(TAGS, reachable, containing) == base


def test_pick_base_tag_skips_newer_cut() -> None:
    tags = {"0.6.3.0", "0.6.3.10", "0.7.0.0", "0.8.0.0"}
    assert run.pick_base_tag(tags, {"0.6.3.0", "0.7.0.0"}, set()) == "0.7.0.0"


@pytest.mark.parametrize(
    "reachable, containing",
    [
        pytest.param(set(), set(), id="shallow"),
        pytest.param({"0.7.0.0"}, {"0.7.0.0"}, id="only-head"),
    ],
)
def test_pick_base_tag_fails(reachable: set[str], containing: set[str]) -> None:
    with pytest.raises(RuntimeError):
        run.pick_base_tag({"0.7.0.0"}, reachable, containing)


def test_tag_order() -> None:
    tags = ["0.6.3.10", "20260918", "0.6.3.9", "0.7.0.0"]
    assert sorted(tags, key=run.tag_order) == [
        "20260918",
        "0.6.3.9",
        "0.6.3.10",
        "0.7.0.0",
    ]
