use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{UnorderedMap},
    env,
    json_types::U128,
    log,
    near_bindgen,
    AccountId,
    Promise,
};

use std::{thread::sleep, time::Duration};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Register {
    pub account_id: AccountId,
    pub tippers: UnorderedMap<AccountId, U128>,
    pub expiration: Duration,
}

// impl Default for Register {
//     fn default() -> Self {
//         Self {
//             account_id: "",
//             expiration: Duration.from_secs(2592000),
//         }
//     }
// }

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(account_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            account_id,
            tippers: UnorderedMap::new(b"tippers".to_vec()),
            expiration: Duration.from_secs(2592000),
        }
    }

    pub fn increase_tip(&self, &account_id: AccountId, amount: U128) {

    }

    pub fn return_funds_and_self_destruct_pending_expiration(&self) {
        sleep(self.expiration);
        // TODO: iterate through tippers and return funds
        // TODO: Algorithm for proportionally dividing gas?
    }
}