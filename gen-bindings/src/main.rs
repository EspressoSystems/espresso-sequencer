use ethers::prelude::{Abigen, MultiAbigen};
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

fn find_paths(dir: &str, ext: &str) -> Vec<PathBuf> {
    glob::glob(&format!("{dir}/**/*{ext}"))
        .unwrap()
        .map(|entry| entry.unwrap())
        .collect()
}

/// Read the contract ABI and generate rust bindings.
fn main() -> Result<(), ()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap();

    // 1. Generate bindings for all zkevm-contracts
    //
    // Hardhat's debug files trip up MultiAbigen
    // otherwise we could use MultiAbigen::from_json_files instead
    let artifacts: Vec<_> = find_paths(
        workspace_dir
            .join("zkevm-contracts/artifacts/contracts")
            .to_str()
            .unwrap(),
        ".json",
    )
    .into_iter()
    .filter(|path| !path.to_str().unwrap().ends_with(".dbg.json"))
    .collect();

    let abigens: Vec<_> = artifacts
        .iter()
        .map(|path| Abigen::from_file(path).unwrap())
        .collect();

    let mut gen = MultiAbigen::from_abigens(abigens);

    // 2. Generate bindings for other contracts (including Matic)
    // TODO Is this redundant with step 3 below?

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for abigen in MultiAbigen::from_json_files(manifest_dir.join("contracts/abi"))
        .expect("Failed to read contracts")
        .iter()
    {
        gen.push(abigen.clone());
    }

    let bindings = gen.build().unwrap();
    let bindings_dir = workspace_dir.join("contract-bindings/src/bindings");
    bindings.write_to_module(&bindings_dir, false).unwrap();

    println!("zkevm-contract bindings written to {bindings_dir:?}");

    // 3. Generate bindings for HotShot specific contracts
    let hotshot_contracts_path = manifest_dir.join("../contracts");
    assert!(env::set_current_dir(hotshot_contracts_path).is_ok());

    // Ensure we start with a clean state
    Command::new("forge")
        .args(["clean"])
        .output()
        .expect("failed to execute process");

    // Compute the bindings
    Command::new("forge")
        .args(["bind"])
        .output()
        .expect("failed to execute process");

    // Move the bindings to the right directory
    Command::new("mv")
        .args([
            "out/bindings/src/counter.rs",
            bindings_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to execute process");

    // Update the mod.rs file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(bindings_dir.join("mod.rs"))
        .unwrap();

    if let Err(e) = writeln!(file, "pub mod counter; ") {
        format!("Couldn't write to file: {e}");
    }

    Ok(())
}
