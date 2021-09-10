//! This contract allows users to send tips to registered typto receivers backed by storage on blockchain.
//!
//! The contract provides methods to [tip users][send_tip].
//!
//! [send_tip]: struct.Contract.html#method.send_tip

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::{Vector},
    env,
    json_types::U128,
    log,
    near_bindgen,
    AccountId,
    Promise,
};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub registered_receivers: Vector<AccountId>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            registered_receivers: Vector::new(b"registered_receivers".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            registered_receivers: Vector::new(b"registered_receivers".to_vec()),
        }
    }

    fn get_balance(&self) -> u128 {
        env::account_balance()
    }

    fn is_registered(&self, receiver_id: String) -> bool {
        self.registered_receivers.iter().any(|e| e == receiver_id)
    }

    pub fn register_receiver(&mut self, receiver_id: String) {
        let check: String = receiver_id.clone();
        assert!(env::is_valid_account_id(receiver_id.as_bytes()), "Invalid receiver account");
        if !self.is_registered(check) {
            self.registered_receivers.push(&receiver_id);
        }
    }

    #[payable]
    pub fn send_tip(&mut self, receiver_id: String, tip_amount: U128) {
        assert!(env::is_valid_account_id(env::signer_account_id().as_bytes()), "Invalid receiver account");
        assert!(self.get_balance() >= tip_amount.0, "Insufficient funds");
        let clone = receiver_id.clone();
        assert!(self.is_registered(clone));
        let clone = receiver_id.clone();
        Promise::new(receiver_id).transfer(tip_amount.0);
        log!("Transferred {} tokens from {} to {}", tip_amount.0, env::signer_account_id(), clone);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn alice() -> AccountId {
        "alice.testnet".to_string()
    }
    fn bob() -> AccountId {
        "bob.testnet".to_string()
    }

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: bob(), // Recipient of the transaction
            signer_account_id: alice(), // Originator of the transaction
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_send_tip() {
        let mut context = get_context(alice(), 0);
        const AMOUNT_TO_SEND: u128 = 1_000_000_000_000_000_000_000_000;
        context.account_balance = AMOUNT_TO_SEND;
        testing_env!(context.clone());
        let mut contract = Contract::new();
        contract.register_receiver(bob());
        contract.send_tip(bob(), AMOUNT_TO_SEND);
        assert_eq!(contract.get_balance(), 0, "Account balance should be liquidated.");
    }

    #[test]
    fn test_get_balance() {
        let mut context = get_context(alice(), 0);
        const AMOUNT_TO_SEND: u128 = 1_000_000_000_000_000_000_000_000;
        context.account_balance = AMOUNT_TO_SEND;
        testing_env!(context.clone());
        let mut contract = Contract::new();
        contract.register_receiver(bob());
        assert_eq!(contract.get_balance(), AMOUNT_TO_SEND, "Account balance should be equal to initial balance.");
    }

    #[test]
    #[should_panic]
    fn test_panic_user_not_registered() {
        let mut context = get_context(alice(), 0);
        const AMOUNT_TO_SEND: u128 = 1_000_000_000_000_000_000_000_000;
        context.account_balance = AMOUNT_TO_SEND;
        testing_env!(context.clone());
        let mut contract = Contract::new();
        contract.send_tip(bob(), AMOUNT_TO_SEND);
    }

    #[test]
    #[should_panic]
    fn test_panic_insufficient_funds() {
        let mut context = get_context(alice(), 0);
        const AMOUNT_TO_SEND: u128 = 1_000_000_000_000_000_000_000_000;
        const ACCOUNT_BALANCE: u128 = 1;
        context.account_balance = ACCOUNT_BALANCE;
        testing_env!(context.clone());
        let mut contract = Contract::new();
        contract.send_tip(bob(), AMOUNT_TO_SEND);
    }
}