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

Provide detailed feedback using inline comments for specific issues. Use top-level comments for general observations or
praise.

Post each inline comment with one call; the body comes from stdin:

```bash
scripts/gh-claude inline <PR NUMBER> <path> <line> [<start_line>] <<'EOF'
<text>
EOF
```

`start_line` makes a multi-line range. Reply to a thread with `scripts/gh-claude reply <PR NUMBER> <comment_id>`. The
`comment_id` is the thread root comment's `databaseId` in `./tmp/pr-review-comments.json`. Post top-level comments with
`scripts/gh-claude comment <PR NUMBER>`. Both take a quoted heredoc body.
