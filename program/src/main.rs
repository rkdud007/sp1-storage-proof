//! A simple program to be proven inside the zkVM.

#![no_main]

use alloy_dyn_abi::DynSolType;
use alloy_primitives::{
    hex::{self, FromHex},
    keccak256, Address, Bytes, FixedBytes, Keccak256, StorageKey, B256, U256,
};
use rlp::Rlp;
use serde::{Deserialize, Serialize};
sp1_zkvm::entrypoint!(main);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessorInput {
    // U256 type
    pub tasks_root: B256,
    // U256 type
    pub results_root: B256,
    pub proofs: Vec<ProcessedBlockProofs>,
    pub tasks: Vec<ProcessedTask>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcessedTask {
    DatalakeCompute(ProcessedDatalakeCompute),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessedDatalakeCompute {
    /// encoded computational task
    pub encoded_task: Bytes,
    /// computational task commitment
    pub task_commitment: B256,
    /// raw evaluation result of target compiled task
    pub compiled_result: U256,
    /// results merkle tree's entry value
    pub result_commitment: B256,
    /// merkle proof for tasks
    pub task_proof: Vec<B256>,
    /// merkle proof for results
    pub result_proof: Vec<B256>,
    /// encoded datalake
    pub encoded_datalake: Bytes,
    // ex. block sampled datalake / transaction datalake
    pub datalake_type: u8,
    // ex. "header", "account", "storage"
    pub property_type: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessedMPTProof {
    pub block_number: u64,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessedBlockProofs {
    pub chain_id: u128,
    pub mmr_with_headers: Vec<MMRWithHeader>,
    pub accounts: Vec<ProcessedAccount>,
    pub storages: Vec<ProcessedStorage>,
    pub transactions: Vec<ProcessedTransaction>,
    pub transaction_receipts: Vec<ProcessedReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessedReceipt {
    pub key: String,
    pub block_number: u64,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessedTransaction {
    pub key: String,
    pub block_number: u64,
    pub proof: Vec<Bytes>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessedAccount {
    pub address: Address,
    pub account_key: String,
    pub proofs: Vec<ProcessedMPTProof>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessedStorage {
    pub address: Address,
    pub slot: B256,
    pub storage_key: StorageKey,
    pub proofs: Vec<ProcessedMPTProof>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MMRWithHeader {
    pub mmr_meta: MMRMeta,
    pub headers: Vec<ProcessedHeader>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MMRMeta {
    pub id: u64,
    pub root: String,
    pub size: u64,
    // hex encoded
    pub peaks: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessedHeader {
    pub rlp: Vec<u8>,
    pub proof: ProcessedHeaderProof,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProcessedHeaderProof {
    pub leaf_idx: u64,
    pub mmr_path: Vec<String>,
}

pub fn main() {
    // processor input
    let processor_input = sp1_zkvm::io::read::<ProcessorInput>();

    // let root = sp1_zkvm::io::read::<String>();

    println!("Step 0 - Reading input from the zkVM");

    //=========================================================================

    // 0. decode task => list out target values to be verified
    // 1. verify Header (MMR)
    // 2. verify Account/Storage/Transaction/Receipt (MPT)
    // 3. retrieve verified values + compute result
    // 4. construct fact hash and commit

    //=========================================================================

    sp1_zkvm::io::commit(&processor_input.tasks_root.to_string());
}
