#!/usr/bin/env python
import argparse
import subprocess
import time
import tomlkit

RESULTS = "bench-compilation-results.toml"


def cached(func):
    """A decorator to cache function results to a file"""

    def wrapper(key, *args, **kwargs):
        try:
            with open(RESULTS) as f:
                data = tomlkit.load(f)
        except (FileNotFoundError,):
            print(f"No cache at {RESULTS}, making a new one.")
            data = {}
        if key in data:
            return data[key]
        ret = func(key, *args, **kwargs)
        data[key] = ret
        with open(RESULTS, "w") as f:
            tomlkit.dump(data, f)
        return ret

    return wrapper


@cached
def benchmark(key: str, setup_command: str, timed_command: str):
    print(f"Running setup command: {setup_command}")
    with open("bench-compilation.log", "a") as f:
        subprocess.run(
            setup_command,
            shell=True,
            stdout=f,
            stderr=subprocess.STDOUT,
        )
        print(f"Running timed command: {timed_command}")
        tic = time.time()
        subprocess.run(
            timed_command,
            shell=True,
            stdout=f,
            stderr=subprocess.STDOUT,
        )
        toc = time.time()
        return toc - tic


def main():
    parser = argparse.ArgumentParser(
        description=f"""Cargo compilation benchmark

This script benchmarks the compilation time of a Rust project using
different profiles. It runs the setup command (default: "cargo clean")
before each timed command (default: "cargo build").

Results are cached into TOML file in {RESULTS} where keys are profiles and values
are the seconds it took to run the timed command.

The script generates new cargo profile for all combinations of the "strip" and
"debug" values and runs the timed command using these profiles.""",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument("--timed", help="Timed command to run", default="cargo build")
    parser.add_argument("--setup", help="Setup command to run", default="cargo clean")
    parser.add_argument(
        "--strip", help="Strip values", default="none,debuginfo,symbols"
    )
    parser.add_argument(
        "--debug", help="Debug values", default="none,line-tables-only,limited,full"
    )
    args = parser.parse_args()

    with open("Cargo.toml") as f:
        cargo_toml = tomlkit.load(f)
    profile_section = cargo_toml["profile"]
    profiles = []
    for strip in args.strip.split(","):
        for debug in args.debug.split(","):
            profile = f"debug-{debug}-strip-{strip}"
            profiles.append(profile)
            profile_section[profile] = {
                "inherits": "test",
                "debug": debug,
                "strip": strip,
            }

    # Write the new profiles to Cargo.toml, tomlkit preserves the existing style.
    with open("Cargo.toml", "w") as f:
        tomlkit.dump(cargo_toml, f)

    # Run the benchmarks
    for profile in profiles:
        command = f"{args.timed} --profile {profile}"
        print(f"Testing profile {profile}")
        time = benchmark(profile, args.setup, command)
        print(f"Time: {time:.2f}s")


if __name__ == "__main__":
    main()
