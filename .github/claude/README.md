# Claude on GitHub

## Files

| File                    | Purpose                                                                                                                                                                                                  |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude_gh.py`          | `claude-gh` CLI. Jobs `review`, `mention`, `code-review`; steps `start` (post status comment), `run` (Claude records actions in `tmp/actions.json`, posts nothing), `apply` (post the recorded actions). |
| `review.md`             | Runbook for the `review` job.                                                                                                                                                                            |
| `review-backport.md`    | Appended to `review.md` on backport PRs (branch `backport-<n>-to-*`).                                                                                                                                    |
| `mention.md`            | Prompt for `@claude` mentions.                                                                                                                                                                           |
| `code-review.md`        | Appended to the system prompt of the `/code-review` comment command.                                                                                                                                     |
| `backport-resolve.md`   | Prompt for resolving backport conflicts (`.github/workflows/backport.yml`).                                                                                                                              |
| `maintain-agents-md.md` | Prompt for the weekly AGENTS.md audit (`.github/workflows/claude-agents-md.yml`).                                                                                                                        |
| `eval/`                 | Offline evaluation of the `review` job on PRs with known bugs.                                                                                                                                           |
| `tests/`                | Unit tests for `claude_gh.py` and the eval's read-only GitHub. No network.                                                                                                                               |

Commands: `just claude::fmt`, `just claude::lint`, `just claude::check`, `just claude::test`.

## PR review in CI

`.github/workflows/claude-review.yml` runs on non-draft, non-fork PRs when opened, reopened or marked ready. `start`
posts a status comment with the write token. `run` checks out the PR head, fetches team review comments into
`tmp/pr-review-comments.json` (plus original-PR context for backports), and runs `claude-gh run review` with a read
token and `ANTHROPIC_API_KEY`; Claude reads the diff and checkout and records inline comments, candidates with verdicts,
and a summary. `apply` posts the record with the write token and edits the status comment with the summary and cost
footer.

## Evaluating review changes

`eval/eval.py` replays the `review` job on the PRs in `eval/cases.toml`, using the working tree's `claude_gh.py` and
`review.md`.

- Never posts or modifies anything on GitHub: only the `run` step executes, against a GitHub whose write methods raise.
- Needs no local state: all files go under `tmp/claude-eval/`, worktrees under `tmp/claude-eval/wt/<pr>`.

### Prerequisites

- GitHub read access: `GH_TOKEN` if set, else `gh auth token`, else unauthenticated (60 requests per hour, enough for a
  few PRs).
- Claude: `ANTHROPIC_API_KEY`, or an existing `claude` login.
- Optional `CLAUDE_CONFIG_DIR`: a login without personal settings and CLAUDE.md:
  `CLAUDE_CONFIG_DIR=$PWD/tmp/claude-config claude /login`, then `export CLAUDE_CONFIG_DIR=$PWD/tmp/claude-config`.
- Optional `CLAUDE_CLI_PATH`: path to a `claude` binary. Needed on NixOS, where the SDK's bundled CLI does not run.

### Commands

```sh
just claude::eval                       # all cases, opus
just claude::eval --model fable 4670    # one case, another model
just claude::eval --prepare-only 4456   # worktree and event only, no Claude run, no cost
just claude::eval --out tmp/claude-eval/my-run 4456
```

### Results

`tmp/claude-eval/<UTC time>-<model>/` (or `--out`, which must not exist):

- `<pr>/event.json`, `<pr>/log.txt`, `<pr>/exit`, `<pr>/actions.json` (the record).
- `report.md`: per case the expected bug, cost and duration, the summary's findings, wrong candidates with evidence, and
  unjudged candidates.

### Scoring

Per case in `report.md`, compare findings and wrong candidates with `Bug`:

- caught: a finding names the bug.
- partial: a finding points at the right code with the wrong failure, or the bug was a candidate judged wrong (check the
  evidence).
- missed: neither.

### Adding a case

Append a `[[case]]` to `eval/cases.toml`: `pr` (introduced the bug), `fixed_by` (fix PR numbers), `bug` (one sentence:
what a reviewer of `pr` should notice). Keep only PRs whose diff introduced the bug, confirmed from the fix PR.

### Cost

Per PR: $0.6-2.5 with opus, about 2x with fable.

### Known limitations

- Prior review comments are not replayed (empty `tmp/pr-review-comments.json`): thread handling (runbook step 1) and
  skipping existing feedback are not evaluated.
- The PR title and body stay visible to the reviewer and may describe the bug or a later fix (a PR that fixes one bug
  and introduces another).
- One run per PR is noisy; results vary between runs. Compare over several runs before concluding.
