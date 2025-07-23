use num::{CheckedAdd, CheckedSub, One, Zero};
use std::collections::BTreeMap;

pub trait Config {
    type BlockNumber: Zero + CheckedAdd + CheckedSub + Copy + One;
    type AccountId: Ord + Clone;
    type Nonce: Zero + CheckedAdd + CheckedSub + Copy + One;
}
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// The current block number.
    block_number: T::BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        /* TODO: Return the current block number. */
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        /* TODO: Increment the current block number by one. */
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .unwrap_or(T::BlockNumber::zero());
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        /* TODO: Get the current nonce of `who`, and increment it by one. */
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());

        self.nonce
            .insert(who.clone(), nonce.checked_add(&T::Nonce::one()).unwrap());
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::one())
    }
}

#[cfg(test)]
mod test {
    use crate::system::{Config, Pallet};
    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system = Pallet::<TestConfig>::new();

        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());

        assert_eq!(system.block_number(), 1);
        assert_eq!(system.get_nonce(&"alice".to_string()), 1);
        assert_eq!(system.get_nonce(&"bob".to_string()), 0);
    }
}
