mod setup;

use contract::BigDecimal;

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

    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "2".to_string())
        .assert_success();

    let result = e.withdraw_nft(
        &users.alice,
        price_data(&tokens, None, None, Some(100000)),
        e.nft_contract.account_id(),
        "1".to_string(),
    );

    assert!(!result.is_ok())
}

#[test]
fn test_deposit_nft_borrow_ft() {
    let (e, tokens, users) = basic_setup();

    e.mint_nft(&users.alice, "1".to_string());
    e.mint_nft(&users.alice, "2".to_string());

    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "1".to_string())
        .assert_success();

    let borrow_amount = d(200, 18);
    e.borrow(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, Some(100000), None, Some(1000000)),
        borrow_amount,
    )
    .assert_success();

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.borrowed.balance, borrow_amount);
    assert!(asset.borrow_apr > BigDecimal::zero());
    assert_eq!(asset.supplied.balance, borrow_amount);
    assert!(asset.supply_apr > BigDecimal::zero());

    let account = e.get_account(&users.alice);
    assert_eq!(account.supplied[0].balance, borrow_amount);
    assert_eq!(account.supplied[0].token_id, tokens.ndai.account_id());
    assert!(account.supplied[0].apr > BigDecimal::zero());

    assert_eq!(
        account.nft_supplied[0].nft_contract_id,
        e.nft_contract.account_id()
    );
    assert_eq!(account.nft_supplied[0].nft_token_id, "1".to_string());

    assert_eq!(account.borrowed[0].balance, borrow_amount);
    assert_eq!(account.borrowed[0].token_id, tokens.ndai.account_id());
    assert!(account.borrowed[0].apr > BigDecimal::zero());
}

#[test]
fn test_borrow_and_withdraw_nft() {
    let (e, tokens, users) = basic_setup();

    e.mint_nft(&users.alice, "1".to_string());
    e.mint_nft(&users.alice, "2".to_string());

    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "1".to_string())
        .assert_success();

    let supply_amount = d(100, 24);
    e.supply_to_collateral(&users.alice, &tokens.wnear, supply_amount)
        .assert_success();

    let account = e.get_account(&users.alice);
    assert_eq!(
        account.nft_supplied[0].nft_contract_id,
        e.nft_contract.account_id()
    );
    assert_eq!(account.nft_supplied[0].nft_token_id, "1".to_string());

    let asset = e.get_asset(&e.nft_contract);
    assert_eq!(asset.nft_supplied[0].owner_id, users.alice.account_id());
    assert_eq!(asset.nft_supplied[0].token_id, "1".to_string());

    let borrow_amount = d(200, 18);
    e.borrow_and_withdraw_nft(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, Some(100000), None, Some(1000000)),
        borrow_amount,
        e.nft_contract.account_id(),
        "1".to_string(),
    )
    .assert_success();

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.borrowed.balance, borrow_amount);
    assert!(asset.borrow_apr > BigDecimal::zero());
    assert_eq!(asset.supplied.balance, 0);
    assert_eq!(asset.supply_apr, BigDecimal::zero());

    let account = e.get_account(&users.alice);
    assert_eq!(account.nft_supplied.len(), 0);
    assert_eq!(account.borrowed[0].balance, borrow_amount);
    assert_eq!(account.borrowed[0].token_id, tokens.ndai.account_id());
    assert!(account.borrowed[0].apr > BigDecimal::zero());
}
