use near_sdk::{ext_contract, Gas};
use near_sdk::json_types::{U128};

pub const GAS_FOR_FT_TRANSFER: Gas = 10_000_000_000_000;

#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_balance_of(&self, account_id: AccountId);
    fn storage_balance_of(&self, account_id: AccountId);
}

#[ext_contract(ext_self)]
pub trait Contract {
    fn claim_reward_callback(&mut self, amount: U128, account_id: AccountId);
}