# sp1-storage-proof

Disclamer: This project is under development, playing around with sp1 and alloy & reth codes

## Goal: program that verify storage proof and return valid value of evm state

- 1. input : pass rpc `get_ethProof` response value as an input of the program
- 2. process : calculate storage trie's root base on given path - probably need to add trie implementation, [there is gold reth code](https://github.com/paradigmxyz/reth/blob/39eb6c6d6e7a523705b34f8dc960148e37367d12/crates/trie/src/trie.rs#L467) that im referring
- 3. output : provide valid storage value from given key ( slot )

## Development

Just sake of convineience, debugging in there
https://github.com/rkdud007/playground/blob/main/sp1/src/main.rs for dev purpose

## Input data format

```rust
 pub struct EIP1186ProofResponse {
    pub address: Address,
    pub balance: U256,
    pub code_hash: H256,
    pub nonce: U64,
    pub storage_hash: H256,
    pub account_proof: Vec<Bytes>,
    pub storage_proof: Vec<StorageProof>,
}


pub struct StorageProof {
    pub key: H256,
    pub proof: Vec<Bytes>,
    pub value: U256,
}
```

## FYI

with `alloy` depend on `keccak` precompile library,
with computing 6 keccak hash speed around:

```
Proof generation time: 8s
```

verification is always around

```
Verification time: 406.374542ms
```

The relevat code is in this [commit](https://github.com/rkdud007/sp1-storage-proof/tree/64eb31a88c5566dd442d5d4f4f5b11d8ba50867cs)
