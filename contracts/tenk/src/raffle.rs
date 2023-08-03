use std::marker::PhantomData;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::IntoStorageKey;

/// This is similar to the raffle collection but doesn't keep track of past winners
#[derive(BorshSerialize, BorshDeserialize)]
#[cfg_attr(not(feature = "expensive-debug"), derive(Debug))]
pub struct Raffle {
    prefix: Vec<u8>,
    current_token_id: u64,
    #[borsh_skip]
    el: PhantomData<u64>,
}

impl Raffle {
    /// Create new vector with zero elements. Use `id` as a unique identifier on the trie.
    pub fn new<S>(prefix: S) -> Self
    where
        S: IntoStorageKey,
    {
        Self {
            prefix: prefix.into_storage_key(),
            current_token_id: 0,
            el: PhantomData,
        }
    }

    pub fn current_id(&self) -> u64 {
      self.current_token_id
    }

    pub fn draw(&mut self) -> u64 {
        self.current_token_id += 1;
        let id: u64 = self.current_token_id;
        id
    }
}