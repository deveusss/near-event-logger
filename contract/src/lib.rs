use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::*;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Gas;
pub use near_sdk::{
    assert_one_yocto, env, json_types::U128, near_bindgen, require, serde_json::json, AccountId,
    Balance, BorshStorageKey, IntoStorageKey, PanicOnDefault, Promise, PromiseOrValue,
    PromiseResult,
};

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLogEntry {
    time: u64,
    operation: String,
    transaction_hash: String
}
impl EventLogEntry {
    fn to_json_string(&self) -> String {
        near_sdk::serde_json::to_string(self)
            .ok()
            .unwrap_or_else(|| env::abort())
    }

    fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }

    pub(crate) fn emit(self) {
        near_sdk::env::log_str(&self.to_json_event_string());
    }
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert_initialized();
        Contract { owner_id }
    }
}
#[near_bindgen]
impl EventLogger for Contract {
    fn log_event(
        &mut self,
        time: u64,
        operation: String,
        transaction_hash: String
    ) {


        let event = EventLogEntry {
            time,
            operation,
            transaction_hash
        };
        event.emit();

    }
}

pub trait EventLogger {
    fn log_event(
        &mut self,
        time: u64,
        operation: String,
        transaction_has: String
    );
}

pub(crate) fn assert_initialized() {
    assert!(!env::state_exists(), "Already initialized");
}
pub trait Ownable {
    fn assert_owner(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            self.owner(),
            "Ownable: predecessor is not the owner"
        );
    }
    fn owner(&self) -> &AccountId;
    fn transfer_ownership(&mut self, owner: AccountId);
}
#[near_bindgen]
impl Ownable for Contract {
    fn owner(&self) -> &AccountId {
        &self.owner_id
    }

    fn transfer_ownership(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner_id = owner;
    }
}
