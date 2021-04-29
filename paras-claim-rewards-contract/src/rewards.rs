use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId};
use near_sdk::json_types::{U128};
use near_sdk::collections::{Vector};
use near_sdk::serde::{Deserialize, Serialize};


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Reward {
    amount: u128,
    memo: String,
}

#[derive(Deserialize, Serialize)]
pub struct WrappedReward {
    amount: U128,
    memo: String
}


#[derive(BorshDeserialize, BorshSerialize)]
pub struct Rewards {
    rewards: Vector<Reward>,
    amount: u128,
}

impl Rewards{
    pub fn new(account_id: AccountId) -> Self {
        Self {
            rewards: Vector::new(account_id.as_bytes().to_vec()),
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
        amount: U128,
        memo: String,
    ) -> Self {
        Self {
            amount: amount.into(),
            memo: memo,
        }
    }
    pub fn get_amount(&self) -> u128 {
        self.amount
    }
    pub fn get_memo(&self) -> String {
        self.memo.clone()
    }

    pub fn to_wreward(&self) -> WrappedReward {
        WrappedReward::new(self)
    }
}

impl WrappedReward {
    pub fn new(
        reward: &Reward
    ) -> Self {
        Self {
            amount: reward.get_amount().into(),
            memo: reward.get_memo()
        }
    }
}