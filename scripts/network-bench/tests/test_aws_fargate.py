from fakes import awsb, fleet


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
