module

public import NewProtocolSpec.Restart
public import NewProtocolSpec.Proofs.Traces

/-!
# Safety across a restart

The history before a crash, the steps a restart loads, and the history after it
obey the signing rules together (`safe_across_restart`), when the summary the node
stored is faithful (`Summarises`) and the node keeps the bars it sets from it
(`RestartOK`). Read per prefix, the same holds of the spliced trace
(`safe_trace_across_restart`).

Votes before the crash are at or before `Persisted.highestVoted` and votes after
it at `Persisted.firstAllowed` or later, so no rule relates two votes across the
crash except a timeout vote's lock, which the stored lock covers. What the history
after the crash holds under the restart's configuration, it holds under the
original one, since the stored anchor was held before the crash.

The proof reads less of the summary than `Summarises` states: `Resumes` asks only
that each step after the crash that signs or decides holds the anchor by then.
`safe_across_resume` is proved from it, and `safe_across_restart` follows
(`Summarises.resumes`).
-/

@[expose] public section

namespace NewProtocol

/-! ## Histories joined end to end -/

namespace History

theorem upTo_append_left {pre post : History} {n : Nat} (h : n ≤ pre.length) :
    (pre ++ post).upTo n = pre.upTo n := by
  simp [upTo, List.take_append_of_le_length h]

theorem upTo_append_right (pre post : History) (n : Nat) :
    (pre ++ post).upTo (pre.length + n) = pre ++ post.upTo n := by
  simp [upTo, List.take_length_add_append]

theorem received_append {pre post : History} {i : Input} :
    (pre ++ post).Received i ↔ pre.Received i ∨ post.Received i := by
  simp only [Received, List.mem_append, or_and_right, exists_or]

theorem sent_append {pre post : History} {m : Message} :
    (pre ++ post).Sent m ↔ pre.Sent m ∨ post.Sent m := by
  simp only [Sent, List.mem_append, or_and_right, exists_or]

theorem decidedView_append {pre post : History} {v : ViewNumber} :
    (pre ++ post).DecidedView v ↔ pre.DecidedView v ∨ post.DecidedView v := by
  simp only [DecidedView, List.mem_append, or_and_right, exists_or]

theorem received_inputsOnly {h : History} {i : Input} : h.inputsOnly.Received i ↔ h.Received i := by
  constructor
  · rintro ⟨st, hst, rfl⟩
    obtain ⟨a, ha, rfl⟩ := List.mem_map.mp hst
    exact ⟨a, ha, rfl⟩
  · rintro ⟨st, hst, rfl⟩
    exact ⟨_, List.mem_map_of_mem hst, rfl⟩

theorem sent_inputsOnly {h : History} {m : Message} : ¬ h.inputsOnly.Sent m := by
  simp [Sent, inputsOnly]

theorem length_inputsOnly (h : History) : h.inputsOnly.length = h.length := by
  simp [inputsOnly]

theorem sent_restoredSteps {P : Persisted} {m : Message} : ¬ P.restoredSteps.Sent m := by
  rintro ⟨st, hst, hm⟩
  obtain ⟨x, -, rfl⟩ := List.mem_map.mp hst
  simp at hm

theorem received_restoredSteps {P : Persisted} {i : Input} :
    P.restoredSteps.Received i ↔ ∃ x ∈ P.reconstructed, i = .blockReconstructed x.1 x.2 := by
  constructor
  · rintro ⟨st, hst, rfl⟩
    obtain ⟨x, hx, rfl⟩ := List.mem_map.mp hst
    exact ⟨x, hx, rfl⟩
  · rintro ⟨x, hx, rfl⟩
    exact ⟨_, List.mem_map_of_mem hx, rfl⟩

theorem decidedView_inputsOnly {h : History} {v : ViewNumber} : ¬ h.inputsOnly.DecidedView v := by
  rintro ⟨st, hst, blocks, c1, c2, b, hm, -⟩
  obtain ⟨x, -, rfl⟩ := List.mem_map.mp hst
  cases hm

theorem decidedView_restoredSteps {P : Persisted} {v : ViewNumber} : ¬ P.restoredSteps.DecidedView v := by
  rintro ⟨st, hst, blocks, c1, c2, b, hm, -⟩
  obtain ⟨x, -, rfl⟩ := List.mem_map.mp hst
  cases hm

end History

/-! ## What the proof reads of a resumption -/

/--
What `safe_across_resume` reads of the stored state `P`, the history `pre` before
the crash and the history `post` after it: `Summarises`, with the anchor held by
each step after the crash that acts (`Step.Acts`) rather than before the crash.

A node that held its anchor before the crash meets it (`Summarises.resumes`), and
so may one that learns its anchor after the crash, before it acts on it.
-/
structure Resumes (cfg : Config) (P : Persisted) (pre post : History) : Prop where
  /-- The stored anchor is no earlier than the configured one. -/
  anchorAfter : cfg.anchorView ≤ P.anchorView

  /-- Each step that acts holds the anchor's `Cert1` by then. -/
  anchorCertHeld : ∀ n st, post[n]? = some st → st.Acts →
    (pre ++ P.restoredSteps ++ post.upTo (n + 1)).HasCert1 cfg P.anchorCert

  /-- Each step that acts holds the anchor's proposal by then. -/
  anchorHeld : ∀ n st, post[n]? = some st → st.Acts →
    (pre ++ P.restoredSteps ++ post.upTo (n + 1)).HasProposal cfg P.anchorBlock

  /--
  A vote2 over a block the node holds at the anchor's view with the anchor's
  payload comes after the anchor's payload.
  -/
  anchorPayload : ∀ n (v : Vote2) (b : Block), post.SentAt n (.vote2 v) →
    (pre ++ P.restoredSteps ++ post.upTo (n + 1)).HasProposal cfg b →
    b.viewNumber = P.anchorView → b.payloadCommit = P.anchorBlock.payloadCommit →
    v.data = ⟨blockHash b, b.epoch, b.blockHeader.blockNumber⟩ →
    (pre ++ P.restoredSteps ++ post.upTo (n + 1)).HasPayload cfg P.anchorView P.anchorBlock.payloadCommit

  /--
  The anchor's `Cert2` under the restart's configuration is the original one, or
  each step that acts holds a `Cert2` over the anchor by then, at a view before
  `Persisted.firstAllowed`.
  -/
  anchorCert2Held : ∀ n st, post[n]? = some st → st.Acts →
    (P.config cfg).anchorCert2 = cfg.anchorCert2
      ∨ ∃ c2, (pre ++ P.restoredSteps ++ post.upTo (n + 1)).HasCert2 c2
        ∧ c2.data = P.anchorCert.data.toVote2 ∧ c2.view < P.firstAllowed

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

/-! ## Lifting what the restarted node holds -/

section Lift

variable {cfg : Config} {P : Persisted} {pre post q : History}

/-- The restarted history received what the spliced one did: the outputs dropped are not inputs. -/
theorem received_restart {i : Input} :
    (pre.inputsOnly ++ P.restoredSteps ++ q).Received i ↔ (pre ++ P.restoredSteps ++ q).Received i := by
  simp only [History.received_append, History.received_inputsOnly]

/-- What the node received before the crash, the spliced history received. -/
theorem received_restart_of_pre {i : Input} (h : pre.Received i) : (pre ++ P.restoredSteps ++ q).Received i :=
  History.received_append.mpr (Or.inl (History.received_append.mpr (Or.inl h)))

/-- A `Cert2` the node held before the crash, the spliced history holds. -/
theorem hasCert2_restart_of_pre {c : Cert2} (h : pre.HasCert2 c) : (pre ++ P.restoredSteps ++ q).HasCert2 c := by
  rcases h with h | ⟨c1, p, h⟩
  · exact Or.inl (received_restart_of_pre h)
  · exact Or.inr ⟨c1, p, received_restart_of_pre h⟩

/-- A `Cert1` the node held before the crash, the spliced history holds. -/
theorem hasCert1_restart_of_pre {c : Cert1} (h : pre.HasCert1 cfg c) : (pre ++ P.restoredSteps ++ q).HasCert1 cfg c := by
  rcases h with h | h | ⟨c2, p, h⟩
  · exact Or.inl h
  · exact Or.inr (Or.inl (received_restart_of_pre h))
  · exact Or.inr (Or.inr ⟨c2, p, received_restart_of_pre h⟩)

/-- A proposal the node held before the crash, the spliced history holds. -/
theorem hasProposal_restart_of_pre {b : Block} (h : pre.HasProposal cfg b) :
    (pre ++ P.restoredSteps ++ q).HasProposal cfg b := by
  rcases h with h | ⟨s, share, h⟩ | ⟨c1, c2, h⟩
  · exact Or.inl h
  · exact Or.inr (Or.inl ⟨s, share, received_restart_of_pre h⟩)
  · exact Or.inr (Or.inr ⟨c1, c2, received_restart_of_pre h⟩)

/-- A payload the node held before the crash, the spliced history holds. -/
theorem hasPayload_restart_of_pre {v : ViewNumber} {pc : PayloadCommit} (h : pre.HasPayload cfg v pc) :
    (pre ++ P.restoredSteps ++ q).HasPayload cfg v pc := by
  rcases h with h | h
  · exact Or.inl h
  · exact Or.inr (received_restart_of_pre h)

/-- A `Cert1` the restarted history holds, the spliced one holds, given that it holds the stored anchor's. -/
theorem hasCert1_of_restart (hheld : (pre ++ P.restoredSteps ++ q).HasCert1 cfg P.anchorCert) {c : Cert1}
    (h : (pre.inputsOnly ++ P.restoredSteps ++ q).HasCert1 (P.config cfg) c) :
    (pre ++ P.restoredSteps ++ q).HasCert1 cfg c := by
  rcases h with h | h | ⟨c2, p, h⟩
  · obtain rfl : c = P.anchorCert := h
    exact hheld
  · exact Or.inr (Or.inl (received_restart.mp h))
  · exact Or.inr (Or.inr ⟨c2, p, received_restart.mp h⟩)

/-- A proposal the restarted history holds, the spliced one holds, given that it holds the stored anchor's. -/
theorem hasProposal_of_restart (hheld : (pre ++ P.restoredSteps ++ q).HasProposal cfg P.anchorBlock) {b : Block}
    (h : (pre.inputsOnly ++ P.restoredSteps ++ q).HasProposal (P.config cfg) b) :
    (pre ++ P.restoredSteps ++ q).HasProposal cfg b := by
  rcases h with h | ⟨s, share, h⟩ | ⟨c1, c2, h⟩
  · obtain rfl : b = P.anchorBlock := h
    exact hheld
  · exact Or.inr (Or.inl ⟨s, share, received_restart.mp h⟩)
  · exact Or.inr (Or.inr ⟨c1, c2, received_restart.mp h⟩)

/-- A `Cert2` the restarted history holds, the spliced one holds. -/
theorem hasCert2_of_restart {c : Cert2} (h : (pre.inputsOnly ++ P.restoredSteps ++ q).HasCert2 c) :
    (pre ++ P.restoredSteps ++ q).HasCert2 c := by
  rcases h with h | ⟨c1, p, h⟩
  · exact Or.inl (received_restart.mp h)
  · exact Or.inr ⟨c1, p, received_restart.mp h⟩

/--
A payload the restarted history holds, the spliced one holds too, given the anchor's
payload where it is read.
-/
theorem hasPayload_of_restart {v : ViewNumber} {pc : PayloadCommit}
    (hanchor : v = P.anchorView → pc = P.anchorBlock.payloadCommit →
      (pre ++ P.restoredSteps ++ q).HasPayload cfg P.anchorView P.anchorBlock.payloadCommit)
    (h : (pre.inputsOnly ++ P.restoredSteps ++ q).HasPayload (P.config cfg) v pc) :
    (pre ++ P.restoredSteps ++ q).HasPayload cfg v pc := by
  rcases h with ⟨hv, hpc⟩ | h
  · rw [hv, hpc]
    exact hanchor hv hpc
  · exact Or.inr (received_restart.mp h)

/--
A proposal at `Persisted.firstAllowed` or later that opens an epoch under the
restart's configuration does so under the original one: the spliced history holds
a `Cert2` over the stored anchor, or it is the configured one.
-/
theorem opensEpochJustified_of_restart
    (hheld : (pre ++ P.restoredSteps ++ q).HasProposal cfg P.anchorBlock)
    (hc2 : (P.config cfg).anchorCert2 = cfg.anchorCert2
      ∨ ∃ c2, (pre ++ P.restoredSteps ++ q).HasCert2 c2 ∧ c2.data = P.anchorCert.data.toVote2
        ∧ c2.view < P.firstAllowed)
    {p : Proposal} (hv : P.firstAllowed ≤ p.viewNumber)
    (h : OpensEpochJustified (P.config cfg) (pre.inputsOnly ++ P.restoredSteps ++ q) p) :
    OpensEpochJustified cfg (pre ++ P.restoredSteps ++ q) p := by
  intro henter
  obtain ⟨⟨parent, hp, hpv, hph⟩, c2, hc2', hlt, hdata⟩ := h henter
  refine ⟨⟨parent, hasProposal_of_restart hheld hp, hpv, hph⟩, ?_⟩
  rcases hc2' with hc2' | rfl
  · exact ⟨c2, Or.inl (hasCert2_of_restart hc2'), hlt, hdata⟩
  · rcases hc2 with heq | ⟨c2', hc2', hd', hv'⟩
    · exact ⟨_, Or.inr heq, hlt, hdata⟩
    · exact ⟨c2', Or.inl hc2', Nat.lt_of_lt_of_le hv' hv, hd'.trans hdata⟩

/-- A proposal the spliced history holds was held before the crash or received after it. -/
theorem hasProposal_pre_post {n : Nat} {b : Block}
    (h : (pre ++ P.restoredSteps ++ post.upTo n).HasProposal cfg b) :
    (pre ++ post).HasProposal cfg b := by
  have hrec : ∀ i, (pre ++ P.restoredSteps ++ post.upTo n).Received i →
      (∀ v pc, i ≠ .blockReconstructed v pc) → (pre ++ post).Received i := by
    intro i hi hne
    rcases History.received_append.mp hi with hi | ⟨st, hst, e⟩
    · rcases History.received_append.mp hi with hi | hi
      · exact History.received_append.mpr (Or.inl hi)
      · obtain ⟨x, -, rfl⟩ := History.received_restoredSteps.mp hi
        exact absurd rfl (hne _ _)
    · exact History.received_append.mpr (Or.inr ⟨st, List.mem_of_mem_take hst, e⟩)
  rcases h with h | ⟨s, share, h⟩ | ⟨c1, c2, h⟩
  · exact Or.inl h
  · exact Or.inr (Or.inl ⟨s, share, hrec _ h (by intro _ _ e; cases e)⟩)
  · exact Or.inr (Or.inr ⟨c1, c2, hrec _ h (by intro _ _ e; cases e)⟩)

/-- A faithful summary meets what the proof reads: what the node held before the crash, every later step holds. -/
theorem Summarises.resumes (hs : Summarises cfg P pre post) : Resumes cfg P pre post :=
  { anchorAfter := hs.anchorAfter
    anchorCertHeld := fun _ _ _ _ => hasCert1_restart_of_pre hs.anchorCertHeld
    anchorHeld := fun _ _ _ _ => hasProposal_restart_of_pre hs.anchorHeld
    anchorPayload := fun _ v b ⟨st, hst, hm⟩ hb hbv hbpc hd =>
      hasPayload_restart_of_pre
        (hs.anchorPayload v b ⟨st, List.mem_of_getElem? hst, hm⟩ (hasProposal_pre_post hb) hbv hbpc hd)
    anchorCert2Held := fun _ _ _ _ =>
      hs.anchorCert2Held.imp id fun ⟨c2, h, hd, hv⟩ => ⟨c2, hasCert2_restart_of_pre h, hd, hv⟩
    vote1Below := hs.vote1Below
    vote2Below := hs.vote2Below
    timeoutBelow := hs.timeoutBelow
    vote2Locked := hs.vote2Locked
    decidedBelow := hs.decidedBelow }

end Lift

/-- Lock order is transitive. -/
theorem EpochViewLE.trans {e e' e'' : EpochNumber} {v v' v'' : ViewNumber} :
    EpochViewLE e v e' v' → EpochViewLE e' v' e'' v'' → EpochViewLE e v e'' v'' := by
  rintro (h1 | ⟨rfl, h1⟩) (h2 | ⟨rfl, h2⟩)
  · exact Or.inl (Nat.lt_trans h1 h2)
  · exact Or.inl h1
  · exact Or.inl h2
  · exact Or.inr ⟨rfl, Nat.le_trans h1 h2⟩

/-! ## Where a step of the splice sits -/

namespace Persisted

variable {P : Persisted} {pre q : History}

/-- A restored step sends nothing and decides nothing. -/
theorem output_restoredSteps {st : Step} (h : st ∈ P.restoredSteps) : st.output = [] := by
  obtain ⟨x, -, rfl⟩ := List.mem_map.mp h
  rfl

theorem highestVoted_lt_firstAllowed (P : Persisted) : P.highestVoted.toNat < P.firstAllowed.toNat := by
  unfold firstAllowed
  show _ < (if P.anchorView + 1 ≤ P.highestVoted + 1 then P.highestVoted + 1 else P.anchorView + 1).toNat
  split
  · exact Nat.lt_succ_self _
  · rename_i h
    have h : ¬ P.anchorView.toNat + 1 ≤ P.highestVoted.toNat + 1 := h
    show _ < P.anchorView.toNat + 1
    omega

/-- A view at or before `Persisted.highestVoted` is before one at `Persisted.firstAllowed` or later. -/
theorem lt_of_below_after {a b : ViewNumber} (ha : a ≤ P.highestVoted) (hb : P.firstAllowed ≤ b) : a < b :=
  Nat.lt_of_le_of_lt ha (Nat.lt_of_lt_of_le P.highestVoted_lt_firstAllowed hb)

/-- A step of the splice is one of the history before the crash, a restored one, or one after. -/
theorem getElem?_splice {n : Nat} {st : Step} (h : (pre ++ P.restoredSteps ++ q)[n]? = some st) :
    (n < pre.length ∧ pre[n]? = some st) ∨ st.output = []
      ∨ ∃ k, n = pre.length + P.restoredSteps.length + k ∧ q[k]? = some st := by
  rcases Nat.lt_or_ge n pre.length with h1 | h1
  · left
    rw [List.append_assoc, List.getElem?_append_left h1] at h
    exact ⟨h1, h⟩
  · rcases Nat.lt_or_ge n (pre.length + P.restoredSteps.length) with h2 | h2
    · right; left
      rw [List.getElem?_append_left (by simp; omega), List.getElem?_append_right h1] at h
      exact output_restoredSteps (List.mem_of_getElem? h)
    · right; right
      refine ⟨n - (pre.length + P.restoredSteps.length), by omega, ?_⟩
      rw [List.getElem?_append_right (by simp; omega)] at h
      simpa using h

theorem getElem?_restarted (k : Nat) :
    (pre.inputsOnly ++ P.restoredSteps ++ q)[pre.length + P.restoredSteps.length + k]? = q[k]? := by
  rw [List.getElem?_append_right (by simp [History.length_inputsOnly])]
  simp [History.length_inputsOnly]

theorem upTo_splice_pre {n : Nat} (h : n ≤ pre.length) : (pre ++ P.restoredSteps ++ q).upTo n = pre.upTo n := by
  rw [List.append_assoc]
  exact History.upTo_append_left h

theorem upTo_splice (k : Nat) :
    (pre ++ P.restoredSteps ++ q).upTo (pre.length + P.restoredSteps.length + k)
      = pre ++ P.restoredSteps ++ q.upTo k := by
  rw [show pre.length + P.restoredSteps.length = (pre ++ P.restoredSteps).length by simp]
  exact History.upTo_append_right _ _ _

theorem upTo_splice_succ (k : Nat) :
    (pre ++ P.restoredSteps ++ q).upTo (pre.length + P.restoredSteps.length + k + 1)
      = pre ++ P.restoredSteps ++ q.upTo (k + 1) :=
  upTo_splice (k + 1)

theorem upTo_restarted (k : Nat) :
    (pre.inputsOnly ++ P.restoredSteps ++ q).upTo (pre.length + P.restoredSteps.length + k)
      = pre.inputsOnly ++ P.restoredSteps ++ q.upTo k := by
  rw [show pre.length + P.restoredSteps.length = (pre.inputsOnly ++ P.restoredSteps).length by
    simp [History.length_inputsOnly]]
  exact History.upTo_append_right _ _ _

theorem upTo_restarted_succ (k : Nat) :
    (pre.inputsOnly ++ P.restoredSteps ++ q).upTo (pre.length + P.restoredSteps.length + k + 1)
      = pre.inputsOnly ++ P.restoredSteps ++ q.upTo (k + 1) :=
  upTo_restarted (k + 1)

theorem sent_splice {m : Message} : (pre ++ P.restoredSteps ++ q).Sent m ↔ pre.Sent m ∨ q.Sent m := by
  simp [History.sent_append, History.sent_restoredSteps]

theorem sent_restarted {m : Message} : (pre.inputsOnly ++ P.restoredSteps ++ q).Sent m ↔ q.Sent m := by
  simp [History.sent_append, History.sent_restoredSteps, History.sent_inputsOnly]

theorem decidedView_splice {v : ViewNumber} :
    (pre ++ P.restoredSteps ++ q).DecidedView v ↔ pre.DecidedView v ∨ q.DecidedView v := by
  simp [History.decidedView_append, History.decidedView_restoredSteps]

theorem decidedView_restarted {v : ViewNumber} :
    (pre.inputsOnly ++ P.restoredSteps ++ q).DecidedView v ↔ q.DecidedView v := by
  simp [History.decidedView_append, History.decidedView_restoredSteps, History.decidedView_inputsOnly]

end Persisted

/-- A history followed by steps that send and decide nothing obeys the signing rules if it does. -/
theorem SafeHistory.append_silent {cfg : Config} {node : PubKey} {honestIn : EpochNumber → Prop}
    {h q : History} (hs : SafeHistory cfg node honestIn h) (hq : ∀ st ∈ q, st.output = []) :
    SafeHistory cfg node honestIn (h ++ q) := by
  have hidx : ∀ {n st}, (h ++ q)[n]? = some st → (n < h.length ∧ h[n]? = some st) ∨ st.output = [] := by
    intro n st hst
    rcases Nat.lt_or_ge n h.length with hn | hn
    · rw [List.getElem?_append_left hn] at hst
      exact Or.inl ⟨hn, hst⟩
    · rw [List.getElem?_append_right hn] at hst
      exact Or.inr (hq _ (List.mem_of_getElem? hst))
  have hsent : ∀ {m}, (h ++ q).Sent m → h.Sent m := by
    intro m hm
    rcases History.sent_append.mp hm with hm | ⟨st, hst, hm⟩
    · exact hm
    · rw [hq st hst] at hm
      cases hm
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
  · intro n vote ⟨st, hst, hm⟩ he
    rcases hidx hst with ⟨hn, hst⟩ | hout
    · rw [History.upTo_append_left (Nat.succ_le_of_lt hn)]
      exact hs.vote1Justified n vote ⟨st, hst, hm⟩ he
    · simp [hout] at hm
  · intro v v' h1 h2
    exact hs.vote1Once v v' (hsent h1) (hsent h2)
  · intro n vote ⟨st, hst, hm⟩ he
    rcases hidx hst with ⟨hn, hst⟩ | hout
    · rw [History.upTo_append_left (Nat.succ_le_of_lt hn)]
      exact hs.vote2Justified n vote ⟨st, hst, hm⟩ he
    · simp [hout] at hm
  · intro v v' h1 h2
    exact hs.vote2Once v v' (hsent h1) (hsent h2)
  · intro n vote ⟨st, hst, hm⟩ he tv htv
    rcases hidx hst with ⟨hn, hst⟩ | hout
    · rw [History.upTo_append_left (Nat.succ_le_of_lt hn)] at htv
      exact hs.vote2BeforeTimeout n vote ⟨st, hst, hm⟩ he tv htv
    · simp [hout] at hm
  · intro n vote ⟨st, hst, hm⟩ he v2 hv2
    rcases hidx hst with ⟨hn, hst⟩ | hout
    · rw [History.upTo_append_left (Nat.le_of_lt hn)] at hv2
      exact hs.timeoutLock n vote ⟨st, hst, hm⟩ he v2 hv2
    · simp [hout] at hm
  · intro n st blocks c1 c2 hst hm he
    rcases hidx hst with ⟨hn, hst⟩ | hout
    · rw [History.upTo_append_left (Nat.succ_le_of_lt hn)]
      exact hs.decideJustified n st blocks c1 c2 hst hm he
    · simp [hout] at hm
  · intro n st i blocks c1 c2 hst hi he
    rcases hidx hst with ⟨hn, hst⟩ | hout
    · rw [History.upTo_append_left (Nat.le_of_lt hn)]
      exact hs.decideOnce n st i blocks c1 c2 hst hi he
    · simp [hout] at hi

/-! ## The splice obeys the signing rules -/

/--
**Safety across a resumption.** A history before a crash that obeys the signing
rules, followed by the steps the restart loads and a history after it, obeys them
as a whole, when the stored state meets `Resumes` for the two histories, the
history after obeys the rules under the restart's configuration behind the inputs
of the one before, and keeps the restart's bars.
-/
theorem safe_across_resume {cfg : Config} {node : PubKey} {honestIn : EpochNumber → Prop}
    {P : Persisted} {pre post : History}
    (hpre : SafeHistory cfg node honestIn pre) (hres : Resumes cfg P pre post)
    (hpost : SafeHistory (P.config cfg) node honestIn (pre.inputsOnly ++ P.restoredSteps ++ post))
    (hok : RestartOK P post) :
    SafeHistory cfg node honestIn (pre ++ P.restoredSteps ++ post) := by
  have hpost_at : ∀ {k : Nat} {st : Step}, post[k]? = some st →
      (pre.inputsOnly ++ P.restoredSteps ++ post)[pre.length + P.restoredSteps.length + k]? = some st :=
    fun hst => (Persisted.getElem?_restarted _).trans hst
  have hsentq : ∀ {k : Nat} {st : Step} {m : Message}, post[k]? = some st → Output.send m ∈ st.output → post.Sent m :=
    fun hst hm => ⟨_, List.mem_of_getElem? hst, hm⟩
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_, ?_, ?_⟩
  · intro n vote ⟨st, hst, hm⟩ he
    rcases Persisted.getElem?_splice hst with ⟨hn, hst⟩ | hout | ⟨k, rfl, hq⟩
    · rw [Persisted.upTo_splice_pre (Nat.succ_le_of_lt hn)]
      exact hpre.vote1Justified n vote ⟨st, hst, hm⟩ he
    · simp [hout] at hm
    · obtain ⟨hsig, hj⟩ := hpost.vote1Justified _ vote ⟨st, hpost_at hq, hm⟩ he
      have hafter := hok.vote1After vote (hsentq hq hm)
      rw [Persisted.upTo_restarted_succ] at hj
      rw [Persisted.upTo_splice_succ]
      refine ⟨hsig, ?_⟩
      rcases hj with ⟨sender, p, vid, hrec, hwf, hval, hsafe, hopens, hfor⟩
        | ⟨sender, r, hrec, hwf, hsafe, hagain⟩
      · have hv : P.firstAllowed ≤ p.viewNumber := by rw [← hfor.1]; exact hafter
        exact Or.inl ⟨sender, p, vid, received_restart.mp hrec,
          ⟨hwf.parentEarlier, hwf.covered, hwf.epoch, hwf.height⟩, hval, hsafe,
          opensEpochJustified_of_restart (hres.anchorHeld k st hq ⟨_, hm, rfl⟩)
            (hres.anchorCert2Held k st hq ⟨_, hm, rfl⟩) hv hopens, hfor⟩
      · exact Or.inr ⟨sender, r, received_restart.mp hrec, hwf, hsafe, hagain⟩
  · intro v v' h1 h2 he hep hview
    rcases Persisted.sent_splice.mp h1 with h1 | h1 <;> rcases Persisted.sent_splice.mp h2 with h2 | h2
    · exact hpre.vote1Once v v' h1 h2 he hep hview
    · have := Persisted.lt_of_below_after (hres.vote1Below v h1) (hok.vote1After v' h2)
      rw [hview] at this
      exact absurd this (Nat.lt_irrefl _)
    · have := Persisted.lt_of_below_after (hres.vote1Below v' h2) (hok.vote1After v h1)
      rw [hview] at this
      exact absurd this (Nat.lt_irrefl _)
    · exact hpost.vote1Once v v' (Persisted.sent_restarted.mpr h1) (Persisted.sent_restarted.mpr h2) he hep hview
  · intro n vote ⟨st, hst, hm⟩ he
    rcases Persisted.getElem?_splice hst with ⟨hn, hst⟩ | hout | ⟨k, rfl, hq⟩
    · rw [Persisted.upTo_splice_pre (Nat.succ_le_of_lt hn)]
      exact hpre.vote2Justified n vote ⟨st, hst, hm⟩ he
    · simp [hout] at hm
    · obtain ⟨hsig, hav, c, b, hc, hb, hcert, hpay, hview, hdata⟩ :=
        hpost.vote2Justified _ vote ⟨st, hpost_at hq, hm⟩ he
      rw [Persisted.upTo_restarted_succ] at hc hb hpay
      rw [Persisted.upTo_splice_succ]
      have hheld := hres.anchorHeld k st hq ⟨_, hm, rfl⟩
      refine ⟨hsig, Nat.lt_of_le_of_lt hres.anchorAfter hav, c, b,
        hasCert1_of_restart (hres.anchorCertHeld k st hq ⟨_, hm, rfl⟩) hc,
        hasProposal_of_restart hheld hb, hcert, hasPayload_of_restart ?_ hpay, hview, hdata⟩
      intro hbv hbpc
      exact hres.anchorPayload k vote b ⟨st, hq, hm⟩ (hasProposal_of_restart hheld hb) hbv hbpc
        (by rw [hdata, hcert.2]; rfl)
  · intro v v' h1 h2 he hep hview
    rcases Persisted.sent_splice.mp h1 with h1 | h1 <;> rcases Persisted.sent_splice.mp h2 with h2 | h2
    · exact hpre.vote2Once v v' h1 h2 he hep hview
    · have := Persisted.lt_of_below_after (hres.vote2Below v h1) (hok.vote2After v' h2)
      rw [hview] at this
      exact absurd this (Nat.lt_irrefl _)
    · have := Persisted.lt_of_below_after (hres.vote2Below v' h2) (hok.vote2After v h1)
      rw [hview] at this
      exact absurd this (Nat.lt_irrefl _)
    · exact hpost.vote2Once v v' (Persisted.sent_restarted.mpr h1) (Persisted.sent_restarted.mpr h2) he hep hview
  · intro n vote ⟨st, hst, hm⟩ he tv htv htve
    rcases Persisted.getElem?_splice hst with ⟨hn, hst⟩ | hout | ⟨k, rfl, hq⟩
    · rw [Persisted.upTo_splice_pre (Nat.succ_le_of_lt hn)] at htv
      exact hpre.vote2BeforeTimeout n vote ⟨st, hst, hm⟩ he tv htv htve
    · simp [hout] at hm
    · rw [Persisted.upTo_splice_succ] at htv
      rcases Persisted.sent_splice.mp htv with htv | htv
      · exact Persisted.lt_of_below_after (hres.timeoutBelow tv htv) (hok.vote2After vote (hsentq hq hm))
      · refine hpost.vote2BeforeTimeout _ vote ⟨st, hpost_at hq, hm⟩ he tv ?_ htve
        rw [Persisted.upTo_restarted_succ]
        exact Persisted.sent_restarted.mpr htv
  · intro n vote ⟨st, hst, hm⟩ he v2 hv2 hv2e
    rcases Persisted.getElem?_splice hst with ⟨hn, hst⟩ | hout | ⟨k, rfl, hq⟩
    · rw [Persisted.upTo_splice_pre (Nat.le_of_lt hn)] at hv2
      exact hpre.timeoutLock n vote ⟨st, hst, hm⟩ he v2 hv2 hv2e
    · simp [hout] at hm
    · rw [Persisted.upTo_splice] at hv2
      rcases Persisted.sent_splice.mp hv2 with hv2 | hv2
      · exact EpochViewLE.trans (hres.vote2Locked v2 hv2) (hok.lockKept vote (hsentq hq hm))
      · refine hpost.timeoutLock _ vote ⟨st, hpost_at hq, hm⟩ he v2 ?_ hv2e
        rw [Persisted.upTo_restarted]
        exact Persisted.sent_restarted.mpr hv2
  · intro n st blocks c1 c2 hst hm he
    rcases Persisted.getElem?_splice hst with ⟨hn, hst⟩ | hout | ⟨k, rfl, hq⟩
    · rw [Persisted.upTo_splice_pre (Nat.succ_le_of_lt hn)]
      exact hpre.decideJustified n st blocks c1 c2 hst hm he
    · simp [hout] at hm
    · obtain ⟨head, rest, hbl, hc2, hcommit, hc1, hcert, hlinked, hall⟩ :=
        hpost.decideJustified _ st blocks c1 c2 (hpost_at hq) hm he
      rw [Persisted.upTo_restarted_succ] at hc2 hc1 hall
      rw [Persisted.upTo_splice_succ]
      exact ⟨head, rest, hbl, hasCert2_of_restart hc2, hcommit,
        hasCert1_of_restart (hres.anchorCertHeld k st hq ⟨_, hm, rfl⟩) hc1, hcert,
        hlinked, fun b hb => ⟨hasProposal_of_restart (hres.anchorHeld k st hq ⟨_, hm, rfl⟩) (hall b hb).1,
          Nat.lt_of_le_of_lt hres.anchorAfter (hall b hb).2⟩⟩
  · intro n st i blocks c1 c2 hst hi he
    rcases Persisted.getElem?_splice hst with ⟨hn, hst⟩ | hout | ⟨k, rfl, hq⟩
    · rw [Persisted.upTo_splice_pre (Nat.le_of_lt hn)]
      exact hpre.decideOnce n st i blocks c1 c2 hst hi he
    · simp [hout] at hi
    · obtain ⟨hnd, hnot⟩ := hpost.decideOnce _ st i blocks c1 c2 (hpost_at hq) hi he
      obtain ⟨_, _, -, -, -, -, -, -, hall⟩ :=
        hpost.decideJustified _ st blocks c1 c2 (hpost_at hq) (List.mem_of_getElem? hi) he
      refine ⟨hnd, fun b hb hd => ?_⟩
      rw [Persisted.upTo_splice, List.append_assoc (pre ++ P.restoredSteps)] at hd
      rcases Persisted.decidedView_splice.mp hd with hd | hd
      · exact absurd (Nat.lt_of_le_of_lt (hres.decidedBelow _ hd) (hall b hb).2) (Nat.lt_irrefl _)
      · apply hnot b hb
        rw [Persisted.upTo_restarted, List.append_assoc (pre.inputsOnly ++ P.restoredSteps)]
        exact Persisted.decidedView_restarted.mpr hd

/--
**Safety across a restart.** A history before a crash that obeys the signing rules,
followed by the steps the restart loads and a history after it, obeys them as a
whole, when the stored summary is faithful to the history before, the history
after obeys the rules under the restart's configuration behind the inputs of the
one before, and keeps the restart's bars.
-/
theorem safe_across_restart {cfg : Config} {node : PubKey} {honestIn : EpochNumber → Prop}
    {P : Persisted} {pre post : History}
    (hpre : SafeHistory cfg node honestIn pre) (hsum : Summarises cfg P pre post)
    (hpost : SafeHistory (P.config cfg) node honestIn (pre.inputsOnly ++ P.restoredSteps ++ post))
    (hok : RestartOK P post) :
    SafeHistory cfg node honestIn (pre ++ P.restoredSteps ++ post) :=
  safe_across_resume hpre hsum.resumes hpost hok

/-! ## The spliced trace -/

/-- Up to the end of the prefix, the spliced trace is the prefix. -/
theorem Trace.history_splice_le {s : History} {r : Trace} {n : Nat} (h : n ≤ s.length) :
    (Trace.splice s r).history n = s.upTo n := by
  apply List.ext_getElem
  · simp [Trace.history, History.upTo, Nat.min_eq_left h]
  · intro i h1 h2
    have hi : i < s.length := by simp [Trace.history] at h1; omega
    simp [Trace.history, History.upTo, Trace.splice, hi]

/-- Past the end of the prefix, the spliced trace is the prefix followed by the trace. -/
theorem Trace.history_splice_add (s : History) (r : Trace) (k : Nat) :
    (Trace.splice s r).history (s.length + k) = s ++ r.history k := by
  apply List.ext_getElem
  · simp [Trace.history]
  · intro i h1 h2
    by_cases hi : i < s.length
    · simp [Trace.history, Trace.splice, hi, List.getElem_append_left hi]
    · simp [Trace.history, Trace.splice, hi, List.getElem_append_right (Nat.le_of_not_lt hi)]

/--
**Safety of a spliced trace.** Every prefix of the trace made of a history before a
crash, the steps the restart loads, and a trace after it obeys the signing rules,
when every prefix of each part meets the premises of `safe_across_restart`.
-/
theorem safe_trace_across_restart {cfg : Config} {node : PubKey} {honestIn : EpochNumber → Prop}
    {P : Persisted} {pre : History} {r : Trace}
    (hpre : ∀ n, SafeHistory cfg node honestIn (pre.upTo n))
    (hsum : ∀ n, Summarises cfg P pre (r.history n))
    (hpost : ∀ n, SafeHistory (P.config cfg) node honestIn
      (pre.inputsOnly ++ P.restoredSteps ++ r.history n))
    (hok : ∀ n, RestartOK P (r.history n)) :
    ∀ n, SafeHistory cfg node honestIn ((Trace.splice (pre ++ P.restoredSteps) r).history n) := by
  intro n
  have hsafe : SafeHistory cfg node honestIn pre := by
    have := hpre pre.length
    rwa [History.upTo, List.take_length] at this
  rcases Nat.lt_or_ge n (pre ++ P.restoredSteps).length with hn | hn
  · rw [Trace.history_splice_le (Nat.le_of_lt hn)]
    rcases Nat.lt_or_ge n pre.length with h1 | h1
    · rw [History.upTo_append_left (Nat.le_of_lt h1)]
      exact hpre n
    · obtain ⟨j, rfl⟩ : ∃ j, n = pre.length + j := ⟨n - pre.length, by omega⟩
      rw [History.upTo_append_right]
      exact hsafe.append_silent fun _ hst => Persisted.output_restoredSteps (List.mem_of_mem_take hst)
  · obtain ⟨k, rfl⟩ : ∃ k, n = (pre ++ P.restoredSteps).length + k := ⟨n - (pre ++ P.restoredSteps).length, by omega⟩
    rw [Trace.history_splice_add]
    exact safe_across_restart hsafe (hsum k) (hpost k) (hok k)

end NewProtocol
