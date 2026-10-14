module

public import NewProtocolDiff.Restart

/-!
# Checking a rejoined trace against the file it resumes

A node restarted with nothing stored writes a second trace whose header states the
view bar it recovered and the anchor and lock it adopted (`Rejoined`), and names
the file of its run before the wipe. This module pairs the two, reads the header as
a `NewProtocol.Rejoin`, and checks exactly the hypotheses of
`NewProtocol.safe_across_wipe`: what the node adopted is sound for the history
before the wipe (`CheckRejoins`), the history after it keeps the bars
(`CheckRestartOK` of `NewProtocol.Rejoin.persisted`), and the signing rules hold of
the history after the wipe under the adopted configuration. `rejoin_pair_safe` is
what makes a pass mean that the spliced history obeys the signing rules.

The machine is not run across a wipe, and the protocol rules are not checked
there: the rejoined node's view is placed where the network puts it, not where the
certificates it holds would.
-/

@[expose] public section

namespace NewProtocolDiff

open NewProtocol NewProtocol.Lists History

/--
What a rejoined trace's header states it adopted with view bar `bar`, against the
history `pre` it resumes and its own history `post`, under configuration `cfg`.

An error when the header names no anchor certificate. `none` when neither history
holds a block under it, which breaks `Rejoins.anchorHeld`: the node may have
received the anchor's proposal before the wipe.
-/
def rejoinOf (cfg : Config) (pre post : History) (said : Preamble) (bar : Nat) :
    Except String (Option Rejoin) := do
  let some c := said.anchorCert | throw s!"rejoined at view {said.anchorView} with no `anchorCert`"
  return (restartAnchor cfg (pre ++ post) c).map fun b => ⟨⟨bar⟩, b, c⟩

/-! ## The premises in bounded form -/

section Bounded

variable (cfg : Config) (R : Rejoin) (pre post : History)

-- The anchor payload condition outgrows the default instance size.
set_option synthInstance.maxSize 1024 in
/--
Each field of `NewProtocol.Rejoins`, by name, and whether `pre` and `post` meet it.

`anchorPayload` compares a vote's epoch and height before its block hash, which
is opaque to `decide`.
-/
def rejoinChecks : List (String × Bool) :=
  [ ("rejoin:anchorAfter", decide (cfg.anchorView ≤ R.anchorView)),
    ("rejoin:anchorAbove", decide (R.bar < R.anchorCert.view)),
    ("rejoin:anchorCertHeld", decide (∀ i, (hi : i < post.length) → post[i].Acts →
      R.anchorCert ∈ cert1sHeld cfg (pre ++ post.take (i + 1)))),
    ("rejoin:anchorHeld", decide (∀ i, (hi : i < post.length) → post[i].Acts →
      R.anchorBlock ∈ proposalsHeld cfg (pre ++ post.take (i + 1)))),
    ("rejoin:anchorPayload", decide (∀ i, (hi : i < post.length) → ∀ v ∈ vote2sOf (post[i]'hi),
      ∀ b ∈ proposalsHeld cfg (pre ++ post.take (i + 1)), b.viewNumber = R.anchorView →
        b.payloadCommit = R.anchorBlock.payloadCommit → v.data.epoch = b.epoch →
        v.data.blockNumber = b.blockHeader.blockNumber → v.data.blockHash = blockHash b →
        (pre ++ post.take (i + 1)).HasPayload cfg R.anchorView R.anchorBlock.payloadCommit)),
    ("rejoin:anchorCert2Held", decide (∀ i, (hi : i < post.length) → post[i].Acts →
      (R.config cfg).anchorCert2 = cfg.anchorCert2
        ∨ (R.config cfg).anchorCert2 ∈ cert2sHeld (pre ++ post.take (i + 1)))),
    ("rejoin:vote1Below", decide (∀ st ∈ pre, ∀ v ∈ vote1sOf st, v.view ≤ R.bar)),
    ("rejoin:vote2Below", decide (∀ st ∈ pre, ∀ v ∈ vote2sOf st, v.view ≤ R.bar)),
    ("rejoin:timeoutBelow", decide (∀ st ∈ pre, ∀ v ∈ timeoutVotesOf st, v.view ≤ R.bar)),
    ("rejoin:vote2Locked", decide (∀ st ∈ pre, ∀ v ∈ vote2sOf st,
      EpochViewLE v.data.epoch v.view R.anchorCert.data.epoch R.anchorCert.view)),
    ("rejoin:decidedBelow", decide (∀ st ∈ pre, ∀ d ∈ decidesOf st, ∀ b ∈ d.1,
      b.viewNumber ≤ R.anchorView)) ]

/-- `NewProtocol.Rejoins`, decided on finite histories. -/
def CheckRejoins : Prop := (rejoinChecks cfg R pre post).all (·.2)

instance : Decidable (CheckRejoins cfg R pre post) := by unfold CheckRejoins; infer_instance

/-- Every premise of a rejoin on `pre` and `post`, by name, and whether it holds. -/
def checkRejoin : List (String × Bool) := rejoinChecks cfg R pre post ++ barsChecks R.persisted post

end Bounded

/-! ## Soundness -/

section Soundness

variable {cfg : Config} {R : Rejoin} {pre post : History}

/-- **A pair that passes the rejoin check adopted what is sound for it.** -/
theorem checkRejoins_sound (h : decide (CheckRejoins cfg R pre post) = true) :
    Rejoins cfg R pre post := by
  replace h := of_decide_eq_true h
  simp only [CheckRejoins, rejoinChecks, List.all_cons, List.all_nil, Bool.and_true, Bool.and_eq_true,
    decide_eq_true_eq] at h
  obtain ⟨hafter, habove, hcert, hheld, hpay, hc2, h1, h2, ht, hlock, hdec⟩ := h
  refine ⟨hafter, habove, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
  · intro n st hst hact
    obtain ⟨hn, rfl⟩ := List.getElem?_eq_some_iff.mp hst
    exact mem_cert1sHeld.mp (hcert n hn hact)
  · intro n st hst hact
    obtain ⟨hn, rfl⟩ := List.getElem?_eq_some_iff.mp hst
    exact mem_proposalsHeld.mp (hheld n hn hact)
  · rintro n v b ⟨st, hst, hm⟩ hb hv hpc hd
    obtain ⟨hn, rfl⟩ := List.getElem?_eq_some_iff.mp hst
    exact hpay n hn v (mem_vote2sOf.mpr hm) b (mem_proposalsHeld.mpr hb) hv hpc (by rw [hd])
      (by rw [hd]) (by rw [hd])
  · intro n st hst hact
    obtain ⟨hn, rfl⟩ := List.getElem?_eq_some_iff.mp hst
    exact (hc2 n hn hact).imp id hasCert2_of_mem
  · rintro v ⟨st, hst, hm⟩; exact h1 st hst v (mem_vote1sOf.mpr hm)
  · rintro v ⟨st, hst, hm⟩; exact h2 st hst v (mem_vote2sOf.mpr hm)
  · rintro v ⟨st, hst, hm⟩; exact ht st hst v (mem_timeoutVotesOf hm)
  · rintro v ⟨st, hst, hm⟩; exact hlock st hst v (mem_vote2sOf.mpr hm)
  · rintro v ⟨st, hst, blocks, c1, c2, b, hm, hb, rfl⟩
    exact hdec st hst (blocks, c1, c2) (mem_decidesOf.mpr hm) b hb

/--
**A rejoined pair that passes every check obeys the signing rules across the
wipe**, given that the history before the wipe does and that the validity reports
after it were truthful.

`hpre` is a `SafeHistory` rather than a check: the history before the wipe may
itself be a splice, which no single run of `checkSafe` covers. Each file of a chain
is checked when it is itself the argument, and `check` fails if any does;
`chainHistory` refuses a file that `check` leaves out of scope, so every link is
established.
-/
theorem rejoin_pair_safe {node : PubKey} {honestIn : EpochNumber → Prop}
    (hpre : SafeHistory cfg node honestIn pre) (hr : decide (CheckRejoins cfg R pre post) = true)
    (hpost : checkSafe (R.config cfg) node (pre.inputsOnly ++ post) = true)
    (hv' : ValidityTruthful (pre.inputsOnly ++ post))
    (hok : decide (CheckRestartOK R.persisted post) = true) :
    SafeHistory cfg node honestIn (pre ++ post) :=
  safe_across_wipe hpre (checkRejoins_sound hr) (checkSafe_sound hpost hv' honestIn)
    (checkRestartOK_sound hok)

end Soundness

/-! ## Checking a file -/

/-- What checking a rejoined trace against the files it resumes found. -/
inductive RejoinFinding where
  /-- The premises were checked; `broken` names those that failed. -/
  | checked (anchorView : Nat) (resumes : String) (broken : List String)
  /-- The node never adopted an anchor, so it signed nothing to check. -/
  | unanchored
  /-- A file it resumes is out of scope, so the history before the wipe is not established. -/
  | unchecked (why : String)

/--
Check a rejoined trace, whose header said `said` and `r` and whose history is
`post`, against what it resumes: what it adopted on the spliced history before the
wipe, the bars on `post`, and the signing rules on `pre.inputsOnly ++ post` under
`Rejoin.config cfg`.

An error when the resumed file is missing or unreadable, when the header names no
anchor certificate, or when the two runs disagree on the node, epoch height or
decide buffer.
-/
def checkRejoined (path : System.FilePath) (said : Preamble) (r : Rejoined) (post : History) :
    IO (Except String RejoinFinding) := do
  let some bar := r.bar | return .ok .unanchored
  match ← resumedBy path said r.resumes post with
  | .error (.unreadable e) => return .error e
  | .error (.unchecked why) => return .ok (.unchecked why)
  | .ok (resumes, node, cfg, pre) =>
  return do
    let some R ← rejoinOf cfg pre post said bar
      | pure (.checked said.anchorView resumes ["rejoin:anchorHeld"])
    let rules := checkRejoin cfg R pre post ++ checkRules (R.config cfg) ⟨node⟩ (pre.inputsOnly ++ post)
    pure (.checked R.anchorView.toNat resumes (rules.filterMap fun (n, ok) => if ok then none else some n))

end NewProtocolDiff
