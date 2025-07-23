mod balances;
mod system;
use crate::system::Config;
use crate::balances::Config as BalanceConfig;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}
// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl BalanceConfig for Runtime  {
    type  AccountId = types::AccountId;
    type Balance = types::Balance;
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let alice = &"alice".to_string();
    let bob = &"bob".to_string();
    let charlie = &"charlie".to_string();
    let mut runtime = Runtime::new();

    runtime.balances.set_balance(alice, 100);

    // start emulating a block
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    runtime.system.inc_nonce(alice);
    let _transfer_res = runtime
        .balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| eprintln!("error: {e}"));

    // second transaction
    runtime.system.inc_nonce(alice);
    let _transfer_res = runtime
        .balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| eprintln!("error: {e}"));

    println!("{runtime:#?}");
}
