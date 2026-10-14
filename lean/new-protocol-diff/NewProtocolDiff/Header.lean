module

public import NewProtocolDiff.Json
public import NewProtocolSpec.History

/-!
# The trace header

Every trace states, on its first line, which node it is, where its chain is
anchored, how far behind the decided view it kept decide inputs, how many blocks
make an epoch, and the certificate it held at genesis:

    # trace {"node": "…", "anchor": "…", "decideBuffer": 20, "epochHeight": 100,
             "anchorCert": {…}}

The node, the anchor and the decide buffer are not optional: a run checked against
the wrong node key, anchor or buffer is judged just as confidently as a real one.

A node restarted from storage adds what it loaded, and the file of its run before
the crash:

    "restart": {"lock": {…}, "highestVoted": 11, "vote2Bar": 11,
                "reconstructed": [[1, "…"], …], "resumes": "…node0.1.jsonl"}

Every key of `restart` is required; `lock`, `vote2Bar` and `resumes` may be `null`.

A node restarted with nothing stored adds the view bar it recovered, and the file
of its run before the wipe:

    "rejoin": {"bar": 4, "resumes": "…node0.1.jsonl"}

Its `anchor`, `anchorView` and `anchorCert` are the ones it adopted, rewritten
when it adopts them. Every key of `rejoin` is required; `bar` is `null` until it
adopts, and `resumes` may be `null`.
-/

@[expose] public section

open Lean

namespace NewProtocolDiff

open NewProtocol

/-- What a restarted node loaded from storage, as its header states it. -/
structure Restored where
  /-- The locked `Cert1` it restored. -/
  lock : Option Cert1

  /-- The latest view it voted, proposed or timed out in before the crash. -/
  highestVoted : Nat

  /-- The view at or below which it casts no vote2 after the restart, when it bars any. -/
  vote2Bar : Option Nat

  /-- The saved proposals it marks as having their payload, by view and payload commitment. -/
  reconstructed : List (Nat × PayloadCommit)

  /-- The file of its run before the crash, in the same directory; `none` when that run recorded no step. -/
  resumes : Option String

/-- What a node restarted with nothing stored adopted, as its header states it. -/
structure Rejoined where
  /-- The view bar it recovered from the network; `none` until it adopted an anchor. -/
  bar : Option Nat

  /-- The file of its run before the wipe, in the same directory; `none` when that run recorded no step. -/
  resumes : Option String

/-- What a recorder said about the run it recorded. -/
structure Preamble where
  node : Nat
  anchor : Nat

  /-- How far behind the decided view the recording kept decide inputs. -/
  decideBuffer : Nat

  /-- Blocks to an epoch; zero when absent, which is the static-committee reading. -/
  epochHeight : Nat

  /-- The certificate the node held at the genesis view, when it held one. -/
  anchorCert : Option Cert1

  /--
  The view of the block the node started from.

  After genesis only for a node restored from storage or rejoined after a wipe.
  Zero when absent.
  -/
  anchorView : Nat

  /-- What the node loaded, when it was restarted from storage. -/
  restart : Option Restored

  /-- What the node adopted, when it was restarted with nothing stored. -/
  rejoin : Option Rejoined

/-- Read the `# trace` header line. -/
def readPreamble (text : String) : Except String Preamble := do
  let tag := "# trace "
  let line := (text.splitOn "\n").headD ""
  unless line.startsWith tag do
    throw "no `# trace` header line"
  let json ← (Json.parse (line.drop tag.length).toString).mapError
    fun e => s!"its `# trace` header is not JSON: {e}"
  let field (name : String) : Except String Json :=
    (json.getObjVal? name).mapError fun _ => s!"its `# trace` header has no `{name}`"
  let ident (name : String) : Except String Nat := do
    (cryptoFromJson (← field name)).mapError
      fun e => s!"its `# trace` header's `{name}`: {e}"
  let buffer ← (Json.getNat? (← field "decideBuffer")).mapError
    fun e => s!"its `# trace` header's `decideBuffer`: {e}"
  let height := (json.getObjVal? "epochHeight").toOption.bind (Json.getNat? · |>.toOption)
  let anchorCert : Option Cert1 ← match json.getObjVal? "anchorCert" with
    | .error _ => pure none
    | .ok j =>
      let decoded : Except String Cert1 := fromJson? j
      match decoded with
      | .ok c => pure (some c)
      | .error e => throw s!"its `# trace` header's `anchorCert`: {e}"
  let anchorView := (json.getObjVal? "anchorView").toOption.bind (Json.getNat? · |>.toOption)
  let key {α : Type} [FromJson α] (obj : String) (j : Json) (name : String) : Except String α := do
    let v ← (j.getObjVal? name).mapError fun _ => s!"its `# trace` header's `{obj}` has no `{name}`"
    (fromJson? v).mapError fun e => s!"its `# trace` header's `{obj}.{name}`: {e}"
  let restart : Option Restored ← match json.getObjVal? "restart" with
    | .error _ => pure none
    | .ok j =>
      pure (some ⟨← key "restart" j "lock", ← key "restart" j "highestVoted",
        ← key "restart" j "vote2Bar", ← key "restart" j "reconstructed", ← key "restart" j "resumes"⟩)
  let rejoin : Option Rejoined ← match json.getObjVal? "rejoin" with
    | .error _ => pure none
    | .ok j => pure (some ⟨← key "rejoin" j "bar", ← key "rejoin" j "resumes"⟩)
  pure ⟨← ident "node", ← ident "anchor", buffer, height.getD 0, anchorCert, anchorView.getD 0,
    restart, rejoin⟩

/--
The configuration a header describes.

The anchor sits at genesis, where no rule reads its payload commitment. Its
identity is the one its certificate names, as `ConfigCoherent.anchorCertBlock`
asks.
-/
def Preamble.config (said : Preamble) : Config :=
  let anchorCert : Cert1 := said.anchorCert.getD
    ⟨⟨⟨said.anchor⟩, epochOf 0 said.epochHeight, 0⟩, ViewNumber.genesis⟩
  let anchor : Block :=
    ⟨⟨⟨0⟩, 0⟩, ViewNumber.genesis, epochOf 0 said.epochHeight, anchorCert, none,
      anchorCert.data.blockHash⟩
  ⟨anchor, anchorCert, said.decideBuffer, said.epochHeight⟩

/--
The certificate a trace's blocks name as their parent at the genesis view.

A test harness that never seeds a genesis certificate records none in its header,
and the one `Preamble.config` would make up from the anchor leaf is not the one
the node's first proposal names: the harness builds its genesis proposal apart
from the anchor leaf, and the two commit to different things. The certificate the
blocks name is the one the node treated as genesis.
-/
def genesisParentIn (h : History) : Option Cert1 :=
  h.findSome? fun st => match st.input with
    | .proposal _ p _ | .epochChange _ _ p =>
      if p.parentCert.view = ViewNumber.genesis then some p.parentCert else none
    | _ => none

/-- The configuration of a recorded run: its header's, with the genesis certificate its blocks name if the header has none. -/
def Preamble.configFor (said : Preamble) (h : History) : Config :=
  { said with anchorCert := said.anchorCert <|> genesisParentIn h }.config

/--
Who leads each view in each epoch, as the recorder saw it: the
`# leader <view> <epoch> <key>` lines.

At an epoch boundary the two committees may each have a leader for one view. A
view and epoch with no line, or only one reading `unknown`, has no leader here.
-/
def readLeaders (text : String) : Std.TreeMap ViewNumber (List (EpochNumber × PubKey)) :=
  (text.splitOn "\n").foldl (init := {}) fun leaders line =>
    let tag := "# leader "
    if !line.startsWith tag then leaders
    else
      match (line.drop tag.length).toString.trimAscii.toString.splitOn " " with
      | [v, e, key] =>
        match v.toNat?, e.toNat? with
        | some v, some e =>
          if key == "unknown" then leaders
          else
            match cryptoFromJson (Json.str (key.replace "\"" "")) with
            | .ok k =>
              let others := ((leaders.get? ⟨v⟩).getD []).filter (·.1 != ⟨e⟩)
              leaders.insert ⟨v⟩ ((⟨e⟩, ⟨k⟩) :: others)
            | .error _ => leaders
        | _, _ => leaders
      | _ => leaders

/-- The leader of epoch `e` at view `v`, in a schedule `readLeaders` read. -/
def leaderIn (leaders : Std.TreeMap ViewNumber (List (EpochNumber × PubKey)))
    (e : EpochNumber) (v : ViewNumber) : Option PubKey :=
  ((leaders.get? v).getD []).lookup e

/--
Whether the trace says its timeouts sign no lock: the `# timeouts sign no lock`
line. Before the certificate rule a timeout vote and certificate carry no lock,
which the specification does not model.
-/
def signsNoLock (text : String) : Bool :=
  (text.splitOn "\n").any (· == "# timeouts sign no lock")

/-- The traces an argument names: itself, or every `*.jsonl` in it. -/
def tracesUnder (path : System.FilePath) : IO (Array System.FilePath) := do
  if ← path.isDir then
    let entries ← path.readDir
    let files := entries.filterMap fun e =>
      if e.path.extension == some "jsonl" then some e.path else none
    return files.qsort (·.toString < ·.toString)
  else
    return #[path]

end NewProtocolDiff
