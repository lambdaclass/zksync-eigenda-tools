use tokio_postgres::NoTls;

use crate::{
    blob_info::BlobInfo, client::EigenClientRetriever, verify_blob::decode_blob_info,
    DB_CONN_CONFIG, EIGENDA_API_URL,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlobData {
    pub blob_info: BlobInfo,
    pub blob: Vec<u8>,
}

// Retrieve each blob's blob_info from the database
async fn get_blobs_info_from_db() -> anyhow::Result<Vec<BlobInfo>> {
    let (db_client, db_connection) = tokio_postgres::connect(DB_CONN_CONFIG, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = db_connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let timestamp =
        chrono::NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")?;
    let rows = db_client
            .query("SELECT inclusion_data, sent_at FROM data_availability WHERE sent_at > $1 AND inclusion_data IS NOT NULL ORDER BY sent_at", &[&timestamp])
            .await?;

    println!("Retrieved {} blobs from the database", rows.len());

    let mut blobs_info = Vec::new();
    for row in rows {
        let inclusion_data = row.get(0);
        let (blob_header, blob_verification_proof) = decode_blob_info(inclusion_data)?;
        blobs_info.push(BlobInfo {
            blob_header: blob_header.into(),
            blob_verification_proof: blob_verification_proof.into(),
        });
    }
    Ok(blobs_info)
}

async fn get_blob_from_disperser(blob_info: BlobInfo) -> anyhow::Result<Vec<u8>> {
    let client = EigenClientRetriever::new(EIGENDA_API_URL).await?;
    let data = client
        .get_blob_data(blob_info)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Blob not found"))?;

    Ok(data)
}

pub async fn get_blobs() -> anyhow::Result<Vec<BlobData>> {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    let mut blobs_data = Vec::new();
    let blob_infos = get_blobs_info_from_db().await?;
    for blob_info in blob_infos {
        let blob = get_blob_from_disperser(blob_info.clone()).await?;
        let blob_data = BlobData { blob_info, blob };
        blobs_data.push(blob_data);
    }
    Ok(blobs_data)
}
