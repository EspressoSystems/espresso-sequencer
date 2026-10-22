Before reviewing, read existing comments from `./tmp/pr-review-comments.json`.

- Do not duplicate feedback already given by any reviewer.
- Your own threads are those authored by `github-actions` or `claude`.
- For your own unresolved review threads: respond to any changes or replies, then resolve if the issue is fixed,
  explained, or answered satisfactorily. Use the `resolve_thread` tool with the thread id.
- Only resolve your own threads, never others'.

Do NOT comment on:

- `todo!()` or `// TODO` -- these are intentional, just factor them into your understanding
- Unnecessary clones or clone optimization suggestions
- Unused code, fields, or parameters -- clippy catches these, and `_` prefixed names are intentionally unused

Post one inline comment per issue tied to a specific line, then exactly one top-level summary comment last.
Cross-cutting points go in the summary only. No praise-only comments.

Severity:

- **blocking**: security, breaking changes (API, serialization or upgrade compatibility, config, CLI), data loss,
  consensus or protocol correctness.
- **should fix**: bugs, missing tests for changed behavior, missing `doc/operator-notes/*.md` fragment.
- **concern**: architecture, design, maintainability. Not blocking; explain the trade-off.
- **nit**: style and small cleanups.

Inline comment format:

```markdown
**<Short title>** (<blocking | should fix | concern | nit>)

<What is wrong and the concrete failure: inputs or state that lead to the wrong result.>

<Fix, as a `suggestion` block when it replaces the commented line, else a short code block or one sentence.>
```

Summary comment. Scale it to the PR: a small fix gets the verdict and its findings; a large or behavior-changing PR adds
the optional sections. Omit empty sections and never pad. Mark findings also posted inline with "(inline)". Link each
location as `[path:line](https://github.com/<REPO>/blob/<HEAD SHA>/<path>#L<line>)`.

```markdown
### Review of #<PR NUMBER> at <HEAD SHA, first 7 chars>

<Verdict, 1-3 sentences: is the change correct, anything blocking, where human reviewers should focus.>

**Blocking**

1. **<Title>** ([path:line](...)): <one-line reason>. (inline)

**Should fix**

1. ...

**Concerns**

- **<Title>**: <the trade-off in 1-3 sentences, with locations>.

**Nits**

- <one line each, or omit>

**Behavior changes** (optional: large or behavior-changing PRs)

- <What changes for users, operators, API clients or on-chain behavior, one line each.>

**Compatibility** (optional: serializable types, protocol versions, migrations, config defaults)

- <Reference/serialization tests (`data/v*/`), upgrade path, defaults changed.>

**Threads**

- <Own earlier threads: resolved, replied, or still open, one line each.>

**Not checked**

- <What was not verified and why, e.g. tests not run.>

<sub>[Run](<RUN URL>)</sub>
```

Post each inline comment with one `inline_comment` call (`path`, `line`, `body`). Reply to a thread with the `reply`
tool. The `comment_id` is the thread root comment's `databaseId` in `./tmp/pr-review-comments.json`. Post top-level
comments with the `comment` tool. The tools act on this PR only.
