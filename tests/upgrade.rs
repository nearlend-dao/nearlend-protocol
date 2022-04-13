mod setup;

use crate::setup::*;
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> c3b16a5 (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
>>>>>>> bb5561c (Fix farm claim all, add potential farms into the account view, xBooster token)
use contract::{AccountFarmView, AssetView};
use near_sdk::serde::Deserialize;
use near_sdk::AccountId;

#[test]
fn test_upgrade() {
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_2_0_wasm_bytes());
=======
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_1_1_wasm_bytes());
>>>>>>> c2e1d85 (Addressing minor issues. Introducting state migration for upgrades)
=======
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_2_0_wasm_bytes());
>>>>>>> 899b7fd (Fix farm claim all, add potential farms into the account view, xBooster token)
=======

#[test]
fn test_upgrade() {
<<<<<<< HEAD
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_1_0_wasm_bytes());
>>>>>>> d7df3ed (Add contract upgrade integration test)
=======
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_1_1_wasm_bytes());
>>>>>>> 9f1cff0 (Addressing minor issues. Introducting state migration for upgrades)
=======
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_2_0_wasm_bytes());
>>>>>>> c3b16a5 (Fix farm claim all, add potential farms into the account view, xBooster token)
=======

const LATEST_VERSION: &'static str = "0.4.0";

#[test]
fn test_version() {
    let (e, _tokens, _users) = basic_setup();

    let version: String = e
        .near
        .view_method_call(e.contract.contract.get_version())
        .unwrap_json();

    assert_eq!(version, LATEST_VERSION);
}

#[test]
fn test_upgrade_with_private_key() {
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_3_0_wasm_bytes());
=======

#[test]
fn test_upgrade() {
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_1_0_wasm_bytes());
>>>>>>> 775689c (Add contract upgrade integration test)
=======
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_1_1_wasm_bytes());
>>>>>>> c2e1d85 (Addressing minor issues. Introducting state migration for upgrades)

    let amount = d(100, 24);
    e.contract_ft_transfer_call(&tokens.wnear, &users.alice, amount, "")
        .assert_success();

    let asset = e.get_asset(&tokens.wnear);
    assert_eq!(asset.supplied.balance, amount);

<<<<<<< HEAD
    // The version is not available
    assert!(e
        .near
        .view_method_call(e.contract.contract.get_version())
        .is_err());

    e.redeploy_latest_by_key();

    let asset = e.get_asset(&tokens.wnear);
    assert_eq!(asset.supplied.balance, amount);

    let version: String = e
        .near
        .view_method_call(e.contract.contract.get_version())
        .unwrap_json();

    assert_eq!(version, LATEST_VERSION);
}

#[test]
fn test_upgrade_by_owner() {
    let (e, tokens, users) = basic_setup_with_contract(burrowland_wasm_bytes());
>>>>>>> b9665e0 (Add remote upgrade functionality by owner)
=======
    let (e, tokens, users) = basic_setup_with_contract(burrowland_0_2_0_wasm_bytes());
>>>>>>> bb5561c (Fix farm claim all, add potential farms into the account view, xBooster token)

    let amount = d(100, 24);
    e.contract_ft_transfer_call(&tokens.wnear, &users.alice, amount, "")
        .assert_success();

    let asset = e.get_asset(&tokens.wnear);
    assert_eq!(asset.supplied.balance, amount);

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> c3b16a5 (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
>>>>>>> bb5561c (Fix farm claim all, add potential farms into the account view, xBooster token)
    #[derive(Debug, Deserialize)]
    #[serde(crate = "near_sdk::serde")]
    pub struct AccountDetailedViewV020 {
        pub account_id: AccountId,
        pub supplied: Vec<AssetView>,
        pub collateral: Vec<AssetView>,
        pub borrowed: Vec<AssetView>,
        pub farms: Vec<AccountFarmView>,
    }

    let account: Option<AccountDetailedViewV020> = e
<<<<<<< HEAD
=======
    let version: String = e
>>>>>>> b9665e0 (Add remote upgrade functionality by owner)
        .near
        .view_method_call(e.contract.contract.get_version())
        .unwrap_json();

<<<<<<< HEAD
<<<<<<< HEAD
=======
    let account = e.get_account(&users.alice);
>>>>>>> d7df3ed (Add contract upgrade integration test)
=======
>>>>>>> c3b16a5 (Fix farm claim all, add potential farms into the account view, xBooster token)
    assert_eq!(account.supplied[0].balance, amount);
    assert_eq!(account.supplied[0].token_id, tokens.wnear.account_id());
=======
    assert_eq!(version, LATEST_VERSION);

    e.deploy_contract_by_owner(burrowland_0_4_0_fake_wasm_bytes())
        .assert_success();
>>>>>>> b9665e0 (Add remote upgrade functionality by owner)
=======
    let account = e.get_account(&users.alice);
=======
        .near
        .view_method_call(e.contract.contract.get_account(users.alice.account_id()))
        .unwrap_json();
    let account = account.unwrap();

>>>>>>> bb5561c (Fix farm claim all, add potential farms into the account view, xBooster token)
    assert_eq!(account.supplied[0].balance, amount);
    assert_eq!(account.supplied[0].token_id, tokens.wnear.account_id());

    e.redeploy_latest();
>>>>>>> 775689c (Add contract upgrade integration test)

    let asset = e.get_asset(&tokens.wnear);
    assert_eq!(asset.supplied.balance, amount);

<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 9f1cff0 (Addressing minor issues. Introducting state migration for upgrades)
    let config: Config = e
=======
    let version: String = e
>>>>>>> b9665e0 (Add remote upgrade functionality by owner)
        .near
        .view_method_call(e.contract.contract.get_version())
        .unwrap_json();

    assert_eq!(version, "0.4.0-fake");

<<<<<<< HEAD
    e.deploy_contract_by_owner(burrowland_wasm_bytes())
        .assert_success();

<<<<<<< HEAD
=======
>>>>>>> d7df3ed (Add contract upgrade integration test)
=======
>>>>>>> 9f1cff0 (Addressing minor issues. Introducting state migration for upgrades)
=======
    let config: Config = e
        .near
        .view_method_call(e.contract.contract.get_config())
        .unwrap_json();
    assert_eq!(config.max_num_assets, 10);
    assert_eq!(config.maximum_recency_duration_sec, 90);
    assert_eq!(config.maximum_staleness_duration_sec, 15);

>>>>>>> c2e1d85 (Addressing minor issues. Introducting state migration for upgrades)
    let asset = e.get_asset(&tokens.wnear);
    assert_eq!(asset.supplied.balance, amount);

    let version: String = e
        .near
        .view_method_call(e.contract.contract.get_version())
        .unwrap_json();

    assert_eq!(version, LATEST_VERSION);
}

#[test]
fn test_degrade_fails() {
    let (e, _tokens, _users) = basic_setup();

    assert!(!e
        .deploy_contract_by_owner(burrowland_0_3_0_wasm_bytes())
        .is_ok());

    let version: String = e
        .near
        .view_method_call(e.contract.contract.get_version())
        .unwrap_json();

    assert_eq!(version, LATEST_VERSION);
=======
    let account = e.get_account(&users.alice);
    assert_eq!(account.supplied[0].balance, amount);
>>>>>>> 775689c (Add contract upgrade integration test)
}
