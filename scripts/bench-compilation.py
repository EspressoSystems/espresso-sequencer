#!/usr/bin/env python
import argparse
import os
import subprocess
import sys
import time
import tomlkit
import typing
from memory_profiler import memory_usage

RESULTS = "bench-compilation-results.toml"


def store_result(
    *,
    exec_duration: float,
    max_memory_mb: float,
    strip: str,
    debug: str,
    lto: str,
    split_debuginfo: str,
    commands: dict[str, str],
):
    """A decorator to cache function results to a file"""
    try:
        with open(RESULTS) as f:
            data = tomlkit.load(f)
    except (FileNotFoundError,):
        print(f"No cache at {RESULTS}, making a new one.")
        data = {}
    result: dict[str, typing.Any] = commands.copy()
    result["lto"] = lto
    result["strip"] = strip
    result["debug"] = debug
    result["time"] = round(exec_duration, 1)
    result["memory_usage_mb"] = int(round(max_memory_mb, 0))
    profile = mk_profile(strip, debug, lto, split_debuginfo)
    data[profile] = result
    with open(RESULTS, "w") as f:
        tomlkit.dump(data, f)


def result_exists(profile):
    try:
        with open(RESULTS) as f:
            data = tomlkit.load(f)
    except (FileNotFoundError,):
        return False
    return profile in data


def mk_profile(strip: str, debug: str, lto: str, split_debuginfo: str):
    return f"debug-{debug}_strip-{strip}_lto-{lto}_split-debuginfo_{split_debuginfo}"


def run(log_file: typing.IO, dry_run: bool, command: str):
    if dry_run:
        return
    proc = subprocess.run(
        command,
        shell=True,
        stdout=log_file,
        stderr=subprocess.STDOUT,
    )
    if proc.returncode != 0:
        print(f"Command failed: {command} with return code {proc.returncode}")
        sys.exit(1)


def measure(log_file: typing.IO, dry_run: bool, command: str) -> tuple[float, float]:
    if dry_run:
        return 0.0, 0.0
    tic = time.time()
    proc = subprocess.Popen(
        command,
        shell=True,
        stdout=log_file,
        stderr=subprocess.STDOUT,
    )
    mems = []
    while True:
        # check if process terminated
        return_code = proc.poll()
        if return_code is not None:
            if return_code != 0:
                print(f"Command failed: {command} with return code {return_code}")
                sys.exit(1)
            toc = time.time()
            break

        # measure memory usage
        mems.append(
            memory_usage(
                proc=proc,
                interval=0.1,
                timeout=None,
                include_children=True,
                max_usage=True,
            )
        )
    return toc - tic, max(mems)


def benchmark(
    dry_run: bool = False,
    *,
    strip: str,
    debug: str,
    lto: str,
    split_debuginfo: str,
    setup_command: str,
    pre_build_command: str,
    cache_invalidation_command: str,
    timed_command: str,
):
    profile = mk_profile(strip, debug, lto, split_debuginfo)
    if result_exists(profile):
        print(f"Skipping {profile}, already benchmarked.")
        return

    print(f"Testing profile {profile}")

    with open("bench-compilation.log", "a") as f:
        print(f"Running setup command: {setup_command}")
        run(f, dry_run, setup_command)

        if pre_build_command:
            print(f"Running pre-build command: {pre_build_command}")
            run(f, dry_run, pre_build_command)

        if cache_invalidation_command:
            print(f"Running cache invalidation command: {cache_invalidation_command}")
            run(f, dry_run, cache_invalidation_command)

        print(f"Running timed command: {timed_command}")
        exec_duration, max_memory_mb = measure(f, dry_run, timed_command)
        print(
            f"Timed command at {exec_duration:.1f} seconds, used {max_memory_mb:.0f} MB"
        )
        commands = {
            "setup": setup_command,
            "pre_build": pre_build_command,
            "cache_invalidation": cache_invalidation_command,
            "timed": timed_command,
        }
        store_result(
            exec_duration=exec_duration,
            max_memory_mb=max_memory_mb,
            strip=strip,
            debug=debug,
            lto=lto,
            split_debuginfo=split_debuginfo,
            commands=commands,
        )


def main():
    parser = argparse.ArgumentParser(
        description=f"""Cargo compilation benchmark

The point of this script is to simulate the rust development workflow developers
experience when working on this project. The project is large and compilation is
not quick, therefore it's important cargo is configured wisely to balance
compilation time and debugging experience.

This script benchmarks the compilation time of a Rust project using
different profiles. It runs

1. The setup command (default: "cargo clean")
2. A pre-build command (default: "cargo build")
3. A cache invalidation command (defaults to touching all lib.rs files in the repo)
4. A timed command (default: "cargo build")

Results are store in TOML file in {RESULTS}.

The script generates new cargo profile for all combinations of the "strip",
"debug" and "lto" values and runs the timed command using these profiles.
""",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--timed", help="Timed command to run", default="cargo build")
    parser.add_argument(
        "--setup",
        help="Setup command to run",
        default="cargo clean",
    )
    parser.add_argument(
        "--pre-build",
        help="Pre-build cargo command to run",
        default="cargo build",
    )
    parser.add_argument(
        "--invalidate-cache-command",
        help="Command to run to invalidate the cargo cache",
        default="find . -path '*/src/lib.rs' -exec touch {} \\;",
    )
    parser.add_argument(
        "--strip", help="Strip values", default="none,debuginfo,symbols"
    )
    parser.add_argument(
        "--debug", help="Debug values", default="none,line-tables-only,limited,full"
    )
    parser.add_argument(
        "--profile-arg",
        help="Profile argument to pass to commands, use 'cargo-profile' for nextest run",
        default="profile",
    )
    parser.add_argument("--lto", help="LTO values", default="false,off")
    parser.add_argument("--dry-run", help="Only print commands", action="store_true")
    parser.add_argument(
        "--split-debuginfo", help="split-debuginfo values", type=str, default="off"
    )
    args = parser.parse_args()

    # Show plan
    print("Running with the following configuration:\n")
    print(tomlkit.dumps(vars(args)))
    print(f"CARGO_BUILD_JOBS={os.environ.get('CARGO_BUILD_JOBS')}")
    print()

    with open("Cargo.toml") as f:
        cargo_toml = tomlkit.load(f)
    profile_section = cargo_toml["profile"]
    profiles = []
    for strip in args.strip.split(","):
        for debug in args.debug.split(","):
            for lto in args.lto.split(","):
                for split_debuginfo in args.split_debuginfo.split(","):
                    profile = mk_profile(strip, debug, lto, split_debuginfo)
                    profiles.append(profile)
                    profile_section[profile] = {
                        "inherits": "dev",
                        "debug": debug,
                        "strip": strip,
                        # lto = false needs to be a boolean in Cargo.toml
                        "lto": False if lto.lower() == "false" else lto,
                        "split-debuginfo": split_debuginfo,
                    }

    # Write the new profiles to Cargo.toml, tomlkit preserves the existing style.
    with open("Cargo.toml", "w") as f:
        tomlkit.dump(cargo_toml, f)

    # Run the benchmarks
    for strip in args.strip.split(","):
        for debug in args.debug.split(","):
            for lto in args.lto.split(","):
                for split_debuginfo in args.split_debuginfo.split(","):
                    profile = mk_profile(strip, debug, lto, split_debuginfo)
                    command = f"{args.timed} --{args.profile_arg} {profile}"
                    pre_build_command = (
                        (f"{args.pre_build} --{args.profile_arg} {profile}")
                        if args.pre_build
                        else ""
                    )
                    benchmark(
                        args.dry_run,
                        strip=strip,
                        debug=debug,
                        lto=lto,
                        split_debuginfo=split_debuginfo,
                        setup_command=args.setup,
                        pre_build_command=pre_build_command,
                        cache_invalidation_command=args.invalidate_cache_command,
                        timed_command=command,
                    )


if __name__ == "__main__":
    main()
