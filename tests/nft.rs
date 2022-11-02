mod setup;

use crate::setup::*;
use contract::BigDecimal;
use near_sdk::json_types::U128;
use near_sdk_sim::transaction::ExecutionStatus;

#[test]
fn test_deposit_nft() {
    let (e, _, users) = basic_setup();

    e.mint_nft(&users.alice, "1".to_string());
    e.mint_nft(&users.alice, "2".to_string());

    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "2".to_string())
        .assert_success();

    let account = e.get_account(&users.alice);
    assert_eq!(
        account.nft_supplied[0].nft_contract_id,
        e.nft_contract.account_id()
    );
    assert_eq!(account.nft_supplied[0].nft_token_id, "2".to_string());
}

#[test]
fn test_deposit_nft_fail() {
    let (e, _, users) = basic_setup();

    e.mint_nft(&users.alice, "1".to_string());
    e.mint_nft(&users.bob, "2".to_string());

    let res =
        e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "2".to_string());

    let err = match res.status() {
        ExecutionStatus::Failure(e) => e.to_string(),
        _ => panic!("Should fail with error"),
    };
    assert!(err.contains("Sender must be the token owner"));
}

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
fn test_withdraw_nft_not_in_pool() {
    let (e, tokens, users) = basic_setup();

    e.mint_nft(&users.alice, "1".to_string());
    e.mint_nft(&users.alice, "2".to_string());

    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "2".to_string())
        .assert_success();

    let res = e.withdraw_nft(
        &users.alice,
        price_data(&tokens, None, None, Some(100000)),
        e.nft_contract.account_id(),
        "1".to_string(),
    );

    let err = match res.status() {
        ExecutionStatus::Failure(e) => e.to_string(),
        _ => panic!("Should fail with error"),
    };
    assert!(err.contains("NFT not found in the NFT pool"));
}

#[test]
fn test_withdraw_nft_not_owner() {
    let (e, tokens, users) = basic_setup();

    e.mint_nft(&users.alice, "1".to_string());
    e.mint_nft(&users.bob, "2".to_string());

    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "1".to_string())
        .assert_success();
    e.supply_nft_to_collateral(&users.bob, e.nft_contract.account_id(), "2".to_string())
    .assert_success();

    let res = e.withdraw_nft(
        &users.alice,
        price_data(&tokens, None, None, Some(100000)),
        e.nft_contract.account_id(),
        "2".to_string(),
    );

    let err = match res.status() {
        ExecutionStatus::Failure(e) => e.to_string(),
        _ => panic!("Should fail with error"),
    };
    assert!(err.contains("You are not authorized. You must be using the owner account"));
}

#[test]
fn test_withdraw_nft_fail_health_factor() {
    let (e, tokens, users) = basic_setup();

    // Supply 10 Near at $10
    let supply_amount = d(10, 24);
    e.supply_to_collateral(&users.alice, &tokens.wnear, supply_amount)
        .assert_success();

    // Supply 1 NFT ($30)
    e.mint_nft(&users.alice, "1".to_string());
    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "1".to_string())
        .assert_success();

    let account = e.get_account(&users.alice);
    assert_eq!(
        account.nft_supplied[0].nft_contract_id,
        e.nft_contract.account_id()
    );
    assert_eq!(account.nft_supplied[0].nft_token_id, "1".to_string());

    // Borrowe 60 DAI
    let borrow_amount = d(60, 18);
    let res = e.borrow_and_withdraw(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, Some(100000), None, Some(300000)),
        borrow_amount,
    );
    res.assert_success();
    // println!("{:#?}", res.logs());

    let result = e.withdraw_nft(
        &users.alice,
        price_data(&tokens, Some(100000), None, Some(300000)),
        e.nft_contract.account_id(),
        "1".to_string(),
    );

    let err = match result.status() {
        ExecutionStatus::Failure(e) => e.to_string(),
        _ => panic!("Should fail with health error"),
    };
    assert!(err.contains("self.compute_max_discount(account, &prices) == BigDecimal::zero()"));

    let account = e.get_account(&users.alice);
    assert_eq!(
        account.nft_supplied[0].nft_contract_id,
        e.nft_contract.account_id()
    );
    assert_eq!(account.nft_supplied[0].nft_token_id, "1".to_string());
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
/// Bob liquidates Alice and makes nice profit (NFT).
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
        &[av(tokens.wnear.account_id(), bobs_amount - wnear_amount_in)],
    );
    assert!(find_asset(&account.supplied, &tokens.wnear.account_id()).apr > BigDecimal::zero());
    assert_eq!(
        account.nft_supplied[0].nft_contract_id,
        e.nft_contract.account_id()
    );
    assert_eq!(account.nft_supplied[0].nft_token_id, "1".to_string());
}

/// Alice puts 1000 USDC and 1 NFT ($30) and borrows 50 NEAR at 10$, 50 USDT. Prices go up. REKT
/// Bob liquidates Alice but doesn't meet requirement.
#[test]
fn test_liquidation_nft_decrease_health_factor() {
    let (e, tokens, users) = basic_setup();

    // Change asset config Near volatility_ratio = 95%
    e.update_asset(
        tokens.wnear.account_id(),
        AssetConfig {
            reserve_ratio: 2500,
            target_utilization: 8000,
            target_utilization_rate: U128(1000000000003593629036885046),
            max_utilization_rate: U128(1000000000039724853136740579),
            volatility_ratio: 9500, // Change to 95%
            extra_decimals: 0,
            can_deposit: true,
            can_withdraw: true,
            can_use_as_collateral: true,
            can_borrow: true,
        },
    );

    // Change asset config NFT volatility_ratio = 95%
    e.update_asset(
        e.nft_contract.account_id(),
        AssetConfig {
            reserve_ratio: 2500,
            target_utilization: 8000,
            target_utilization_rate: U128(0),
            max_utilization_rate: U128(0),
            volatility_ratio: 9500, // Change to 95%
            extra_decimals: 0,
            can_deposit: true,
            can_withdraw: true,
            can_use_as_collateral: true,
            can_borrow: false,
        },
    );

    let extra_decimals_mult = d(1, 12);

    // Alice: deposit 1000 usdc
    let supply_amount = d(1000, 18);
    e.supply_to_collateral(
        &users.alice,
        &tokens.nusdc,
        supply_amount / extra_decimals_mult,
    )
    .assert_success();

    // Alice: deposit 1 NFT ($30)
    e.mint_nft(&users.alice, "1".to_string());
    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "1".to_string())
        .assert_success();

    // Alice: Borrow 50 Near at $10
    let wnear_borrow_amount = d(50, 24);
    e.borrow_and_withdraw(
        &users.alice,
        &tokens.wnear,
        price_data(&tokens, Some(100000), None, Some(300000)),
        wnear_borrow_amount,
    )
    .assert_success();

    // Alice: Borrow 50 USDT
    let usdt_borrow_amount = d(50, 18);
    e.borrow_and_withdraw(
        &users.alice,
        &tokens.nusdt,
        price_data(&tokens, Some(100000), None, Some(300000)),
        usdt_borrow_amount,
    )
    .assert_success();

    // Check Alice account: Supplied: 1000 USDC
    let account = e.get_account(&users.alice);
    assert_balances(
        &account.supplied,
        &[av(tokens.nusdc.account_id(), supply_amount)],
    );
    // Check Alice account: Borrowed: 50 NEAR, 50 USDT
    assert_balances(
        &account.borrowed,
        &[
            av(tokens.wnear.account_id(), wnear_borrow_amount),
            av(tokens.nusdt.account_id(), usdt_borrow_amount),
        ],
    );
    assert!(find_asset(&account.borrowed, &tokens.wnear.account_id()).apr > BigDecimal::zero());
    assert!(find_asset(&account.borrowed, &tokens.nusdt.account_id()).apr > BigDecimal::zero());

    // Bod: Has 100 NEAR
    let wnear_bobs_amount = d(100, 24);
    e.contract_ft_transfer_call(&tokens.wnear, &users.bob, wnear_bobs_amount, "")
        .assert_success();

    // Bod: Has 100 USDT
    let usdt_bobs_amount = d(100, 18);
    e.contract_ft_transfer_call(
        &tokens.nusdt,
        &users.bob,
        usdt_bobs_amount / extra_decimals_mult,
        "",
    )
    .assert_success();

    // Check Bob account: Supplied: 100 NEAR, 100 USDT
    let account = e.get_account(&users.bob);
    assert_balances(
        &account.supplied,
        &[
            av(tokens.wnear.account_id(), wnear_bobs_amount),
            av(tokens.nusdt.account_id(), usdt_bobs_amount),
        ],
    );
    assert!(find_asset(&account.supplied, &tokens.wnear.account_id()).apr > BigDecimal::zero());
    assert!(find_asset(&account.supplied, &tokens.nusdt.account_id()).apr > BigDecimal::zero());

    // Assuming 5% discount for 1.5 NEAR at 20$.
    // let wnear_amount_in = d(15, 23);
    // let nft_out = d(30, 24);
    let wnear_amount_in = d(15, 23); // 1.5 Near
    let res = e.liquidate_nft(
        &users.bob,
        &users.alice,
        price_data(&tokens, Some(200000), None, Some(300000)),
        vec![asset_amount(&tokens.wnear, wnear_amount_in)],
        vec![nft_asset(e.nft_contract.account_id(), "1".to_string())],
    );
    // println!("{:#?}", res.logs());

    let err = match res.status() {
        ExecutionStatus::Failure(e) => e.to_string(),
        _ => panic!("Should fail with liquidation health error"),
    };
    assert!(err.contains("The health factor of liquidation account can't decrease."));

    // Alice
    let account = e.get_account(&users.alice);
    assert_eq!(account.nft_supplied.len(), 1);
    assert_balances(
        &account.borrowed,
        &[
            av(tokens.wnear.account_id(), wnear_borrow_amount),
            av(tokens.nusdt.account_id(), usdt_borrow_amount),
        ],
    );
    assert!(find_asset(&account.borrowed, &tokens.wnear.account_id()).apr > BigDecimal::zero());

    // Bob
    let account = e.get_account(&users.bob);
    assert_balances(
        &account.supplied,
        &[
            av(tokens.wnear.account_id(), wnear_bobs_amount),
            av(tokens.nusdt.account_id(), usdt_bobs_amount),
        ],
    );

    assert_eq!(account.nft_supplied.len(), 0);
}
