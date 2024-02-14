use ethers_contract::BaseContract;
use ethers_core::abi::parse_abi;
use revm::{
    db::InMemoryDB,
    primitives::{address, AccountInfo, TransactTo, U256},
    Evm,
};

fn main() {
    // this is a random address
    let balance = U256::from(111);
    let address = "0x4838b106fce9647bdf1e7877bf73ce8b0bad5f97"
        .parse()
        .unwrap();
    let info = AccountInfo {
        balance,
        ..Default::default()
    };
    // initialise empty in-memory-db
    let mut cache_db = InMemoryDB::default();

    // Populate the DB pre-state,
    // TODO: Make this data witnessed via merkle patricia proofs.
    cache_db.insert_account_info(address, info);
    // ETH/USDT pair on Uniswap V2
    let pool_address = address!("0d4a11d5EEaaC28EC3F61d100daF4d40471f1852");

    // generate abi for the calldata from the human readable interface
    let abi = BaseContract::from(
            parse_abi(&[
                "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)",
            ]).unwrap(),
        );
    // encode abi into Bytes
    let encoded = abi.encode("getReserves", ()).unwrap();

    // initialise an empty (default) EVM
    let mut evm = Evm::builder()
        .with_db(cache_db)
        .modify_tx_env(|tx| {
            // fill in missing bits of env struct
            // change that to whatever caller you want to be
            tx.caller = address!("4838b106fce9647bdf1e7877bf73ce8b0bad5f97");
            // account you want to transact with
            tx.transact_to = TransactTo::Call(pool_address);
            // calldata formed via abigen
            // tx.data = encoded.0.into();
            // transaction value in wei
            tx.value = U256::from(111);
        })
        .build();

    // execute transaction without writing to the DB
    let ref_tx = evm.transact().unwrap();
    println!("{:?}", ref_tx);
    // select ExecutionResult struct
    let result = ref_tx.result;

    println!("{:?}", result)
}
