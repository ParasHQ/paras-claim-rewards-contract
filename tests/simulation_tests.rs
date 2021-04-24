use near_sdk::json_types::U128;
use near_sdk::{AccountId};
use near_sdk::serde_json::json;
use near_sdk::serde_json;
use near_sdk_sim::{call, to_yocto, transaction::ExecutionStatus, view, DEFAULT_GAS, UserAccount};

use crate::utils::{init, ptoy};
mod utils;

#[test]
fn simulate_total_supply() {
    let (_, ft, _, _, _) = init();

    // let total_supply: U128 = view!(ft.ft_total_supply()).unwrap_json();
    let total_supply: U128 = ft.view(ft.account_id(), "ft_total_supply", b"").unwrap_json();

    assert_eq!(ptoy(100_000_000), total_supply.0);
}

#[test]
fn simulate_deposit_amount() {
    let (root, ft, claim, _, _) = init();

    let outcome = root.call(
        ft.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": claim.valid_account_id(),
            "amount": U128::from(ptoy(10_000_000)),
            "msg": "".to_string(),
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        1
    );

    //println!("{:?}", outcome.promise_results());
    let outcome_unwrap: U128 = outcome.unwrap_json();
    println!("[DEPOSIT] Deposited : {}", serde_json::to_string(&outcome_unwrap).unwrap());
    println!("[DEPOSIT] Gas burnt : {} TeraGas", outcome.gas_burnt() as f64 / 1e12);

    assert_eq!(outcome_unwrap, U128::from(ptoy(10_000_000)));


}
#[test]
fn simulate_push_reward() {
    let (root, ft, claim, _, user1) = init();

    // Deposit amount first

    root.call(
        ft.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": claim.valid_account_id(),
            "amount": U128::from(ptoy(10_000_000)),
            "msg": "".to_string(),
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        1
    );

    let claim_account = claim.account().unwrap();
    let initial_storage_usage = claim_account.storage_usage;

    // assert user1 is not on the rewards record
    let user1_reward_outcome = view!(claim.get_reward_amount(user1.valid_account_id()));
    assert_eq!(true, user1_reward_outcome.is_err());

    let outcome = call!(
        root,
        claim.push_reward(
            user1.valid_account_id(),
            U128::from(ptoy(10)),
            "".to_string()
        ),
        deposit = 1
    );

    // storage price for 1 account (along with 1 reward)
    // 1e19 per byte (https://github.com/near/nearcore/pull/3881)
    let claim_account = claim.account().unwrap();
    let storage_price_one_account = (claim_account.storage_usage - initial_storage_usage) as u128 * 10u128.pow(19);
    println!("[PUSH REWARD] Storage price for 1 account: {} NEAR", storage_price_one_account as f64 / 1e24);

    let current_storage_usage = claim_account.storage_usage;

    // gas price for adding 1 reward
    println!("[PUSH REWARD] Gas burnt for 1 account: {} TeraGas", outcome.gas_burnt() as f64 / 1e12);
    //println!("{:#?}", outcome.promise_results());

    let outcome = call!(
        root,
        claim.push_reward(
            user1.valid_account_id(),
            U128::from(ptoy(10)),
            "".to_string()
        ),
        deposit = 1
    );

    let claim_account = claim.account().unwrap();
    let storage_price_one_account = (claim_account.storage_usage - current_storage_usage) as u128 * 10u128.pow(19);
    println!("[PUSH REWARD] Storage price for adding reward: {} NEAR", storage_price_one_account as f64 / 1e24);
    println!("[PUSH REWARD] Gas burnt for adding reward: {} TeraGas ", outcome.gas_burnt() as f64 / 1e12);

    // assert reward
    let user1_reward: U128 = view!(claim.get_reward_amount(user1.valid_account_id())).unwrap_json();
    assert_eq!(user1_reward, U128::from(ptoy(20)));

}

#[test]
fn simulate_claim_reward_full() {
    let (root, ft, claim, _, user1) = init();

    // Deposit amount first

    root.call(
        ft.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": claim.valid_account_id(),
            "amount": U128::from(ptoy(10_000_000)),
            "msg": "".to_string(),
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        1
    );

    //push reward to user1
    call!(
        root,
        claim.push_reward(
            user1.valid_account_id(),
            U128::from(ptoy(10)),
            "".to_string()
        ),
        deposit = 1
    );
    // user1 look at his reward

    let user1_reward: U128 = view!(claim.get_reward_amount(user1.valid_account_id())).unwrap_json();
    assert_eq!(user1_reward, U128::from(ptoy(10)));

    let user1_balance_before: U128 = root.view(
        ft.account_id(),
        "ft_balance_of",
        &json!({
            "account_id": user1.account_id(),
        }).to_string().into_bytes()
    )
    .unwrap_json();

    // user1 claim all reward

    let outcome = call!(
        user1,
        claim.claim_reward(user1_reward),
        deposit = 1
    );


    println!("[CLAIM REWARD] Gas burnt for claim reward: {} TeraGas ", outcome.gas_burnt() as f64 / 1e12);

    // assert user1 reward is 0
    let user1_reward: U128 = view!(claim.get_reward_amount(user1.valid_account_id())).unwrap_json();
    assert_eq!(user1_reward, U128::from(ptoy(0)));

    // assert user1 reward is sent to user1
    let user1_balance: U128 = root.view(
        ft.account_id(),
        "ft_balance_of",
        &json!({
            "account_id": user1.account_id(),
        }).to_string().into_bytes()
    )
    .unwrap_json();

    let user1_balance: u128 = user1_balance.into();
    let user1_balance_before: u128 = user1_balance_before.into();
    assert_eq!(user1_balance - user1_balance_before, ptoy(10));
}

// NEGATIVE
#[test]
fn simulate_push_reward_invalid_account() {
    let (root, ft, claim, _, user1) = init();

    // Deposit amount first

    root.call(
        ft.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": claim.valid_account_id(),
            "amount": U128::from(ptoy(10_000_000)),
            "msg": "".to_string(),
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        1
    );

    let outcome = call!(
        user1,
        claim.push_reward(
            user1.valid_account_id(),
            U128::from(ptoy(10)),
            "".to_string()
        ),
        deposit = 1
    );
    
    assert_eq!(outcome.promise_errors().len(), 1);

    assert!(format!("{:?}", outcome.promise_errors().remove(0))
        .contains("assertion failed: `(left == right)"));

}

#[test]
fn simulate_claim_to_non_registered_user() {
    let (root, ft, claim, alice, _) = init();

    // Deposit amount first

    root.call(
        ft.account_id(),
        "ft_transfer_call",
        &json!({
            "receiver_id": claim.valid_account_id(),
            "amount": U128::from(ptoy(10_000_000)),
            "msg": "".to_string(),
        }).to_string().into_bytes(),
        DEFAULT_GAS,
        1
    );

    //push reward to user1
    call!(
        root,
        claim.push_reward(
            alice.valid_account_id(),
            U128::from(ptoy(10)),
            "".to_string()
        ),
        deposit = 1
    );
    // alice look at his reward

    let alice_reward: U128 = view!(claim.get_reward_amount(alice.valid_account_id())).unwrap_json();
    assert_eq!(alice_reward, U128::from(ptoy(10)));

    let alice_balance_before: U128 = root.view(
        ft.account_id(),
        "ft_balance_of",
        &json!({
            "account_id": alice.account_id(),
        }).to_string().into_bytes()
    )
    .unwrap_json();

    // user1 claim all reward

    let outcome = call!(
        alice,
        claim.claim_reward(alice_reward),
        deposit = 1
    );

    println!("[CLAIM REWARD FAIL] Gas burnt for failed claim reward: {} TeraGas ", outcome.gas_burnt() as f64 / 1e12);

    // assert alice reward is 0 (FAIL still reduce amount)
    let user1_reward: U128 = view!(claim.get_reward_amount(alice.valid_account_id())).unwrap_json();
    assert_eq!(user1_reward, U128::from(ptoy(0)));

    // assert alice reward is not sent to alice
    let alice_balance: U128 = root.view(
        ft.account_id(),
        "ft_balance_of",
        &json!({
            "account_id": alice.account_id(),
        }).to_string().into_bytes()
    )
    .unwrap_json();

    let alice_balance: u128 = alice_balance.into();
    let alice_balance_before: u128 = alice_balance_before.into();
    assert_eq!(alice_balance, alice_balance_before);

}