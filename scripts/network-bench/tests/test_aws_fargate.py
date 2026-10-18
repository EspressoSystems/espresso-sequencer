import json
import os
import signal
import subprocess
from collections.abc import Callable
from pathlib import Path
from typing import Any

import chaos as ch
import netbench
import pytest
from fakes import (
    DESCRIBE,
    DONE_STATE,
    FARGATE_SHAPE,
    ROLE_ARN,
    STS_CALL,
    FakeClock,
    FakeRunner,
    FakeSystem,
    FargateFleetHarness,
    FargateHarness,
    FargateRunner,
    arn,
    aws_verbs,
    awsb,
    completed,
    fake_image,
    fake_images,
    fake_preflight,
    fargate_output,
    fargate_task,
    fleet,
    isolated_env,
    mapping,
    no_children,
    pg_settings,
    pricing,
    rds_spec,
    rds_tag_runner,
    remote,
    respond_to_pulls,
    sts_response,
    task_arn,
    valid_result,
    write_collected_run,
)

EXEC_ROLE_ARN = f"{ROLE_ARN}-exec"


def test_ec2_names_leave_out_the_fargate_task() -> None:
    hosts = fleet(3, fargate=True)
    assert awsb.ec2_names(hosts) == ["ctl", "node1", "node2"]
    assert awsb.ec2_node_names(hosts) == ["node1", "node2"]
    assert [h["name"] for h in awsb.ec2_node_hosts(hosts)] == ["node1", "node2"]


def test_ec2_names_of_an_ec2_fleet_are_every_host() -> None:
    hosts = fleet(3)
    assert awsb.ec2_names(hosts) == list(hosts)
    assert awsb.ec2_node_names(hosts) == ["node0", "node1", "node2"]


def test_pg_host_is_ctl_for_a_fargate_query_node() -> None:
    assert awsb.pg_host("node0", {"node0"}) == "ctl"
    assert awsb.pg_host("node0", set()) == "node0"
    assert awsb.pg_json("node1", {"node1"}) == f"{awsb.BENCH_DIR}/pg.node1.json"
    assert awsb.pg_json("node1", set()) == awsb.PG_JSON


def test_pg_shell_reads_the_given_pg_json() -> None:
    script = awsb.pg_shell("pg_isready", "/opt/bench/pg-node1.json")
    assert "/opt/bench/pg-node1.json" in script
    assert awsb.PG_JSON in awsb.pg_shell("pg_isready")


def tf_resource(kind: str, name: str) -> str:
    source = (awsb.TERRAFORM_SRC / "main.tf").read_text()
    start = source.index(f'resource "{kind}" "{name}" {{')
    return source[start : source.index("\n}\n", start)]


def test_the_log_group_arn_names_the_group_without_its_wildcard() -> None:
    group = arn("logs", "log-group:/espresso-bench/fleet1")
    assert awsb.arn_parts(group) == ("log-group", "/espresso-bench/fleet1")
    assert awsb.arn_parts(f"{group}:*") == ("log-group", "/espresso-bench/fleet1")


def test_the_exec_role_belongs_to_the_fleet_of_the_scheduler_role() -> None:
    assert awsb.role_fleet(EXEC_ROLE_ARN) == awsb.role_fleet(ROLE_ARN) == "fleet1"


# TEST:sweep-plan-fargate-order-ok
def test_the_sweep_stops_the_cluster_before_it_deletes_what_holds_the_security_group():
    arns = [
        arn("ec2", "key-pair/key-1"),
        arn("ec2", "security-group/sg-1"),
        arn("logs", "log-group:/espresso-bench/f:*"),
        arn("elasticfilesystem", "file-system/fs-1"),
        arn("elasticfilesystem", "access-point/fsap-1"),
        arn("rds", "db:espresso-bench-f"),
        arn("ec2", "instance/i-1"),
        arn("ecs", "task-definition/espresso-bench-f-node0:3"),
        arn("ecs", "cluster/espresso-bench-f"),
        arn("scheduler", "schedule-group/espresso-bench-f"),
    ]
    verbs = [
        " ".join(step["args"][:2]) if step["action"] == "aws" else step["action"]
        for step in awsb.sweep_plan(arns)
    ]
    assert verbs == [
        "scheduler delete-schedule-group",
        "delete_ecs_cluster",
        "ecs deregister-task-definition",
        "ecs delete-task-definitions",
        "ec2 terminate-instances",
        "ec2 wait",
        "delete_rds",
        "efs delete-access-point",
        "delete_efs",
        "logs delete-log-group",
        "delete_security_group",
        "ec2 delete-key-pair",
    ]


# TEST:sweep-iam-exec-role-ok
def test_the_sweep_deletes_the_exec_role_with_the_scheduler_role() -> None:
    mappings = [
        mapping(
            arn("scheduler", "schedule-group/espresso-bench-fleet1"),
            "fleet1",
            None,
            None,
        )
    ]
    runner = rds_tag_runner(mappings, [], roles=[ROLE_ARN, EXEC_ROLE_ARN])
    awsb.sweep(FakeSystem(run=runner), awsb.REGION, "fleet1")
    deleted = [
        c[c.index("--role-name") + 1] for c in runner.calls if "delete-role" in c
    ]
    assert deleted == ["espresso-bench-fleet1", "espresso-bench-fleet1-exec"]


def test_the_cluster_loses_its_tasks_before_it_is_deleted() -> None:
    runner = FakeRunner({("aws",): completed()})
    runner.respond("list-tasks", lambda _: completed(stdout='["t1", "t2"]'))
    awsb.delete_ecs_cluster(runner, awsb.REGION, "c1")
    verbs = aws_verbs(runner)
    assert verbs == [
        ("ecs", "list-tasks"),
        ("ecs", "stop-task"),
        ("ecs", "stop-task"),
        ("ecs", "wait"),
        ("ecs", "delete-cluster"),
    ]


def test_a_cluster_that_is_gone_is_tolerated() -> None:
    runner = FakeRunner(
        {("aws",): completed(returncode=254, stderr="ClusterNotFoundException")}
    )
    awsb.delete_ecs_cluster(runner, awsb.REGION, "c1")
    assert aws_verbs(runner) == [("ecs", "list-tasks")]


def efs_runner(targets: list[list[str]]) -> FakeRunner:
    """`describe-mount-targets` answers the successive `targets`, the last one repeating."""
    runner = FakeRunner({("aws",): completed()})

    def describe(argv: list[str]) -> subprocess.CompletedProcess:
        found = targets.pop(0) if len(targets) > 1 else targets[0]
        return completed(stdout=json.dumps(found))

    runner.respond("describe-mount-targets", describe)
    return runner


# TEST:efs-delete-waits-for-mount-targets-ok
def test_the_file_system_is_deleted_once_its_mount_targets_are_gone(
    clock: FakeClock,
) -> None:
    runner = efs_runner([["fsmt-1"], ["fsmt-1"], []])
    awsb.delete_efs(FakeSystem(run=runner, clock=clock), awsb.REGION, "fs-1")
    assert aws_verbs(runner) == [
        ("efs", "describe-mount-targets"),
        ("efs", "delete-mount-target"),
        ("efs", "describe-mount-targets"),
        ("efs", "describe-mount-targets"),
        ("efs", "delete-file-system"),
    ]
    assert clock.sleeps == [awsb.EFS_POLL_S]


def test_a_mount_target_that_stays_fails_the_sweep(clock: FakeClock) -> None:
    runner = efs_runner([["fsmt-1"]])
    with pytest.raises(awsb.Refused, match="still has mount targets"):
        awsb.delete_efs(FakeSystem(run=runner, clock=clock), awsb.REGION, "fs-1")
    assert ("efs", "delete-file-system") not in aws_verbs(runner)


def test_a_file_system_that_is_gone_is_tolerated() -> None:
    runner = FakeRunner(
        {("aws",): completed(returncode=254, stderr="FileSystemNotFound")}
    )
    awsb.delete_efs(FakeSystem(run=runner), awsb.REGION, "fs-1")
    assert aws_verbs(runner) == [("efs", "describe-mount-targets")]


# TEST:fargate-reaper-schedules-ok
def test_one_reaper_time_drives_the_rds_and_efs_schedules() -> None:
    assert "var.reaper_at" in tf_resource("aws_scheduler_schedule", "rds_delete")
    for name in ("efs_mount_target_delete", "efs_access_point_delete", "efs_delete"):
        assert "var.reaper_at" in tf_resource("aws_scheduler_schedule", name)
    assert "delete_at" not in (awsb.TERRAFORM_SRC / "variables.tf").read_text()


GOLDEN_DIR = Path(__file__).parent / "golden"
EXPIRES_AT = "2026-09-29T17:00:00Z"


def task_cfg(**changes: Any) -> Any:
    fields: dict[str, Any] = {
        "tag": "x",
        "nodes": 3,
        "query_db": "rds",
        "db_modes": ("rds",),
        "query_placement": "fargate",
        "consensus_storage": "journal",
        "load": netbench.BenchConfig(submit_nodes=1),
        **changes,
    }
    return awsb.RunConfig(**fields)


def fargate_manifest() -> dict:
    """The fleet manifest keys `render_task_def` reads, for a 3-node fleet with node0 on
    Fargate."""
    hosts = awsb.place_fargate(awsb.plan_hosts(task_cfg()), {"fargate": FARGATE_SHAPE})
    return {
        "name": "fleet1",
        "hosts": hosts,
        "fargate": fargate_output(["node0"]),
    }


def task_def(cfg: Any) -> dict:
    return awsb.render_task_def(
        cfg, fargate_manifest(), "node0", fake_images(), EXPIRES_AT
    )


GOLDEN_TASK_DEFS = {
    "fargate-task-def.json": task_cfg(latency="decaf-2025", tcp_cc="bbr"),
    "fargate-task-def-no-latency.json": task_cfg(),
}


# TEST:fargate-task-def-golden-ok, TEST:fargate-task-def-no-latency-golden-ok
@pytest.mark.parametrize("name", GOLDEN_TASK_DEFS)
def test_the_task_definition_matches_its_golden_file(name: str) -> None:
    rendered = json.dumps(task_def(GOLDEN_TASK_DEFS[name]), indent=2) + "\n"
    assert rendered == (GOLDEN_DIR / name).read_text()


def sysctl_check(proc: Path, sysctls: str) -> subprocess.CompletedProcess[str]:
    script = f"set -euo pipefail\n{awsb._SYSCTL_CHECK}"
    env = {**os.environ, "BENCH_PROC_SYS": str(proc), "BENCH_SYSCTLS": sysctls}
    return subprocess.run(
        ["bash", "-c", script], env=env, capture_output=True, text=True, check=False
    )


def proc_sys(tmp_path: Path, **values: str) -> Path:
    for key, value in values.items():
        path = tmp_path / key.replace("__", "/")
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(value)
    return tmp_path


# TEST:fargate-entry-checks-sysctl-ok
def test_the_wrapper_accepts_the_sysctls_its_definition_asked_for(tmp_path) -> None:
    proc = proc_sys(
        tmp_path,
        net__ipv4__tcp_congestion_control="bbr\n",
        net__ipv4__tcp_rmem="4096\t131072\t268435456\n",
    )
    asked = (
        "net.ipv4.tcp_congestion_control=bbr;net.ipv4.tcp_rmem=4096 131072 268435456"
    )
    done = sysctl_check(proc, asked)
    assert (done.returncode, done.stderr) == (0, "")


def test_the_wrapper_exits_78_on_a_sysctl_with_another_value(tmp_path) -> None:
    proc = proc_sys(tmp_path, net__ipv4__tcp_congestion_control="cubic\n")
    done = sysctl_check(proc, "net.ipv4.tcp_congestion_control=bbr")
    assert done.returncode == awsb.FARGATE_SYSCTL_RC == 78
    assert done.stderr.strip() == (
        "sysctl net.ipv4.tcp_congestion_control=cubic, expected bbr"
    )


def test_the_wrapper_fails_on_a_sysctl_the_kernel_lacks(tmp_path) -> None:
    done = sysctl_check(tmp_path, "net.ipv4.tcp_congestion_control=bbr")
    assert done.returncode not in (0, 78)


def test_the_wrapper_has_no_sysctl_to_check_without_latency(tmp_path) -> None:
    assert sysctl_check(tmp_path, "").returncode == 0


# TEST:fargate-entry-env-literal-ok
def test_the_wrapper_exports_node_env_values_literally(tmp_path) -> None:
    env_file = tmp_path / "node.env"
    env_file.write_text(
        "MNEMONIC=test test junk\nPASSWORD=pa$$word `x` $(y)\nTOKEN=abc==\nEMPTY=\n"
    )
    script = f'env_file={env_file}\n{awsb._EXPORT_ENV}printf "%s|" "$MNEMONIC" "$PASSWORD" "$TOKEN" "$EMPTY"'
    done = subprocess.run(
        ["bash", "-c", script], capture_output=True, text=True, check=True
    )
    assert done.stdout == "test test junk|pa$$word `x` $(y)|abc==||"


def env_names(cfg: Any) -> set[str]:
    environment = task_def(cfg)["containerDefinitions"][0]["environment"]
    return {e["name"] for e in environment}


def test_the_definition_checks_sysctls_only_with_latency() -> None:
    assert "BENCH_SYSCTLS" not in env_names(task_cfg())
    assert "BENCH_SYSCTLS" in env_names(task_cfg(latency="decaf-2025"))
    assert task_def(task_cfg())["containerDefinitions"][0]["systemControls"] == []


# TEST:fargate-sysctls-drop-default-qdisc-ok
@pytest.mark.parametrize("tcp_cc", ["cubic", "bbr"])
def test_a_task_cannot_set_the_global_default_qdisc(tcp_cc: str) -> None:
    sysctls = awsb.fargate_sysctls(task_cfg(latency="decaf-2025", tcp_cc=tcp_cc))
    names = {ctl["namespace"] for ctl in sysctls}
    assert "net.core.default_qdisc" not in names
    assert names == {key for key, _ in awsb.latency.tcp_sysctls(tcp_cc)} - {
        "net.core.default_qdisc"
    }


# TEST:fargate-task-def-image-digest-ok
def test_the_task_runs_the_image_digest_of_the_run() -> None:
    images = fake_images()
    images["espresso-node"] = {
        **images["espresso-node"],
        "digest": "sha256:" + "7" * 64,
    }
    rendered = awsb.render_task_def(
        task_cfg(), fargate_manifest(), "node0", images, EXPIRES_AT
    )
    assert rendered["containerDefinitions"][0]["image"].endswith("@sha256:" + "7" * 64)


@pytest.fixture
def harness(isolated: Path, monkeypatch: pytest.MonkeyPatch) -> FargateHarness:
    return FargateHarness(monkeypatch, isolated)


def first(runner: FakeRunner, *needle: str) -> int:
    """The index of the first call containing every needle."""
    return next(
        i
        for i, call in enumerate(runner.calls)
        if all(n in " ".join(call) for n in needle)
    )


def ctl_commands(runner: FakeRunner) -> list[str]:
    ctl_ip = fleet(1)["ctl"]["public_ip"]
    return [
        call[-1]
        for call in runner.calls
        if call[0] == "ssh" and call[-2].endswith(f"@{ctl_ip}")
    ]


def fargate_runner(**kwargs: Any) -> FargateRunner:
    return FargateRunner([DONE_STATE], describe=DESCRIBE, **kwargs)


@pytest.mark.usefixtures("valid")
def test_a_three_node_run_with_the_query_node_on_fargate(harness: FargateHarness):
    runner = fargate_runner()
    assert harness.run(runner) == awsb.EXIT_OK
    tfvars = netbench.read_json(harness.fleet_dir / "terraform/terraform.tfvars.json")
    # TEST:fargate-plan-hosts-ok
    assert sorted(tfvars["hosts"]) == ["ctl", "node1", "node2"]
    assert tfvars["fargate"] == {"nodes": ["node0"]}
    assert "reaper_at" in tfvars
    # TEST:fargate-run-before-render-ok
    assert first(runner, "register-task-definition") < first(runner, "run-task")
    assert first(runner, "run-task") < first(runner, "describe-tasks")
    assert first(runner, "describe-tasks") < first(runner, "rsync", "node.env")
    ctl_env = (harness.run_dir / "hosts/ctl/ctl.env").read_text()
    assert "ESPRESSO_API_NODE_URL=http://10.0.0.2:8080" in ctl_env
    assert runner.ran("rsync", "ubuntu@203.0.113.1:/efs/node0/bench/")
    topology = netbench.read_json(harness.run_dir / "topology.json")
    assert topology["nodes"]["node0"] == "http://10.0.0.2:8080"
    # No ssh was addressed to the task.
    assert not any(call[-2].endswith("@") for call in runner.calls if call[0] == "ssh")
    assert harness.run_dir.joinpath("hosts/node0/task.json").exists()
    assert not harness.run_dir.joinpath("hosts/node0/start.sh").exists()


@pytest.mark.usefixtures("valid")
def test_the_task_advertises_its_network_interface(harness: FargateHarness):
    assert harness.run(fargate_runner()) == awsb.EXIT_OK
    env = (harness.run_dir / "hosts/node0/node.env").read_text()
    # TEST:fargate-node-env-advertise-eni-ok
    assert "ESPRESSO_NODE_CLIQUENET_ADVERTISE_ADDRESS=10.0.0.2:9000\n" in env
    dns = "ip-10-0-0-2.eu-west-1.compute.internal"
    assert f"ESPRESSO_NODE_LIBP2P_ADVERTISE_ADDRESS={dns}:1769\n" in env
    assert "ESPRESSO_NODE_POSTGRES_DATABASE=espresso\n" in env
    assert (
        awsb.nb.read_json(harness.run_dir / "manifest.json")["hosts_info"]["node0"][
            "public_ip"
        ]
        == ""
    )
    assert (
        netbench.read_json(harness.fleet_dir / "hosts.json")["node0"]["public_ip"] == ""
    )


@pytest.mark.usefixtures("valid")
def test_the_go_is_written_before_the_nodes_start_and_the_spread_includes_the_task(
    harness: FargateHarness,
):
    runner = fargate_runner()
    assert harness.run(runner) == awsb.EXIT_OK
    # TEST:fargate-go-file-written-ok
    go = first(runner, "echo", "> /efs/node0/bench/go")
    assert go < first(runner, "docker start espresso-node")
    # The wrapper starts the node 0.2 s after the docker starts of the validators.
    manifest = netbench.read_json(harness.run_dir / "manifest.json")
    assert manifest["start_spread_s"] == pytest.approx(0.2)
    # TEST:fargate-start-spread-includes-task-ok
    assert "started_at" in " ".join(ctl_commands(runner))


def test_the_go_holds_the_start_time_in_nanoseconds(isolated: Path):
    runner = fargate_runner()
    hosts = remote(runner, isolated)
    awsb.signal_go(hosts, "node0", 1015.5)
    assert "echo 1015500000000 > /efs/node0/bench/go" in runner.calls[-1][-1]


@pytest.mark.usefixtures("valid")
def test_the_task_is_collected_through_ctl(harness: FargateHarness):
    runner = fargate_runner()
    assert harness.run(runner) == awsb.EXIT_OK
    commands = ctl_commands(runner)
    # TEST:fargate-collect-via-ctl-ok
    script = next(c for c in commands if "du -sb /efs/node0/store" in c)
    assert "du -sb /efs/node0/payload > du-payload.txt" in script
    assert "gzip -kf espresso-node.log" in script
    fetch = next(
        call
        for call in runner.calls
        if call[0] == "rsync" and call[-1] == str(harness.run_dir / "hosts/node0")
    )
    assert "--exclude=node.env" in fetch
    assert any(arg.endswith(":/efs/node0/bench/") for arg in fetch)
    # TEST:fargate-freeze-stop-task-ok
    freeze = first(runner, "stop-task", "--reason freeze")
    assert freeze < first(runner, "wait tasks-stopped") < first(runner, "du -sb /efs")


@pytest.mark.usefixtures("valid")
def test_postgres_is_gated_sampled_and_summarized_from_ctl(harness: FargateHarness):
    runner = fargate_runner()
    assert harness.run(runner) == awsb.EXIT_OK
    endpoint = f"{awsb.BENCH_DIR}/pg.node0.json"
    commands = ctl_commands(runner)
    # TEST:fargate-pg-gates-on-ctl-ok
    gates = [c for c in commands if "pg_isready" in c]
    assert gates
    assert all(endpoint in c for c in gates)
    stats = next(c for c in commands if "pg-stats-node0.json" in c)
    assert endpoint in stats
    # TEST:fargate-hostmon-pg-on-ctl-ok
    sampler = next(c for c in commands if "bench-host" in c)
    assert f"--role ctl --pg {endpoint}" in sampler
    assert not any("--role query" in c for c in commands)
    assert (harness.run_dir / "hosts/ctl/pg.node0.json").exists()
    assert not (harness.run_dir / "hosts/node0/pg.json").exists()


@pytest.mark.usefixtures("valid")
def test_the_tasks_and_definitions_are_gone_before_the_destroy(
    harness: FargateHarness,
):
    runner = fargate_runner()
    assert harness.run(runner) == awsb.EXIT_OK
    destroy = first(runner, "tofu", "destroy")
    # TEST:fargate-down-stops-tasks-first-ok
    assert first(runner, "list-tasks") < first(runner, "stop-task", "--reason down")
    assert first(runner, "deregister-task-definition") < destroy
    assert first(runner, "delete-task-definitions") < destroy
    manifest = netbench.read_json(harness.run_dir / "manifest.json")
    family = "arn:aws:ecs:eu-west-1:1:task-definition/espresso-bench-run1-node0:1"
    assert manifest["tasks"] == {"node0": task_arn("node0")}
    assert manifest["task_definitions"] == {"node0": family}


def fails_at_startup(harness: FargateHarness, runner: FargateRunner) -> str:
    """Runs and returns the summary: the run failed before any node started, and still
    destroyed the fleet."""
    assert harness.run(runner) == awsb.EXIT_FAILED
    assert not runner.ran("docker start espresso-node")
    assert runner.ran("tofu", "destroy")
    return (harness.run_dir / "summary.md").read_text()


# TEST:fargate-register-validation-fails
def test_a_task_definition_aws_refuses_fails_the_run_with_its_message(
    harness: FargateHarness,
):
    refused = (
        "An error occurred (ClientException) when calling the RegisterTaskDefinition "
        "operation: sysctl net.ipv4.tcp_rmem is not allowed"
    )
    runner = fargate_runner(register_error=refused)
    summary = fails_at_startup(harness, runner)
    assert "sysctl net.ipv4.tcp_rmem is not allowed" in summary
    assert not runner.ran("run-task")


# TEST:fargate-run-task-failures-fails
def test_a_task_aws_cannot_place_fails_the_run_with_its_reason(harness: FargateHarness):
    failures = [{"arn": "arn:x", "reason": "RESOURCE:MEMORY", "detail": "no capacity"}]
    summary = fails_at_startup(harness, fargate_runner(run_task_failures=failures))
    assert "run-task of node0 failed: RESOURCE:MEMORY no capacity" in summary


# TEST:fargate-sysctl-rejected-at-start-fails
def test_a_container_that_cannot_start_fails_the_run_with_the_reason(
    harness: FargateHarness,
):
    stopped = fargate_task(
        "STOPPED",
        stopped_reason="Task failed to start",
        container_reason="CannotStartContainerError: sysctl net.ipv4.tcp_rmem not allowed",
    )
    runner = fargate_runner(
        task_states=[{"node0": fargate_task("PENDING")}, {"node0": stopped}]
    )
    summary = fails_at_startup(harness, runner)
    assert "node0 task stopped: Task failed to start" in summary
    assert "CannotStartContainerError: sysctl net.ipv4.tcp_rmem not allowed" in summary
    # Teardown still stops what is left and deletes the definition.
    assert runner.ran("deregister-task-definition")


# TEST:fargate-sysctl-mismatch-exit-fails
def test_a_wrapper_exit_on_a_sysctl_mismatch_fails_the_run_with_the_log_tail(
    harness: FargateHarness,
):
    stopped = fargate_task(
        "STOPPED", stopped_reason="Essential container exited", exit_code=78
    )
    tail = "sysctl net.ipv4.tcp_congestion_control=cubic, expected bbr"
    runner = fargate_runner(task_states=[{"node0": stopped}], log_lines=[tail])
    summary = fails_at_startup(harness, runner)
    assert "espresso-node exit 78" in summary
    assert tail in summary


# TEST:fargate-start-timeout-fails
def test_a_task_that_never_runs_fails_after_the_start_timeout(harness: FargateHarness):
    pending = fargate_task("PROVISIONING")
    runner = fargate_runner(task_states=[{"node0": pending}])
    summary = fails_at_startup(harness, runner)
    assert f"not ready after {awsb.FARGATE_START_S:.0f} s" in summary
    assert "'node0': 'PROVISIONING'" in summary


def test_a_running_task_without_the_ready_marker_is_not_ready(harness: FargateHarness):
    runner = fargate_runner()
    runner.ready = False
    summary = fails_at_startup(harness, runner)
    assert "'node0': 'RUNNING'" in summary


# TEST:fargate-running-no-ip-fails
def test_a_running_task_without_an_address_fails_the_run(harness: FargateHarness):
    running = fargate_task()
    running["attachments"][0]["details"].pop(0)
    summary = fails_at_startup(
        harness, fargate_runner(task_states=[{"node0": running}])
    )
    assert (
        "node0 is RUNNING but its network interface has no privateIPv4Address"
        in summary
    )


# EDGE:fargate-teardown-before-launch
def test_a_failure_before_the_first_task_leaves_nothing_to_stop(
    harness: FargateHarness,
):
    runner = fargate_runner(register_error="ClientException: no")
    fails_at_startup(harness, runner)
    assert runner.ran("list-tasks")
    assert not runner.ran("stop-task")
    assert not runner.ran("deregister-task-definition")


# EDGE:fargate-interrupt-during-wait
def test_an_interrupt_while_waiting_stops_the_tasks(harness: FargateHarness):
    pending = fargate_task("PROVISIONING")
    runner = fargate_runner(task_states=[{"node0": pending}])
    system = FakeSystem(run=runner)
    runner.on_describe = lambda n: system.fire(signal.SIGINT) if n == 2 else None
    assert awsb.cmd_run(harness.args(), system) == awsb.EXIT_FAILED
    assert runner.ran("stop-task", "--reason freeze")
    assert runner.ran("tofu", "destroy")
    assert "interrupted" in (harness.run_dir / "summary.md").read_text()


# TEST:fargate-stopped-mid-load-fails
def test_a_task_that_stops_during_the_load_fails_check_nodes(isolated: Path):
    stopped = fargate_task(
        "STOPPED", stopped_reason="OutOfMemoryError: container killed"
    )
    runner = fargate_runner(task_states=[{"node0": stopped}], log_lines=["oom"])
    tasks = awsb.FargateTasks(
        runner, awsb.REGION, fargate_output(["node0"]), {"node0": task_arn("node0")}
    )
    hosts = remote(runner, isolated)
    with pytest.raises(awsb.RemoteError, match="node0 task stopped: OutOfMemoryError"):
        awsb.check_nodes(hosts, frozenset(), tasks)


def test_a_running_task_passes_check_nodes(isolated: Path):
    runner = fargate_runner()
    tasks = awsb.FargateTasks(
        runner, awsb.REGION, fargate_output(["node0"]), {"node0": task_arn("node0")}
    )
    awsb.check_nodes(remote(runner, isolated), frozenset(), tasks)


# EDGE:fargate-ssh-attempt
def test_ssh_to_a_task_is_a_bug_that_fails_in_the_remote(isolated: Path):
    runner = fargate_runner()
    hosts = awsb.Remote(
        runner, isolated.relative_to(Path.cwd()), Path("key"), fleet(3, fargate=True)
    )
    with pytest.raises(ValueError, match="node0 runs on Fargate and has no ssh"):
        hosts.ssh("node0", "true")
    assert runner.calls == []


def offerings(*azs: str) -> subprocess.CompletedProcess:
    body = {"InstanceTypeOfferings": [{"Location": az} for az in azs]}
    return completed(stdout=json.dumps(body))


def rds_options(argv: list[str]) -> subprocess.CompletedProcess:
    option = {
        "EngineVersion": "18.2",
        "StorageType": "gp3",
        "AvailabilityZones": [{"Name": "eu-west-1a"}],
    }
    return completed(stdout=json.dumps({"OrderableDBInstanceOptions": [option]}))


def described_types(**shapes: tuple[int, int]) -> Callable[[list[str]], Any]:
    """`describe-instance-types` of type name -> (vCPUs, MiB), all arm64, for the types the
    call asks for."""

    def describe(argv: list[str]) -> subprocess.CompletedProcess:
        asked = argv[argv.index("--instance-types") + 1 : argv.index("--region")]
        types = [
            {
                "InstanceType": name.replace("_", "."),
                "ProcessorInfo": {"SupportedArchitectures": ["arm64"]},
                "VCpuInfo": {"DefaultVCpus": vcpus},
                "MemoryInfo": {"SizeInMiB": memory_mib},
            }
            for name, (vcpus, memory_mib) in shapes.items()
            if name.replace("_", ".") in asked
        ]
        return completed(stdout=json.dumps({"InstanceTypes": types}))

    return describe


# TEST:fargate-shape-from-node-type-ok
@pytest.mark.parametrize(
    ("shape", "cpu", "memory_mib"),
    [((16, 32768), 16384, 32768), ((8, 16384), 8192, 16384), ((4, 8192), 4096, 8192)],
)
def test_the_task_has_the_vcpus_and_memory_of_the_node_type(shape, cpu, memory_mib):
    runner = FakeRunner()
    runner.respond("describe-instance-types", described_types(c8g_x=shape))
    task = awsb.fargate_shape(runner, awsb.REGION, "c8g.x", "arm64")
    assert task == {"cpu": cpu, "memory_mib": memory_mib, "arch": "arm64"}


# TEST:fargate-shape-unsupported-fails
@pytest.mark.parametrize("shape", [(32, 65536), (16, 16384), (2, 3072)])
def test_a_node_type_fargate_does_not_offer_is_refused(shape):
    runner = FakeRunner()
    runner.respond("describe-instance-types", described_types(c8g_x=shape))
    with pytest.raises(awsb.Refused, match="a size fargate does not offer"):
        awsb.fargate_shape(runner, awsb.REGION, "c8g.x", "arm64")


def test_preflight_leaves_the_fargate_node_type_out_of_the_ec2_checks(
    monkeypatch: pytest.MonkeyPatch,
):
    cfg = task_cfg(nodes=2, query_nodes=2)
    prefix = ("aws", "--profile", "timeboost-dev")
    runner = FakeRunner(
        {
            STS_CALL: sts_response(awsb.ACCOUNT),
            (*prefix, "ec2", "describe-images"): completed(stdout='"ami-0abc"'),
        }
    )
    runner.respond(
        "describe-instance-types",
        described_types(c8g_2xlarge=(8, 16384), c8g_4xlarge=(16, 32768)),
    )
    runner.respond("Values=c8g.2xlarge", lambda _: offerings("eu-west-1a"))
    runner.respond("get-products", pricing)
    runner.respond("describe-orderable-db-instance-options", rds_options)
    monkeypatch.setattr(
        awsb, "resolve_image", lambda _http_get, ref, _platform: fake_image(ref)
    )
    hosts = awsb.plan_hosts(cfg)
    pre = awsb.preflight(FakeSystem(run=runner), cfg, hosts)
    assert set(pre["instance_prices"]) == {"c8g.2xlarge"}
    assert pre["fargate"] == FARGATE_SHAPE
    placed = awsb.place_fargate(hosts, pre)
    assert [h["name"] for h in placed if "fargate" in h] == ["node0", "node1"]
    assert not runner.ran("Values=c8g.4xlarge")


@pytest.fixture
def fleet_harness(
    isolated: Path, monkeypatch: pytest.MonkeyPatch
) -> FargateFleetHarness:
    monkeypatch.setattr(awsb, "write_report", lambda *a, **k: valid_result())
    return FargateFleetHarness(monkeypatch, isolated)


def test_up_mounts_the_file_system_on_ctl_and_prepares_the_database(
    fleet_harness: FargateFleetHarness,
):
    runner = fleet_harness.up_fargate()
    manifest = fleet_harness.fleet()
    assert manifest["fargate"] == fargate_output(["node0"])
    assert "node0" not in manifest["hosts_info"]
    mount = next(c for c in ctl_commands(runner) if "mount -t nfs4" in c)
    assert "fs-1.efs.eu-west-1.amazonaws.com:/ /efs" in mount
    assert "nfsvers=4.1" in mount
    assert "mkdir -p /efs/node0/bench" in mount
    endpoint = fleet_harness.fleet_dir / "hosts/ctl/pg.node0.json"
    assert netbench.read_json(endpoint)["database"] == "espresso"
    assert runner.ran("rsync", f"{awsb.BENCH_DIR}/pg.node0.json")
    assert any("pg_isready" in c for c in ctl_commands(runner))


def test_a_run_on_the_fleet_wipes_the_mailbox_after_stopping_the_old_task(
    fleet_harness: FargateFleetHarness,
):
    runner = fleet_harness.up_fargate()
    runner.launched.append("node0")
    mark = len(runner.calls)
    assert fleet_harness.run(runner, "--query-db", "rds") == awsb.EXIT_OK
    calls = runner.calls[mark:]
    joined = [" ".join(call) for call in calls]
    stop = next(i for i, c in enumerate(joined) if "stop-task" in c and "reset" in c)
    wipe = next(i for i, c in enumerate(joined) if "find /efs/node0 -mindepth 1" in c)
    launch = next(i for i, c in enumerate(joined) if "register-task-definition" in c)
    assert stop < wipe < launch
    assert any("DROP DATABASE" in c and "pg.node0.json" in c for c in joined)
    assert fleet_harness.fleet()["phase"] == "idle"


# EDGE:fleet-retag
def test_a_retagged_run_registers_a_task_definition_with_the_new_digest(
    fleet_harness: FargateFleetHarness, monkeypatch: pytest.MonkeyPatch
):
    new_digest = f"sha256:{'1' * 64}"
    monkeypatch.setattr(
        awsb,
        "resolve_image",
        lambda _http, ref, _platform: {**fake_image(ref), "digest": new_digest},
    )
    runner = fleet_harness.up_fargate()
    respond_to_pulls(runner, {})
    assert (
        fleet_harness.run(runner, "--query-db", "rds", "--tag", "other") == awsb.EXIT_OK
    )
    task_def = runner.task_defs["espresso-bench-fleet1-node0"]
    image = task_def["containerDefinitions"][0]["image"]
    assert image == f"{awsb.GHCR_ORG}/espresso-node:other@{new_digest}"


def test_a_fargate_fleet_refuses_a_colocated_run(fleet_harness: FargateFleetHarness):
    runner = fleet_harness.up_fargate()
    mark = len(runner.calls)
    with pytest.raises(awsb.Refused, match="--query-db colocated was not provisioned"):
        fleet_harness.run(runner, "--query-db", "colocated")
    assert runner.calls[mark:] == []


def test_a_fargate_fleet_refuses_sqlite_before_it_touches_a_host(
    fleet_harness: FargateFleetHarness,
):
    runner = fleet_harness.up_fargate()
    mark = len(runner.calls)
    with pytest.raises(awsb.Refused, match="no local disk for sqlite"):
        fleet_harness.run(runner, "--query-engine", "sqlite", "--query-db", "colocated")
    assert runner.calls[mark:] == []


def test_the_fleet_flag_cannot_move_the_query_nodes(fleet_harness: FargateFleetHarness):
    runner = fleet_harness.up_fargate()
    with pytest.raises(awsb.Refused, match="--query-placement"):
        fleet_harness.run(runner, "--query-placement", "ec2")


# EDGE:fargate-old-manifest
def test_a_config_from_before_the_flag_loads_on_ec2():
    saved = awsb.config_to_json(awsb.RunConfig(tag="x"))
    del saved["query_placement"]
    assert awsb.config_from_manifest(saved).query_placement == "ec2"
    assert awsb.fargate_specs(awsb.plan_hosts(awsb.RunConfig(tag="x"))) == []
    assert (
        awsb.fargate_names({"hosts": awsb.plan_hosts(awsb.RunConfig(tag="x"))}) == set()
    )


@pytest.mark.usefixtures("valid")
def test_two_fargate_query_nodes_get_a_task_and_a_database_each(
    harness: FargateHarness,
):
    runner = fargate_runner(query_nodes=2)
    assert harness.run(runner, "--query-nodes", "2") == awsb.EXIT_OK
    assert sorted(runner.launched) == ["node0", "node1"]
    for node, database in (("node0", "espresso"), ("node1", "espresso_node1")):
        env = (harness.run_dir / "hosts" / node / "node.env").read_text()
        assert f"ESPRESSO_NODE_POSTGRES_DATABASE={database}\n" in env
        assert (harness.run_dir / "hosts/ctl" / f"pg.{node}.json").exists()
    commands = ctl_commands(runner)
    create = next(c for c in commands if "CREATE DATABASE" in c)
    assert "CREATE DATABASE espresso_node1" in create
    sampler = next(c for c in commands if "bench-host" in c)
    assert "--pg /opt/bench/pg.node0.json --pg /opt/bench/pg.node1.json" in sampler
    for node in ("node0", "node1"):
        assert any(f"du -sb /efs/{node}/store" in c for c in commands)


REFUSED = {
    "sqlite": ({"query_engine": "sqlite"}, "no local disk for sqlite"),
    "colocated": ({"db_modes": ("colocated",)}, "needs --query-db rds"),
    "rds-and-volume": ({"db_modes": ("rds", "volume")}, "needs --query-db rds"),
    "chaos": ({"chaos": ch.ChaosConfig()}, "--chaos runs docker over ssh"),
    "bbr3": ({"tcp_cc": "bbr3"}, "--tcp-cc bbr3 needs the host kernel"),
    "bbr-hold": ({"tcp_cc": "bbr_hold"}, "--tcp-cc bbr_hold needs the host kernel"),
    "leader-trace": ({"leader_trace": True}, "--leader-trace writes to a host"),
    "latency-two-query": (
        {"latency": "decaf-2025", "query_nodes": 2},
        "cannot shape the links between fargate query nodes",
    ),
    "netem-full": (
        {"latency": "decaf-2025", "netem": "full"},
        "--netem full adds jitter and loss",
    ),
    "root-gb": ({"root_gb": "200"}, "--root-gb sizes a disk"),
}


# TEST:fargate-latency-two-query-fails, TEST:fargate-netem-full-fails,
# TEST:fargate-sqlite-fails, TEST:fargate-colocated-fails, TEST:fargate-chaos-fails,
# TEST:fargate-bbr3-fails, TEST:fargate-bbr-hold-fails, TEST:fargate-leader-trace-fails,
# TEST:fargate-root-gb-fails
@pytest.mark.parametrize("case", REFUSED)
def test_what_a_task_cannot_run_is_refused(case: str) -> None:
    changes, reason = REFUSED[case]
    with pytest.raises(awsb.Refused, match=reason):
        awsb.plan_hosts(task_cfg(**changes))


def test_the_flags_of_a_refused_run_reach_no_aws_call(
    harness: FargateHarness, monkeypatch: pytest.MonkeyPatch
) -> None:
    runner = FakeRunner()
    args = awsb.parse_args(
        ["run", "--tag", "t", "--nodes", "3", "--yes", "--query-placement", "fargate"]
    )
    with pytest.raises(awsb.Refused, match="needs --query-db rds"):
        awsb.cmd_run(args, FakeSystem(run=runner))
    assert runner.calls == []


def test_ec2_hosts_have_no_fargate_refusal() -> None:
    awsb.check_fargate(
        awsb.RunConfig(tag="x", query_engine="sqlite", leader_trace=True)
    )


def test_the_placement_flag_defaults_to_ec2_and_reaches_the_config() -> None:
    assert (
        awsb.config_from_args(awsb.parse_args(["plan", "--tag", "x"])).query_placement
        == "ec2"
    )
    args = awsb.parse_args(["plan", "--tag", "x", "--query-placement", "fargate"])
    assert awsb.config_from_args(args).query_placement == "fargate"


def test_the_placement_changes_the_run_hash() -> None:
    hosts = awsb.plan_hosts(task_cfg())
    args = (hosts, fake_images(), b"genesis")
    assert awsb.run_config_hash(task_cfg(), *args) != awsb.run_config_hash(
        task_cfg(query_placement="ec2"), *args
    )


def test_plan_prices_the_tasks_and_the_file_system(
    isolated: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    out = isolated_env(monkeypatch, isolated, "planned")
    pre = {**fake_preflight(), "rds_engine_version": "18.2", "fargate": FARGATE_SHAPE}
    monkeypatch.setattr(awsb, "preflight", lambda *_: pre)
    argv = ["plan", "--tag", "x", "--nodes", "3", "--max-usd", "50", *FARGATE_FLAGS]
    runner = FakeRunner(states=[])
    assert awsb.cmd_plan(awsb.parse_args(argv), FakeSystem(run=runner)) == awsb.EXIT_OK
    manifest = netbench.read_json(out / "planned/fleet.json")
    items = {line["item"] for line in manifest["estimate"]["lines"]}
    assert {
        "fargate vcpu",
        "fargate memory",
        "efs storage",
        "efs elastic writes",
    } <= items
    assert "instance c8g.4xlarge" in items
    query = next(h for h in manifest["hosts"] if h["name"] == "node0")
    assert query["fargate"] == FARGATE_SHAPE
    tfvars = netbench.read_json(out / "planned/terraform/terraform.tfvars.json")
    assert tfvars["fargate"] == {"nodes": ["node0"]}
    assert "node0" not in tfvars["hosts"]
    assert not (out / "planned/hosts/node0/user-data.sh").exists()
    assert (
        "nfs-common postgresql-client"
        in (out / "planned/hosts/ctl/user-data.sh").read_text()
    )
    assert not runner.ran("tofu", "apply")


FARGATE_FLAGS = ["--query-placement", "fargate", "--db-modes", "rds"]


def test_the_confirmation_lists_the_fargate_resources_and_counts_instances() -> None:
    system = FakeSystem(answer=True)
    awsb.confirm_create(system.ask, task_cfg(query_nodes=2), Path("."), 3)
    (prompt,) = system.prompts
    assert "Create 3 instances" in prompt
    assert "1 ecs cluster, 1 efs file system, 2 access points" in prompt


def test_the_estimate_prices_each_task_at_its_architecture() -> None:
    hosts = awsb.place_fargate(awsb.plan_hosts(task_cfg()), {"fargate": FARGATE_SHAPE})
    arm = awsb.fargate_cost_lines(hosts, task_cfg(), 3600)
    x86 = {**FARGATE_SHAPE, "arch": "amd64"}
    other = awsb.place_fargate(awsb.plan_hosts(task_cfg()), {"fargate": x86})
    amd = awsb.fargate_cost_lines(other, task_cfg(), 3600)
    assert arm[0]["qty"] == amd[0]["qty"] == 16
    assert arm[0]["usd"] < amd[0]["usd"]
    assert arm[1]["qty"] == 32
    assert (
        awsb.fargate_cost_lines(
            awsb.plan_hosts(awsb.RunConfig(tag="x")), task_cfg(), 3600
        )
        == []
    )


def test_the_efs_writes_are_not_an_hourly_cost() -> None:
    hosts = awsb.place_fargate(awsb.plan_hosts(task_cfg()), {"fargate": FARGATE_SHAPE})
    lines = awsb.fargate_cost_lines(hosts, task_cfg(), 3600)
    estimate = {
        "expected_s": 3600.0,
        "lines": lines,
    }
    hourly = sum(line["usd"] for line in lines if line["item"] != "efs elastic writes")
    assert awsb.estimate_rate(estimate) == pytest.approx(hourly)


def wait_for_go(
    tmp_path: Path, content: str, deadline: int
) -> subprocess.CompletedProcess[str]:
    go = tmp_path / "go"
    go.write_text(content)
    script = f'set -euo pipefail\ngo={go}\n{awsb._WAIT_FOR_GO}echo "at=$at"'
    env = {**os.environ, "BENCH_DEADLINE": str(deadline)}
    return subprocess.run(
        ["bash", "-c", script], env=env, capture_output=True, text=True, check=False
    )


def test_the_wrapper_reads_the_start_time_from_go(tmp_path) -> None:
    done = wait_for_go(tmp_path, "1015500000000\n", 2**40)
    assert (done.returncode, done.stdout) == (0, "at=1015500000000\n")


def test_the_wrapper_gives_up_on_an_empty_go_at_the_deadline(tmp_path) -> None:
    done = wait_for_go(tmp_path, "", 1)
    assert done.returncode == 1
    assert "no go before the deadline" in done.stderr


def test_the_wrapper_does_not_take_a_go_without_its_newline(tmp_path) -> None:
    assert wait_for_go(tmp_path, "10155", 1).returncode == 1


def test_ctl_mounts_for_fresh_lookups_and_creates_the_go_the_task_polls(
    isolated: Path,
):
    runner = fargate_runner()
    awsb.mount_efs(remote(runner, isolated), fargate_output(["node0"]), ["node0"])
    script = runner.calls[-1][-1]
    assert "lookupcache=none" in script
    assert "touch /efs/node0/bench/go" in script


def test_nothing_is_stopped_for_a_run_that_started_no_task(isolated: Path):
    runner = fargate_runner()
    tasks = awsb.FargateTasks(runner, awsb.REGION, fargate_output(["node0"]), {})
    tasks.stop("freeze")
    assert runner.calls == []


def test_a_fleet_that_failed_before_its_outputs_has_no_cluster_to_stop(
    fleet_harness: FargateFleetHarness,
):
    runner = FakeRunner()
    cfg = task_cfg()
    fleet_dir = fleet_harness.fleet_dir
    fleet_dir.mkdir(parents=True)
    netbench.write_json(fleet_dir / "fleet.json", {"phase": "applying", "hosts": []})
    state = awsb.FleetState(
        FakeSystem(run=runner),
        cfg,
        fleet_dir,
        awsb.Terraform(runner, fleet_dir, {}),
        awsb.Interrupts(FakeClock(), no_children),
    )
    awsb.stop_fargate(state)
    assert runner.calls == []


def latency_runner(**kwargs: Any) -> FargateRunner:
    """The probe of a 3 node decaf-2025 fleet pings node0, whose round trip the folded matrix
    puts on the egress of the pinging node."""
    shaping = awsb.latency_shaping(task_cfg(latency="decaf-2025"), 3)
    (probe,) = awsb.latency.probe_pairs(
        shaping.assignment, shaping.delays, shaping.intra
    )
    return fargate_runner(pings={"10.0.0.2": probe.expected_ms}, **kwargs)


@pytest.mark.usefixtures("valid")
def test_latency_shapes_the_validators_toward_the_task_and_never_the_task(
    harness: FargateHarness,
):
    runner = latency_runner()
    assert harness.run(runner, "--latency", "decaf-2025") == awsb.EXIT_OK
    shaped = [
        c
        for c in (" ".join(call) for call in runner.calls if call[0] == "ssh")
        if "tc qdisc add" in c
    ]
    # TEST:fargate-shape-ec2-only-ok
    assert len(shaped) == 2
    assert all("match ip dst 10.0.0.2/32" in c for c in shaped)
    manifest = netbench.read_json(harness.run_dir / "manifest.json")
    assert manifest["latency"]["unshaped"] == [0]
    assert all(probe["ok"] for probe in manifest["latency"]["probes"])
    ping = next(call for call in runner.calls if "ping -c 5" in call[-1])
    assert "ping -c 5 -i 0.2 -n -q 10.0.0.2" in ping[-1]
    sysctls = runner.task_defs["espresso-bench-run1-node0"]["containerDefinitions"][0][
        "systemControls"
    ]
    assert {"namespace": "net.ipv4.tcp_congestion_control", "value": "bbr"} in sysctls


def test_a_sysctl_aws_refuses_fails_a_shaped_run_at_registration(
    harness: FargateHarness,
):
    refused = "ClientException: sysctl net.ipv4.tcp_rmem is not allowed on fargate"
    runner = latency_runner(register_error=refused)
    assert harness.run(runner, "--latency", "decaf-2025") == awsb.EXIT_FAILED
    assert refused in (harness.run_dir / "summary.md").read_text()
    assert not runner.ran("tc qdisc add")
    assert not runner.ran("run-task")


def test_a_wrapper_that_finds_another_congestion_control_fails_a_shaped_run(
    harness: FargateHarness,
):
    stopped = fargate_task(
        "STOPPED", stopped_reason="Essential container exited", exit_code=78
    )
    tail = "sysctl net.ipv4.tcp_congestion_control=cubic, expected bbr"
    runner = latency_runner(task_states=[{"node0": stopped}], log_lines=[tail])
    assert harness.run(runner, "--latency", "decaf-2025") == awsb.EXIT_FAILED
    summary = (harness.run_dir / "summary.md").read_text()
    assert "exit 78" in summary
    assert tail in summary
    assert not runner.ran("tc qdisc add")


def test_the_analysis_host_is_node0_unless_node0_is_a_task():
    cfg = task_cfg(nodes=3)
    ec2 = {"hosts": awsb.plan_hosts(cfg)}
    placed = awsb.place_fargate(awsb.plan_hosts(cfg), {"fargate": FARGATE_SHAPE})
    assert awsb.analysis_host(ec2) == "node0"
    assert awsb.analysis_host({"hosts": placed}) == "node1"
    every = awsb.place_fargate(
        awsb.plan_hosts(task_cfg(nodes=3, query_nodes=3)), {"fargate": FARGATE_SHAPE}
    )
    assert awsb.analysis_host({"hosts": every}) == "ctl"


def fargate_run_dir(run_dir: Path) -> dict:
    """A collected 3-node rds run whose node0 is a task: the files of an EC2 node0 are gone,
    and the mailbox files and `task.json` are there instead."""
    manifest = write_collected_run(run_dir)
    cfg = task_cfg(load=netbench.BenchConfig(submit_nodes=2))
    manifest |= {
        "config": awsb.config_to_json(cfg),
        "hosts": awsb.place_fargate(awsb.plan_hosts(cfg), {"fargate": FARGATE_SHAPE}),
        "query_db": "rds",
        "rds_spec": rds_spec(),
    }
    node0, node1 = run_dir / "hosts/node0", run_dir / "hosts/node1"
    for name in ("lscpu.txt", "meminfo.txt", "uname.txt"):
        (node0 / name).rename(node1 / name)
    for name in ("host.jsonl", "chrony.txt", "ready.json"):
        (node0 / name).unlink()
    (run_dir / awsb.EC2_NODE0_FILE).unlink()
    digest = manifest["images"]["espresso-node"]["digest"]
    task = fargate_task()
    task["containers"][0]["imageDigest"] = digest
    netbench.write_json(node0 / "task.json", task)
    series = {"timestamps": [60.0, 120.0, 180.0], "values": [1.0, 1.0, 1.0]}
    metrics = {metric: series for metric, _ in awsb.RDS_METRICS.values()}
    for balance in ("EBSIOBalance%", "EBSByteBalance%"):
        metrics[balance] = {**series, "values": [100.0] * 3}
    (run_dir / awsb.RDS_CLOUDWATCH_FILE).write_text(
        json.dumps({"period_s": 60, "metrics": metrics})
    )
    netbench.write_json(run_dir / "manifest.json", manifest)
    return manifest


# TEST:fargate-validity-no-coverage-ok
def test_a_task_is_judged_without_host_samples_chrony_or_ebs_balance(tmp_path):
    fargate_run_dir(tmp_path)
    result = awsb.write_report(tmp_path)
    assert result["validity"]["valid"], result["validity"]["reasons"]
    assert not result["validity"]["noisy"], result["validity"]["reasons"]
    hosts = result["deployment"]["hosts"]
    assert hosts["node0"] == {
        "instance_type": "fargate 16 vCPU 32 GiB",
        "role": "query",
        "placement": "fargate",
        "vcpus": 16,
        "memory_mib": 32768,
    }
    assert hosts["node1"]["instance_type"] == "c8g.4xlarge"
    assert "node0 fargate 16 vCPU 32 GiB" in (tmp_path / "summary.md").read_text()


def test_the_analysis_comes_from_the_first_ec2_node(tmp_path):
    fargate_run_dir(tmp_path)
    (tmp_path / "hosts/node1/meminfo.txt").write_text("MemTotal:       16000000 kB\n")
    awsb.write_report(tmp_path)
    runner = netbench.read_json(tmp_path / "sysinfo.json")["runner"]
    assert runner["mem_total_bytes"] == 16000000 * 1024


# TEST:fargate-digest-mismatch-fails
def test_a_task_that_ran_another_image_is_invalid(tmp_path):
    fargate_run_dir(tmp_path)
    task = netbench.read_json(tmp_path / "hosts/node0/task.json")
    task["containers"][0]["imageDigest"] = f"sha256:{'9' * 64}"
    netbench.write_json(tmp_path / "hosts/node0/task.json", task)
    validity = awsb.write_report(tmp_path)["validity"]
    assert not validity["valid"]
    assert (
        "node0 pulled another digest than the manifest for espresso-node"
        in (validity["reasons"])
    )


def test_the_postgres_files_of_ctl_are_judged_for_the_task(tmp_path):
    fargate_run_dir(tmp_path)
    ctl = tmp_path / "hosts/ctl"
    netbench.write_json(
        ctl / "pg-settings-node0.json", pg_settings(shared_buffers="1GB")
    )
    sample = {"ts": 100.0, "ssl_backends": {"false": 2}}
    netbench.write_jsonl(ctl / "pg-stats-node0.jsonl", iter([sample]))
    reasons = awsb.write_report(tmp_path)["validity"]["reasons"]
    assert any("postgres shared_buffers=1GB" in r for r in reasons)
    assert "node0 postgres backends connected without TLS" in reasons


def test_the_task_journal_limit_is_not_halved_for_postgres():
    hosts = awsb.plan_hosts(task_cfg())
    ec2 = next(h for h in hosts if h["name"] == "node0")
    task = {**ec2, "fargate": FARGATE_SHAPE}
    assert awsb._journal_max_bytes(task) == 2 * awsb._journal_max_bytes(ec2)
