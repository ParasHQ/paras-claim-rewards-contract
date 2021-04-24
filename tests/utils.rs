use paras_claim_rewards_contract::ContractContract as ClaimContract;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT,
};

// Load in contract bytes at runtime
near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    FT_WASM_BYTES => "res/fungible_token.wasm",
    CLAIM_WASM_BYTES => "res/paras_claim_rewards_contract.wasm",
}

pub const FT_ID: &str = "ft";
pub const CLAIM_ID: &str = "claim";
pub const USER1_ID: &str = "user1user1user1user1user1user1user1user1user1user1user1user1user";

/// PARAS to yoctoPARAS
pub fn ptoy(paras_amount: u128) -> u128 {
    paras_amount * 10u128.pow(24)
}

pub fn register_user(user: &near_sdk_sim::UserAccount) {
    user.call(
        FT_ID.to_string(),
        "storage_deposit",
        &json!({
            "account_id": user.valid_account_id()
        })
        .to_string()
        .into_bytes(),
        near_sdk_sim::DEFAULT_GAS / 2,
        near_sdk::env::storage_byte_cost() * 125, // attached deposit
    )
    .assert_success();
}

pub fn init() -> (UserAccount, UserAccount, ContractAccount<ClaimContract>, UserAccount, UserAccount) {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);

    let ft = root.deploy(
        &FT_WASM_BYTES,
        FT_ID.to_string(),
        STORAGE_AMOUNT, // attached deposit
    );
    ft.call(
        FT_ID.into(), 
        "new_default_meta",
        &json!({
            "owner_id": root.valid_account_id(),
            "total_supply": U128::from(ptoy(100_000_000)),
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS / 2,
        0,
    )
    .assert_success();

    let claim = deploy!(
        contract: ClaimContract,
        contract_id: CLAIM_ID,
        bytes: &CLAIM_WASM_BYTES,
        signer_account: root,
        init_method: new(
            root.valid_account_id(),
            ft.valid_account_id()
        )
    );

    register_user(&claim.user_account);

    let alice = root.create_user(
        "alice".to_string(),
        to_yocto("100") // initial balance
    );

    let user1 = root.create_user(
        USER1_ID.into(),
        to_yocto("100")
    );
    register_user(&user1);

    (root, ft, claim, alice, user1)
}