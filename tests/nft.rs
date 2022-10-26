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

/// Alice puts 1000 USDC and 1 NFT ($30) and borrows 50 NEAR at 10$. Prices go up. REKT
/// Bob liquidates Alice and makes nice profit.
#[test]
fn test_liquidation_nft_alice_by_bob() {
    let (e, tokens, users) = basic_setup();

    let extra_decimals_mult = d(1, 12);

    let supply_amount = d(1000, 18);
    e.supply_to_collateral(
        &users.alice,
        &tokens.nusdc,
        supply_amount / extra_decimals_mult,
    )
    .assert_success();

    e.mint_nft(&users.alice, "1".to_string());
    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "1".to_string())
        .assert_success();
    // Collateral sum = $1000 usdc * 95% + $30 nft * 30% = 950 + 9 = $959

    let borrow_amount = d(50, 24);
    e.borrow_and_withdraw(
        &users.alice,
        &tokens.wnear,
        price_data(&tokens, Some(100000), None, Some(300000)),
        borrow_amount,
    )
    .assert_success();
    // Borrow sum =(50 * $10) Near / 60% = $833.333333
    // Borrow sum =(50 * $12) Near / 60% = $1000

    let account = e.get_account(&users.alice);
    assert_balances(
        &account.supplied,
        &[av(tokens.nusdc.account_id(), supply_amount)],
    );
    assert_balances(
        &account.borrowed,
        &[av(tokens.wnear.account_id(), borrow_amount)],
    );
    assert!(find_asset(&account.borrowed, &tokens.wnear.account_id()).apr > BigDecimal::zero());

    // Bob has 100 NEAR
    let bobs_amount = d(100, 24);
    e.contract_ft_transfer_call(&tokens.wnear, &users.bob, bobs_amount, "")
        .assert_success();

    let account = e.get_account(&users.bob);
    assert_balances(
        &account.supplied,
        &[av(tokens.wnear.account_id(), bobs_amount)],
    );
    assert!(find_asset(&account.supplied, &tokens.wnear.account_id()).apr > BigDecimal::zero());

    // Liquidate NFT
    let wnear_amount_in = d(245, 22); // 2.45 Near
                                      // let wnear_amount_in = d(25, 23);

    // Assuming 2% discount for 2.45 NEAR at 12$.
    // let wnear_amount_in = d(245, 22);
    // let nft_out = d(30, 24);
    let res = e.liquidate_nft(
        &users.bob,
        &users.alice,
        price_data(&tokens, Some(120000), None, Some(300000)),
        vec![asset_amount(&tokens.wnear, wnear_amount_in)],
        vec![nft_asset(e.nft_contract.account_id(), "1".to_string())],
    );
    res.assert_success();
    // println!("{:#?}", res.logs());

    let account = e.get_account(&users.alice);
    assert_balances(
        &account.borrowed,
        &[av(
            tokens.wnear.account_id(),
            borrow_amount - wnear_amount_in,
        )],
    );
    assert!(find_asset(&account.borrowed, &tokens.wnear.account_id()).apr > BigDecimal::zero());
    assert_eq!(account.nft_supplied.len(), 0);

    let account = e.get_account(&users.bob);
    assert_balances(
        &account.supplied,
        &[
            av(tokens.wnear.account_id(), bobs_amount - wnear_amount_in),
        ],
    );
    assert!(find_asset(&account.supplied, &tokens.wnear.account_id()).apr > BigDecimal::zero());
    assert_eq!(
        account.nft_supplied[0].nft_contract_id,
        e.nft_contract.account_id()
    );
    assert_eq!(account.nft_supplied[0].nft_token_id, "1".to_string());
}
