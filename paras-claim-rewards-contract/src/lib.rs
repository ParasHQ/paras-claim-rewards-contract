use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, assert_one_yocto, Promise, log};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::collections::{LookupMap};

near_sdk::setup_alloc!();

use crate::utils::{ext_fungible_token, GAS_FOR_FT_TRANSFER};
use crate::rewards::{Rewards, Reward, RewardD};
mod utils;
mod rewards;
mod token_receiver;

/*
    Implementation of claim rewards.
*/
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    token: AccountId,
    records: LookupMap<AccountId, Rewards>,
    deposited_amount: u128,
}

#[near_bindgen]
impl Contract{
    #[init]
    pub fn new(
        owner: ValidAccountId,
        token: ValidAccountId,
    ) -> Self {
        assert!(!env::state_exists(), "ERR_CONTRACT_ALREADY_INTIALIZED");
        let this = Self {
            owner: owner.into(),
            token: token.into(),
            records: LookupMap::new(b"t".to_vec()),
            deposited_amount: 0,
        };
        this
    }

    fn internal_deposit(&mut self, amount: u128) {
        self.deposited_amount = self.deposited_amount.checked_add(amount).expect("ERR_INTEGER_OVERFLOW");
    }

    pub fn get_rewards(&self, from_index: u64, limit: u64, account_id: ValidAccountId) -> Vec<RewardD> {
        let user_rewards = self.records.get(account_id.as_ref()).unwrap();
        (from_index..std::cmp::min(from_index + limit, user_rewards.get_rewards_len())).rev()
            .map(|index| user_rewards.get_reward(index).to_reward_d())
            .collect()
    }

    pub fn get_reward_amount(&self, account_id: ValidAccountId) -> U128 {
        let current_rewards = self.records.get(account_id.as_ref()).unwrap();
        current_rewards.internal_reward_amount().into()
    }
    
    
    #[payable]
    pub fn claim_reward(&mut self, amount: U128) -> Promise {
        assert_one_yocto();
        let mut current_rewards = self.records.get(&env::predecessor_account_id()).unwrap();
        let current_amount = current_rewards.internal_reward_amount();
        let amount: u128 = amount.into();
        assert!(amount <= current_amount, "ERR_AMOUNT_TOO_HIGH");

        log!("Claiming reward : {} PARAS", (amount as f64 / 1e24));
        current_rewards.internal_set_reward_amount(current_amount.checked_sub(amount).expect("ERR_INTEGER_OVERFLOW"));

        self.records.insert(&env::predecessor_account_id(), &current_rewards);

        ext_fungible_token::ft_transfer(
            env::predecessor_account_id().into(),
            amount.into(),
            None,
            &self.token,
            1,
            GAS_FOR_FT_TRANSFER
        )
    }

    #[payable]
    pub fn push_reward(&mut self, account_id: ValidAccountId, amount: U128, memo: String) {
        assert_eq!(self.owner, env::predecessor_account_id(), "ERR_NOT_OWNER");
        assert_one_yocto();
        assert!(self.deposited_amount >= amount.into(), "ERR_DEPOSITED_AMOUNT_NOT_ENOUGH");
        let mut current_rewards = self.records.get(account_id.as_ref()).unwrap_or(Rewards::new(account_id.clone().into()));
        let new_reward: Reward = Reward::new(
            amount.into(),
            memo,
        );
        self.deposited_amount = self.deposited_amount.checked_sub(amount.into()).expect("ERR_INTEGER_OVERFLOW");

        // insert new record to current_record and set reward amount
        let current_amount = current_rewards.internal_reward_amount();
        current_rewards.internal_add_new_reward(new_reward);
        current_rewards.internal_set_reward_amount(current_amount.checked_add(amount.into()).expect("ERR_INTEGER_OVERFLOW"));
        self.records.insert(account_id.as_ref(), &current_rewards);

        log!("Current reward for {} : {} PARAS", account_id.to_string(), current_rewards.internal_reward_amount() as f64 / 1e24);

    }

}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env};


    const TEN_PARAS_TOKEN: U128 = U128(10_000_000_000_000_000_000_000_000);

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn setup_contract() -> (VMContextBuilder, Contract) {
        let mut context = VMContextBuilder::new();
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        let contract = Contract::new(accounts(1).into(), accounts(2).into());
        (context, contract)
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(accounts(1).into(), accounts(2).into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.deposited_amount, 0);
        assert_eq!(contract.owner, accounts(1).to_string());
        assert_eq!(contract.token, accounts(2).to_string());
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_internal_deposit() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
                .predecessor_account_id(accounts(2))
                .attached_deposit(1)
                .build());
        contract.ft_on_transfer(accounts(3), U128(10), "".to_string());
        assert_eq!(contract.deposited_amount, 10);
    }

    #[test]
    fn test_push_reward() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
                .predecessor_account_id(accounts(2))
                .attached_deposit(1)
                .build());
        contract.ft_on_transfer(accounts(3), TEN_PARAS_TOKEN, "".to_string());
        testing_env!(context
                .predecessor_account_id(accounts(1))
                .attached_deposit(1)
                .build());
        contract.push_reward(accounts(3), TEN_PARAS_TOKEN, "first reward".to_string());
        assert_eq!(contract.deposited_amount, 0);
        assert_eq!(contract.get_reward_amount(accounts(3)), TEN_PARAS_TOKEN.into());
        assert_eq!(contract.records.get(accounts(3).as_ref()).unwrap().get_reward(0).get_amount(), TEN_PARAS_TOKEN.into());
        assert_eq!(contract.records.get(accounts(3).as_ref()).unwrap().get_reward(0).get_memo(), "first reward");
    }

    #[test]
    fn test_claim_reward() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
                .predecessor_account_id(accounts(2))
                .attached_deposit(1)
                .build());
        contract.ft_on_transfer(accounts(3), TEN_PARAS_TOKEN, "".to_string());
        testing_env!(context
                .predecessor_account_id(accounts(1))
                .attached_deposit(1)
                .build());
        contract.push_reward(accounts(3), TEN_PARAS_TOKEN, "first reward".to_string());
        testing_env!(context
                .predecessor_account_id(accounts(3))
                .attached_deposit(1)
                .build());
        contract.claim_reward(TEN_PARAS_TOKEN);
        assert_eq!(contract.get_reward_amount(accounts(3)), U128(0));
    }

    #[test]
    #[should_panic(expected = "ERR_DEPOSITED_AMOUNT_NOT_ENOUGH")]
    fn test_not_enough_push_reward() {
        let (mut context, mut contract) = setup_contract();
        testing_env!(context
                .predecessor_account_id(accounts(1))
                .attached_deposit(1)
                .build());
        contract.push_reward(accounts(3).into(), TEN_PARAS_TOKEN, "".to_string());
    }
}
