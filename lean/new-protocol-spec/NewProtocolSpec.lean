module

public import NewProtocolSpec.Base
public import NewProtocolSpec.Types
public import NewProtocolSpec.Interface
public import NewProtocolSpec.Validity
public import NewProtocolSpec.History
public import NewProtocolSpec.Rules
public import NewProtocolSpec.Network
public import NewProtocolSpec.Timing
public import NewProtocolSpec.Properties
public import NewProtocolSpec.Restart
public import NewProtocolSpec.Lists
public import NewProtocolSpec.Proofs.Traces
public import NewProtocolSpec.Proofs.Restart
public import NewProtocolSpec.Proofs.Inputs
public import NewProtocolSpec.Proofs.Votes
public import NewProtocolSpec.Proofs.Certificates
public import NewProtocolSpec.Proofs.Safety
public import NewProtocolSpec.Proofs.Decide
public import NewProtocolSpec.Proofs.Liveness.Basic
public import NewProtocolSpec.Proofs.Liveness.First
public import NewProtocolSpec.Proofs.Liveness.Stable
public import NewProtocolSpec.Proofs.Liveness.View
public import NewProtocolSpec.Proofs.Liveness.Epochs
public import NewProtocolSpec.Witness
public import NewProtocolSpec.Checks

/-!
# The consensus specification

What an honest node must do, stated over its history: the inputs it received and
the outputs it produced. Nothing here describes how a node stores what it has
seen, schedules its work or talks to its own modules. Those are left to
implementations, which are checked against these rules by proof (a Lean machine)
or on recorded traces (the Rust node).

## Modules

* `Base`: view, epoch and block numbers, and the epoch a block falls in
* `Types`: the data nodes exchange
* `Interface`: the configuration, and the inputs and outputs of a step
* `Validity`: conditions on data alone, such as a well-formed proposal
* `History`: a node's history, and what it holds, is locked on, and is in
* `Rules`: the signing rules, the rest of the protocol, and what a node owes
* `Network`: committees, one trace per honest node, and certificates that stand
  for real votes
* `Timing`: times, promptness, and synchrony after GST
* `Properties`: no fork, agreement on decides, and a chain that keeps growing
* `Restart`: what a restarted node stored, when that is faithful to its history,
  and the bars it sets on resuming; for a restart with nothing stored, what the
  node adopts from the network instead (`Rejoin`, `Rejoins`)
* `Lists`: a history as finite lists, each proved to hold what a predicate names
* `Proofs/`: `noFork` (`Proofs/Safety.lean`) and `decideAgreement`
  (`Proofs/Decide.lean`), from the signing rules and the verification premises
  alone; `ChainGrows` for every epoch height (`Proofs/Liveness/Epochs.lean`), from
  the full protocol, promptness and synchrony; `safe_across_restart` and
  `safe_across_wipe` (`Proofs/Restart.lean`), the signing rules across a restart
  and across wiped storage
* `Witness`: a network meeting every safety premise, with a block committed and
  an epoch opened behind it
* `Checks`: the axiom footprint and the field lists of the rules and premises

## What is fixed and what is free

The specification fixes what goes on the wire, when a node may sign, what it must
eventually do, and what it may propose after a timeout. It leaves free how a node
checks the signing rules (a lock is one way), how it stores and prunes, how
promptly it acts within the bound `δ`, and how messages spread, as long as the
delivery assumptions of `Synchrony` hold.

## Not covered

A restart is covered as a splice of two histories around the steps the restart
loads, given a stored summary (`Persisted`) faithful to the history before it. A
restart with nothing stored is covered given what the node adopts (`Rejoins`); how
it queries the network for the bar and waits for the anchor is described in the
reference, not proved, and that the node decided nothing above the anchor before
the crash (`Rejoins.decidedBelow`) is assumed. Not covered: the durability of that summary, fetching what a node missed, restarts
at the version before the timeout epoch bound, anything before GST beyond safety,
and more than the tolerated number of faulty nodes in a committee. A decide may
skip ancestors the node does not hold, so no node is promised every committed
block.
-/
