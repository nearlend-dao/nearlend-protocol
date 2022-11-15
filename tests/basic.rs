mod setup;

use crate::setup::*;

use contract::{BigDecimal, MS_PER_YEAR};
use near_sdk::{serde_json::json, json_types::U128};

const SEC_PER_YEAR: u32 = (MS_PER_YEAR / 1000) as u32;

#[macro_use]
extern crate approx;

#[test]
fn test_init_env() {
    let e = Env::init();
    let _tokens = Tokens::init(&e);
    let _users = Users::init(&e);
}

#[test]
fn test_mint_tokens() {
    let e = Env::init();
    let tokens = Tokens::init(&e);
    let users = Users::init(&e);
    e.mint_tokens(&tokens, &users.alice);
}

#[test]
fn test_dev_setup() {
    let e = Env::init();
    let tokens = Tokens::init(&e);
    e.setup_assets(&tokens);
    e.deposit_reserves(&tokens);

    let asset = e.get_asset(&tokens.wnear);
    assert_eq!(asset.reserved, d(10000, 24));
}

#[test]
fn test_supply() {
    let (e, tokens, users) = basic_setup();

    let amount = d(100, 24);
    e.contract_ft_transfer_call(&tokens.wnear, &users.alice, amount, "")
        .assert_success();

    let asset = e.get_asset(&tokens.wnear);
    assert_eq!(asset.supplied.balance, amount);

    let account = e.get_account(&users.alice);
    assert_eq!(account.supplied[0].balance, amount);
    assert_eq!(account.supplied[0].token_id, tokens.wnear.account_id());
}

#[test]
fn test_deposit() {
    let (e, tokens, users) = basic_setup();

    let supply_amount = d(100, 24);   
    //supply collateral 
    e.supply_to_collateral(&users.alice, &tokens.wnear, supply_amount)
        .assert_success();
    // view asset
    let asset = e.get_asset(&tokens.wnear);
    assert_eq!(asset.supplied.balance,supply_amount.clone());
}

#[test]
fn test_deposit_greate_than_the_balance() {
    let (e, tokens, users) = basic_setup();

    let supply_amount = d(10000000, 24);
    // supply  collateral greater  than the balance account 
    e.supply_to_collateral(&users.alice, &tokens.wnear, supply_amount);

    // view asset, deposit fail
    let asset = e.get_asset(&tokens.wnear);
    assert_eq!(asset.supplied.balance,0);
    let balance: U128 =  e.get_balance(&tokens.wnear, &users.alice);    
    println!("balance : {:?}" , balance);
    println!("supply amount: {}" , supply_amount);

}





#[test]
fn test_borrow() {
    let (e, tokens, users) = basic_setup();
    // alice deposit wnear
    let supply_amount = d(100, 24);
    e.supply_to_collateral(&users.alice, &tokens.wnear, supply_amount)
        .assert_success();

    // alice borrow dai
    let borrow_amount = d(200, 18);
    e.borrow(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, Some(100000), None),
        borrow_amount,
    )
    .assert_success();
    // view asset
    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.borrowed.balance, borrow_amount);
    assert!(asset.borrow_apr > BigDecimal::zero());
    assert_eq!(asset.supplied.balance, borrow_amount);
    assert!(asset.supply_apr > BigDecimal::zero());

    // view account alice
    let account = e.get_account(&users.alice);
    assert_eq!(account.supplied[1].balance, borrow_amount);
    assert_eq!(account.supplied[1].token_id, tokens.ndai.account_id());
    assert!(account.supplied[1].apr > BigDecimal::zero());
    assert_eq!(account.borrowed[0].balance, borrow_amount);
    assert_eq!(account.borrowed[0].token_id, tokens.ndai.account_id());
    assert!(account.borrowed[0].apr > BigDecimal::zero());
}

#[test]
fn test_borrow_with_price_data_none() {
    // borrow when oracle price hasn't updated price yet
    let (e, tokens, users) = basic_setup();
    let supply_amount = d(100, 24);
    e.supply_to_collateral(&users.alice, &tokens.wnear, supply_amount)
        .assert_success();
    let borrow_amount = d(200, 18);

    e.borrow(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, None, None),
        borrow_amount,
    );
    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.borrowed.balance, 0);
}

#[test]
fn test_borrow_greater_than_collateral() {
    let (e, tokens, users) = basic_setup();

    let supply_amount = d(100, 24);
    let supply_amount1 = d(100000, 24);
    // alice deposit 
    e.supply_to_collateral(&users.alice, &tokens.wnear, supply_amount)
        .assert_success();
    // bob deposit
    e.supply_to_collateral(&users.bob, &tokens.wnear, supply_amount1)
        .assert_success();
    let borrow_amount = d(20000, 18);

    // Alice borrows more money than deposited into the pool many times
    e.borrow(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, Some(100000), None),
        borrow_amount,
    );
    // view asset
    let asset = e.get_asset(&tokens.ndai);
    println!("borrowed balance : {:?}" , asset.borrowed.balance);
    println!("borrow apr: {:?}", asset.borrow_apr);
    println!("supply apr: {:?}", asset.supply_apr);
    assert_eq!(asset.borrowed.balance, 0);
    
}

#[test]
fn test_borrow_and_withdraw() {
    let (e, tokens, users) = basic_setup();

    let supply_amount = d(100, 24);
    e.supply_to_collateral(&users.alice, &tokens.wnear, supply_amount)
        .assert_success();
        
    let borrow_amount = d(200, 18);
    e.borrow_and_withdraw(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, Some(100000), None),
        borrow_amount,
    )
    .assert_success();

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.borrowed.balance, borrow_amount);
    assert!(asset.borrow_apr > BigDecimal::zero());
    assert_eq!(asset.supplied.balance, 0);
    assert_eq!(asset.supply_apr, BigDecimal::zero());

    let account = e.get_account(&users.alice);
    assert_eq!(account.borrowed[0].balance, borrow_amount);
    assert_eq!(account.borrowed[0].token_id, tokens.ndai.account_id());
    assert!(account.borrowed[0].apr > BigDecimal::zero());
}

#[test]
fn test_interest() {
    let (e, tokens, users) = basic_setup();

    let supply_amount = d(10000, 24);
    e.supply_to_collateral(&users.alice, &tokens.wnear, supply_amount)
        .assert_success();

    let borrow_amount = d(8000, 18);
    e.borrow_and_withdraw(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, Some(100000), None),
        borrow_amount,
    )
    .assert_success();

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.borrowed.balance, borrow_amount);
    assert_relative_eq!(asset.borrow_apr.f64(), 0.08f64);

    e.skip_time(SEC_PER_YEAR);

    let expected_borrow_amount = borrow_amount * 108 / 100;

    let asset = e.get_asset(&tokens.ndai);
    assert_relative_eq!(asset.borrowed.balance as f64, expected_borrow_amount as f64);

    let account = e.get_account(&users.alice);
    assert_relative_eq!(
        account.borrowed[0].balance as f64,
        expected_borrow_amount as f64
    );
    assert_eq!(account.borrowed[0].token_id, tokens.ndai.account_id());
}
