# Software Releases

Release tags have the form `MAJOR.MINOR.PHASE.PATCH`, for example `0.6.0.7`.

| Part          | Meaning                                                                                                         |
| ------------- | --------------------------------------------------------------------------------------------------------------- |
| `MAJOR.MINOR` | Protocol version the release branch implements (`0.6` for `V0_6`).                                              |
| `PHASE`       | Distinguishes release branches for the same protocol version: pre vs post activation, or a new cut from `main`. |
| `PATCH`       | Increments with every tag on the branch. `.0` marks the cut point.                                              |

Tags are immutable. A bad tag is superseded by the next one and never deployed.

Docker images are tagged with the git tag: git tag `0.6.0.7` produces
`ghcr.io/espressosystems/espresso-network/espresso-node:0.6.0.7`.

## Branches

- Release branches are named `release-MAJOR.MINOR.PHASE`, e.g. `release-0.6.0`, cut from `main` with `just release-cut`.
  All changes land via reviewed PR.
- Experimental branches `release-MAJOR.MINOR.PHASE--<topic>` (double dash) branch off a release branch for devnet
  validation. CI builds docker images for them. Release automation ignores them.
- Backports: add the label `backport release-MAJOR.MINOR.PHASE` to a PR on `main`. On merge, `backport.yml` opens a
  backport PR against the release branch and tries to resolve conflicts (PRs it touched carry the label
  `claude-resolved`). After the merge, comment `/backport 123` on the tracker instead. Manual backports use
  `git cherry-pick -x`.

## Tracker issue

Every release branch has one issue titled `Release MAJOR.MINOR.PHASE` with label `release-tracker`. The bot regenerates
its body on every push to `main` or `release-*`, when a PR against a release branch opens or closes, after every `/tag`
or cut, and on tracker commands. Sections:

- Deployed line: the release each network runs, `Deployed: mainnet <tag> · decaf <tag>`, from the release titles.
- Tag log: tags on the branch with date, commit, GitHub release state (pre-release or release, with the networks it is
  published on), operator notes (✅ stated, ⬜ missing) and build runs.
- Backports from `main` to the branch: commits on `main` since the `.0` tag. A row shows ✅ when the commit is on the
  branch: its backport PR (head `backport-<PR>-to-<branch>`) merged, `git cherry` finds the same patch, a branch commit
  has the same PR number, or a `[Backport ...]` commit has the same title. Backport PR status is appended when one
  exists.
- Forward-ports from the branch to `main`: commits on the release branch since the cut, ✅ when also on `main`.
- Row status: ✅ landed, ☑️ marked `/done`, 🟨 backport PR open, ⬜ not ported, ⏭️ marked `/skip`. Emoji instead of
  task-list checkboxes, which anyone with edit access could tick by accident.
- Experimental branches: open `release-X.Y.Z--*` branches with their tip.
- Human notes: free text below `<!-- HUMAN NOTES BELOW -->`, preserved verbatim.

Commands are comments on the tracker issue by a user with write access to the repository; comments by others are
ignored. `<sha>` is a commit sha prefix of at least 7 characters.

| Command                  | Effect                                                                                                               |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| `/tag`                   | Tag the branch tip with the next patch, create a GitHub pre-release, build. Lines 2..n are the tag's operator notes. |
| `/tag X.Y.Z.N`           | Same with an explicit tag. Must match the branch version and be new.                                                 |
| `/notes X.Y.Z.N`         | Replace the tag's operator notes with lines 2..n.                                                                    |
| `/publish X.Y.Z.N decaf` | Publish the tag to `decaf` or `mainnet`. Lines 2..n are notes for that network only.                                 |
| `/done <sha>`            | Mark a commit ☑️ that was ported outside the backport workflow.                                                      |
| `/skip <sha>`            | Mark a commit ⏭️ and strike it through: deliberately not ported.                                                     |
| `/unmark <sha>`          | Undo `/done` or `/skip`.                                                                                             |
| `/backport 123`          | Open a backport PR for merged PR 123 against this release branch (`#123` works too).                                 |

Marks are replayed from the issue's comment history, so the body can always be regenerated.

### Operator notes

Each tag's release body starts with a block between `<!-- operator-notes -->` and `<!-- /operator-notes -->`, above the
generated PR list. It states what an operator must do when moving to the tag from the previous tag on the same line. The
`.0` tag describes the move from the previous line, including protocol activation.

- `/tag` lines 2..n fill the block. Without them the block reads `_Not yet stated._`.
- `/notes X.Y.Z.N` replaces the block. `none` renders `No operator action required.` If the markers were deleted in the
  GitHub UI, the block is inserted again at the top.
- Suggested content: urgency (High, Medium, Low), config or env changes, DB migration and its duration, restart order, a
  deadline as view number plus UTC time.
- The comment author and time are the record of who stated the notes.

### Publishing to a network

`/publish X.Y.Z.N decaf|mainnet` makes the tag the release operators deploy on that network. One tag can serve decaf
first and mainnet later. Only `/publish` turns a pre-release into a release.

- Title: `X.Y.Z.N [decaf]`, `[mainnet]` or `[decaf, mainnet]`. Only the release currently published on a network carries
  its marker. The previous holder loses the marker, stays a full release and gets the line
  `Superseded on <network> by <tag> (<date>)`.
- Latest: always the release published on mainnet. Every edit sends `make_latest` explicitly, because GitHub defaults to
  true.
- Body: one block per network between `<!-- publish:<network> prev=<tag|none> date=<date> -->` and
  `<!-- /publish:<network> -->`, above the notes block. It holds the deploy date, the previous tag, the `docker pull`
  line, the lines 2..n of `/publish`, the operator actions of every tag in the range and a compare link. Text outside
  the bot's markers is kept.
- Range of tags whose notes the block collects, for current holder P and target X:

  | Case                      | Range                                   |
  | ------------------------- | --------------------------------------- |
  | P on the same line as X   | tags after P up to X                    |
  | P on another line         | `X.Y.Z.0` up to X                       |
  | no P (first publish)      | X only                                  |
  | X older than P (rollback) | empty; the block says `Rollback from P` |

  Tags with a stated action get a subsection, newest first. Tags with `none` fold into one `<tags>: no action` line.

- `/notes X.Y.Z.N` re-renders every current block, so the per-tag notes and the cumulative view stay in step.
- Gates, each reported as a tracker comment without changing anything: the tag belongs to the tracker's version, the
  release exists, a `build.yml` run for the tag succeeded, and every tag in the range has stated notes (the comment
  lists the missing tags; `/notes <tag> none` fixes a quiet one).
- Network issue: the bot comments the same text on the issue `Releases: <network>` (see below).
- Rollback: `/publish <older tag> <network>`.
- Rerunning `/publish` for the current holder only re-renders its block, keeping its date and previous tag. It is also
  the recovery after a partial failure. Notes given on lines 2..n replace the stored ones.
- `just release-notes X.Y.Z.N decaf` previews the edits and the tracker comment without writing.

### Following a network

Operators get one notification per promotion from the issue `Releases: decaf` or `Releases: mainnet` (label
`release-network`). The bot opens each issue on the first `/publish` for the network. Its body holds the current tag,
the `docker pull` line and the promotion history (tag, date, who, previous tag, release link), and `/publish` adds one
comment. Subscribe to the issue on its page (sidebar, Subscribe). A rerun of `/publish` for the current holder adds no
comment.

- Do not rely on Watch > Releases or `releases.atom`: both include pre-releases, so every `/tag` notifies, and the feed
  also lists bare tags.
- Anyone can comment on the issue, and a comment notifies all subscribers. Issues cannot be locked, because the workflow
  token cannot comment on locked issues.
- Pinning is a one-time manual step per issue: a maintainer runs `gh issue pin <number>`. The bot does not pin, since a
  pin with the workflow token is unverified.

## Process

1. Cut the branch: `just release-cut` bumps `PHASE` from the highest existing `release-X.Y.Z`; `just release-cut 0.7.0`
   is for the branch that first activates a new protocol version on any network, decaf in practice. Only a bump of one
   part is accepted: a `PHASE` bump is allowed on any existing `MAJOR.MINOR` line, so an older protocol line can still
   get a new cut after a newer one exists, while a `MINOR` or `MAJOR` bump only applies to the highest version overall.
   An optional second argument is the source ref, default `main`. The recipe pushes the branch from your machine, since
   repository rules stop the workflow token from creating `release-*` branches, then runs the Release Branch workflow,
   which tags `X.Y.Z.0` as a pre-release, creates the backport label and the tracker issue, and builds images. Running
   the workflow from the Actions UI works once the branch exists, with `source_ref` set to the branch tip sha.
2. Land backport PRs and fixes on the release branch. Watch the tracker checklist.
3. Comment `/tag` on the tracker after each batch worth testing, with the operator notes on lines 2..n or `none`. The
   bot replies with the tag, the GitHub pre-release and a link to the `build.yml` run.
4. Validate the tag on devnet, or push an experimental branch for ad hoc changes. Fix the notes with `/notes X.Y.Z.N`.
5. Optionally preview with `just release-notes X.Y.Z.N decaf`. Once a tag is validated, comment
   `/publish X.Y.Z.N decaf`, with network-specific deadlines on lines 2..n. Only releases are meant for operators. A
   later mainnet promotion of the same tag is `/publish X.Y.Z.N mainnet` on the same tracker. Announce with the release
   link.
6. Delete the release branch when it is no longer needed, shipped or abandoned. The tracker is closed with a final tag
   list and labelled `release-closed`.

## Workflows

| Workflow                     | Trigger                                                                | Runs                                                  |
| ---------------------------- | ---------------------------------------------------------------------- | ----------------------------------------------------- |
| `release-branch.yml`         | `workflow_dispatch`, branch `delete`                                   | `scripts/release cut`, `teardown`                     |
| `tag-release.yml`            | `/tag`, `/backport`, `/notes`, `/publish` comment, `workflow_dispatch` | `scripts/release tag`, `backport`, `notes`, `publish` |
| `update-release-tracker.yml` | push to `main`/`release-*`, mark comments, dispatch                    | `scripts/release refresh`                             |
| `build.yml`                  | dispatched by `cut` and `tag` for the new tag                          | docker images                                         |

Tags pushed by workflows do not fire `push` events, so `cut` and `tag` dispatch `build.yml` and the tracker refresh
explicitly.

Tracker refreshes run only on `main`. A run on a release branch (push, backport PR, dispatch) re-dispatches
`update-release-tracker.yml` on `main` with the original ref, so the refresh always uses `main`'s workflow and
`scripts/release` and still targets that branch's tracker. Dispatching from a feature branch also runs `main`'s copy;
test changes to `scripts/release` with the local commands below.

## Local use

All logic is in `scripts/release` (stdlib Python, tests in `scripts/test_release.py`, run with `just py::test`). It
needs `gh auth` and an `origin` remote.

```sh
just release-body 0.6.0          # render the tracker body to stdout, no writes
just release-tag release-0.6.0   # same as commenting /tag
just release-tag release-0.6.0 0.6.0.5
scripts/release tag --branch release-0.6.0 --dry-run   # show the tag, sha and tracker comment, no writes
scripts/release --help
just release-notes 0.7.0.4 decaf   # preview /publish, no writes
```

`--local` makes both dry runs read the release branch from the checkout instead of `origin/*` (`main` still comes from
`origin/main`, without fetching), so a release branch and `X.Y.Z.0` tag that exist only locally can be inspected:

```sh
git branch release-0.0.1 && git tag -a 0.0.1.0 release-0.0.1 -m "Release 0.0.1.0"
scripts/release refresh --dry-run --local --version 0.0.1
scripts/release tag --dry-run --local --branch release-0.0.1
```

## Recovery

`/tag` pushes the git tag first, then creates the pre-release, then dispatches `build.yml`. If a later step fails, the
bot comments the error on the tracker and the tag stays. Finish by hand rather than tagging again, then state the notes
with `/notes X.Y.Z.N`:

```sh
gh release create X.Y.Z.N --target <sha> --title X.Y.Z.N --generate-notes --prerelease
gh workflow run build.yml --ref X.Y.Z.N
```

If `just release-cut` pushed the branch but the dispatch failed, rerunning it is rejected because the branch exists.
Dispatch the workflow directly with the branch tip; `cut` skips whatever already exists:

```sh
gh workflow run release-branch.yml -f version=X.Y.Z -f source_ref=$(git rev-parse origin/release-X.Y.Z)
```

## Protection

- Tracker commands and `workflow_dispatch` require write access (`admin`, `maintain` or `write`). The workflow `if:`
  filters on `author_association` to skip runs cheaply; `scripts/release` checks the commenter's permission via the
  collaborators API, and mark replay ignores commands by users without write access.
- Trackers are only recognized when opened by `github-actions[bot]`. They are not locked: the workflow token cannot
  comment on locked issues.
- `/notes` and `/publish` need the same write access as `/tag`. There is no environment approval: the decision stays on
  the tracker.
- Concurrency groups are job-level, so skipped runs from outside comments or fork PRs never cancel a pending run. The
  `notes` and `publish` jobs share the group `release-publish`, because a run edits releases of more than one line.
- Tag protection rules cannot exempt the workflow token, so tags are not protected yet.
- Floating docker tags per network (`decaf`, `mainnet`) and automated promotion are not part of this process. Operators
  pin release tags. `/publish` never moves a tag.
