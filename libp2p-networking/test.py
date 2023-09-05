#!/usr/bin/env python3

from enum import Enum
from functools import reduce
from typing import Final
from subprocess import run, Popen
from time import sleep
from os import environ

class NodeType(Enum):
    CONDUCTOR = "Conductor"
    REGULAR = "Regular"
    BOOTSTRAP = "Bootstrap"

def gen_invocation(
        node_type: NodeType,
        num_nodes: int,
        to_connect_addrs: list[str],
        conductor_addr: str,
        num_rounds: int,
        bound_addr: str,
    ) -> tuple[list[str], str]:
    aggr_list = lambda x, y: f'{x},{y}'
    to_connect_list : Final[str] = reduce(aggr_list, to_connect_addrs);
    out_file_name : Final[str] = f'out_{node_type}_{bound_addr[-4:]}';
    fmt_cmd = [
        f'cargo run --no-default-features --features=async-std-executor --example=counter --profile=release-lto -- ' \
        f' --bound_addr={bound_addr} '\
        f' --node_type={node_type.value} '\
        f' --num_nodes={num_nodes} '\
        f' --num_gossip={num_rounds} '\
        f' --to_connect_addrs={to_connect_list} '\
        f' --conductor_addr={conductor_addr} '];
    return (fmt_cmd, out_file_name)

# construct a map:

if __name__ == "__main__":
    # cleanup

    run("rm -f out_*".split())


    # params
    START_PORT : Final[int] = 9100;
    NUM_REGULAR_NODES : Final[int] = 100;
    NUM_NODES_PER_BOOTSTRAP : Final[int] = 10;
    NUM_BOOTSTRAP : Final[int] = (int) (NUM_REGULAR_NODES / NUM_NODES_PER_BOOTSTRAP);
    TOTAL_NUM_NODES: Final[int] = NUM_BOOTSTRAP + NUM_REGULAR_NODES + 1;
    NUM_ROUNDS = 100;

    bootstrap_addrs : Final[list[str]] = list(map(lambda x: f'127.0.0.1:{x + START_PORT}', range(0, NUM_BOOTSTRAP)));
    normal_nodes_addrs : Final[list[str]] = list(map(lambda x: f'127.0.0.1:{x + START_PORT + NUM_BOOTSTRAP}', range(0, NUM_REGULAR_NODES)));
    conductor_addr : str = f'127.0.0.1:{START_PORT + NUM_BOOTSTRAP + NUM_REGULAR_NODES + 1}';

    regular_cmds : list[tuple[list[str], str]] = [];
    bootstrap_cmds : list[tuple[list[str], str]] = [];
    print("doing conductor")
    conductor_cmd : Final[tuple[list[str], str]] = \
        gen_invocation(
            node_type=NodeType.CONDUCTOR,
            num_nodes=TOTAL_NUM_NODES,
            to_connect_addrs=bootstrap_addrs + normal_nodes_addrs,#  + normal_nodes_addrs + [conductor_addr],
            conductor_addr=conductor_addr,
            num_rounds=NUM_ROUNDS,
            bound_addr=conductor_addr
    );
    print("dfone concuctor")

    for i in range(0, len(bootstrap_addrs)):
        bootstrap_addr = bootstrap_addrs[i];
        regulars_list = normal_nodes_addrs[i * NUM_NODES_PER_BOOTSTRAP: (i + 1) * NUM_NODES_PER_BOOTSTRAP];

        bootstrap_cmd = gen_invocation(
            node_type=NodeType.BOOTSTRAP,
            num_nodes=TOTAL_NUM_NODES,
            to_connect_addrs=bootstrap_addrs,
            conductor_addr=conductor_addr,
            num_rounds=NUM_ROUNDS,
            bound_addr=bootstrap_addr,
        );
        bootstrap_cmds.append(bootstrap_cmd);

        for regular_addr in regulars_list:
            regular_cmd = gen_invocation(
                node_type=NodeType.REGULAR,
                num_nodes=TOTAL_NUM_NODES,
                # NOTE may need to remove regular_addr from regulars_list
                to_connect_addrs= [bootstrap_addr],
                num_rounds=NUM_ROUNDS,
                bound_addr=regular_addr,
                conductor_addr=conductor_addr
            );
            regular_cmds.append(regular_cmd);

    print(regular_cmds)

    TIME_TO_SPIN_UP_BOOTSTRAP : Final[int] = 0;
    TIME_TO_SPIN_UP_REGULAR : Final[int] = 0;
    env = environ.copy();
    env["RUST_BACKTRACE"] = "full"

    print("spinning up bootstrap")
    for (node_cmd, file_name) in bootstrap_cmds:
        print("running bootstrap", file_name)
        file = open(file_name, 'w')
        Popen(node_cmd[0].split(), start_new_session=True, stdout=file, stderr=file, env=env);

    sleep(TIME_TO_SPIN_UP_BOOTSTRAP);

    print("spinning up regulars")
    for (node_cmd, file_name) in regular_cmds:
        file = open(file_name, 'w')
        Popen(node_cmd[0].split(), start_new_session=True, stdout=file, stderr=file, env=env);

    sleep(TIME_TO_SPIN_UP_REGULAR);

    file = open(conductor_cmd[1], 'w')
    print("spinning up conductor")
    Popen(conductor_cmd[0][0].split(), start_new_session=True, stdout=file, stderr=file, env=env);

