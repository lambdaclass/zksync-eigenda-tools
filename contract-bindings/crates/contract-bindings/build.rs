use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use ethers_contract_abigen::Abigen;


// This build.rs script compiles the eigenda contracts (in the eigenda/contracts submodule dir)
// and copies the artifacts to the src/generated/abis directory in this crate, such that they
// can be used in the contract bindings crate.
// The goal is for these abis to be distributed with this eigenda-contract-bindings crate.
fn main() {
    // Path to eigenda contracts
    let input_contracts_dir =
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/eigenda/contracts"));
    // Check if the directory exists
    if !input_contracts_dir.exists() {
        panic!(
            "Contracts directory not found at: {:?}",
            input_contracts_dir
        );
    }

    // Create destination directory for contract artifacts
    let output_abis_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated/abis"));
    fs::create_dir_all(&output_abis_dir).expect("Failed to create contracts directory");

    // Navigate to the contracts directory and run forge build
    let status = Command::new("forge")
        .current_dir(&input_contracts_dir)
        .arg("build")
        .arg("--force") // Force recompilation
        .status()
        .expect("Failed to execute forge build");
    if !status.success() {
        panic!("forge build failed with status: {}", status);
    }

    // List of contract artifacts we need
    let artifacts = [
        "IEigenDACertVerifier.sol/IEigenDACertVerifier.json",
        "IEigenDACertVerifierBase.sol/IEigenDACertVerifierBase.json",
        "IEigenDACertVerifierRouter.sol/IEigenDACertVerifierRouter.json",
        // Add more artifacts as needed
    ];

    // Copy artifacts to our abis directory
    let forge_out_dir = input_contracts_dir.join("out");
    for artifact in artifacts.iter() {
        let src_path = forge_out_dir.join(artifact);
        if !src_path.exists() {
            panic!("Artifact not found: {:?}", src_path);
        }

        let json_file_name = Path::new(artifact).file_name().unwrap();
        let dst_path = output_abis_dir.join(json_file_name);

        fs::copy(&src_path, &dst_path)
            .unwrap_or_else(|_| panic!("Failed to copy artifact: {:?}", src_path));

        println!("Copied artifact: {:?}", json_file_name);
    }

    let json_file_name = Path::new("IEigenDACertVerifier.json").file_name().unwrap();
    Abigen::new("IEigenDACertVerifier", output_abis_dir.join(json_file_name).to_str().unwrap()).unwrap()
        .generate().unwrap()
        .write_to_file(Path::new("src/IEigenDACertVerifier.rs")).unwrap();

    let json_file_name = Path::new("IEigenDACertVerifierBase.json").file_name().unwrap();
    Abigen::new("IEigenDACertVerifierBase", output_abis_dir.join(json_file_name).to_str().unwrap()).unwrap()
        .generate().unwrap()
        .write_to_file(Path::new("src/IEigenDACertVerifierBase.rs")).unwrap();

    let json_file_name = Path::new("IEigenDACertVerifierRouter.json").file_name().unwrap();
    Abigen::new("IEigenDACertVerifierRouter", output_abis_dir.join(json_file_name).to_str().unwrap()).unwrap()
        .generate().unwrap()
        .write_to_file(Path::new("src/IEigenDACertVerifierRouter.rs")).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=eigenda/contracts/src");
}
