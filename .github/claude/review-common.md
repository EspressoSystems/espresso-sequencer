Before reviewing, read `./tmp/pr.json` (title, body, author, state, base and head refs), `./tmp/pr.diff` and the
existing comments in `./tmp/pr-review-comments.json` with Read.

- Do not duplicate feedback already given by any reviewer.
- Your own threads are those authored by `github-actions` or `claude`.
- For your own unresolved review threads: respond to any changes or replies, then resolve if the issue is fixed,
  explained, or answered satisfactorily. Use the `resolve_thread` tool with the thread id.
- Only resolve your own threads, never others'.

Do NOT comment on:

- `todo!()` or `// TODO` -- these are intentional, just factor them into your understanding
- Unnecessary clones or clone optimization suggestions
- Unused code, fields, or parameters -- clippy catches these, and `_` prefixed names are intentionally unused

Be terse. No praise, no restating the diff.

Severity labels, always the icon followed by its word: ❌ blocking (security, breaking API/serialization/upgrade/config
compatibility, data loss, consensus correctness), ⚠️ should fix (bugs, missing tests for changed behavior, missing
`doc/operator-notes/*.md` fragment), 💭 concern (architecture, design), 🔹 nit.

Inline comment, one per issue tied to a line:

```markdown
⚠️ should fix: **<Short title>**

<1-2 sentences: the concrete failure.>

<Fix: `suggestion` block when it replaces the line, else one sentence or a short code block.>
```

Then exactly one summary with the `comment` tool, last:

```markdown
### Review of #<PR NUMBER> at <HEAD SHA, first 7 chars>

✅ Security · ✅ Compatibility · ⚠️ Tests · ✅ Behavior

- ❌ blocking: <few words> ([path:line](https://github.com/<REPO>/blob/<HEAD SHA>/<path>#L<line>))
- 💭 concern: <few words, one extra sentence only if not posted inline> ([path:line](...))

Not checked: <only if something was not verified>.
```

- Checklist: ✅ fine, ⚠️ issue, ❌ blocking. Omit areas that do not apply.
- One line per finding, most severe first. No findings: write `No issues found.`

Post inline comments with `inline_comment` (`path`, `line`, `body`). Reply to a thread with `reply`; the `comment_id` is
the thread root comment's `databaseId` in `./tmp/pr-review-comments.json`. The tools act on this PR only.
