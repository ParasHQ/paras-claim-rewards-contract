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
    rewards: Vector<Reward>
}

impl Rewards{
    pub fn new() -> Self {
        Self {
            rewards: Vector::new(b"r".to_vec())
        }
    }

    pub fn internal_add_new_reward(&mut self, reward: Reward) {
        self.rewards.push(&reward);
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

}