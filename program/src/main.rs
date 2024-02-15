//! A simple program to be proven inside the zkVM.

#![no_main]

use std::str::FromStr as _;

use alloy_dyn_abi::DynSolType;
use alloy_primitives::{
    hex::{self, FromHex},
    keccak256, FixedBytes, Keccak256, U256,
};
use rlp::Rlp;
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

    //=========================================================================

    // Step 1. Compute the key hash of target node
    let mut hasher = Keccak256::new();
    // [KEY] Current node's key is keccak256(storage_slot)
    hasher.update(storage_key);
    let key_hash = hasher.finalize();
    println!("current key: \n{}\n", key_hash);

    // Step 2. Verify the merkle proof
    let mut current_hash = key_hash;

    // Decode serialized siblings
    let siblings_type: DynSolType = "bytes[]".parse().unwrap();
    let bytes = Vec::from_hex(siblings).expect("Invalid hex string");
    let serialized_siblings = siblings_type.abi_decode(&bytes).unwrap();

    if let Some(siblings) = serialized_siblings.as_array() {
        for sibling in siblings {
            // Step 2: Decode the siblings
            let sibling_hex = format!("0x{}", hex::encode(sibling.as_bytes().unwrap()));

            let siblings_bytes = hex::decode(sibling_hex.as_bytes()).expect("Invalid hex string");
            // RLP decode the RLP encoded node
            let siblings_rlp = Rlp::new(&siblings_bytes);
            let node: Vec<u8> = siblings_rlp.data().unwrap().to_vec();
            // Step 2.1: Compute the Key of the sibling node
            hasher = Keccak256::new();
            hasher.update(node);
            current_hash = hasher.finalize();
            println!("sibling key: \n{:?}\n", current_hash);
        }
    }

    // Step 3. Verify the root
    let root_bytes: FixedBytes<32> = FixedBytes::from_hex(root).unwrap();
    // Computed hash should match the given root
    if current_hash != root_bytes.as_slice() {
        println!(
            "original root: \n{:?}\n",
            hex::encode(root_bytes.as_slice())
        );

        panic!("Invalid merkle proof :/");
    }
    println!("✅ åMerkle proof verified");

    //=========================================================================

    sp1_zkvm::io::write(&storage_value);
}
