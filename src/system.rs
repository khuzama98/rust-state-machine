use std::collections::BTreeMap;

type Nonce = u32;
type BlockNumber = u32;
type AccountId = String;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
    /// The current block number.
    block_number: BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<AccountId, Nonce>,
}

impl Pallet {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> u32 {
        /* TODO: Return the current block number. */
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        /* TODO: Increment the current block number by one. */
        self.block_number = self.block_number.checked_add(1).unwrap_or(0);
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &String) {
        /* TODO: Get the current nonce of `who`, and increment it by one. */
        let nonce = *self.nonce.get(who).unwrap_or(&0);

        self.nonce
            .insert(who.to_string(), nonce.checked_add(1).unwrap());
    }

    pub fn get_nonce(&self, who: &String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {
    use crate::system::Pallet;

    #[test]
    fn init_system() {
        let mut system = Pallet::new();
        /* TODO: Create a test which checks the following:
            - Increment the current block number.
            - Increment the nonce of `alice`.

            - Check the block number is what we expect.
            - Check the nonce of `alice` is what we expect.
            - Check the nonce of `bob` is what we expect.
        */
        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());

        assert_eq!(system.block_number(), 1);
        assert_eq!(system.get_nonce(&"alice".to_string()), 1);
        assert_eq!(system.get_nonce(&"bob".to_string()), 0);
    }
}
