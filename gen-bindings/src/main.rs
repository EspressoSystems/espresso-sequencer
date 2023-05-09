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

    // Generate bindings for HotShot specific contracts
    let hotshot_contracts_path = workspace_dir.join("contracts");

    // Compile HotShot specific contracts
    let status = Command::new("forge")
        .arg("build")
        .arg("--force") // Forge sometimes doesn't recompile when it should.
        .current_dir(&hotshot_contracts_path)
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait for process");

    if !status.success() {
        panic!(
            "Error: `forge build` exited with non-zero exit code: {}",
            status
        );
    }

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

    let abigens = MultiAbigen::from_abigens(
        artifacts
            .iter()
            .map(|path| Abigen::from_file(path).unwrap()),
    );

    // Generate bindings
    let bindings = abigens.build().unwrap();

    // Remove existing bindings
    let bindings_dir = workspace_dir.join("contract-bindings/src/bindings");
    if bindings_dir.exists() {
        std::fs::remove_dir_all(&bindings_dir).unwrap();
    }

    bindings.write_to_module(&bindings_dir, false).unwrap();

    // Unfortunately the bindings are not always correctly formatted.
    Command::new("rustfmt")
        .arg(bindings_dir.join("mod.rs").to_str().unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!("Contract bindings written to {bindings_dir:?}");

    Ok(())
}
