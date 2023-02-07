use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use contract_bindings::TestHermezContracts;
use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};

/// This script works but loading the state back into anvil currently does not
/// set the block number correctly and the zkevm-node cannot handle this.
#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let name = "anvil-deploy";

    // Stop the container, in case it's running.
    Command::new("docker")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(["stop", name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Remove the container, in case it exists.
    Command::new("docker")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .args(["rm", name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // TODO: Figure out why this breaks carriage returns.
    Command::new("docker")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("run")
        .arg("--rm") // remove container after it exits
        .args(["--name", name])
        .args(["-p", "8545:8545"])
        .args([
            "-v",
            &format!(
                "{}:/state",
                (Path::new(env!("CARGO_MANIFEST_DIR"))
                    .parent()
                    .unwrap()
                    .join("anvil-state")
                    .to_str()
                    .unwrap())
            ),
        ])
        .arg("-it")
        .arg("ghcr.io/foundry-rs/foundry:latest")
        .arg("anvil --host 0.0.0.0 --dump-state /state/state.json")
        .spawn()
        .unwrap();

    std::thread::sleep(Duration::from_secs(1));

    let system =
        TestHermezContracts::deploy("http://localhost:8545", "http://localhost:8126").await;

    Command::new("docker")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("kill")
        .args(["--signal", "INT"])
        .arg("anvil-deploy")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // Create an .env.anvil file with contract addresses for anvil.
    let file = File::open(".env").unwrap();
    let lines: Vec<_> = io::BufReader::new(file).lines().collect();
    let mut new_lines: Vec<String> = vec![];
    for line in lines {
        let line = line.unwrap();
        let new_line = if line.starts_with("ESPRESSO_ZKEVM_ROLLUP_ADDRESS=") {
            format!(
                "ESPRESSO_ZKEVM_ROLLUP_ADDRESS={:?}",
                system.rollup.address()
            )
        } else if line.starts_with("ESPRESSO_ZKEVM_MATIC_ADDRESS=") {
            format!("ESPRESSO_ZKEVM_MATIC_ADDRESS={:?}", system.matic.address())
        } else if line.starts_with("ESPRESSO_ZKEVM_GER_ADDRESS=") {
            format!(
                "ESPRESSO_ZKEVM_GER_ADDRESS={:?}",
                system.global_exit_root.address()
            )
        } else {
            line
        };
        new_lines.push(new_line);
    }

    let mut file = File::create(".env.anvil").unwrap();
    writeln!(
        file,
        "# This file is updated via `cargo run --bin anvil-deploy`, don't edit by hand."
    )
    .unwrap();
    writeln!(file).unwrap();
    for line in new_lines {
        writeln!(file, "{line}").unwrap();
    }
}
