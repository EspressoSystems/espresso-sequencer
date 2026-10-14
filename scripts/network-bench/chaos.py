"""Chaos mode of `aws-bench run --chaos`: the event schema of `chaos.jsonl`, the pure fault
scheduler and its probe, fault rows with their intervals, and the parts of the chaos report.

Stdlib only: `aws-bench`, `netbench.py` and `throughput-plot` import it, and the hosts get it
next to them.
"""

import gzip
import json
import random
import re
import shlex
import statistics
from collections import Counter
from collections.abc import Collection, Iterable, Mapping, Sequence
from dataclasses import dataclass
from datetime import UTC, datetime
from itertools import accumulate, pairwise
from pathlib import Path
from typing import Literal, NamedTuple, NotRequired, TypedDict

# No faults in the last `kill_down_s` plus this of the load, so a late kill recovers under load.
RECOVERY_ALLOWANCE_S = 120.0
LAG_BLOCKS = 2
LAG_VIEWS = 3
GATE_TICKS = 2
# Seconds between counter samples of the load driver.
COUNTER_POLL_S = 1.0
# `next_view_timeout` of the nodes (12000 ms, `crates/espresso/types/src/v0/config.rs`). A view
# that began before a fault ended times out up to this long after it.
VIEW_TIMEOUT_S = 12.0
# A fault's effect on the views outlasts its `rejoined` by a view timeout and a sample.
FLOOR_TAIL_S = VIEW_TIMEOUT_S + COUNTER_POLL_S
SYNC_STATUS_TTL_S = 5.0
SYNC_SETTLE_S = 2 * SYNC_STATUS_TTL_S

ChaosKind = Literal["restart", "kill", "wipe", "mass-restart"]
MASS_RESTART: ChaosKind = "mass-restart"

# Missing blocks, leaves and vid_common of a query node's `sync-status`.
Missing = tuple[int, int, int]
MISSING_NAMES = ("blocks", "leaves", "vid_common")


@dataclass(frozen=True)
class ChaosConfig:
    minutes: int = 10
    rate_mb_s: float = 4.0
    seed: int = 42
    kinds: tuple[ChaosKind, ...] = ("restart", "kill", "wipe")
    gap_s: float = 45.0
    kill_down_s: float = 60.0
    recover_timeout_s: float = 300.0
    # Up query nodes kept at every fault: one serves scans and heights while another catches up.
    query_floor: int = 2


ChaosEventKind = Literal[
    "fault", "started", "rejoined", "caught_up", "timeout", "restored", "interrupted"
]


class ChaosEvent(TypedDict):
    """A line of `chaos.jsonl`. `query`: the node serves the query API, so it recovers only
    once caught up."""

    ts: float
    iso: str
    event: ChaosEventKind
    node: str
    query: bool
    kind: ChaosKind | None
    height: int | None
    after_s: float | None
    missing: NotRequired[Missing]


def chaos_event(
    ts: float,
    event: ChaosEventKind,
    node: str,
    query: bool,
    kind: ChaosKind | None,
    height: int | None,
    after_s: float | None,
    missing: Missing | None = None,
) -> ChaosEvent:
    event_ = ChaosEvent(
        ts=ts,
        iso=datetime.fromtimestamp(ts, UTC).isoformat(),
        event=event,
        node=node,
        query=query,
        kind=kind,
        height=height,
        after_s=after_s,
    )
    if missing is not None:
        event_["missing"] = missing
    return event_


def read_events(run_dir: Path) -> list[ChaosEvent]:
    """Empty for a run without `chaos.jsonl`. Events written before `query` existed get it
    from the roles of `topology.json`."""
    path = run_dir / "chaos.jsonl"
    if not path.exists():
        return []
    events: list[ChaosEvent] = [
        json.loads(line) for line in path.read_text().splitlines()
    ]
    if all("query" in e for e in events):
        return events
    roles = json.loads((run_dir / "topology.json").read_text())["roles"]
    return [{**e, "query": "query" in roles[e["node"]]} for e in events]


def chaos_log_line(event: ChaosEvent) -> str:
    node, after_s = event["node"], event["after_s"] or 0.0
    match event["event"]:
        case "fault":
            return f"{event['kind']} {node} at height {event['height']}"
        case "started":
            return f"{node} started after {after_s:.0f} s"
        case "rejoined":
            return f"{node} rejoined after {after_s:.0f} s"
        case "caught_up":
            return f"{node} caught up after {after_s:.0f} s"
        case "timeout":
            line = f"{node} not recovered after {after_s:.0f} s"
            if "missing" in event:
                counts = ", ".join(
                    f"{n} {c}"
                    for n, c in zip(MISSING_NAMES, event["missing"], strict=True)
                )
                line += f" (missing {counts})"
            return line
        case "restored":
            return f"{node} restored"
        case "interrupted":
            return f"{node} interrupted"


# `stuck`: past its recover timeout; still faulty, no longer gated.
NodeHealth = Literal["up", "down", "recovering", "catching_up", "stuck"]
OPEN: tuple[NodeHealth, ...] = ("down", "recovering", "catching_up")


class NodeState(TypedDict):
    """`since` is the fault time: timeouts measure from it. `started` is the fault time, or the
    start of a killed node: recoveries measure from it. `up_at` is the first tick the node
    answered after it. `streak_at` is the first tick of the passing streak. `last_missing` is
    the last `Missing` answered since the fault."""

    health: NodeHealth
    kind: ChaosKind | None
    since: float
    started: float
    up_at: float | None
    streak: int
    streak_at: float
    last_missing: Missing | None


class ChaosState(TypedDict):
    nodes: dict[str, NodeState]
    order: list[str]
    cursor: int
    faults: int
    next_fault_at: float
    stop_at: float
    # Progress threshold of the last mass restart: the highest height at it, raised by the first
    # answer of each restarted node. The network decided anew once the tip exceeds it.
    mass_height: int | None
    # Nodes whose mass restart ssh failed -> time of that restart, issued again by the next tick.
    mass_retry: dict[str, float]
    # Node -> stake amount; faulty stake is bounded by a third of the total.
    stake: dict[str, int]


class ChaosObservation(TypedDict):
    """`None`: the probe got no answer. `query_height` and `missing` only have the query nodes."""

    height: dict[str, int | None]
    voted_view: dict[str, int | None]
    query_height: dict[str, int | None]
    missing: dict[str, Missing | None]


class ChaosAction(TypedDict):
    node: str
    op: Literal["restart", "kill", "start", "wipe"]


class ChaosStep(TypedDict):
    state: ChaosState
    actions: list[ChaosAction]
    events: list[ChaosEvent]


def halts(stake: Mapping[str, int], faulty: Collection[str]) -> bool:
    """Whether the `faulty` nodes hold a third of the total stake or more, which stops consensus."""
    return 3 * sum(stake[name] for name in faulty) >= sum(stake.values())


def mass_restart_only(cfg: ChaosConfig) -> bool:
    return cfg.kinds == (MASS_RESTART,)


def chaos_init(
    cfg: ChaosConfig,
    nodes: Sequence[str],
    stake: Mapping[str, int],
    t_load: float,
    warmup_s: float,
    load_s: float,
) -> ChaosState:
    order = list(nodes)
    random.Random(cfg.seed).shuffle(order)
    first = t_load + warmup_s
    stop_at = first + load_s - cfg.kill_down_s - RECOVERY_ALLOWANCE_S
    return ChaosState(
        nodes={
            name: NodeState(
                health="up",
                kind=None,
                since=t_load,
                started=t_load,
                up_at=None,
                streak=0,
                streak_at=t_load,
                last_missing=None,
            )
            for name in nodes
        },
        order=order,
        cursor=0,
        faults=0,
        # A lone mass restart is issued once, midway between the first fault time and `stop_at`.
        next_fault_at=(first + stop_at) / 2 if mass_restart_only(cfg) else first,
        stop_at=stop_at,
        mass_height=None,
        mass_retry={},
        stake=dict(stake),
    )


def open_faults(state: ChaosState) -> list[str]:
    """Nodes still recovering from a fault; a stuck node is no longer one."""
    return [name for name, s in state["nodes"].items() if s["health"] in OPEN]


def stuck_nodes(state: ChaosState) -> list[str]:
    return [name for name, s in state["nodes"].items() if s["health"] == "stuck"]


class _Reference(NamedTuple):
    """What a recovering node is compared to. `progressed`: always true, except after a mass
    restart until the tip exceeds the progress threshold `ChaosState.mass_height`."""

    tip: int | None
    view: int | None
    query_tip: int | None
    progressed: bool


def chaos_step(
    state: ChaosState,
    cfg: ChaosConfig,
    queries: Collection[str],
    peers: dict[str, list[str]],
    obs: ChaosObservation,
    now: float,
) -> ChaosStep:
    """One controller tick. Pure: `state` is not modified, `now` is in the ctl epoch."""
    nodes = {name: s.copy() for name, s in state["nodes"].items()}
    events: list[ChaosEvent] = []
    actions: list[ChaosAction] = []

    def emit(
        event: ChaosEventKind,
        name: str,
        after_s: float | None = None,
        missing: Missing | None = None,
        ts: float = now,
    ) -> None:
        events.append(
            chaos_event(
                ts,
                event,
                name,
                name in queries,
                nodes[name]["kind"],
                obs["height"][name],
                after_s,
                missing,
            )
        )

    for name in queries:
        if (seen := obs["missing"][name]) is not None:
            nodes[name]["last_missing"] = seen

    up = [name for name, s in nodes.items() if s["health"] == "up"]
    tip = _tip(obs["height"], up)
    normal = _Reference(
        tip,
        _tip(obs["voted_view"], up),
        _tip(obs["query_height"], [name for name in up if name in queries]),
        True,
    )
    # A restarted node reports the height it stored, which can be above the height probed
    # before the restart: its first answer raises the threshold. The tip then only exceeds it
    # through a block decided after the restart, which a lone node cannot produce.
    mass_height = _raised(state["mass_height"], nodes, obs)
    mass = _mass_reference(nodes, obs, queries, mass_height)
    for name, s in nodes.items():
        ref = mass if s["kind"] == MASS_RESTART else normal
        if s["health"] == "down" and now >= s["since"] + cfg.kill_down_s:
            actions.append({"node": name, "op": "start"})
            s.update(health="recovering", started=now, streak=0)
            emit("started", name, now - s["since"])
        elif ref.tip is None:
            continue
        elif s["health"] == "recovering":
            # The fault or start action ran before this tick: the answer is the new process.
            if s["up_at"] is None and obs["height"][name] is not None:
                s["up_at"] = now
            good = (
                ref.progressed
                and _near(obs["height"][name], ref.tip, LAG_BLOCKS)
                and _near(obs["voted_view"][name], ref.view, LAG_VIEWS)
            )
            if _passed(s, good, now):
                nxt: NodeHealth = "catching_up" if name in queries else "up"
                emit(
                    "rejoined",
                    name,
                    s["streak_at"] - s["started"],
                    ts=s["streak_at"],
                )
                s.update(health=nxt, kind=None if nxt == "up" else s["kind"])
        elif s["health"] == "catching_up" and ref.query_tip is not None:
            # SYNC_SETTLE_S > SYNC_STATUS_TTL_S after the node came up: a sync-status
            # cached from its empty database has expired.
            good = (
                s["up_at"] is not None
                and now >= s["up_at"] + SYNC_SETTLE_S
                and _near(obs["query_height"][name], ref.query_tip, LAG_BLOCKS)
                and obs["missing"][name] == (0, 0, 0)
            )
            if _passed(s, good, now):
                emit(
                    "caught_up",
                    name,
                    s["streak_at"] - s["started"],
                    obs["missing"][name],
                    ts=s["streak_at"],
                )
                s.update(health="up", kind=None)

    for name, s in nodes.items():
        if s["health"] in OPEN and now >= s["since"] + cfg.recover_timeout_s:
            emit("timeout", name, now - s["since"], s["last_missing"])
            if s["health"] == "down":
                # Running for the collection, as `restore` only starts open faults.
                actions.append({"node": name, "op": "start"})
            s.update(health="stuck", streak=0)

    cursor, faults, next_fault_at = (
        state["cursor"],
        state["faults"],
        state["next_fault_at"],
    )
    kind = cfg.kinds[faults % len(cfg.kinds)] if cfg.kinds else None
    due = (
        tip is not None
        and kind is not None
        and next_fault_at <= now < state["stop_at"]
        and not (mass_restart_only(cfg) and faults)
        and not _mass_open(nodes)
        and not state["mass_retry"]
    )
    if state["mass_retry"]:
        for name, at in state["mass_retry"].items():
            _restart(nodes[name], MASS_RESTART, at)
            actions.append({"node": name, "op": "restart"})
            emit("fault", name, ts=at)
    elif due and kind == MASS_RESTART:
        if not any(s["health"] in OPEN for s in nodes.values()):
            # Every node at once: no budget, query floor or peer rule applies. A stuck node is
            # restarted too, and stays stuck: it never counts as recovered.
            for name, s in nodes.items():
                actions.append({"node": name, "op": "restart"})
                if s["health"] != "stuck":
                    _restart(s, kind, now)
                    emit("fault", name)
            mass_height = _tip(obs["height"], nodes)
            faults, next_fault_at = faults + 1, now + cfg.gap_s
    elif (
        due
        and tip is not None
        and kind is not None
        and (
            i := _fault_target(
                state["order"],
                cursor,
                nodes,
                _lagging(nodes, obs, tip),
                queries,
                peers,
                state["stake"],
                cfg.query_floor,
            )
        )
        is not None
    ):
        name = state["order"][i]
        nodes[name].update(
            health="down" if kind == "kill" else "recovering",
            kind=kind,
            since=now,
            started=now,
            up_at=None,
            streak=0,
            last_missing=None,
        )
        actions.append({"node": name, "op": kind})
        emit("fault", name)
        cursor, faults, next_fault_at = (
            (i + 1) % len(state["order"]),
            faults + 1,
            now + cfg.gap_s,
        )

    return ChaosStep(
        state=ChaosState(
            nodes=nodes,
            order=state["order"],
            cursor=cursor,
            faults=faults,
            next_fault_at=next_fault_at,
            stop_at=state["stop_at"],
            mass_height=mass_height,
            mass_retry={},
            stake=state["stake"],
        ),
        actions=actions,
        events=events,
    )


def revert_failed(
    prev: ChaosState, step: ChaosStep, failed: Collection[str]
) -> ChaosStep:
    """`step` without the actions of the `failed` nodes: their state and events as before it,
    so that the next tick retries them. A failed fault also keeps the schedule as before. A
    failed node of a mass restart is restarted again alone at the time of the restart; the
    other nodes keep their state and events."""
    if not failed:
        return step
    state = step["state"]
    mass_at = {
        e["node"]: e["ts"]
        for e in step["events"]
        if e["event"] == "fault" and e["kind"] == MASS_RESTART
    }
    faulted = not mass_at and any(
        a["node"] in failed and a["op"] != "start" for a in step["actions"]
    )
    schedule = prev if faulted else state
    return ChaosStep(
        state=ChaosState(
            nodes={
                name: prev["nodes"][name] if name in failed else s
                for name, s in state["nodes"].items()
            },
            order=state["order"],
            cursor=schedule["cursor"],
            faults=schedule["faults"],
            next_fault_at=schedule["next_fault_at"],
            stop_at=state["stop_at"],
            mass_height=state["mass_height"],
            mass_retry={name: at for name, at in mass_at.items() if name in failed},
            stake=state["stake"],
        ),
        actions=[a for a in step["actions"] if a["node"] not in failed],
        events=[e for e in step["events"] if e["node"] not in failed],
    )


def restarted_events(step: ChaosStep, ts: float) -> list[ChaosEvent]:
    """A `started` event at `ts`, when the restarts of `step` finished, for each node of a mass
    restart that was restarted."""
    return [
        chaos_event(ts, "started", e["node"], e["query"], e["kind"], None, ts - e["ts"])
        for e in step["events"]
        if e["event"] == "fault" and e["kind"] == MASS_RESTART
    ]


def _restart(s: NodeState, kind: ChaosKind, now: float) -> None:
    s.update(
        health="recovering",
        kind=kind,
        since=now,
        started=now,
        up_at=None,
        streak=0,
        last_missing=None,
    )


def _passed(s: NodeState, good: bool, now: float) -> bool:
    """Advances the passing streak of `s`; True, with the streak reset, once it is long
    enough."""
    if not good:
        s["streak"] = 0
        return False
    if s["streak"] == 0:
        s["streak_at"] = now
    s["streak"] += 1
    if s["streak"] < GATE_TICKS:
        return False
    s["streak"] = 0
    return True


def _mass_open(nodes: Mapping[str, NodeState]) -> bool:
    """A mass restart has nodes that did not recover yet; no other fault is issued."""
    return any(
        s["kind"] == MASS_RESTART and s["health"] in OPEN for s in nodes.values()
    )


def _raised(
    mass_height: int | None, nodes: Mapping[str, NodeState], obs: ChaosObservation
) -> int | None:
    """`mass_height` or the height of the first answer after a mass restart of a node, if higher."""
    first = [
        name
        for name, s in nodes.items()
        if s["kind"] == MASS_RESTART
        and s["health"] == "recovering"
        and s["up_at"] is None
    ]
    return max(
        (h for h in (mass_height, _tip(obs["height"], first)) if h is not None),
        default=None,
    )


def _mass_reference(
    nodes: Mapping[str, NodeState],
    obs: ChaosObservation,
    queries: Collection[str],
    mass_height: int | None,
) -> _Reference:
    """After a mass restart no node is up: the reference is the highest answer of any node but
    a stuck one."""
    names = [name for name, s in nodes.items() if s["health"] != "stuck"]
    tip = _tip(obs["height"], names)
    return _Reference(
        tip,
        _tip(obs["voted_view"], names),
        _tip(obs["query_height"], [name for name in names if name in queries]),
        tip is not None and mass_height is not None and tip > mass_height,
    )


def _lagging(
    nodes: Mapping[str, NodeState], obs: ChaosObservation, tip: int
) -> set[str]:
    """Up nodes that did not answer or are behind `tip`."""
    return {
        name
        for name, s in nodes.items()
        if s["health"] == "up" and not _near(obs["height"][name], tip, LAG_BLOCKS)
    }


def _tip(values: Mapping[str, int | None], names: Iterable[str]) -> int | None:
    seen = [v for name in names if (v := values[name]) is not None]
    return max(seen, default=None)


def _near(value: int | None, tip: int | None, lag: int) -> bool:
    return value is not None and tip is not None and value >= tip - lag


def _fault_target(
    order: list[str],
    cursor: int,
    nodes: dict[str, NodeState],
    lagging: Collection[str],
    queries: Collection[str],
    peers: dict[str, list[str]],
    stake: Mapping[str, int],
    query_floor: int,
) -> int | None:
    """Index in `order` of the first node from `cursor` on whose fault keeps the faulty stake
    below a third, the up query floor and 2 up peers for itself and for every faulty node;
    `None` when there is none. A `lagging` node counts as faulty everywhere and is never a
    target."""
    up = {name for name, s in nodes.items() if s["health"] == "up"} - set(lagging)
    down = nodes.keys() - up
    up_queries = sum(q in up for q in queries)

    def up_peers(name: str, without: str) -> int:
        return sum(p in up and p != without for p in peers[name])

    for step in range(len(order)):
        i = (cursor + step) % len(order)
        name = order[i]
        if name not in up:
            continue
        if halts(stake, [*down, name]):
            continue
        if up_queries - (name in queries) < query_floor:
            continue
        if up_peers(name, name) < 2:
            continue
        if any(
            up_peers(m, name) < 2 for m in nodes if m not in up and name in peers[m]
        ):
            continue
        return i
    return None


# `is_fully_synced` is a method of `SyncStatusQueryData`, not a serialized field: it holds when
# all three counts are 0.
MISSING_JQ = '"' + ",".join(rf"\(.{name}.missing)" for name in MISSING_NAMES) + '"'


def probe_command(urls: Mapping[str, str], queries: Collection[str]) -> str:
    """Shell run on ctl: one background job per node of `urls` printing `<node> <height> <voted
    view>`, plus `<query height> <blocks,leaves,vid_common missing>` for query nodes; `-` for a
    field that did not answer.
    Nodes run in parallel so a hung node costs one curl timeout, not one per field."""
    lines = []
    for name, url in urls.items():
        base = shlex.quote(url)
        height = f"$(curl -sf -m 2 {base}/v1/status/block-height)"
        view = (
            f"$(curl -sf -m 2 {base}/v1/status/metrics | "
            "awk '$1==\"consensus_last_voted_view\"{print $2}')"
        )
        fields = ["${h:--}", "${v:--}"]
        script = f"h={height}; v={view}; "
        if name in queries:
            script += (
                f"q=$(curl -sf -m 2 {base}/v1/node/block-height); "
                f"s=$(curl -sf -m 2 {base}/v1/node/sync-status | jq -r {shlex.quote(MISSING_JQ)}); "
            )
            fields += ["${q:--}", "${s:--}"]
        lines.append(f'({script}echo "{name} {" ".join(fields)}") &')
    return "\n".join([*lines, "wait"])


def parse_probe(
    stdout: str, nodes: Iterable[str], queries: Collection[str]
) -> ChaosObservation:
    """Raises `ValueError` for a node without a line or with missing fields."""
    rows = {
        line.split()[0]: line.split() for line in stdout.splitlines() if line.strip()
    }
    obs = ChaosObservation(height={}, voted_view={}, query_height={}, missing={})
    for name in nodes:
        fields = rows.get(name)
        if fields is None or len(fields) != (5 if name in queries else 3):
            raise ValueError(
                f"probe output has no complete line for {name}: {stdout!r}"
            )
        obs["height"][name] = _probe_int(fields[1])
        obs["voted_view"][name] = _probe_int(fields[2])
        if name in queries:
            obs["query_height"][name] = _probe_int(fields[3])
            obs["missing"][name] = _probe_missing(fields[4])
    return obs


def _probe_missing(field: str) -> Missing | None:
    if field == "-":
        return None
    blocks, leaves, vid_common = map(int, field.split(","))
    return blocks, leaves, vid_common


def _probe_int(field: str) -> int | None:
    return None if field == "-" else int(float(field))


@dataclass(slots=True)
class FaultRow:
    """One fault and the time of each later event of its node; `None` while it has not
    happened. `missing` is the last count an event of the node carried."""

    node: str
    query: bool
    kind: ChaosKind
    fault: float
    started: float | None = None
    rejoined: float | None = None
    caught_up: float | None = None
    timeout: float | None = None
    restored: float | None = None
    interrupted: float | None = None
    missing: Missing | None = None
    height: int | None = None


def fault_rows(events: Iterable[ChaosEvent]) -> list[FaultRow]:
    """One row per fault, in the order of the faults."""
    rows: list[FaultRow] = []
    latest: dict[str, FaultRow] = {}
    for e in events:
        if e["event"] == "fault":
            if e["kind"] is None:
                raise ValueError(f"fault event without a kind: {e}")
            latest[e["node"]] = FaultRow(
                e["node"], e["query"], e["kind"], e["ts"], height=e["height"]
            )
            rows.append(latest[e["node"]])
            continue
        row, ts = latest[e["node"]], e["ts"]
        match e["event"]:
            case "started":
                row.started = ts
            case "rejoined":
                row.rejoined = ts
            case "caught_up":
                row.caught_up = ts
            case "timeout":
                row.timeout = ts
            case "restored":
                row.restored = ts
            case "interrupted":
                row.interrupted = ts
        if "missing" in e:
            row.missing = e["missing"]
    return rows


Phase = Literal["down", "recovering", "catching up"]


def fault_span(row: FaultRow, end: float) -> tuple[float, float]:
    """(start, stop) of the time `row`'s node is faulty: from the fault until it caught up (a
    query node) or rejoined (a validator), else until the fault was interrupted, else until
    `end`. A timed out node is stuck, faulty until `end`."""
    recovered = row.caught_up if row.query else row.rejoined
    return row.fault, _first(recovered, row.interrupted, end)


def max_concurrent_faulty(rows: Iterable[FaultRow], end: float) -> int:
    """Most nodes faulty at once."""
    spans = [fault_span(r, end) for r in rows]
    return max((sum(a <= start < b for a, b in spans) for start, _ in spans), default=0)


def phase_segments(row: FaultRow, end: float) -> list[tuple[Phase, float, float]]:
    """(phase, start, stop) of a fault within its `fault_span`: a kill is down until the node
    starts (`started`, else the start at its timeout), then recovering until it rejoins, then
    catching up (a query node only). The phase open at the span's stop runs to there."""
    stop = fault_span(row, end)[1]
    up = _first(row.started, row.timeout, stop) if row.kind == "kill" else row.fault
    segments: list[tuple[Phase, float, float]] = []
    if up > row.fault:
        segments.append(("down", row.fault, up))
    if up < stop:
        segments.append(("recovering", up, _first(row.rejoined, stop)))
    if row.query and row.rejoined is not None and row.rejoined < stop:
        segments.append(("catching up", row.rejoined, stop))
    return segments


def _first(*times: float | None) -> float:
    """The first time that is not `None`; the last one never is."""
    return next(t for t in times if t is not None)


class StepChaos(TypedDict):
    """What the faults did to one load step."""

    start_s: float
    # Faults active during the step, as "node kind".
    faults: str
    # Nodes whose process was stopped or started during the step: their CPU counter reset.
    restarted: frozenset[str]


class ChaosView(TypedDict):
    """The chaos report of `render`: `lead` replaces the headline, `faults` follows the chart,
    `steps` has one entry per step of the result."""

    lead: list[str]
    faults: list[str]
    steps: list[StepChaos]


def step_chaos(
    spans: Iterable[tuple[float, float]], rows: list[FaultRow], t0: float, end: float
) -> list[StepChaos]:
    """Per step, given as its (start, end): the faults active in it, and the nodes whose process
    stopped or started. `start_s` counts from `t0`."""
    windows = [(r.node, r.kind, *fault_span(r, end)) for r in rows]
    # A kill that timed out while down is started at its timeout.
    restarts = [
        (r.node, ts)
        for r in rows
        for ts in (
            r.fault,
            r.timeout if r.kind == "kill" and r.started is None else r.started,
        )
        if ts is not None
    ]
    return [
        {
            "start_s": start - t0,
            "faults": ", ".join(
                f"{node} {kind}"
                for node, kind, a, b in windows
                if a < stop and b > start
            ),
            "restarted": frozenset(node for node, ts in restarts if start <= ts < stop),
        }
        for start, stop in spans
    ]


def chaos_verdict(
    rows: list[FaultRow],
    events: Sequence[ChaosEvent],
    error: str | None,
    failed: bool,
) -> str:
    """Fails on a timeout or a failed run; a fault the run's end interrupted is named, not
    blamed."""
    reasons = [
        f"{e['node']} ({e['kind']}) not recovered after {e['after_s']:.0f} s"
        for e in events
        if e["event"] == "timeout"
    ]
    if not reasons and failed:
        reasons.append(error or "the run produced no result")
    interrupted = ", ".join(
        f"{r.node} ({r.kind})" for r in rows if r.interrupted is not None
    )
    tail = f"; interrupted faults: {interrupted}" if interrupted else ""
    if reasons:
        return f"- Verdict: **fail**: {'; '.join(reasons)}{tail}"
    if interrupted:
        return f"- Verdict: **pass**: no timeout{tail}"
    if not rows:
        return "- Verdict: **pass**: no fault issued, no timeout"
    return "- Verdict: **pass**: every fault recovered, no timeout"


def recovery_table(rows: list[FaultRow], kinds: Sequence[str]) -> list[str]:
    """Seconds to the `rejoined` and `caught_up` events per kind, from the fault, or from the
    start of a killed node; only query nodes catch up."""

    def spread(took: list[float]) -> tuple[str, str]:
        if not took:
            return "-", "-"
        return f"{statistics.median(took):.0f}", f"{max(took):.0f}"

    lines = [
        "| kind | faults | from | rejoin p50 (s) | rejoin max (s) | caught up p50 (s) | caught up max (s) |",
        "|---|---:|---|---:|---:|---:|---:|",
    ]
    for kind in kinds:
        of_kind = [r for r in rows if r.kind == kind]
        rejoin = spread(
            [
                r.rejoined - _first(r.started, r.fault)
                for r in of_kind
                if r.rejoined is not None
            ]
        )
        caught = spread(
            [
                r.caught_up - _first(r.started, r.fault)
                for r in of_kind
                if r.caught_up is not None
            ]
        )
        origin = "start" if kind == "kill" else "fault"
        lines.append(
            f"| {kind} | {len(of_kind)} | {origin} | {' | '.join((*rejoin, *caught))} |"
        )
    return lines


def fault_table(rows: list[FaultRow], t0: float) -> list[str]:
    """One row per fault with the seconds since `t0` of each step of its recovery; the counts
    still missing at a timeout, when there was one."""

    def cell(ts: float | None) -> str:
        return "-" if ts is None else f"{ts - t0:.0f}"

    def missing(row: FaultRow) -> str:
        if row.missing is None:
            return "-"
        counts = zip(MISSING_NAMES, row.missing, strict=True)
        return ", ".join(f"{name} {count}" for name, count in counts)

    if not rows:
        return []
    show = any(r.missing is not None for r in rows)
    return [
        "### Faults",
        "",
        "| node | kind | at (s) | started (s) | rejoined (s) | caught up (s) |"
        + (" missing |" if show else ""),
        "|---|---|---|---|---|---|" + ("---|" if show else ""),
        *(
            f"| {r.node} | {r.kind} | {cell(r.fault)} | {cell(r.started)} | "
            f"{cell(r.rejoined)} | {cell(r.caught_up)} |"
            + (f" {missing(r)} |" if show else "")
            for r in rows
        ),
        "",
    ]


def read_heights(run_dir: Path) -> list[tuple[float, int]]:
    """(time a validator first showed the height, height) of `heights.jsonl`, one poller per
    validator at `HEIGHT_POLL_S` (0.1 s); empty without the file."""
    path = run_dir / "heights.jsonl"
    if not path.exists():
        return []
    records = (json.loads(line) for line in path.read_text().splitlines())
    return [
        (r["validator"], r["height"]) for r in records if r["validator"] is not None
    ]


def mass_restart_lines(
    rows: Sequence[FaultRow], heights: Iterable[tuple[float, int]], t0: float
) -> list[str]:
    """One line per mass restart (the rows of that kind with one fault time): its time since
    `t0`, seconds from the end of its restarts (the last `started`, else the fault) to the first
    height above any shown until then, then seconds from the fault to the last `rejoined` and
    to the last `caught_up`. `never` for a step that did not happen."""
    heights = list(heights)
    lines = []
    for at in dict.fromkeys(r.fault for r in rows if r.kind == MASS_RESTART):
        group = [r for r in rows if r.kind == MASS_RESTART and r.fault == at]
        restarted = max((r.started for r in group if r.started is not None), default=at)
        before = max((h for ts, h in heights if ts <= restarted), default=None)
        decide = min(
            (
                ts
                for ts, h in heights
                if ts > restarted and (before is None or h > before)
            ),
            default=None,
        )
        queries = [r.caught_up for r in group if r.query]
        lines.append(
            f"- Mass restart: all {len(group)} nodes at {at - t0:.0f} s, "
            f"first decide after {_after(decide, restarted)}, "
            f"all rejoined after {_after_all([r.rejoined for r in group], at)} "
            f"(all caught up after {_after_all(queries, at)})"
        )
    return lines


def _after(ts: float | None, at: float) -> str:
    return "never" if ts is None else f"{ts - at:.0f} s"


def _after_all(times: Sequence[float | None], at: float) -> str:
    """The last of `times`; `never` unless all happened."""
    seen = [t for t in times if t is not None]
    return _after(max(seen) if len(seen) == len(times) and seen else None, at)


def floor_span(row: FaultRow, end: float) -> tuple[float, float]:
    """(start, stop) of `row` for the floor: from the fault until its node `rejoined`, so a
    query node's catch-up does not count, plus `FLOOR_TAIL_S`. A timed out node stays faulty
    until `end`, an interrupted fault ends at `interrupted` plus the tail."""
    over = row.rejoined if row.rejoined is not None else row.interrupted
    if row.timeout is not None or over is None:
        return row.fault, end
    return row.fault, over + FLOOR_TAIL_S


class CounterSplit(NamedTuple):
    """Counter increase and seconds of the sample intervals within a fault and outside."""

    during: float
    during_s: float
    outside: float
    outside_s: float


def counter_split(
    samples: Sequence[tuple[float, float]],
    spans: Sequence[tuple[float, float]],
    start: float,
    stop: float,
) -> CounterSplit:
    """Increase of a counter over the intervals between consecutive `samples`, (ts, value),
    within [`start`, `stop`]. An interval overlapping any of the fault `spans` counts as
    during. `samples` are the highest value over the nodes that answered: it drops when the
    leading node restarts or a sample lacks it. The running maximum over all `samples` is
    diffed, so a drop adds nothing and the rise back to the old value is not counted."""
    during = [0.0, 0.0]
    outside = [0.0, 0.0]
    peak = list(accumulate((v for _, v in samples), max))
    series = [(ts, p) for (ts, _), p in zip(samples, peak, strict=True)]
    inside = [c for c in series if start <= c[0] <= stop]
    for (a, value_a), (b, value_b) in pairwise(inside):
        into = during if any(s < b and e > a for s, e in spans) else outside
        into[0] += value_b - value_a
        into[1] += b - a
    return CounterSplit(*during, *outside)


def common_window(
    series: Sequence[Sequence[tuple[float, float]]], start: float, stop: float
) -> tuple[float, float]:
    """[`start`, `stop`] clipped to the time every non-empty series has samples in, so counters
    sampled on different grids cover the same interval."""
    firsts = [s[0][0] for s in series if s]
    lasts = [s[-1][0] for s in series if s]
    return max([start, *firsts]), min([stop, *lasts])


_VIEW_SAMPLE = re.compile(
    r'"ts": ([\d.]+), "node".*?"consensus_current_view": ([\d.]+)'
)


def read_consensus(run_dir: Path) -> list[dict[str, float]]:
    """The samples of `consensus.jsonl`; empty without the file."""
    path = run_dir / "consensus.jsonl"
    if not path.exists():
        return []
    return [json.loads(line) for line in path.read_text().splitlines()]


def view_samples(
    run_dir: Path, consensus: Sequence[Mapping[str, float]]
) -> list[tuple[float, float]]:
    """(ts, highest view over the nodes): the `views` of the `consensus` samples, else the
    scrapes of `metrics.jsonl` per second (runs from before `views`; a second holds only the
    nodes scraped in it); empty without either."""
    if consensus and all("views" in r for r in consensus):
        return [(r["ts"], r["views"]) for r in consensus]
    path = run_dir / "metrics.jsonl"
    if not path.exists():
        return []
    best: dict[int, float] = {}
    with path.open() as f:
        for line in f:
            if m := _VIEW_SAMPLE.search(line):
                second = int(float(m[1]))
                best[second] = max(best.get(second, 0.0), float(m[2]))
    return [(float(t), v) for t, v in sorted(best.items())]


def counter_samples(
    consensus: Sequence[Mapping[str, float]], key: str
) -> list[tuple[float, float]]:
    """(ts, highest `key` over the nodes) of the `consensus` samples."""
    return [(r["ts"], r[key]) for r in consensus]


def floor_lines(timeouts: CounterSplit, views: CounterSplit) -> list[str]:
    """The view timeout floor: timeouts and views outside any fault, and with one."""

    def part(name: str, timed_out: float, views: float, seconds: float) -> str:
        rate = f"{100 * timed_out / views:.2f} %" if views else "-"
        return f"{name} {timed_out:.0f} timeouts in {views:.0f} views ({rate}) over {seconds:.0f} s"

    return [
        "- View timeouts: "
        + part("outside faults", timeouts.outside, views.outside, views.outside_s)
        + "; "
        + part("during faults", timeouts.during, views.during, views.during_s)
    ]


class LogPattern(NamedTuple):
    """A node log message counted under `name`; `prefix` is matched against `fields.message`
    (messages carry suffixes). With `field`, one count per value of that field, as
    `name[value]`."""

    name: str
    prefix: str
    field: str | None = None


LOG_PATTERNS: tuple[LogPattern, ...] = (
    LogPattern(
        "stub",
        "parent state unavailable; queued on parent for retry (from_header stub inserted)",
    ),
    LogPattern(
        "no-propose", "timeout: we were the leader but did not propose", "missing"
    ),
    LogPattern("no-vote1", "timeout: we did not vote1", "missing"),
    LogPattern("header-failed", "header creation failed"),
    LogPattern("frontier", "fetching block frontier from peers"),
    LogPattern("unsafe", "proposal no longer safe against the current lock"),
    LogPattern("parent-fetch", "parent proposal missing; requesting fetch"),
    LogPattern("cert2-fetch", "have certificate2 but no proposal; requesting fetch"),
)


def log_label(line: str) -> tuple[str, float] | None:
    """(label, epoch seconds) of a node log line matching a `LOG_PATTERNS` entry, else
    `None`. Lines that are not JSON (the container's own output) match nothing."""
    if not line.startswith("{"):
        return None
    for pattern in LOG_PATTERNS:
        if pattern.prefix not in line:
            continue
        record = json.loads(line)
        fields = record["fields"]
        if not fields["message"].startswith(pattern.prefix):
            continue
        label = pattern.name
        if pattern.field is not None:
            label += f"[{fields[pattern.field]}]"
        return label, datetime.fromisoformat(record["timestamp"]).timestamp()
    return None


def count_log(
    lines: Iterable[str],
    windows: Sequence[tuple[float, float]],
    start: float,
    stop: float,
) -> tuple[Counter[str], Counter[str]]:
    """(during, outside) counts per label of the matching lines within [`start`, `stop`];
    during: inside one of the fault `windows` of the node."""
    during: Counter[str] = Counter()
    outside: Counter[str] = Counter()
    for line in lines:
        if (hit := log_label(line)) is None or not start <= hit[1] <= stop:
            continue
        label, ts = hit
        into = during if any(a <= ts <= b for a, b in windows) else outside
        into[label] += 1
    return during, outside


def log_events(
    lines: Iterable[str], start: float, stop: float
) -> list[tuple[float, str]]:
    """(epoch seconds, label) of the `LOG_PATTERNS` lines within [`start`, `stop`]."""
    hits = (hit for line in lines if (hit := log_label(line)) is not None)
    return [(ts, label) for label, ts in hits if start <= ts <= stop]


def node_log_events(
    run_dir: Path, node: str, start: float, stop: float
) -> list[tuple[float, str]]:
    """`log_events` of the logs of `node`."""
    events: list[tuple[float, str]] = []
    for path in node_log_files(run_dir, node):
        with gzip.open(path, "rt", errors="replace") as f:
            events += log_events(f, start, stop)
    return events


class NodeCounts(TypedDict):
    region: str
    during: dict[str, int]
    outside: dict[str, int]


def node_log_files(run_dir: Path, node: str) -> list[Path]:
    """The logs of `node` that exist: the one a wipe saved, then the current one."""
    host = run_dir / "hosts" / node
    names = ("espresso-node.wiped.log.gz", "espresso-node.log.gz")
    return [host / n for n in names if (host / n).exists()]


def node_counts(
    run_dir: Path,
    regions: Mapping[str, str],
    rows: Sequence[FaultRow],
    start: float,
    stop: float,
    end: float,
) -> dict[str, NodeCounts]:
    """Log counters of every node of `regions` that has a log, split by its own `floor_span`
    windows."""
    out: dict[str, NodeCounts] = {}
    for node, region in regions.items():
        files = node_log_files(run_dir, node)
        if not files:
            continue
        windows = [floor_span(r, end) for r in rows if r.node == node]
        during: Counter[str] = Counter()
        outside: Counter[str] = Counter()
        for path in files:
            with gzip.open(path, "rt", errors="replace") as f:
                d, o = count_log(f, windows, start, stop)
            during += d
            outside += o
        out[node] = NodeCounts(
            region=region, during=dict(during), outside=dict(outside)
        )
    return out


def _label_order(label: str) -> tuple[int, str]:
    name = label.partition("[")[0]
    names = [p.name for p in LOG_PATTERNS]
    return names.index(name), label


def log_table(counts: Mapping[str, NodeCounts]) -> list[str]:
    """One row per label seen: during faults and outside them in total, then outside per
    region with its node count in the header."""
    labels = sorted(
        {
            label
            for c in counts.values()
            for d in (c["during"], c["outside"])
            for label in d
        },
        key=_label_order,
    )
    if not labels:
        return []
    regions = sorted({c["region"] for c in counts.values()})
    sizes = Counter(c["region"] for c in counts.values())

    def total(label: str, key: Literal["during", "outside"], region: str | None) -> int:
        return sum(
            c[key].get(label, 0)
            for c in counts.values()
            if region is None or c["region"] == region
        )

    return [
        "| log event | during fault | outside fault | "
        + " | ".join(f"{r} ({sizes[r]})" for r in regions)
        + " |",
        "|---|" + "---:|" * (2 + len(regions)),
        *(
            f"| {label} | {total(label, 'during', None)} | {total(label, 'outside', None)} | "
            + " | ".join(str(total(label, "outside", r)) for r in regions)
            + " |"
            for label in labels
        ),
    ]
