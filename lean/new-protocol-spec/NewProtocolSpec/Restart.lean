module

public import NewProtocolSpec.Network

/-!
# Across a restart

A node that crashes loses what it held in memory and resumes from what it stored.
The signing rules read every vote the node ever sent, so a node that forgot its
votes could sign twice. It stores a summary of them instead:
`Persisted` is that summary, `Summarises` says it is faithful to the history
before the crash, and `RestartOK` is the bars the node sets from it on resuming.

A restart is modelled as a splice with no new input: the history before the
crash, the steps that loading saved proposals amounts to
(`Persisted.restoredSteps`), and the history after it are one history
(`Trace.splice`). It obeys the signing rules when the history after the crash does
under the restart's configuration (`Persisted.config`), judged behind the inputs of
the history before it (`History.inputsOnly`): `safe_across_restart`, and per prefix
of the spliced trace `safe_trace_across_restart` (`NewProtocolSpec.Proofs.Restart`).

Not covered: the durability of the summary itself, a crash during a write, and
restarts at the version before the timeout epoch bound.
-/

@[expose] public section

namespace NewProtocol

/--
What a restarted node loaded from storage.

The anchor is the newest decided block with its `Cert1`, the lock is the locked
`Cert1`, and `highestVoted` is the latest view the node signed anything in. The
restarted node starts from the anchor (`Persisted.config`) and casts no vote1 or
vote2 at or before `highestVoted` (`RestartOK`).
-/
structure Persisted where
  /-- The newest decided block. -/
  anchorBlock : Block

  /-- A `Cert1` over the anchor block, possibly a re-vote's at a later view. -/
  anchorCert : Cert1

  /-- The restored lock, the latest `Cert1` the node locked on before the crash. -/
  lock : Cert1

  /-- The latest view the node voted, proposed or timed out in. -/
  highestVoted : ViewNumber

  /-- The saved proposals the node marks as having their payload, by view and payload commitment. -/
  reconstructed : List (ViewNumber × PayloadCommit)
deriving Repr

/-- The anchor's view. -/
abbrev Persisted.anchorView (P : Persisted) : ViewNumber := P.anchorBlock.viewNumber

/--
The configuration a restarted node runs under: the original one with the anchor
replaced by the stored one.

Data-only rules read the epoch height alone (`ProposalWellFormed`, `EntersEpoch`),
so both configurations agree on them.
-/
def Persisted.config (cfg : Config) (P : Persisted) : Config :=
  { cfg with anchorBlock := P.anchorBlock, anchorCert := P.anchorCert }

/--
The first view a restarted node may vote in: after the anchor's view and after
every view it signed anything in before the crash.
-/
def Persisted.firstAllowed (P : Persisted) : ViewNumber :=
  max (P.anchorView + 1) (P.highestVoted + 1)

/--
The steps loading the saved proposals amounts to: each payload the node marks as
held arrives as `Input.blockReconstructed`, and nothing is sent.
-/
def Persisted.restoredSteps (P : Persisted) : History :=
  P.reconstructed.map fun x => { input := .blockReconstructed x.1 x.2, output := [] }

/--
The history with every output dropped: what a node that received everything `h`
received and sent nothing would have.

The history after a restart is judged behind it. Holding more inputs only widens
what a node holds (`History.HasCert1`, `History.HasProposal`), never what it sent.
-/
def History.inputsOnly (h : History) : History :=
  h.map fun st => { st with output := [] }

/--
The stored summary `P` is faithful to the history `pre` before the crash, for the
history `post` after it.

Each field is something storage must have written before the node acted on it.
`vote2Locked` asks the stored lock to move forward in lock order (`LockLE`), epoch
first, then view.
-/
structure Summarises (cfg : Config) (P : Persisted) (pre post : History) : Prop where
  /-- The stored anchor is no earlier than the configured one. -/
  anchorAfter : cfg.anchorView ≤ P.anchorView

  /-- The node held the anchor's `Cert1` before the crash. -/
  anchorCertHeld : pre.HasCert1 cfg P.anchorCert

  /-- The node held the anchor's proposal before the crash. -/
  anchorHeld : pre.HasProposal cfg P.anchorBlock

  /--
  The node held the anchor's payload before the crash, if after it the node
  voted2 over a block it holds at the anchor's view with the anchor's payload.

  A node may decide a block without its payload, so the anchor's payload is owed
  only where a vote2 reads it.
  -/
  anchorPayload : ∀ (v : Vote2) (b : Block), post.Sent (.vote2 v) → (pre ++ post).HasProposal cfg b →
    b.viewNumber = P.anchorView → b.payloadCommit = P.anchorBlock.payloadCommit →
    v.data = ⟨blockHash b, b.epoch, b.blockHeader.blockNumber⟩ →
    pre.HasPayload cfg P.anchorView P.anchorBlock.payloadCommit

  /--
  The anchor's `Cert2` under the restart's configuration is the original one, or
  the node held a `Cert2` over the anchor before the crash, at a view before
  `Persisted.firstAllowed`.
  -/
  anchorCert2Held : (P.config cfg).anchorCert2 = cfg.anchorCert2
    ∨ ∃ c2, pre.HasCert2 c2 ∧ c2.data = P.anchorCert.data.toVote2 ∧ c2.view < P.firstAllowed

  /--
  Every saved proposal marked as reconstructed is one the node held before the
  crash, with its payload unless it is the anchor's. Loading it becomes an
  `Input.blockReconstructed` (`Persisted.restoredSteps`), which a vote2 reads.
  -/
  reconstructedHeld : ∀ x ∈ P.reconstructed,
    ∃ b, pre.HasProposal cfg b
      ∧ b.viewNumber = x.1
      ∧ b.payloadCommit = x.2
      ∧ ((x.1 = P.anchorView ∧ x.2 = P.anchorBlock.payloadCommit)
        ∨ pre.HasPayload cfg x.1 x.2)

  /-- Every vote1 before the crash is at or before `Persisted.highestVoted`. -/
  vote1Below : ∀ v : Vote1, pre.Sent (.vote1 v) → v.view ≤ P.highestVoted

  /-- Every vote2 before the crash is at or before `Persisted.highestVoted`. -/
  vote2Below : ∀ v : Vote2, pre.Sent (.vote2 v) → v.view ≤ P.highestVoted

  /-- Every timeout vote before the crash is at or before `Persisted.highestVoted`. -/
  timeoutBelow : ∀ v : TimeoutVote, pre.Sent (.timeoutVote v) → v.view ≤ P.highestVoted

  /-- Every vote2 before the crash is no later than the stored lock (`EpochViewLE`). -/
  vote2Locked : ∀ v : Vote2, pre.Sent (.vote2 v) →
    EpochViewLE v.data.epoch v.view P.lock.data.epoch P.lock.view

  /-- Every view decided before the crash is at or before the anchor's. -/
  decidedBelow : ∀ v, pre.DecidedView v → v ≤ P.anchorView

/--
The bars a restarted node sets from what it loaded: no vote before
`Persisted.firstAllowed`, and no timeout vote naming a lock earlier than the
stored one.
-/
structure RestartOK (P : Persisted) (post : History) : Prop where
  /-- Every vote1 after the restart is at `Persisted.firstAllowed` or later. -/
  vote1After : ∀ v : Vote1, post.Sent (.vote1 v) → P.firstAllowed ≤ v.view

  /-- Every vote2 after the restart is at `Persisted.firstAllowed` or later. -/
  vote2After : ∀ v : Vote2, post.Sent (.vote2 v) → P.firstAllowed ≤ v.view

  /-- Every timeout vote after the restart names a lock no earlier than the stored one (`LockLE`). -/
  lockKept : ∀ v : TimeoutVote, post.Sent (.timeoutVote v) → LockLE P.lock v.data.lock

/--
The trace whose first steps are `pre` and whose later steps are those of `r`.

With `pre` the history before a crash followed by `Persisted.restoredSteps`, and
`r` the trace after it, this is one trace across the restart.
-/
def Trace.splice (pre : History) (r : Trace) : Trace :=
  fun n => if h : n < pre.length then pre[n] else r (n - pre.length)

end NewProtocol
