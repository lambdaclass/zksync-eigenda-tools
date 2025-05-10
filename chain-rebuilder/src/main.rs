use crate::utils::get_transactions;
use alloy::{network::Ethereum, primitives::Address, providers::RootProvider};
use std::str::FromStr;

mod blob_info;
mod client;
mod generated;
mod utils;

const EIGENDA_API_URL: &str = "https://disperser-holesky.eigenda.xyz:443";
const BLOB_DATA_JSON: &str = "blob_data.json";
const ABI_JSON: &str = "./abi/commitBatchesSharedBridge.json";
const COMMIT_BATCHES_SELECTOR: &str = "98f81962";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage: cargo run <validatorTimelockAddress> <rpc_url> <block_start> <disperser_url>"
        );
        std::process::exit(1);
    }

    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let validator_timelock_address = Address::from_str(&args[1])?;
    let url = alloy::transports::http::reqwest::Url::from_str(&args[2])?;
    let provider: RootProvider<
        alloy::transports::http::Http<alloy::transports::http::Client>,
        Ethereum,
    > = RootProvider::new_http(url);

    let block_start = args[3].parse::<u64>()?;
    let disperser_url = &args[4];

    get_transactions(
        &provider,
        validator_timelock_address,
        block_start,
        disperser_url,
    )
    .await
}
