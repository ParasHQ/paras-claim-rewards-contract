use crate::*;
use near_sdk::{PromiseOrValue, log};

use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// Callback on receiving tokens by this contract.
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_in = env::predecessor_account_id();
        assert!(msg.is_empty(), "ERR_MSG_INCORRECT");
        assert!(token_in == self.token, "ERR_TOKEN_IS_NOT_PARAS");
        self.internal_deposit(amount.into());
        log!("Deposited amount : {}", self.deposited_amount);
        PromiseOrValue::Value(U128(0))
    }
}
