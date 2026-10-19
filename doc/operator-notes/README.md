# Operator notes

One file per change that requires operator action. The release process collects the files into the release body.

## When a fragment is needed

A PR needs a fragment when it changes any of:

- environment variables
- CLI flags
- config or genesis files
- database migrations
- ports
- storage layout
- restart requirements
- upgrade or activation timing

## Format

File: `doc/operator-notes/<short-topic>.md`, any name. Free markdown starting with a `### <title>` line.

```markdown
### <title>

Urgency: High | Medium | Low

What operators must do: env vars to set, flags to change, migration duration, restart order, deadline as view number and
UTC time.
```

## Rules

- Fragments must not contain `<!--`. The release commands fail naming the file.
- A fragment is added in the PR that needs it. Backports carry the file automatically.
- Fragments are never deleted by the release process.
- A release lists the fragments added since the previous tag on its line. `/publish` lists the fragments added since the
  network's previous release. A fix to a fragment is picked up by the next tag and its `/publish`.
- This file is not collected.
