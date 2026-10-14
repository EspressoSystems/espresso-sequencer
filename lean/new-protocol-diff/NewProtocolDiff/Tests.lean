module

public import NewProtocolDiff.Trace
public import NewProtocolDiff.Replay
public import NewProtocolDiff.Corpus
public import NewProtocolDiff.Restart
public meta import NewProtocolSpec.Interface
public meta import NewProtocolImpl.Machine
public meta import NewProtocolDiff.Json
public meta import NewProtocolDiff.Trace
public meta import NewProtocolDiff.Replay
public meta import NewProtocolDiff.Corpus
public meta import NewProtocolDiff.Header
public meta import NewProtocolDiff.Check
public meta import NewProtocolDiff.Restart

/-!
# Checks on the harness itself

A comparison harness that silently mis-reads its input is worse than none, so
the format is checked at build time in both directions:

* the identity injection does not collide and preserves the recorder's text, and
  every constructor round-trips through JSON — which catches an instance the
  deriving got wrong as well as one a future hand-written override breaks;
* the comparison accepts a run the machine agrees with, and rejects one it does
  not — including the case that matters most, a recorded identity that does not
  match the block, since that is how a hash-namespace mistake would show up;
* a restarted pair that keeps its summary and bars passes, and each premise
  broken alone is reported by its name.
-/

@[expose] public section

namespace NewProtocolDiff
namespace Tests

open Lean NewProtocol

-- `NewProtocol.Event` carries no equality of its own, nothing in the
-- specification comparing two events. The round-trip check needs one, so it is
-- derived here rather than the specification growing an instance for a test.
deriving instance DecidableEq for Event

/-! ## Identities

The injection a recorder's strings go through. Checked on the shapes a recorder
actually emits — base64 and hex of a 32-byte value — and on the property that
matters, which is that different text never collides, including across lengths
and outside ASCII.
-/

private def identityTexts : List String :=
  [ "", "A", "AA", "AAA", "aA", "Aa", "0", "/", "+", "café",
    "3q2+7wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
    "deadbeef00000000000000000000000000000000000000000000000000000000" ]

/-- info: true -/
#guard_msgs in
#eval
  let nats := identityTexts.map identToNat
  (nats.eraseDups).length == identityTexts.length

-- Text in, the same text out: a trace this package re-renders is the one it read.
/-- info: true -/
#guard_msgs in
#eval identityTexts.all fun t => natToIdent (identToNat t) == some t

-- A number naming no string still survives, going out as a number instead.
/-- info: true -/
#guard_msgs in
#eval
  let n := 2 ^ 200
  match natToIdent n with
  | some s => identToNat s == n
  | none => match cryptoFromJson (cryptoToJson n) with
    | .ok m => m == n
    | .error _ => false

/-! ## Round trip -/

/-- A lock for the timeout votes and certificates below. -/
private def lk : Cert1 := ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩

private def prop1 : Proposal :=
  ⟨⟨⟨42⟩, 1⟩, ⟨1⟩, ⟨0⟩, ⟨⟨⟨1⟩, ⟨0⟩, 0⟩, ViewNumber.genesis⟩, none, ⟨3⟩⟩

private def prop2 : Proposal :=
  ⟨⟨⟨8⟩, 2⟩, ⟨5⟩, ⟨0⟩, ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩, some ⟨⟨⟨0⟩, lk⟩, ⟨4⟩⟩, ⟨9⟩⟩

/-- Every input constructor, with `timeoutEvidence` present and absent. -/
private def inputs : List Input :=
  [ .blockReconstructed ⟨1⟩ ⟨42⟩,
    .certificate1 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩,
    .certificate2 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩,
    .headerBuilt ⟨2⟩ ⟨3⟩ ⟨⟨7⟩, 2⟩,
    .proposal ⟨1⟩ prop1 (some ⟨⟨1⟩, ⟨42⟩⟩),
    .proposal ⟨2⟩ prop2 (some ⟨⟨5⟩, ⟨8⟩⟩),
    .blockValidated ⟨1⟩ ⟨3⟩,
    .timeout ⟨4⟩,
    .timeoutCertificate ⟨⟨⟨0⟩, lk⟩, ⟨4⟩⟩,
    .timeoutOneHonest ⟨6⟩,
    .epochChange ⟨⟨⟨9⟩, ⟨1⟩, 2⟩, ⟨5⟩⟩ ⟨⟨⟨9⟩, ⟨1⟩, 2⟩, ⟨5⟩⟩ prop2,
    .revote ⟨1⟩ ⟨⟨⟨⟨9⟩, ⟨1⟩, 2⟩, ⟨5⟩⟩, ⟨6⟩, none⟩,
    .revote ⟨1⟩ ⟨⟨⟨⟨9⟩, ⟨1⟩, 2⟩, ⟨5⟩⟩, ⟨7⟩, some ⟨⟨⟨1⟩, lk⟩, ⟨6⟩⟩⟩ ]

/-- Every output constructor. -/
private def outputs : List Output :=
  [ .send (.proposal prop1),
    .send (.vote1 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩, ⟨1⟩⟩),
    .send (.vote2 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩, ⟨1⟩⟩),
    .send (.timeoutVote ⟨⟨⟨0⟩, lk⟩, ⟨4⟩, ⟨1⟩⟩),
    .send (.timeoutCert ⟨⟨⟨0⟩, lk⟩, ⟨4⟩⟩),
    .send (.cert1 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩),
    .send (.cert2 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩),
    .send (.vidShare ⟨⟨1⟩, ⟨42⟩⟩),
    .send (.epochChange ⟨⟨⟨9⟩, ⟨1⟩, 2⟩, ⟨5⟩⟩ ⟨⟨⟨9⟩, ⟨1⟩, 2⟩, ⟨5⟩⟩ prop2),
    .send (.revote ⟨⟨⟨⟨9⟩, ⟨1⟩, 2⟩, ⟨5⟩⟩, ⟨7⟩, some ⟨⟨⟨1⟩, lk⟩, ⟨6⟩⟩⟩),
    .decided [] ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩ ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩,
    .decided [prop2, prop1] ⟨⟨⟨9⟩, ⟨0⟩, 2⟩, ⟨5⟩⟩ ⟨⟨⟨9⟩, ⟨0⟩, 2⟩, ⟨5⟩⟩ ]

/-- Through `toJson`, the text, and back — the whole path a trace file takes. -/
private def roundTrips {α : Type} [DecidableEq α] [ToJson α] [FromJson α]
    (xs : List α) : Bool :=
  xs.all fun x =>
    match Json.parse (toJson x).compress >>= (fromJson? · : Json → Except String α) with
    | .ok y => y == x
    | .error _ => false

private def inputRoundTrips : Bool := roundTrips inputs

private def outputRoundTrips : Bool := roundTrips outputs

/-- info: true -/
#guard_msgs in #eval inputRoundTrips

/-- info: true -/
#guard_msgs in #eval outputRoundTrips

/-- A whole trace survives writing and reading, collections and all. -/
private def trace : List Event :=
  [ .consensus (.certificate1 ⟨⟨⟨1⟩, ⟨0⟩, 0⟩, ViewNumber.genesis⟩) [],
    .consensus (.proposal ⟨1⟩ prop1 (some ⟨⟨1⟩, ⟨42⟩⟩)) [.send (.vote1 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩, ⟨1⟩⟩)],
    .collect,
    .consensus (.timeout ⟨4⟩)
      [.send (.timeoutVote ⟨⟨⟨0⟩, lk⟩, ⟨4⟩, ⟨1⟩⟩), .send (.cert1 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩)] ]

/-- info: true -/
#guard_msgs in
#eval match parseTrace (renderTrace trace) with
  | .ok es => es == trace
  | .error _ => false

/-- A 32-byte identity survives the JSON path, which a plain number could not. -/
private def wide : Proposal :=
  ⟨⟨⟨2 ^ 250 + 7⟩, 1⟩, ⟨1⟩, ⟨0⟩, ⟨⟨⟨2 ^ 255 - 1⟩, ⟨0⟩, 0⟩, ViewNumber.genesis⟩, none, ⟨2 ^ 248 + 3⟩⟩

/-- info: true -/
#guard_msgs in #eval roundTrips [Input.proposal ⟨2 ^ 200⟩ wide (some ⟨⟨1⟩, ⟨2 ^ 250 + 7⟩⟩)]

-- A hand-written trace may use plain numbers where a recorder writes base64.
/-- info: true -/
#guard_msgs in
#eval match Json.parse "{\"data\":{\"blockHash\":3,\"epoch\":0,\"blockNumber\":1},\"view\":1}" >>=
    (fromJson? · : Json → Except String Cert1) with
  | .ok c => c == (⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩⟩ : Cert1)
  | .error _ => false

/-! ## The comparison

The same run three ways: as the machine plays it, missing the validity report
the vote depends on, and with an identity that does not match the block. The
last is the namespace mistake a structural hash would have caused everywhere.
-/

private def anchor : Block :=
  ⟨⟨⟨0⟩, 0⟩, ViewNumber.genesis, ⟨0⟩, ⟨⟨⟨1⟩, ⟨0⟩, 0⟩, ViewNumber.genesis⟩, none, ⟨1⟩⟩

private def cfg : Config := ⟨anchor, ⟨⟨⟨1⟩, ⟨0⟩, 0⟩, ViewNumber.genesis⟩, 20, 0⟩
private def me : PubKey := ⟨1⟩

private def run (es : List Event) : String :=
  (replay cfg (fun _ _ => some me) me es).report

private def opening : List Event :=
  [ .consensus (.certificate1 ⟨⟨⟨1⟩, ⟨0⟩, 0⟩, ViewNumber.genesis⟩) [],
    .consensus (.proposal ⟨1⟩ prop1 (some ⟨⟨1⟩, ⟨42⟩⟩)) [],
    .consensus (.blockReconstructed ViewNumber.genesis ⟨7⟩) [] ]

private def voteStep (identity : BlockHash) : Event :=
  .consensus (.blockValidated ⟨1⟩ identity)
    [.send (.vote1 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩, me⟩), .send (.vidShare ⟨⟨1⟩, ⟨42⟩⟩)]

/-- info: "OK: 4 steps, marks agree exactly" -/
#guard_msgs in #eval run (opening ++ [voteStep ⟨3⟩])

/-- info: "DIVERGED after 3 steps\n  step 3: the recording voted1 in view 1 of epoch 0, the machine did not" -/
#guard_msgs in
#eval run (opening ++ [.consensus (.timeout ⟨9⟩) [.send (.vote1 ⟨⟨⟨3⟩, ⟨0⟩, 1⟩, ⟨1⟩, me⟩)]])

/-- info: "DIVERGED after 3 steps\n  step 3: the recording voted1 in view 1 of epoch 0, the machine did not" -/
#guard_msgs in #eval run (opening ++ [voteStep ⟨999⟩])

/-! ### Collecting

A collection prunes the machine and must leave what the run still owes. Nothing
records one — see `NewProtocolDiff.Trace` — so without a trace written here the
`collect` arm of `replay` would be reached by nothing at all, and could rot
unnoticed.
-/

/-- info: "OK: 5 steps, marks agree exactly" -/
#guard_msgs in #eval run (opening ++ [.collect, voteStep ⟨3⟩])

/-! ## A boundary of the model is not a failure

A header whose payload commitment is `null` is a block from before a version
boundary. The parser refuses it, and the refusal has to read as out of scope
rather than as a broken trace, or every recorded cutover run fails the build.

The two lines below are written out rather than encoded from the types, because
neither is a value the types admit — that is what makes them worth testing. Kept
on one line each: `parseTrace` splits on newlines, so a step broken across two
would be malformed for the wrong reason and the first test would pass without
touching the case it names.
-/

/-- A step delivering a proposal whose header carries no payload commitment. -/
private def preCutoverLine : String :=
  r#"{"consensus":{"input":{"proposal":{"sender":1,"p":{"blockHeader":"# ++
  r#"{"payloadCommit":null},"viewNumber":1,"parentCert":{"data":"# ++
  r#"{"blockHash":7},"view":0},"timeoutEvidence":null,"identity":9},"# ++
  r#""share":{"view":1,"payloadCommit":3}}},"output":[]}}"#

/-- A step that is simply incomplete. -/
private def brokenLine : String :=
  r#"{"consensus":{"input":{"proposal":{"sender":1}}}}"#

/-- How a parse failure classifies: the boundary of the model, or a bad recording. -/
private def classify (line : String) : String :=
  match parseTrace line with
  | .ok _ => "parsed, which it should not"
  | .error e => if (parseOutOfScope e).isSome then "out of scope" else "malformed"

/-- info: "out of scope" -/
#guard_msgs in #eval classify preCutoverLine

/-- info: "malformed" -/
#guard_msgs in #eval classify brokenLine

/-! ## Across a restart

One node's run before and after a crash. Before: it votes1 and votes2 at view 1,
decides it, votes1 at view 2, times out view 3 under the lock at view 1, and
receives the proposal at view 4. Its summary: the anchor at view 1, the lock at
view 1, `highestVoted` 3, and the payload of view 2 marked as held. After: it
votes1 and votes2 at view 5 and times out view 6 under the lock at view 5.

Identities are written as numbers rather than read through `blockHash`, which is
opaque to `decide`.
-/

/-- The block at view `v` and height `n` on `parent`, with identity `100 + v` and payload `10 + v`. -/
private def blk (v n : Nat) (parent : Cert1) : Block := ⟨⟨⟨10 + v⟩, ⟨n⟩⟩, ⟨v⟩, ⟨0⟩, parent, none, ⟨100 + v⟩⟩

/-- A `Cert1` over `b` formed at view `v`. -/
private def certOn (b : Block) (v : Nat) : Cert1 := ⟨⟨b.identity, b.epoch, b.blockHeader.blockNumber⟩, ⟨v⟩⟩

private def b1 : Block := blk 1 1 cfg.anchorCert
private def c1 : Cert1 := certOn b1 1
private def b2 : Block := blk 2 2 c1
private def b4 : Block := blk 4 3 (certOn b2 2)
private def c4 : Cert1 := certOn b4 4
private def b5 : Block := blk 5 4 c4
private def c5 : Cert1 := certOn b5 5

/-- The `Cert2` matching a `Cert1`. -/
private def commit (c : Cert1) : Cert2 := ⟨c.data.toVote2, c.view⟩

/-- A step. -/
private def st (i : Input) (out : List Output := []) : Step := ⟨i, out⟩

/-- Receive `b` with this node's share. -/
private def recv (b : Block) : Step := st (.proposal ⟨2⟩ b (some ⟨b.viewNumber, b.payloadCommit⟩))

private def vote1 (b : Block) : Output :=
  .send (.vote1 ⟨⟨b.identity, b.epoch, b.blockHeader.blockNumber⟩, b.viewNumber, me⟩)

private def vote2 (c : Cert1) : Output := .send (.vote2 ⟨c.data.toVote2, c.view, me⟩)

private def timeoutVote (v : Nat) (lock : Cert1) : Output := .send (.timeoutVote ⟨⟨⟨0⟩, lock⟩, ⟨v⟩, me⟩)

private def preRun : History :=
  [ st (.certificate1 c1), recv b1,
    st (.blockValidated ⟨1⟩ b1.identity) [vote1 b1],
    st (.blockReconstructed ⟨1⟩ b1.payloadCommit) [vote2 c1],
    st (.certificate2 (commit c1)) [.decided [b1] c1 (commit c1)],
    recv b2,
    st (.blockValidated ⟨2⟩ b2.identity) [vote1 b2],
    st (.timeout ⟨3⟩) [timeoutVote 3 c1],
    recv b4 ]

private def persisted : Persisted := ⟨b1, c1, c1, ⟨3⟩, [(⟨2⟩, b2.payloadCommit)]⟩

private def postRun : History :=
  [ st (.certificate1 c4), recv b5,
    st (.blockValidated ⟨5⟩ b5.identity) [vote1 b5],
    st (.certificate1 c5),
    st (.blockReconstructed ⟨5⟩ b5.payloadCommit) [vote2 c5],
    st (.timeout ⟨6⟩) [timeoutVote 6 c5] ]

/-- The premises of a restart that fail. -/
private def failing (P : Persisted) (pre post : History) : List String :=
  (checkRestart cfg P pre post).filterMap fun (n, ok) => if ok then none else some n

/-- The signing rules the history after a restart breaks under the restart's configuration. -/
private def failingAfter (P : Persisted) (pre post : History) : List String :=
  (checkRules (P.config cfg) me (pre.inputsOnly ++ P.restoredSteps ++ post)).filterMap
    fun (n, ok) => if ok then none else some n

/-- info: [] -/
#guard_msgs in #eval failing persisted preRun postRun

/-- info: true -/
#guard_msgs in #eval checkSafe cfg me preRun

/-- info: [] -/
#guard_msgs in #eval failingAfter persisted preRun postRun

/-- The premises of `NewProtocol.safe_across_restart` can be met. -/
example : Summarises cfg persisted preRun postRun ∧ RestartOK persisted postRun :=
  ⟨checkSummarises_sound (by decide), checkRestartOK_sound (by decide)⟩

-- Each premise broken alone is reported, and only it.
/-- info: ["bars:vote2After"] -/
#guard_msgs in
#eval failing persisted preRun (postRun ++ [st (.timeout ⟨7⟩) [vote2 (certOn b2 3)]])

/-- info: ["bars:lockKept"] -/
#guard_msgs in
#eval failing persisted preRun (postRun ++ [st (.timeout ⟨7⟩) [timeoutVote 7 cfg.anchorCert]])

/-- info: ["summary:timeoutBelow"] -/
#guard_msgs in #eval failing { persisted with highestVoted := ⟨2⟩ } preRun postRun

/-- info: ["summary:vote2Locked"] -/
#guard_msgs in #eval failing { persisted with lock := cfg.anchorCert } preRun postRun

/-- info: ["summary:decidedBelow"] -/
#guard_msgs in
#eval failing persisted (preRun ++ [st (.certificate2 (commit c4)) [.decided [b4] c4 (commit c4)]])
  postRun

/-- info: ["summary:reconstructedHeld"] -/
#guard_msgs in #eval failing { persisted with reconstructed := [(⟨7⟩, ⟨77⟩)] } preRun postRun

/-! ### Edge cases -/

-- A re-vote's certificate over the anchor, at a later view than the block, still finds it.
/-- info: true -/
#guard_msgs in #eval restartAnchor cfg preRun ⟨c1.data, ⟨3⟩⟩ == some b1

/-- info: [] -/
#guard_msgs in
#eval failing { persisted with anchorCert := ⟨c1.data, ⟨3⟩⟩ }
  (preRun ++ [st (.certificate1 ⟨c1.data, ⟨3⟩⟩), st (.certificate2 (commit ⟨c1.data, ⟨3⟩⟩))]) postRun

-- A timeout vote pending at the crash leaves `highestVoted` above the last vote sent.
/-- info: [] -/
#guard_msgs in #eval failing { persisted with highestVoted := ⟨4⟩ } preRun postRun

-- A timeout vote naming the stored lock itself keeps it.
/-- info: [] -/
#guard_msgs in #eval failing persisted preRun (postRun ++ [st (.timeout ⟨7⟩) [timeoutVote 7 c1]])

-- A vote2 on a proposal whose payload only the restart marks as held.
/-- info: ([], []) -/
#guard_msgs in
#eval
  let P := { persisted with reconstructed := persisted.reconstructed ++ [(⟨4⟩, b4.payloadCommit)] }
  let post := [st (.certificate1 c4) [vote2 c4]]
  (failing P preRun post, failingAfter P preRun post)

/-- info: ["vote2Justified"] -/
#guard_msgs in #eval failingAfter persisted preRun [st (.certificate1 c4) [vote2 c4]]

-- The anchor decided without its payload: owed only once a vote2 after the crash reads it.
private def preNoPayload : History :=
  preRun.map fun s =>
    if s.input = .blockReconstructed ⟨1⟩ b1.payloadCommit then { s with input := .timeout ⟨1⟩ } else s

/-- info: [] -/
#guard_msgs in #eval failing persisted preNoPayload postRun

/-- info: ["summary:anchorPayload"] -/
#guard_msgs in
#eval failing persisted preNoPayload (postRun ++ [st (.timeout ⟨7⟩) [vote2 (certOn b1 7)]])

/-! ### Headers and files -/

/-- A header line, with `extra` appended to its object. -/
private def header (extra : String) : String :=
  "# trace {\"node\":1,\"anchor\":1,\"decideBuffer\":20" ++ extra ++ "}\n"

/-- The `restart` object, with `lock` and `resumes` as given. -/
private def restartJson (lock resumes : String) : String :=
  ",\"restart\":{\"lock\":" ++ lock ++ ",\"highestVoted\":3,\"vote2Bar\":3,\"reconstructed\":[[2,12]],\"resumes\":"
    ++ resumes ++ "}"

/-- What `readPreamble` makes of a header's `restart` object. -/
private def restartOf (text : String) : String :=
  match readPreamble text with
  | .error e => e
  | .ok said => match said.restart with
    | none => "none"
    | some r => s!"{r.lock.isSome} {r.highestVoted} {r.vote2Bar} {r.reconstructed.map (·.1)} {r.resumes}"

/-- info: "none" -/
#guard_msgs in #eval restartOf (header "")

/-- info: "true 3 (some 3) [2] (some a.jsonl)" -/
#guard_msgs in #eval restartOf (header (restartJson (toJson c1).compress "\"a.jsonl\""))

/-- info: "false 3 (some 3) [2] none" -/
#guard_msgs in #eval restartOf (header (restartJson "null" "null"))

/-- info: "its `# trace` header's `restart` has no `lock`" -/
#guard_msgs in
#eval restartOf (header ",\"restart\":{\"highestVoted\":3,\"vote2Bar\":3,\"reconstructed\":[],\"resumes\":null}")

-- A restart with no decide before the crash takes the configuration's anchor.
/-- info: true -/
#guard_msgs in
#eval match readPreamble (header (restartJson (toJson cfg.anchorCert).compress "null")) with
  | .ok said => match said.restart.map (persistedOf cfg [] said) with
    | some (.ok (some P)) => P.anchorBlock == cfg.anchorBlock && P.anchorCert == cfg.anchorCert
    | _ => false
  | .error _ => false

-- An empty first run is accepted only for a node anchored at genesis.
/-- info: ok 0 -/
#guard_msgs in
#eval show IO Unit from do
  let .ok said := readPreamble (header (restartJson (toJson cfg.anchorCert).compress "null"))
    | IO.println "unreadable"
  let some r := said.restart | IO.println "no restart"
  match ← resumedBy "x.jsonl" said r [] with
  | .ok (_, _, pre) => IO.println s!"ok {pre.length}"
  | .error e => IO.println e

/-- info: resumes an empty recording but loaded an anchor -/
#guard_msgs in
#eval show IO Unit from do
  let .ok said := readPreamble (header (",\"anchorView\":3" ++ restartJson (toJson c1).compress "null"))
    | IO.println "unreadable"
  let some r := said.restart | IO.println "no restart"
  match ← resumedBy "x.jsonl" said r [] with
  | .ok _ => IO.println "accepted"
  | .error e => IO.println e

/-- A history as trace lines. -/
private def lines (h : History) : String := renderTrace (h.map fun s => .consensus s.input s.output)

-- Two files of one node chain into the history before the crash, the restored steps, and the history after.
/-- info: true -/
#guard_msgs in
#eval show IO Bool from do
  let dir : System.FilePath := ".lake/restart-tests"
  IO.FS.createDirAll dir
  IO.FS.writeFile (dir / "n.0.jsonl") (header "" ++ lines preRun)
  IO.FS.writeFile (dir / "n.1.jsonl") (header (",\"anchorView\":1,\"anchorCert\":" ++ (toJson c1).compress
    ++ restartJson (toJson c1).compress "\"n.0.jsonl\"") ++ lines postRun)
  match ← chainHistory (dir / "n.1.jsonl") with
  | .ok (_, h) => return h == preRun ++ persisted.restoredSteps ++ postRun
  | .error _ => return false

end Tests
end NewProtocolDiff

