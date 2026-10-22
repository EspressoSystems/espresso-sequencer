# PR review

Goal: find every real defect in the PR. Recall over precision.

- Steps 3-10 each collect up to 6 candidates. Do not judge them until Findings.
- Keep every candidate with a nameable failure scenario, including half-believed ones. Dropping those is the main cause
  of missed bugs.

## Inputs

- PR description: `pr_view`. Diff: `pr_diff`, also in `./tmp/pr.diff`. The diff is the scope.
- Checkout: the PR head. Read and Grep it for surrounding code, callers, tests.
- Earlier review comments: `./tmp/pr-review-comments.json`.

## Procedure

1. Prior threads. Read `./tmp/pr-review-comments.json`.
   - Skip findings any reviewer already raised.
   - Own threads: author `github-actions` or `claude`. For each unresolved one: `reply` (comment_id = root comment
     `databaseId`) on what changed; `resolve_thread` if fixed, explained or answered. Never resolve others' threads.
2. Map the change. List changed functions, types, traits, SQL, config. Note what the PR claims to fix or add.
3. Hunks. For each hunk, Read the whole enclosing function. Per line: which input, state, ordering or concurrency makes
   it wrong. Check:
   - inverted or wrong conditions, off-by-one, wrong variable, wrong unit or epoch/height arithmetic
   - unchecked arithmetic, truncating casts, `unwrap`/`expect`/indexing reachable from external input
   - missing `.await`, lock held across `.await`, task spawned and never joined or aborted, channel send that blocks
4. Removed behavior and parity.
   - For each deleted or replaced line: name the invariant it enforced; find where the new code re-establishes it. Not
     found: candidate.
   - If the PR adds a path for something an existing path already does (legacy vs new protocol, decide vs catchup, one
     backend vs another, one handler vs its sibling): Read the existing path. List its side effects in order (writes,
     in-memory updates, sends, spawned tasks, metrics). For each, find the same effect in the new path at the same
     position relative to fallible calls. Missing, conditional, or moved after a `?`/early return: candidate.
5. Failure paths. For each `?`, early return, error arm and `continue` between two side effects (DB write, in-memory
   update, network send, spawned task):
   - What state is left: written, half-written, cached in memory only?
   - Does any later path retry or repair it, or does an early return or cache hit skip it forever?
   - Is the operation idempotent when retried or replayed at startup?
6. Callers and callees. Grep every changed symbol. Per call site: new precondition, changed return or error, changed
   ordering or timing. Per callee changed in the same PR: still safe here?
7. Repo invariants:
   - Persistence: every `SequencerPersistence`/`MembershipPersistence` change covers PostgreSQL, SQLite and filesystem.
   - Migrations: `V{n}__{name}.sql` in both postgres and sqlite dirs; schema-only, no work that scales with DB size.
   - Serializable types: changed layout needs updated `data/v*/` reference files or is a compatibility break.
   - Protocol versions and upgrades: behavior gated on the right version; both sides of a cutover handled.
   - Consensus, epochs, stake table, rewards: epoch-boundary and pre-epoch (epoch `None`) cases.
   - Operator action needed: `doc/operator-notes/*.md` fragment present.
8. Types and Rust idioms:
   - Raw `u64`/`[u8; 32]`/`String` for domain values where a newtype exists or belongs.
   - `bool` parameters or returns that should be two-variant enums; struct-of-`Option` that should be an enum.
   - States that the types allow but the logic forbids; runtime checks a typestate or smart constructor would remove.
   - Validation repeated downstream instead of parsing once at the boundary.
   - Errors: `thiserror` in libraries, `anyhow` in binaries; context lost (`anyhow!("{e}")`, `map_err(|_| ..)`); errors
     logged and dropped where callers need them.
   - Needless `clone`/`Arc`/`Box`, `&String`/`&Vec<T>` parameters, manual loops over iterator adapters, `match` that is
     `?` or `if let`.
9. Tests. Would any test fail without the change? Changed behavior without a test that pins it: candidate.
10. Cleanup, only with a concrete cost: code duplicating an existing helper (name it); derivable state; dead code;
    repeated I/O or work on a hot path; a special case where fixing the shared mechanism is simpler (name the change).

## Findings

- In steps 3-10, call `candidate` the moment a possible defect is noticed, before investigating it further.
- Before posting, call `verdict` for every candidate.
- Drop only with evidence quoted from the code. "Probably fine" or "unlikely" keeps the candidate.
- Post only kept candidates.
- Each: `path`, `line` (RIGHT side of the diff), one-line summary, failure scenario (inputs/state -> wrong result).
- Dedup same defect at same location. Sort by severity. Max 10; correctness before cleanup.
- Severity:
  - **blocking**: security, consensus or protocol correctness, data loss or state not persisted, breaking
    API/serialization/upgrade/config compatibility. A write the existing path performs and the new path skips or can
    skip is blocking, even if a restart or later catchup would repair it.
  - **should fix**: bugs, missing tests for changed behavior, missing operator note, missing storage backend.
  - **concern**: design or architecture trade-off.
  - **nit**: small cleanup.
- Do not report: `todo!()`/`// TODO`, clone micro-optimizations, unused items (clippy covers them).

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

   Checked: <what was verified, e.g. callers of X, failure paths of Y, all three backends>.
   ```

   No findings: `No issues found.` plus the `Checked:` line. No praise, no restating the diff.
