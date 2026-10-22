Before reviewing, read existing comments from `./tmp/pr-review-comments.json`.

- Do not duplicate feedback already given by any reviewer.
- Your own threads are those authored by `github-actions` or `claude`.
- For your own unresolved review threads: respond to any changes or replies, then resolve if the issue is fixed,
  explained, or answered satisfactorily. `scripts/gh-claude resolve <thread_id>`
- Only resolve your own threads, never others'.

Do NOT comment on:

- `todo!()` or `// TODO` -- these are intentional, just factor them into your understanding
- Unnecessary clones or clone optimization suggestions
- Unused code, fields, or parameters -- clippy catches these, and `_` prefixed names are intentionally unused

Post one inline comment per specific issue, then exactly one top-level summary comment last. No praise-only comments.

Inline comment format:

```markdown
**<Short title>** (<blocking | should fix | nit>)

<What is wrong and the concrete failure: inputs or state that lead to the wrong result.>

<Fix, as a `suggestion` block when it replaces the commented line, else a short code block or one sentence.>
```

Summary comment template (omit empty sections; link each location as
`[path:line](https://github.com/<REPO>/blob/<HEAD SHA>/<path>#L<line>)`):

```markdown
### Review of #<PR NUMBER> at <HEAD SHA, first 7 chars>

<Verdict, 1-3 sentences: is the change correct, what it does well if notable, anything blocking.>

**Inline comments** (<n>)

1. **<Title>** ([path:line](...)): <one-line reason>. <blocking | should fix | nit>

**Not posted inline**

- <Smaller or cross-cutting points, each one line with a location.>

**Threads**

- <Own earlier threads: resolved, replied, or still open, one line each.>

**Not checked**

- <What was not verified and why, e.g. tests not run.>

<sub>[Run](<RUN URL>)</sub>
```

Post each inline comment with one call; the body comes from stdin:

```bash
scripts/gh-claude inline <PR NUMBER> <path> <line> <<'EOF'
<text>
EOF
```

Reply to a thread with `scripts/gh-claude reply <PR NUMBER> <comment_id>`. The `comment_id` is the thread root comment's
`databaseId` in `./tmp/pr-review-comments.json`. Post top-level comments with `scripts/gh-claude comment <PR NUMBER>`.
Both take a quoted heredoc body.
