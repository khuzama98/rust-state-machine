mod balances;
mod system;

mod types {
    /*
        TODO: Move your type definitions for `AccountId` and `Balance` here.
    */
    pub type AccountId = String;
    pub type Balance = u128;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet,
    balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
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
    /* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
    let mut runtime = Runtime::new();
    /* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
    runtime.balances.set_balance(alice, 100);

    // start emulating a block
    /* TODO: Increment the block number in system. */
    runtime.system.inc_block_number();
    /* TODO: Assert the block number is what we expect. */
    assert_eq!(runtime.system.block_number(), 1);

    // first transaction
    /* TODO: Increment the nonce of `alice`. */
    runtime.system.inc_nonce(alice);
    /* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
        - The transfer _could_ return an error. We should use `map_err` to print
          the error if there is one.
        - We should capture the result of the transfer in an unused variable like `_res`.
    */
    let _transfer_res = runtime
        .balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e| eprintln!("error: {e}"));

    // second transaction
    /* TODO: Increment the nonce of `alice` again. */
    runtime.system.inc_nonce(alice);
    /* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
    let _transfer_res = runtime
        .balances
        .transfer(alice.clone(), charlie.clone(), 20)
        .map_err(|e| eprintln!("error: {e}"));

    println!("{runtime:#?}");
}
