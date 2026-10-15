module

public import NewProtocolDiff.Header
public import NewProtocolDiff.Trace
public import NewProtocolDiff.Check
public import NewProtocolSpec.Restart
public import NewProtocolSpec.Proofs.Restart

/-!
# Checking a restarted trace against the file it resumes

A node restarted from storage writes a second trace whose header states what it
loaded (`Restored`) and names the file of its run before the crash. This module
pairs the two, reads the header as a `NewProtocol.Persisted`, and checks exactly
the hypotheses of `NewProtocol.safe_across_restart`: the summary is faithful to the
history before the crash (`CheckSummarises`), the history after it keeps the bars
(`CheckRestartOK`), and the signing rules hold of the history after the crash
under the restart's configuration. `restart_pair_safe` is what makes a pass mean
that the spliced history obeys the signing rules.

The machine is not run across a restart, and the protocol rules are not checked
there: the restarted node's view is placed where its storage puts it, not where
the certificates it holds would.
-/

@[expose] public section

namespace NewProtocolDiff

open NewProtocol NewProtocol.Lists History

instance (e : EpochNumber) (v : ViewNumber) (e' : EpochNumber) (v' : ViewNumber) :
    Decidable (EpochViewLE e v e' v') := inferInstanceAs (Decidable (_ ∨ _))

/--
The anchor block a restarted node loaded: the one the history before the crash
held under the certificate's block hash.

Found by hash rather than view, since a re-vote's certificate is at a later view
than its block.
-/
def restartAnchor (cfg : Config) (pre : History) (c : Cert1) : Option Block :=
  (proposalsHeld cfg pre).find? fun b => blockHash b = c.data.blockHash

/--
The summary a restarted trace's header states, against the history `pre` it
resumes under configuration `cfg`.

An error when the header is incomplete: an anchor after genesis without its
certificate, or no lock. `none` when `pre` holds no block under the anchor's
certificate, which breaks `Summarises.anchorHeld`. With the anchor at genesis it
is `cfg`'s.
-/
def persistedOf (cfg : Config) (pre : History) (said : Preamble) (r : Restored) :
    Except String (Option Persisted) := do
  let some lock := r.lock | throw "its `restart` header names no lock"
  let reconstructed := r.reconstructed.map fun x => ((⟨x.1⟩ : ViewNumber), x.2)
  let summary (b : Block) (c : Cert1) : Persisted := ⟨b, c, lock, ⟨r.highestVoted⟩, reconstructed⟩
  if said.anchorView = 0 then
    return some (summary cfg.anchorBlock cfg.anchorCert)
  let some c := said.anchorCert | throw s!"restored from view {said.anchorView} with no `anchorCert`"
  return (restartAnchor cfg pre c).map (summary · c)

/--
Whether the header's vote2 bar covers the one `NewProtocol.RestartOK.vote2After`
asks for. Without it, or below it, the node may re-cast a vote2 at a barred view,
as before the certificate rule.
-/
def barsVote2 (P : Persisted) (r : Restored) : Bool :=
  r.vote2Bar.any fun b => decide (P.firstAllowed ≤ (⟨b⟩ : ViewNumber) + 1)

/-! ## The premises in bounded form -/

section Bounded

variable (cfg : Config) (P : Persisted) (pre post : History)

-- The anchor payload condition outgrows the default instance size.
set_option synthInstance.maxSize 1024 in
/--
Each field of `NewProtocol.Summarises`, by name, and whether `pre` and `post` meet it.

`anchorPayload` compares a vote's epoch and height before its block hash, which
is opaque to `decide`.
-/
def summaryChecks : List (String × Bool) :=
  [ ("summary:anchorAfter", decide (cfg.anchorView ≤ P.anchorView)),
    ("summary:anchorCertHeld", decide (P.anchorCert ∈ cert1sHeld cfg pre)),
    ("summary:anchorHeld", decide (P.anchorBlock ∈ proposalsHeld cfg pre)),
    ("summary:anchorPayload", decide (∀ st ∈ post, ∀ v ∈ vote2sOf st,
      ∀ b ∈ proposalsHeld cfg (pre ++ post), b.viewNumber = P.anchorView →
        b.payloadCommit = P.anchorBlock.payloadCommit → v.data.epoch = b.epoch →
        v.data.blockNumber = b.blockHeader.blockNumber → v.data.blockHash = blockHash b →
        pre.HasPayload cfg P.anchorView P.anchorBlock.payloadCommit)),
    ("summary:anchorCert2Held", decide ((P.config cfg).anchorCert2 = cfg.anchorCert2
      ∨ ∃ c2 ∈ cert2sHeld pre, c2.data = P.anchorCert.data.toVote2 ∧ c2.view < P.firstAllowed)),
    ("summary:reconstructedHeld", decide (∀ x ∈ P.reconstructed,
      ∃ b ∈ proposalsHeld cfg pre, b.viewNumber = x.1 ∧ b.payloadCommit = x.2
        ∧ ((x.1 = P.anchorView ∧ x.2 = P.anchorBlock.payloadCommit)
          ∨ pre.HasPayload cfg x.1 x.2))),
    ("summary:vote1Below", decide (∀ st ∈ pre, ∀ v ∈ vote1sOf st, v.view ≤ P.highestVoted)),
    ("summary:vote2Below", decide (∀ st ∈ pre, ∀ v ∈ vote2sOf st, v.view ≤ P.highestVoted)),
    ("summary:timeoutBelow", decide (∀ st ∈ pre, ∀ v ∈ timeoutVotesOf st, v.view ≤ P.highestVoted)),
    ("summary:vote2Locked", decide (∀ st ∈ pre, ∀ v ∈ vote2sOf st,
      EpochViewLE v.data.epoch v.view P.lock.data.epoch P.lock.view)),
    ("summary:decidedBelow", decide (∀ st ∈ pre, ∀ d ∈ decidesOf st, ∀ b ∈ d.1,
      b.viewNumber ≤ P.anchorView)) ]

/-- Each field of `NewProtocol.RestartOK`, by name, and whether `post` meets it. -/
def barsChecks : List (String × Bool) :=
  [ ("bars:vote1After", decide (∀ st ∈ post, ∀ v ∈ vote1sOf st, P.firstAllowed ≤ v.view)),
    ("bars:vote2After", decide (∀ st ∈ post, ∀ v ∈ vote2sOf st, P.firstAllowed ≤ v.view)),
    ("bars:lockKept", decide (∀ st ∈ post, ∀ v ∈ timeoutVotesOf st, LockLE P.lock v.data.lock)) ]

/-- `NewProtocol.Summarises`, decided on finite histories. -/
def CheckSummarises : Prop := (summaryChecks cfg P pre post).all (·.2)

/-- `NewProtocol.RestartOK`, decided on a finite history. -/
def CheckRestartOK : Prop := (barsChecks P post).all (·.2)

instance : Decidable (CheckSummarises cfg P pre post) := by unfold CheckSummarises; infer_instance
instance : Decidable (CheckRestartOK P post) := by unfold CheckRestartOK; infer_instance

/-- Every premise of a restart on `pre` and `post`, by name, and whether it holds. -/
def checkRestart : List (String × Bool) := summaryChecks cfg P pre post ++ barsChecks P post

end Bounded

/-! ## Soundness -/

section Soundness

variable {cfg : Config} {P : Persisted} {pre post : History}

/-- **A pair that passes the summary check has a faithful summary.** -/
theorem checkSummarises_sound (h : decide (CheckSummarises cfg P pre post) = true) :
    Summarises cfg P pre post := by
  replace h := of_decide_eq_true h
  simp only [CheckSummarises, summaryChecks, List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true,
    decide_eq_true_eq] at h
  obtain ⟨hafter, hcert, hheld, hpay, hc2, hrec, h1, h2, ht, hlock, hdec⟩ := h
  refine ⟨hafter, mem_cert1sHeld.mp hcert, mem_proposalsHeld.mp hheld, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
  · intro v b ⟨st, hst, hm⟩ hb hv hpc hd
    exact hpay st hst v (mem_vote2sOf.mpr hm) b (mem_proposalsHeld.mpr hb) hv hpc (by rw [hd])
      (by rw [hd]) (by rw [hd])
  · rcases hc2 with he | ⟨c2, hm, hd, hv⟩
    · exact Or.inl he
    · exact Or.inr ⟨c2, hasCert2_of_mem hm, hd, hv⟩
  · intro x hx
    obtain ⟨b, hb, hv, hpc, hpay⟩ := hrec x hx
    exact ⟨b, mem_proposalsHeld.mp hb, hv, hpc, hpay⟩
  · rintro v ⟨st, hst, hm⟩; exact h1 st hst v (mem_vote1sOf.mpr hm)
  · rintro v ⟨st, hst, hm⟩; exact h2 st hst v (mem_vote2sOf.mpr hm)
  · rintro v ⟨st, hst, hm⟩; exact ht st hst v (mem_timeoutVotesOf hm)
  · rintro v ⟨st, hst, hm⟩; exact hlock st hst v (mem_vote2sOf.mpr hm)
  · rintro v ⟨st, hst, blocks, c1, c2, b, hm, hb, rfl⟩
    exact hdec st hst (blocks, c1, c2) (mem_decidesOf.mpr hm) b hb

/-- **A history that passes the bars check keeps the bars.** -/
theorem checkRestartOK_sound (h : decide (CheckRestartOK P post) = true) : RestartOK P post := by
  replace h := of_decide_eq_true h
  simp only [CheckRestartOK, barsChecks, List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true,
    decide_eq_true_eq] at h
  obtain ⟨h1, h2, hl⟩ := h
  refine ⟨?_, ?_, ?_⟩
  · rintro v ⟨st, hst, hm⟩; exact h1 st hst v (mem_vote1sOf.mpr hm)
  · rintro v ⟨st, hst, hm⟩; exact h2 st hst v (mem_vote2sOf.mpr hm)
  · rintro v ⟨st, hst, hm⟩; exact hl st hst v (mem_timeoutVotesOf hm)

/--
**A restarted pair that passes every check obeys the signing rules across the
restart**, given that the history before the crash does and that the validity
reports after it were truthful.

`hpre` is a `SafeHistory` rather than a check: for a node restarted twice the
history before the second crash is itself a splice, which no single run of
`checkSafe` covers. Each file of a chain is checked when it is itself the
argument, and `check` fails if any does; `chainHistory` refuses a file that
`check` leaves out of scope, so every link is established.
-/
theorem restart_pair_safe {node : PubKey} {honestIn : EpochNumber → Prop}
    (hpre : SafeHistory cfg node honestIn pre) (hs : decide (CheckSummarises cfg P pre post) = true)
    (hpost : checkSafe (P.config cfg) node (pre.inputsOnly ++ P.restoredSteps ++ post) = true)
    (hv' : ValidityTruthful (pre.inputsOnly ++ P.restoredSteps ++ post))
    (hok : decide (CheckRestartOK P post) = true) :
    SafeHistory cfg node honestIn (pre ++ P.restoredSteps ++ post) :=
  safe_across_restart hpre (checkSummarises_sound hs) (checkSafe_sound hpost hv' honestIn)
    (checkRestartOK_sound hok)

end Soundness

/-! ## Following a chain of files -/

/-- Why the files a restarted trace resumes could not be spliced. -/
inductive ChainError where
  /-- A file is missing or unreadable, or disagrees with the run it resumes. -/
  | unreadable (why : String)
  /-- A file is one `check` leaves out of scope, so the history it ends is not established. -/
  | unchecked (why : String)

/--
A resumed trace's preamble and history.

Refused as `unchecked` when its timeouts sign no lock, which `check` leaves out of
scope, and as `unreadable` when it notes actions before any step, which its history
would leave out.
-/
def readRecorded (path : System.FilePath) : IO (Except ChainError (Preamble × History)) := do
  unless ← path.pathExists do
    return .error (.unreadable s!"resumes {path}, which is missing")
  let text ← IO.FS.readFile path
  if signsNoLock text then
    return .error (.unchecked s!"{path} signs no lock in its timeouts")
  if heldNote text |>.isSome then
    return .error (.unreadable s!"{path} has actions before any step")
  return Except.mapError .unreadable do
    let said ← readPreamble text
    let events ← parseTrace text
    pure (said, historyOf events)

mutual

/--
The node, the configuration of its first run, and its whole history up to the
end of the trace at `path`: each file it resumes, the steps each restart loaded,
and its own steps, spliced.

A file without a `restart` header is the first run, and must start at genesis.
Each file's own premises are checked when it is itself the argument; this only
splices, and refuses a file whose re-cast vote2 `check` leaves out of scope.
-/
partial def chainHistory (path : System.FilePath) (seen : List System.FilePath := []) :
    IO (Except ChainError (Nat × Config × History)) := do
  if seen.contains path then
    return .error (.unreadable s!"{path} resumes itself")
  match ← readRecorded path with
  | .error e => return .error e
  | .ok (said, h) =>
  match said.restart with
  | none =>
    if said.anchorView ≠ 0 then
      return .error (.unreadable
        s!"{path} is restored from view {said.anchorView} and names no file it resumes")
    return .ok (said.node, said.configFor h, h)
  | some r =>
    let resumed ← resumedBy path said r h seen
    return do
      let (_, node, cfg, pre) ← resumed
      let some P ← (persistedOf cfg pre said r).mapError .unreadable
        | throw (.unreadable s!"{path}: its anchor is not among the proposals the run before held")
      unless barsVote2 P r do
        throw (.unchecked s!"{path} may re-cast vote2 below the bar")
      pure (node, cfg, pre ++ P.restoredSteps ++ h)

/--
What the restarted trace at `path`, with header `said` and `r` and history `h`,
resumes: the file's name, the node, the configuration of the node's first run,
and the history up to the crash.

A restart that resumes no file is one whose first run recorded no step: its
history before the crash is empty, which only a node anchored at genesis can
have. An error when `said` names another node, epoch height or decide buffer
than the run it resumes.
-/
partial def resumedBy (path : System.FilePath) (said : Preamble) (r : Restored) (h : History)
    (seen : List System.FilePath := []) :
    IO (Except ChainError (String × Nat × Config × History)) := do
  match r.resumes with
  | some f =>
    let prev := path.parent.getD "." / f
    let chain ← chainHistory prev (path :: seen)
    return do
      let (node, cfg, pre) ← chain
      unless node = said.node do
        throw (.unreadable s!"the node of {path} differs from that of {prev}")
      unless cfg.epochHeight = said.epochHeight ∧ cfg.decideBuffer = said.decideBuffer do
        throw (.unreadable s!"the epoch height or decide buffer of {path} differs from that of {prev}")
      pure (prev.toString, node, cfg, pre)
  | none =>
    if said.anchorView ≠ 0 then
      return .error (.unreadable "resumes an empty recording but loaded an anchor")
    return .ok ("an empty recording", said.node, said.configFor h, [])

end

/-- What checking a restarted trace against the files it resumes found. -/
inductive RestartFinding where
  /-- The premises were checked; `broken` names those that failed. -/
  | checked (anchorView : Nat) (resumes : String) (broken : List String)
  /-- The node may re-cast a vote2 below the bar, as before the certificate rule. -/
  | recasts
  /-- A file it resumes is out of scope, so the history before the crash is not established. -/
  | unchecked (why : String)

/--
Check a restarted trace, whose header said `said` and `r` and whose history is
`post`, against what it resumes: the summary on the spliced history before the
crash, the bars on `post`, and the signing rules on
`pre.inputsOnly ++ P.restoredSteps ++ post` under `P.config cfg`.

An error when the resumed file is missing or unreadable, when the header is
incomplete, or when the two runs disagree on the node, epoch height or decide buffer.
-/
def checkRestarted (path : System.FilePath) (said : Preamble) (r : Restored) (post : History) :
    IO (Except String RestartFinding) := do
  match ← resumedBy path said r post with
  | .error (.unreadable e) => return .error e
  | .error (.unchecked why) => return .ok (.unchecked why)
  | .ok (resumes, node, cfg, pre) =>
  return do
    let some P ← persistedOf cfg pre said r
      | pure (.checked said.anchorView resumes ["summary:anchorHeld"])
    if !barsVote2 P r then return .recasts
    let rules := checkRestart cfg P pre post
      ++ checkRules (P.config cfg) ⟨node⟩ (pre.inputsOnly ++ P.restoredSteps ++ post)
    pure (.checked P.anchorView.toNat resumes (rules.filterMap fun (n, ok) => if ok then none else some n))

end NewProtocolDiff
