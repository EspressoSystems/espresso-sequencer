# PR review

Goal: find every real defect in the PR. Recall over precision.

- Steps 3-10 each collect up to 6 candidates. Do not judge them until Findings.
- Keep every candidate with a nameable failure scenario, including half-believed ones. Dropping those is the main cause
  of missed bugs.

## Inputs

- PR description: `pr_view`. Diff: `pr_diff`, also in `./tmp/pr.diff`. The diff is the scope.
- Checkout: the PR head. Read and Grep it for surrounding code, callers, tests.
- Earlier review comments: `./tmp/pr-review-comments.json`.

## Scope

Perform static review only; CI runs build, lint, format, test, serialization and API compatibility gates separately.
Review whether tests are adequate, not whether they pass.

## Procedure

1. Prior threads. Read `./tmp/pr-review-comments.json`.
   - Skip findings any reviewer already raised.
   - Own threads: author `claude`, or author `github-actions` with a root body starting with `<!-- claude-gh -->`. For
     each unresolved one: `reply` (comment_id = root comment `databaseId`) on what changed; `resolve_thread` if fixed,
     explained or answered. Never resolve others' threads.
2. Map the change: what it touches, what it claims to fix or add.
3. Hunks. For each hunk, Read the whole enclosing function. Per line: which input, state, ordering or concurrency makes
   it wrong: wrong conditions or arithmetic, panics reachable from external input, work that blocks or waits on
   something that is not running yet, tasks never stopped.
4. Removed behavior and parity.
   - For each deleted or replaced line: name the invariant it enforced; find where the new code re-establishes it. Not
     found: candidate.
   - If the PR adds a path for something an existing path already does (another protocol version, another sync route,
     another backend, a sibling handler): Read the existing path. List its side effects in order (writes, in-memory
     updates, sends, spawned tasks). For each, find the same effect in the new path at the same position relative to
     fallible calls. Missing, conditional, or moved after an early return: candidate.
5. Failure paths. For each early return or error branch between two side effects (storage write, in-memory update,
   network send, spawned task):
   - What state is left: written, half-written, cached in memory only?
   - Does any later path retry or repair it, or does an early return or cache hit skip it forever?
   - Is the operation idempotent when retried or replayed at startup?
6. Callers and callees. Grep every changed symbol. Per call site: new precondition, changed return or error, changed
   ordering or timing. Per callee changed in the same PR: still safe here?
   - For each new or changed condition, check or `ensure!`: find where its inputs come from and list every value they
     take, including `None`, defaults, and data written before an upgrade or by the legacy path. A real value that now
     fails or passes: candidate.
7. Invariants untested by CI:
   - Storage: everything written can be read back, by this code and by the release still running on other nodes. No test
     covers this round trip; check it by reading the write and read paths. Every storage backend gets the change.
   - Upgrades: behavior gated on the right protocol version; both sides of a transition handled.
   - Boundaries: first/last item of an epoch or range, state before a feature activates, empty or genesis state.
   - Operator action needed: an operator note is present.
   - Secrets: can anything secret (keys, tokens, passwords, credentials in URLs) leave the process or reach a less
     trusted reader through this change? Follow error rendering and struct dumps, not only explicit fields.
8. Design at the type level: would better types make this defect or this runtime check impossible?
9. Tests. Would any test fail without the change? Changed behavior without a test that pins it: candidate.
10. Cleanup, only with a concrete cost: code duplicating an existing helper (name it); derivable state; dead code;
    repeated I/O or work on a hot path; a special case where fixing the shared mechanism is simpler (name the change).

## Findings

- Call `candidate` the moment a possible defect is noticed in steps 3-10, before investigating further.
- Before posting, call `verdict` for every candidate: `wrong` only when quoted code shows the candidate misreads it;
  `duplicate` when another candidate covers it; otherwise `kept`.
- Low impact, "not a regression", "acceptable", "unlikely" or "probably fine" are not grounds to discard: keep the
  candidate and lower its severity to concern or nit.
- Post every kept candidate: max 10, most severe first, correctness before cleanup.
- Each: `path`, `line` (RIGHT side of the diff), one-line summary, failure scenario (inputs/state -> wrong result).
- Severity:
  - **blocking**: security, consensus or protocol correctness, data loss or state not persisted, breaking
    API/serialization/upgrade/config compatibility. A write the existing path performs and the new path skips or can
    skip is blocking, even if a restart or later catchup would repair it.
  - **should fix**: bugs, missing tests for changed behavior, missing operator note, a storage backend left out.
  - **concern**: design or architecture trade-off.
  - **nit**: small cleanup.
- Do not report: intentional TODOs, micro-optimizations, anything CI checks.

## Output

1. One `inline_comment` per finding:

   ```markdown
   **<severity>**: <title>

   <Failure scenario, 1-3 sentences.>

   <Fix: `suggestion` block if it replaces the line, else one sentence.>
   ```

2. Last, one `comment`:

   ```markdown
   <1-3 lines: what the PR does.>

   - **<severity>**: <few words> ([path:line](https://github.com/<repo>/blob/<head sha>/<path>#L<line>))

   Checked: <what was verified, e.g. callers of X, failure paths of Y, storage round trip>.
   ```

   No findings: `No issues found.` plus the `Checked:` line. No praise, no restating the diff.
