use std::fs;

use crate::{
    blob_info::{
        BatchHeader, BatchMetadata, BlobHeader, BlobInfo, BlobQuorumParam, BlobVerificationProof,
        G1Commitment,
    },
    client::EigenClientRetriever,
    ABI_JSON, BLOB_DATA_JSON, COMMIT_BATCHES_SELECTOR, EIGENDA_API_URL,
};
use alloy::{
    dyn_abi::{DynSolValue, JsonAbiExt},
    json_abi::JsonAbi,
    network::Ethereum,
    primitives::Address,
    providers::{Provider, RootProvider},
};
use ethabi::{ParamType, Token};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BlobData {
    pub blob_info: BlobInfo,
    pub blob: Vec<u8>,
}

/// Helper functions for safe extraction
fn extract_tuple(token: &Token) -> anyhow::Result<&Vec<Token>> {
    match token {
        Token::Tuple(inner) => Ok(inner),
        _ => Err(anyhow::anyhow!("Not a tuple")),
    }
}

fn extract_array(token: &Token) -> anyhow::Result<Vec<Token>> {
    match token {
        Token::Array(tokens) => Ok(tokens.clone()),
        _ => Err(anyhow::anyhow!("Not a uint")),
    }
}

fn extract_uint(token: &Token) -> anyhow::Result<u32> {
    match token {
        Token::Uint(value) => Ok(value.as_u32()),
        _ => Err(anyhow::anyhow!("Not a uint")),
    }
}

fn extract_fixed_bytes<const N: usize>(token: &Token) -> anyhow::Result<Vec<u8>> {
    match token {
        Token::FixedBytes(bytes) => Ok(bytes.clone()),
        _ => Err(anyhow::anyhow!("Not fixed bytes")),
    }
}

fn extract_bytes(token: &Token) -> anyhow::Result<Vec<u8>> {
    match token {
        Token::Bytes(bytes) => Ok(bytes.clone()),
        _ => Err(anyhow::anyhow!("Not bytes")),
    }
}

async fn get_blob(blob_info: BlobInfo,disperser_url: &str) -> anyhow::Result<Vec<u8>> {
    let client = EigenClientRetriever::new(disperser_url).await?;
    let data = client
        .get_blob_data(blob_info)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Blob not found"))?;

    Ok(data)
}

pub(crate) async fn get_transactions(
    provider: &RootProvider<
        alloy::transports::http::Http<alloy::transports::http::Client>,
        Ethereum,
    >,
    validator_timelock_address: Address,
    block_start: u64,
    disperser_url: &str,
) -> anyhow::Result<()> {
    let latest_block = provider.get_block_number().await?;
    let mut json_array = Vec::new();

    let mut i = 0;
    for block_number in block_start..=latest_block {
        i += 1;
        if i % 50 == 0 {
            println!(
                "\x1b[32mProcessed up to block {} of {}\x1b[0m",
                block_number, latest_block
            );
        }
        if let Ok(Some(block)) = provider
            .get_block_by_number(block_number.into(), true)
            .await
        {
            for tx in block.transactions.into_transactions() {
                if let Some(to) = tx.to {
                    if to == validator_timelock_address {
                        let input = tx.clone().input;
                        let selector = &input[0..4];
                        println!("selector {:?}", hex::encode(selector));
                        if selector == hex::decode(COMMIT_BATCHES_SELECTOR)? {
                            match decode_blob_data_input(&input[4..],disperser_url).await {
                                Ok(decoded) => {
                                    for blob in decoded {
                                        json_array.push(blob);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("\x1b[31mError decoding blob data: {}\x1b[0m", e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if json_array.is_empty() {
        println!("\x1b[31mNo transactions found.\x1b[0m");
        return Ok(());
    }

    let json_string = serde_json::to_string_pretty(&json_array)?;
    fs::write(BLOB_DATA_JSON, json_string)?;
    println!("\x1b[32mData stored in blob_data.json file.\x1b[0m");

    Ok(())
}

async fn decode_blob_data_input(input: &[u8],disperser_url: &str) -> anyhow::Result<Vec<BlobData>> {
    let json = std::fs::read_to_string(ABI_JSON)?;
    let json_abi: JsonAbi = serde_json::from_str(&json)?;
    let function = json_abi
        .functions
        .iter()
        .find(|f| f.0 == "commitBatchesSharedBridge")
        .ok_or(anyhow::anyhow!("Function not found"))?
        .1;
    let decoded = function[0].abi_decode_input(input, true)?;
    let commit_data = &decoded[3];
    let commit_data = match commit_data {
        DynSolValue::Bytes(commit_data) => commit_data,
        _ => return Err(anyhow::anyhow!("Commit data is not bytes")),
    };

    let param_types = vec![
        ParamType::Tuple(vec![
            ParamType::Uint(64),
            ParamType::FixedBytes(32),
            ParamType::Uint(64),
            ParamType::Uint(256),
            ParamType::FixedBytes(32),
            ParamType::FixedBytes(32),
            ParamType::Uint(256),
            ParamType::FixedBytes(32),
        ]), // StoredBatchInfo
        ParamType::Array(Box::new(ParamType::Tuple(vec![
            ParamType::Uint(64),
            ParamType::Uint(64),
            ParamType::Uint(64),
            ParamType::FixedBytes(32),
            ParamType::Uint(64),
            ParamType::FixedBytes(32),
            ParamType::FixedBytes(32),
            ParamType::FixedBytes(32),
            ParamType::Bytes,
            ParamType::Bytes,
        ]))), // CommitBatchInfo
    ];

    let decoded = ethabi::decode(&param_types, &commit_data[1..])?;

    let commit_batch_info = if let Some(Token::Array(commit_batch_info)) = decoded.get(1) {
        commit_batch_info
    } else {
        return Err(anyhow::anyhow!("CommitBatchInfo is not an array"));
    };

    let mut blobs = vec![];
    for batch_info in commit_batch_info {
        if let Token::Tuple(batch_info) = batch_info {
            if let Some(Token::Bytes(operator_da_input)) = batch_info.get(9) {
                match get_blob_from_operator_da_input(operator_da_input.clone(),disperser_url).await {
                    Ok(blob_data) => blobs.push(blob_data),
                    Err(_) => return Err(anyhow::anyhow!("Error getting blob data")),
                }
            } else {
                return Err(anyhow::anyhow!("Operator DA input is not bytes"));
            }
        } else {
            return Err(anyhow::anyhow!(
                "CommitBatchInfo components cannot be represented as a tuple"
            ));
        }
    }

    Ok(blobs)
}

async fn get_blob_header(blob_header_tokens: &[Token]) -> anyhow::Result<BlobHeader> {
    let commitment_tokens = extract_tuple(&blob_header_tokens[0])?;

    let x = commitment_tokens[0]
        .clone()
        .into_uint()
        .ok_or(anyhow::anyhow!("x is not a uint"))?;
    let y = commitment_tokens[1]
        .clone()
        .into_uint()
        .ok_or(anyhow::anyhow!("y is not a uint"))?;

    let mut x_bytes = vec![0u8; 32];
    let mut y_bytes = vec![0u8; 32];
    x.to_big_endian(&mut x_bytes);
    y.to_big_endian(&mut y_bytes);

    let data_length = extract_uint(&blob_header_tokens[1])?;
    let blob_quorum_params_tokens = extract_array(&blob_header_tokens[2])?;

    let blob_quorum_params: Vec<BlobQuorumParam> = blob_quorum_params_tokens
        .iter()
        .map(|param| {
            let tuple = extract_tuple(param)?;
            Ok(BlobQuorumParam {
                quorum_number: extract_uint(&tuple[0])?,
                adversary_threshold_percentage: extract_uint(&tuple[1])?,
                confirmation_threshold_percentage: extract_uint(&tuple[2])?,
                chunk_length: extract_uint(&tuple[3])?,
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let blob_header = BlobHeader {
        commitment: G1Commitment {
            x: x_bytes,
            y: y_bytes,
        },
        data_length,
        blob_quorum_params,
    };
    Ok(blob_header)
}

async fn get_blob_verification_proof(
    blob_verification_tokens: &[Token],
) -> anyhow::Result<BlobVerificationProof> {
    let batch_id = extract_uint(&blob_verification_tokens[0])?;
    let blob_index = extract_uint(&blob_verification_tokens[1])?;

    let batch_metadata_tokens = extract_tuple(&blob_verification_tokens[2])?;
    let batch_header_tokens = extract_tuple(&batch_metadata_tokens[0])?;

    let batch_header = BatchHeader {
        batch_root: extract_fixed_bytes::<32>(&batch_header_tokens[0])?,
        quorum_numbers: extract_bytes(&batch_header_tokens[1])?,
        quorum_signed_percentages: extract_bytes(&batch_header_tokens[2])?,
        reference_block_number: extract_uint(&batch_header_tokens[3])?,
    };

    let batch_metadata = BatchMetadata {
        batch_header,
        signatory_record_hash: extract_fixed_bytes::<32>(&batch_metadata_tokens[1])?,
        confirmation_block_number: extract_uint(&batch_metadata_tokens[2])?,
        batch_header_hash: extract_bytes(&batch_metadata_tokens[3])?,
        fee: extract_bytes(&batch_metadata_tokens[4])?,
    };

    let blob_verification_proof = BlobVerificationProof {
        batch_id,
        blob_index,
        batch_metadata,
        inclusion_proof: extract_bytes(&blob_verification_tokens[3])?,
        quorum_indexes: extract_bytes(&blob_verification_tokens[4])?,
    };

    Ok(blob_verification_proof)
}

async fn get_blob_from_operator_da_input(operator_da_input: Vec<u8>,disperser_url: &str) -> anyhow::Result<BlobData> {
    let param_types = vec![ParamType::Tuple(vec![
        // BlobHeader
        ParamType::Tuple(vec![
            ParamType::Tuple(vec![ParamType::Uint(256), ParamType::Uint(256)]), // G1Commitment
            ParamType::Uint(32),                                                // data_length
            ParamType::Array(Box::new(ParamType::Tuple(vec![
                ParamType::Uint(32),
                ParamType::Uint(32),
                ParamType::Uint(32),
                ParamType::Uint(32),
            ]))), // BlobQuorumParam
        ]),
        // BlobVerificationProof
        ParamType::Tuple(vec![
            ParamType::Uint(32), // batch_id
            ParamType::Uint(32), // blob_index
            ParamType::Tuple(vec![
                ParamType::Tuple(vec![
                    ParamType::FixedBytes(32),
                    ParamType::Bytes,
                    ParamType::Bytes,
                    ParamType::Uint(32),
                ]), // BatchHeader
                ParamType::FixedBytes(32), // signatory_record_hash
                ParamType::Uint(32),       // confirmation_block_number
                ParamType::Bytes,          // batch_header_hash
                ParamType::Bytes,          // fee
            ]), // BatchMetadata
            ParamType::Bytes,    // inclusion_proof
            ParamType::Bytes,    // quorum_indexes
        ]),
    ])];

    let decoded = ethabi::decode(&param_types, &operator_da_input[32..])?;
    let blob_info = extract_tuple(&decoded[0])?;

    let blob_header_tokens = extract_tuple(&blob_info[0])?;
    let blob_header = get_blob_header(blob_header_tokens).await?;

    let blob_verification_tokens = extract_tuple(&blob_info[1])?;
    let blob_verification_proof = get_blob_verification_proof(blob_verification_tokens).await?;

    let blob_info = BlobInfo {
        blob_header,
        blob_verification_proof,
    };

    let blob = get_blob(blob_info.clone(),disperser_url).await?;
    Ok(BlobData { blob_info, blob })
}
