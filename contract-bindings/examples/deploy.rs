use std::{path::Path, process::Command, time::Duration};

use contract_bindings::TestHermezContracts;

#[async_std::main]
async fn main() {
    let name = "anvil-deploy";
    let child = Command::new("docker")
        .arg("run")
        .arg("--name")
        .arg(name)
        .arg("-p")
        .arg("8545:8545")
        .arg("-v")
        .arg(format!(
            "{}:/state",
            (Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("state")
                .to_str()
                .unwrap())
        ))
        .arg("-it")
        .arg("ghcr.io/foundry-rs/foundry:latest")
        .arg("anvil --host 0.0.0.0 --dump-state /state/state.json")
        .spawn()
        .unwrap();

    std::thread::sleep(Duration::from_secs(1));

    let _system =
        TestHermezContracts::deploy("http://localhost:8545", "http://localhost:8126").await;

    // `docker stop` hangs for 10 seconds, then doesn't generate the state.json
    // file, suspecting anvil doesn't react to whatever signal docker sends.
    Command::new("kill")
        .args(["-s", "INT", &child.id().to_string()])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Command::new("docker")
        .args(["rm", name])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
