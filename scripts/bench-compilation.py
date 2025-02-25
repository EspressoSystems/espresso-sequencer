#!/usr/bin/env python
import argparse
import os
import subprocess
import sys
import time
import tomlkit
import typing

RESULTS = "bench-compilation-results.toml"


def store_result(
    run_time, *, strip: str, debug: str, lto: str, commands: dict[str, str]
):
    """A decorator to cache function results to a file"""
    try:
        with open(RESULTS) as f:
            data = tomlkit.load(f)
    except (FileNotFoundError,):
        print(f"No cache at {RESULTS}, making a new one.")
        data = {}
    result = commands.copy()
    result["lto"] = lto
    result["strip"] = strip
    result["debug"] = debug
    result["time"] = f"{run_time:.2f}"
    profile = mk_profile(strip, debug, lto)
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


def mk_profile(strip: str, debug: str, lto: str):
    return f"debug-{debug}_strip-{strip}_lto-{lto}"


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


def benchmark(
    dry_run: bool = False,
    *,
    strip: str,
    debug: str,
    lto: str,
    setup_command: str,
    pre_build_command: str,
    cache_invalidation_command: str,
    timed_command: str,
):
    profile = mk_profile(strip, debug, lto)
    if result_exists(profile):
        print(f"Skipping {profile}, already benchmarked.")
        return

    print(f"Testing profile {profile}")

    with open("bench-compilation.log", "a") as f:
        print(f"Running setup command: {setup_command}")
        run(f, dry_run, setup_command)

        print(f"Running pre-build command: {pre_build_command}")
        run(f, dry_run, pre_build_command)

        print(f"Running cache invalidation command: {cache_invalidation_command}")
        run(f, dry_run, cache_invalidation_command)

        print(f"Running timed command: {timed_command}")
        tic = time.time()
        run(f, dry_run, timed_command)
        toc = time.time()
        run_time = toc - tic
        print(f"Timed command at {run_time:.2f} seconds")
        commands = {
            "setup": setup_command,
            "pre_build": pre_build_command,
            "cache_invalidation": cache_invalidation_command,
            "timed": timed_command,
        }
        store_result(run_time, strip=strip, debug=debug, lto=lto, commands=commands)


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
    parser.add_argument("--lto", help="LTO values", default="off,thin")
    parser.add_argument("--dry-run", help="Only print commands", action="store_true")
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
                profile = mk_profile(strip, debug, lto)
                profiles.append(profile)
                profile_section[profile] = {
                    "inherits": "dev",
                    "debug": debug,
                    "strip": strip,
                    "lto": lto,
                }

    # Write the new profiles to Cargo.toml, tomlkit preserves the existing style.
    with open("Cargo.toml", "w") as f:
        tomlkit.dump(cargo_toml, f)

    # Run the benchmarks
    for strip in args.strip.split(","):
        for debug in args.debug.split(","):
            for lto in args.lto.split(","):
                profile = f"debug-{debug}_strip-{strip}_lto-{lto}"
                command = f"{args.timed} --profile {profile}"
                pre_build_command = f"{args.pre_build} --profile {profile}"
                time = benchmark(
                    args.dry_run,
                    strip=strip,
                    debug=debug,
                    lto=lto,
                    setup_command=args.setup,
                    pre_build_command=pre_build_command,
                    cache_invalidation_command=args.invalidate_cache_command,
                    timed_command=command,
                )


if __name__ == "__main__":
    main()
