use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, assert_one_yocto, Promise};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::collections::{LookupMap};
use near_contract_standards::upgrade::Ownable;

near_sdk::setup_alloc!();

use crate::utils::{ext_fungible_token, GAS_FOR_FT_TRANSFER};
use crate::rewards::{Rewards, Reward};
mod utils;
mod rewards;


/*
    Implementation of claim rewards.
*/
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    token: AccountId,
    records: LookupMap<AccountId, Rewards>,
    reward_amount: LookupMap<AccountId, u128>,
}

impl Ownable for Contract {
    fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner = owner;
    }
}

#[near_bindgen]
impl Contract{
    #[init]
    pub fn new(
        owner: ValidAccountId,
        token: ValidAccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let this = Self {
            owner: owner.into(),
            token: token.into(),
            records: LookupMap::new(b"t".to_vec()),
            reward_amount: LookupMap::new(b"c".to_vec()),
        };
        this
    }

    pub fn get_rewards(&self, from_index: u64, limit: u64, account_id: ValidAccountId) -> Vec<Reward> {
        let user_rewards = self.records.get(account_id.as_ref()).unwrap();
        (from_index..std::cmp::min(from_index + limit, user_rewards.get_rewards_len()))
            .map(|index| user_rewards.get_reward(index))
            .collect()
    }

    pub fn reward_amount(&self, account_id: ValidAccountId) -> u128 {
        self.internal_reward_amount(account_id.into())
    }
    
    #[private]

    pub fn internal_reward_amount(&self, account_id: String) -> u128 {
        match self.reward_amount.get(&account_id) {
            Some(value) => {
                value
            },
            None => 0
        }
    }
    
    pub fn claim_reward(&mut self, amount: U128) -> Promise {
        assert_one_yocto();
        let current_amount = self.internal_reward_amount(env::predecessor_account_id());
        let amount: u128 = amount.into();
        assert!(amount < current_amount, "Amount higher than unclaimed rewards");

        let amount_sub = current_amount.checked_sub(amount).expect("ERR_INTEGER_UNDERFLOW");

        self.reward_amount.insert(&env::predecessor_account_id(), &amount_sub);
        ext_fungible_token::ft_transfer(
            env::predecessor_account_id().into(),
            amount.into(),
            None,
            &self.token,
            1,
            GAS_FOR_FT_TRANSFER
        )
    }

    pub fn push_reward(&mut self, account_id: ValidAccountId, amount: U128, memo: String) {
        self.assert_owner();
        let mut current_rewards = self.records.get(account_id.as_ref()).unwrap_or(Rewards::new());
        let new_reward: Reward = Reward::new(
            account_id.clone().into(),
            amount.into(),
            memo,
        );
        let current_amount = self.reward_amount(account_id.clone().into());
        let amount_add = current_amount.checked_add(amount.into()).expect("ERR_INTEGER_OVERFLOW");


        // insert new record to current_record
        current_rewards.internal_add_new_reward(new_reward);
        self.records.insert(account_id.as_ref(), &current_rewards);

        //set reward amount
        self.reward_amount.insert(account_id.as_ref(), &amount_add);
    }

    pub fn init_reward(&mut self, account_id: ValidAccountId) {
        self.assert_owner();
        assert!(self.records.contains_key(account_id.as_ref()), "ERR_ACCOUNT_ALREADY_EXIST");

        self.records.insert(account_id.as_ref(), &Rewards::new());
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    /*
    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
    */
}
