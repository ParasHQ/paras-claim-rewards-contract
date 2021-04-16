use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::collections::{Vector};
use near_sdk::serde::{Deserialize, Serialize};


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
pub struct Reward {
    account_id: AccountId,
    amount: u128,
    memo: String,
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Rewards {
    rewards: Vector<Reward>,
    amount: u128,
}

impl Rewards{
    pub fn new() -> Self {
        Self {
            rewards: Vector::new(b"r".to_vec()),
            amount: 0,
        }
    }

    
    pub fn internal_add_new_reward(&mut self, reward: Reward) {
        self.rewards.push(&reward);
    }

    pub fn internal_set_reward_amount(&mut self, amount: u128) {
        self.amount = amount;
    }

    pub fn internal_reward_amount(&self) -> u128 {
        return self.amount;
    }

    pub fn get_reward(&self, reward_id: u64) -> Reward {
        self.rewards.get(reward_id).expect("ERR_NO_REWARD").into()
    }

    pub fn get_rewards_len(&self) -> u64 {
        self.rewards.len()
    }
}

impl Reward {
    pub fn new(
        account_id: ValidAccountId,
        amount: U128,
        memo: String,
    ) -> Self {
        Self {
            account_id: account_id.into(),
            amount: amount.into(),
            memo: memo,
        }
    }

    pub fn get_account_id(&self) -> AccountId{
        self.account_id.clone()
    }
    pub fn get_amount(&self) -> u128 {
        self.amount
    }
    pub fn get_memo(&self) -> String {
        self.memo.clone()
    }

}