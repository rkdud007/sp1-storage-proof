//! A simple program to be proven inside the zkVM.

#![no_main]

use std::str::FromStr as _;

use alloy_dyn_abi::DynSolType;
use alloy_primitives::{hex::FromHex, keccak256, FixedBytes, Keccak256, U256};
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read storage key
    let storage_key = sp1_zkvm::io::read::<String>();
    // Read storage value
    let storage_value = sp1_zkvm::io::read::<String>();
    // read siblings
    let siblings = sp1_zkvm::io::read::<String>();
    // read root
    let root = sp1_zkvm::io::read::<String>();

    println!("Step 0 - Reading input from the zkVM");

    let mut hasher = Keccak256::new();
    // Step1. Calculate the leaf node
    let key = Vec::from_hex(storage_key).unwrap();
    let storage_value = U256::from_str(&storage_value).unwrap();
    let bytes: [u8; 32] = storage_value.to_be_bytes();
    hasher.update(key);
    // U256 to bytes
    hasher.update(bytes);

    let leaf_hash = keccak256(hasher.clone().finalize());
    // let leaf_hash_formated = format!("0x{:x}", leaf_hash);

    // Decode serialized siblings
    let siblings_type: DynSolType = "bytes[]".parse().unwrap();
    let bytes = Vec::from_hex(siblings).expect("Invalid hex string");
    let serialized_siblings = siblings_type.abi_decode(&bytes).unwrap();

    // Step 2. Verify the merkle proof
    let mut current_hash = leaf_hash;

    if let Some(siblings) = serialized_siblings.as_array() {
        for sibling in siblings {
            // Depending on the Merkle tree structure, you might need to adjust the order
            // of concatenation. This example assumes the current hash is left and the proof element is right.
            hasher = Keccak256::new();
            hasher.update(current_hash);
            hasher.update(sibling.as_bytes().unwrap());
            current_hash = hasher.finalize();
        }
    }

    // Step 3. Verify the root
    let root_bytes: FixedBytes<32> = FixedBytes::from_hex(root).unwrap();
    if current_hash != root_bytes.as_slice() {
        panic!("Invalid merkle proof");
    }
    println!("Merkle proof verified");

    sp1_zkvm::io::write(&storage_value);
}
