import json
import subprocess

import pytest
from fakes import (
    ROLE_ARN,
    FakeClock,
    FakeRunner,
    FakeSystem,
    arn,
    aws_verbs,
    awsb,
    completed,
    fleet,
    mapping,
    rds_tag_runner,
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
    assert awsb.pg_host(fleet(3, fargate=True), "node0") == "ctl"
    assert awsb.pg_host(fleet(3), "node0") == "node0"


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
