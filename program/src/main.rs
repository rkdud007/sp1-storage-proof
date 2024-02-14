//! A simple program to be proven inside the zkVM.

#![no_main]

use alloy_primitives::{hex::FromHex, keccak256, FixedBytes};
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read storage key
    let storage_key = sp1_zkvm::io::read::<String>();
    // Read storage value
    let storage_value = sp1_zkvm::io::read::<String>();
    // read siblings
    let siblings = sp1_zkvm::io::read::<Vec<String>>();
    // read root
    let root = sp1_zkvm::io::read::<String>();

    // Step1. Calculate the leaf node
    let key = Vec::from_hex(storage_key).unwrap();
    let value: Vec<u8> = Vec::from_hex(storage_value).unwrap();
    let mut leaf_bytes = key;
    leaf_bytes.extend(value.clone());
    let leaf_hash = keccak256(leaf_bytes);

    // Step 2. Verify the merkle proof
    let mut current_hash = leaf_hash;
    for sibling in siblings {
        let sibling_bytes: Vec<u8> = Vec::from_hex(&sibling).unwrap();
        let current_bytes = Vec::from_hex(current_hash).unwrap();
        let mut current_hash_bytes = current_bytes;
        current_hash_bytes.extend(sibling_bytes);
        current_hash = keccak256(current_hash_bytes);
    }

    // Step 3. Verify the root
    let root_bytes: FixedBytes<32> = FixedBytes::from_hex(root).unwrap();
    if current_hash != root_bytes.as_slice() {
        panic!("Invalid merkle proof");
    }
    println!("Merkle proof verified, got:{:?}", value);

    sp1_zkvm::io::write(&value);
}
