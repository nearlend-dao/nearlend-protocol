mod setup;

use crate::setup::*;

#[test]
fn test_withdraw_nft() {
    let (e, tokens, users) = basic_setup();

    e.mint_nft(&users.alice, "1".to_string());
    e.mint_nft(&users.alice, "2".to_string());

    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "1".to_string())
        .assert_success();

    e.withdraw_nft(
        &users.alice,
        price_data(&tokens, None, None, Some(100000)),
        e.nft_contract.account_id(),
        "1".to_string(),
    )
    .assert_success();
}

#[test]
fn test_withdraw_nft_fail() {
    let (e, tokens, users) = basic_setup();

    e.mint_nft(&users.alice, "1".to_string());
    e.mint_nft(&users.alice, "2".to_string());

    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "3".to_string())
        .assert_success();

    let result = e.withdraw_nft(
        &users.alice,
        price_data(&tokens, None, None, Some(100000)),
        e.nft_contract.account_id(),
        "1".to_string(),
    );

    assert!(!result.is_ok())
}
