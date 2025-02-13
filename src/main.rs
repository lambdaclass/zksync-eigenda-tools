// use crate::utils::get_transactions;
use utils::get_blobs;

mod blob_info;
mod client;
mod generated;
mod utils;
mod verify_blob;

const EIGENDA_API_URL: &str = "https://disperser-holesky.eigenda.xyz:443";
const DB_CONN_CONFIG: &str = "host=localhost user=postgres password=notsecurepassword dbname=zksync_server_localhost_eigenda";
const BLOB_DATA_JSON: &str = "blob_data.json";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let blobs = get_blobs().await?;
    let json_string = serde_json::to_string_pretty(&blobs)?;
    std::fs::write(BLOB_DATA_JSON, json_string)?;
    println!("\x1b[32mData stored in blob_data.json file.\x1b[0m");
    Ok(())
}
