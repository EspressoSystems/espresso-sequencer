use ethers::prelude::MultiAbigen;

/// Read the contract ABI and generate rust bindings.
fn main() -> Result<(), ()> {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    MultiAbigen::from_json_files(manifest_dir.join("contracts/abi"))
        .expect("Failed to read contracts")
        .build()
        .expect("Failed to build contracts")
        .write_to_module(
            manifest_dir
                .parent() // NOTE: workaround to find directory of crate with ABIs
                .unwrap()
                .join("contract-bindings/src/bindings"),
            false,
        )
        .expect("Failed to write bindings");
    Ok(())
}
