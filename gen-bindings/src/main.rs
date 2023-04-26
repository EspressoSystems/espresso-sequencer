use ethers::prelude::{Abigen, MultiAbigen};
use std::collections::HashSet;
use std::env;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

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
    // Hardhat's debug files trip up MultiAbigen otherwise we could use
    // MultiAbigen::from_json_files instead
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

    let mut abigens = MultiAbigen::from_abigens(
        artifacts
            .iter()
            .map(|path| Abigen::from_file(path).unwrap()),
    );

    // 2. Generate bindings for other contracts (including Matic)
    //    in gen-bindings/contracts/abi.
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for abigen in MultiAbigen::from_json_files(manifest_dir.join("contracts/abi"))
        .expect("Failed to read contracts")
        .iter()
    {
        abigens.push(abigen.clone());
    }

    // 3. Generate bindings for HotShot specific contracts
    let hotshot_contracts_path = workspace_dir.join("contracts");

    // Compile HotShot specific contracts
    Command::new("forge")
        .arg("build")
        .arg("--force") // Forge sometimes doesn't recompile when it should.
        .current_dir(&hotshot_contracts_path)
        .output()
        .expect("failed to execute process");

    // Exclude foundry contracts from the bindings
    let exclude: HashSet<String> = HashSet::from_iter(
        vec![
            "Base.sol",
            "console2.sol",
            "console.sol",
            "Counter.s.sol",
            "Counter.t.sol",
            "IMulticall3.sol",
            "Script.sol",
            "StdAssertions.sol",
            "StdChains.sol",
            "StdCheats.sol",
            "StdError.sol",
            "StdInvariant.sol",
            "StdJson.sol",
            "StdMath.sol",
            "StdStorage.sol",
            "StdUtils.sol",
            "test.sol",
            "Test.sol",
            "Vm.sol",
        ]
        .iter()
        .map(|s| s.to_string()),
    );

    let artifacts: Vec<_> = find_paths(
        hotshot_contracts_path.join("out").to_str().unwrap(),
        ".json",
    )
    .into_iter()
    .filter(|path| {
        !exclude.contains(
            &path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        )
    })
    .collect();

    for abigen in MultiAbigen::from_abigens(
        artifacts
            .iter()
            .map(|path| Abigen::from_file(path).unwrap()),
    )
    .iter()
    {
        abigens.push(abigen.clone());
    }

    // Remove existing bindings
    let bindings_dir = workspace_dir.join("contract-bindings/src/bindings");
    if bindings_dir.exists() {
        std::fs::remove_dir_all(&bindings_dir).unwrap();
    }

    // Generate bindings
    let bindings = abigens.build().unwrap();
    bindings.write_to_module(&bindings_dir, false).unwrap();

    // Unfortunately the bindings are not always correctly formatted.
    Command::new("rustfmt")
        .arg(bindings_dir.join("mod.rs").to_str().unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!("zkevm-contract bindings written to {bindings_dir:?}");

    Ok(())
}
