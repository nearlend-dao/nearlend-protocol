mod setup;

use crate::setup::*;
use contract::FarmId;
use contract::*;

/// Test Rewward farm with only 1 user Alice Deposit:
/// 1. Add farm for Deposit DAI token with config:
/// - 100 NEL/day
/// - Total reward 3000 NEL
/// 2. Deposit 100 DAI to Pool
/// 3. Check reward amount for 3 days, 5 days and 30 days
#[test]
fn test_farm_supplied() {
    let (e, tokens, users) = basic_setup();

    println!("{:?}", e.booster_contract.user_account);
    println!("{:?}", tokens);
    println!("{:?}", users);

    let reward_per_day = d(100, BOOSTER_TOKEN_DECIMALS);
    let total_reward = d(3000, BOOSTER_TOKEN_DECIMALS);

    let farm_id = FarmId::Supplied(tokens.ndai.account_id());
    println!("==========> Farm Id: {:?}", farm_id);
    // add farm reward
    e.add_farm(
        farm_id.clone(),
        &e.booster_contract.user_account,
        reward_per_day,
        d(100, BOOSTER_TOKEN_DECIMALS),
        total_reward,
    );

    let asset = e.get_asset(&tokens.ndai);
    println!("=====> Assets Before deposit: {:?}", asset);
    assert_eq!(asset.farms.len(), 1);
    assert_eq!(asset.farms[0].farm_id, farm_id);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let amount = d(100, 18);

    // deposit 100 nDAI to the farm
    e.contract_ft_transfer_call(&tokens.ndai, &users.alice, amount, "")
        .assert_success();

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    println!("=====> Assets After deposit: {:?}", asset);

    assert_eq!(asset.supplied.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let account = e.get_account(&users.alice);
    assert_balances(&account.supplied, &[av(tokens.ndai.account_id(), amount)]);
    let nel_token_before = e.nel_balance_of(&users.alice).0;
    println!("===> Account before: {:?}", account);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].reward_token_id,
        e.booster_contract.user_account.account_id()
    );
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 3 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 3);

    let farmed_amount = reward_per_day * 3;

    println!("===> farmed_amount: {:?}", farmed_amount);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);
    assert_balances(&account.supplied, &[av(tokens.ndai.account_id(), amount)]);

    println!("===> Account before claim: {:?}", account);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, farmed_amount);

    // claim the reward
    e.account_farm_claim_all(&users.alice).assert_success();
    let nel_token_after_skip_3_days = e.nel_balance_of(&users.alice).0;

    let asset = e.get_asset(&e.booster_contract.user_account);
    println!("{:?}", asset);
    assert_eq!(
        nel_token_after_skip_3_days - nel_token_before,
        farmed_amount
    );
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);

    println!(
        "===> Account NEL token after claim: {:?}",
        &e.nel_balance_of(&users.alice)
    );
    assert_balances(
        &account.supplied,
        &[
            av(tokens.ndai.account_id(), amount),
            //av(e.booster_contract.user_account.account_id(), farmed_amount),
        ],
    );

    println!("===> Account after 3 days claim: {:?}", account);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 2 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 2);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - reward_per_day * 5
    );

    let account = e.get_account(&users.alice);
    assert_balances(&account.supplied, &[av(tokens.ndai.account_id(), amount)]);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        reward_per_day * 2
    );

    // next 30 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 25);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, 0);

    let account = e.get_account(&users.alice);
    assert_balances(&account.supplied, &[av(tokens.ndai.account_id(), amount)]);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        total_reward - farmed_amount
    );

    // Claim all rewards
    e.account_farm_claim_all(&users.alice).assert_success();
    let nel_token_after_skip_30_days = e.nel_balance_of(&users.alice).0;

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(
        nel_token_after_skip_30_days - nel_token_before,
        total_reward
    );
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, amount);
    assert!(asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .is_none());

    let account = e.get_account(&users.alice);
    println!("{:?}", account);
    assert_balances(&account.supplied, &[av(tokens.ndai.account_id(), amount)]);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert!(account.farms[0].rewards.is_empty());

    // next 3 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 3);

    // No reward because remaining_rewards = 0
    // Claim all rewards
    e.account_farm_claim_all(&users.alice).assert_success();
    let nel_token_after_skip_3_days = e.nel_balance_of(&users.alice).0;
    assert_eq!(
        nel_token_after_skip_30_days - nel_token_after_skip_3_days,
        0
    );
}

///  Test Rewward farm with only 2 users Alice & Bob Deposit:
///  1. Add farm for Deposit DAI token with config:
///  - 100 NEL/day
///  - Total reward 3000 NEL
///  2. Alice Deposit 100 DAI to Pool, Bob deposit 300 DAI to Pool
///  3. Check reward amount for 3 days, 5 days and 30 days
#[test]
fn test_farm_supplied_2() {
    let (e, tokens, users) = basic_setup();

    println!("{:?}", e.booster_contract.user_account);
    println!("{:?}", tokens);
    println!("{:?}", users);

    let reward_per_day = d(100, BOOSTER_TOKEN_DECIMALS);
    let total_reward = d(3000, BOOSTER_TOKEN_DECIMALS);

    let farm_id = FarmId::Supplied(tokens.ndai.account_id());
    println!("==========> Farm Id: {:?}", farm_id);
    // add farm reward
    e.add_farm(
        farm_id.clone(),
        &e.booster_contract.user_account,
        reward_per_day,
        d(100, BOOSTER_TOKEN_DECIMALS),
        total_reward,
    );

    let asset = e.get_asset(&tokens.ndai);
    println!("=====> Assets Before deposit: {:?}", asset);
    assert_eq!(asset.farms.len(), 1);
    assert_eq!(asset.farms[0].farm_id, farm_id);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let alice_amount = d(100, 18);
    let bob_amount = d(300, 18);

    let total_amount_deposit = alice_amount + bob_amount;
    let alice_ratio_reward = (alice_amount * 10_000 / total_amount_deposit) as u32;
    let bob_ratio_reward = (bob_amount * 10_000 / total_amount_deposit) as u32;

    // Alice deposit 100 nDAI to the farm
    e.contract_ft_transfer_call(&tokens.ndai, &users.alice, alice_amount, "")
        .assert_success();

    // Bob deposit 300 nDAI to the farm
    e.contract_ft_transfer_call(&tokens.ndai, &users.bob, bob_amount, "")
        .assert_success();

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    println!("=====> Assets After deposit: {:?}", asset);

    assert_eq!(asset.supplied.balance, total_amount_deposit);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let alice_account = e.get_account(&users.alice);
    let bob_account = e.get_account(&users.bob);
    assert_balances(
        &alice_account.supplied,
        &[av(tokens.ndai.account_id(), alice_amount)],
    );
    assert_balances(
        &bob_account.supplied,
        &[av(tokens.ndai.account_id(), bob_amount)],
    );

    let alice_nel_token_before = e.nel_balance_of(&users.alice).0;
    let bob_nel_token_before = e.nel_balance_of(&users.alice).0;
    println!("===> Alice Account before: {:?}", alice_account);

    assert_eq!(alice_account.farms[0].farm_id, farm_id);
    assert_eq!(
        alice_account.farms[0].rewards[0].reward_token_id,
        e.booster_contract.user_account.account_id()
    );
    assert_eq!(
        alice_account.farms[0].rewards[0].boosted_shares,
        find_asset(&alice_account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(alice_account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 3 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 3);

    let farmed_amount = reward_per_day * 3;

    println!("===> farmed_amount: {:?}", farmed_amount);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, total_amount_deposit);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let alice_account = e.get_account(&users.alice);
    let bob_account = e.get_account(&users.bob);
    assert_balances(
        &alice_account.supplied,
        &[av(tokens.ndai.account_id(), alice_amount)],
    );

    println!("===> Account before claim: {:?}", alice_account);

    assert_eq!(alice_account.farms[0].farm_id, farm_id);
    assert_eq!(
        alice_account.farms[0].rewards[0].boosted_shares,
        find_asset(&alice_account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(
        alice_account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(alice_ratio_reward))
    );

    assert_eq!(
        bob_account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(bob_ratio_reward))
    );

    // claim the reward
    e.account_farm_claim_all(&users.alice).assert_success();
    e.account_farm_claim_all(&users.bob).assert_success();
    let alice_nel_token_after_skip_3_days = e.nel_balance_of(&users.alice).0;
    let bob_nel_token_after_skip_3_days = e.nel_balance_of(&users.bob).0;

    let asset = e.get_asset(&e.booster_contract.user_account);
    println!("{:?}", asset);
    assert_eq!(
        alice_nel_token_after_skip_3_days - alice_nel_token_before,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(alice_ratio_reward))
    );
    assert_eq!(
        bob_nel_token_after_skip_3_days - bob_nel_token_before,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(bob_ratio_reward))
    );
    assert_eq!(asset.supplied.balance, 0);

    let alice_account = e.get_account(&users.alice);
    let bob_account = e.get_account(&users.bob);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, total_amount_deposit);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    assert_balances(
        &alice_account.supplied,
        &[av(tokens.ndai.account_id(), alice_amount)],
    );

    assert_eq!(alice_account.farms[0].farm_id, farm_id);
    assert_eq!(
        alice_account.farms[0].rewards[0].boosted_shares,
        find_asset(&alice_account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(alice_account.farms[0].rewards[0].unclaimed_amount, 0);
    assert_eq!(bob_account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 2 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 2);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, total_amount_deposit);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - reward_per_day * 5
    );

    let alice_account = e.get_account(&users.alice);
    let bob_account = e.get_account(&users.bob);

    assert_eq!(alice_account.farms[0].farm_id, farm_id);
    assert_eq!(
        alice_account.farms[0].rewards[0].boosted_shares,
        find_asset(&alice_account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(
        bob_account.farms[0].rewards[0].boosted_shares,
        find_asset(&bob_account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );

    assert_eq!(
        alice_account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(&BigDecimal::from(reward_per_day * 2).mul_ratio(alice_ratio_reward))
    );
    assert_eq!(
        bob_account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(&BigDecimal::from(reward_per_day * 2).mul_ratio(bob_ratio_reward))
    );

    // next 30 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 25);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, total_amount_deposit);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, 0);

    let alice_account = e.get_account(&users.alice);
    let bob_account = e.get_account(&users.bob);

    assert_eq!(
        alice_account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(
            &BigDecimal::from(total_reward - farmed_amount).mul_ratio(alice_ratio_reward)
        )
    );
    assert_eq!(
        bob_account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(
            &BigDecimal::from(total_reward - farmed_amount).mul_ratio(bob_ratio_reward)
        )
    );

    // Claim all rewards
    e.account_farm_claim_all(&users.alice).assert_success();
    e.account_farm_claim_all(&users.bob).assert_success();
    let alice_nel_token_after_skip_30_days = e.nel_balance_of(&users.alice).0;
    let bob_nel_token_after_skip_30_days = e.nel_balance_of(&users.bob).0;

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(
        alice_nel_token_after_skip_30_days - alice_nel_token_before,
        BigDecimal::round_u128(&BigDecimal::from(total_reward).mul_ratio(alice_ratio_reward))
    );
    assert_eq!(
        bob_nel_token_after_skip_30_days - bob_nel_token_before,
        BigDecimal::round_u128(&BigDecimal::from(total_reward).mul_ratio(bob_ratio_reward))
    );
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, total_amount_deposit);
    assert!(asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .is_none());

    let alice_account = e.get_account(&users.alice);
    let bob_account = e.get_account(&users.bob);

    assert_eq!(alice_account.farms[0].farm_id, farm_id);
    assert!(alice_account.farms[0].rewards.is_empty());
    assert_eq!(bob_account.farms[0].farm_id, farm_id);
    assert!(bob_account.farms[0].rewards.is_empty());

    // next 3 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 3);

    // No reward because remaining_rewards = 0
    // Claim all rewards
    e.account_farm_claim_all(&users.alice).assert_success();
    e.account_farm_claim_all(&users.bob).assert_success();
    let alice_nel_token_after_skip_3_days = e.nel_balance_of(&users.alice).0;
    let bob_nel_token_after_skip_3_days = e.nel_balance_of(&users.bob).0;
    assert_eq!(
        alice_nel_token_after_skip_30_days - alice_nel_token_after_skip_3_days,
        0
    );
    assert_eq!(
        bob_nel_token_after_skip_30_days - bob_nel_token_after_skip_3_days,
        0
    );
}

///  Test Reward farm Deposit with only 1 user Alice Deposit:
///  1. Add farm for Deposit USDT token with config:
///  - 100 NEL/day
///  - Total reward 3000 NEL
///  2. Deposit 1000 USDT to Pool
///  4. Check reward amount for 10 days,
///  5. Withdraw 500 USDT
///  6. Check reward 10 days next
#[test]
fn test_farm_supplied_3() {}

///  Test Reward farm Borrow with only 1 user Alice Deposit:
///  1. Add farm for Deposit DAI token with config:
///  - 100 NEL/day
///  - Total reward 3000 NEL
///  2. Deposit 1000 USDT to Pool
///  3. Borrow 100 DAI
///  4. Check reward amount for 3 days, 5 days and 30 days
#[test]
fn test_farm_borrowed() {
    let (e, tokens, users) = basic_setup();

    println!("{:?}", e.booster_contract.user_account);
    println!("{:?}", tokens);
    println!("{:?}", users);

    let reward_per_day = d(100, BOOSTER_TOKEN_DECIMALS);
    let total_reward = d(3000, BOOSTER_TOKEN_DECIMALS);

    let farm_id = FarmId::Supplied(tokens.ndai.account_id());
    println!("==========> Farm Id: {:?}", farm_id);
    // add farm reward
    e.add_farm(
        farm_id.clone(),
        &e.booster_contract.user_account,
        reward_per_day,
        d(100, BOOSTER_TOKEN_DECIMALS),
        total_reward,
    );

    let asset = e.get_asset(&tokens.ndai);
    println!("=====> Assets Before deposit: {:?}", asset);
    assert_eq!(asset.farms.len(), 1);
    assert_eq!(asset.farms[0].farm_id, farm_id);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let dai_amount = d(100, 18);
    let usdt_amount = d(1000, 6);
    let amount = d(100, 18);

    let ratio_reward_borrow = (amount * 10_000 / (dai_amount + amount)) as u32;

    // deposit 1000 nUSDT to the farm
    e.contract_ft_transfer_call(&tokens.nusdt, &users.alice, usdt_amount, "")
        .assert_success();

    // deposit 1000 nDAI to the farm
    e.contract_ft_transfer_call(&tokens.ndai, &users.bob, dai_amount, "")
        .assert_success();

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.borrowed.balance, 0);

    e.borrow(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, None, None, None),
        amount,
    );

    let asset = e.get_asset(&tokens.ndai);
    println!("=====> Assets After deposit: {:?}", asset);

    assert_eq!(asset.borrowed.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let account = e.get_account(&users.alice);
    assert_balances(&account.borrowed, &[av(tokens.ndai.account_id(), amount)]);
    let nel_token_before = e.nel_balance_of(&users.alice).0;
    println!("===> Account before: {:?}", account);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].reward_token_id,
        e.booster_contract.user_account.account_id()
    );
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 3 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 3);

    let farmed_amount = reward_per_day * 3;

    println!("===> farmed_amount: {:?}", farmed_amount);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(ratio_reward_borrow))
    );

    // claim the reward
    e.account_farm_claim_all(&users.alice).assert_success();
    let nel_token_after_skip_3_days = e.nel_balance_of(&users.alice).0;

    let asset = e.get_asset(&e.booster_contract.user_account);
    println!("{:?}", asset);
    assert_eq!(
        nel_token_after_skip_3_days - nel_token_before,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(ratio_reward_borrow))
    );
    assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    //assert_eq!(asset.borrowed.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);

    println!(
        "===> Account NEL token after claim: {:?}",
        &e.nel_balance_of(&users.alice)
    );
    //assert_balances(&account.borrowed, &[av(tokens.ndai.account_id(), amount)]);

    println!("===> Account after 3 days claim: {:?}", account);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 2 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 2);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    //assert_eq!(asset.borrowed.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - reward_per_day * 5
    );

    let account = e.get_account(&users.alice);
    //assert_balances(&account.borrowed, &[av(tokens.ndai.account_id(), amount)]);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(
            &BigDecimal::from(reward_per_day * 2).mul_ratio(ratio_reward_borrow)
        )
    );

    // next 30 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 25);

    let asset = e.get_asset(&tokens.ndai);
    //assert_eq!(asset.borrowed.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, 0);

    let account = e.get_account(&users.alice);
    //assert_balances(&account.borrowed, &[av(tokens.ndai.account_id(), amount)]);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(
            &BigDecimal::from(total_reward - farmed_amount).mul_ratio(ratio_reward_borrow)
        )
    );

    // Claim all rewards
    e.account_farm_claim_all(&users.alice).assert_success();
    let nel_token_after_skip_30_days = e.nel_balance_of(&users.alice).0;

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(
        nel_token_after_skip_30_days - nel_token_before,
        BigDecimal::round_u128(&BigDecimal::from(total_reward).mul_ratio(ratio_reward_borrow))
    );
    assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    //assert_eq!(asset.borrowed.balance, amount);
    assert!(asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .is_none());

    let account = e.get_account(&users.alice);
    println!("{:?}", account);
    //assert_balances(&account.borrowed, &[av(tokens.ndai.account_id(), amount)]);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert!(account.farms[0].rewards.is_empty());

    // next 3 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 3);

    // No reward because remaining_rewards = 0
    // Claim all rewards
    e.account_farm_claim_all(&users.alice).assert_success();
    let nel_token_after_skip_3_days = e.nel_balance_of(&users.alice).0;
    assert_eq!(
        nel_token_after_skip_30_days - nel_token_after_skip_3_days,
        0
    );
}

///  Test Reward farm Borrow with only 1 user Alice Deposit:
///  1. Add farm for Deposit USDT token with config:
///  - 100 NEL/day
///  - Total reward 3000 NEL
///  2. Alice Deposit 2000 USDT, Bob deposit 1000 nDAI to Pool
///  3. Alice Borrow 1000 DAI
///  4. Check reward of Alice amount for 10 days,
///  5. Alice Repay 500 DAI
///  6. Check reward Alice 10 days next
#[test]
fn test_farm_borrowed_2() {
    let (e, tokens, users) = basic_setup();

    let reward_per_day = d(100, BOOSTER_TOKEN_DECIMALS);
    let total_reward = d(3000, BOOSTER_TOKEN_DECIMALS);

    let farm_id = FarmId::Supplied(tokens.ndai.account_id());
    println!("==========> Farm Id: {:?}", farm_id);
    // add farm reward
    e.add_farm(
        farm_id.clone(),
        &e.booster_contract.user_account,
        reward_per_day,
        d(100, BOOSTER_TOKEN_DECIMALS),
        total_reward,
    );

    let asset = e.get_asset(&tokens.ndai);
    println!("=====> Assets Before deposit: {:?}", asset);
    assert_eq!(asset.farms.len(), 1);
    assert_eq!(asset.farms[0].farm_id, farm_id);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let dai_amount = d(1000, 18);
    let usdt_amount = d(2000, 6);
    let amount = d(1000, 18);

    let ratio_reward_borrow = (amount * 10_000 / (dai_amount + amount)) as u32;

    // deposit 2000 nUSDT to the farm
    e.contract_ft_transfer_call(&tokens.nusdt, &users.alice, usdt_amount, "")
        .assert_success();

    // deposit 1000 nDAI to the farm
    e.contract_ft_transfer_call(&tokens.ndai, &users.bob, dai_amount, "")
        .assert_success();

    let asset = e.get_asset(&tokens.ndai);
    println!("=====> Assets After deposit: {:?}", asset);
    println!("===> DAI supplied: {:?}", &asset.supplied.balance);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.borrowed.balance, 0);

    e.borrow(
        &users.alice,
        &tokens.ndai,
        price_data(&tokens, None, None, None),
        amount,
    );

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.borrowed.balance, amount);
    // e.withdraw(
    //     &users.alice,
    //     &tokens.ndai,
    //     price_data(&tokens, None, None, None),
    //     amount,
    // );
    // let asset = e.get_asset(&tokens.ndai);
    println!("===> DAI supplied: {:?}", &asset.supplied.balance);

    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let account = e.get_account(&users.alice);
    assert_balances(&account.borrowed, &[av(tokens.ndai.account_id(), amount)]);
    let nel_token_before = e.nel_balance_of(&users.alice).0;
    println!("===> Account before: {:?}", account);

    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 10 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 10);

    let farmed_amount = reward_per_day * 10;

    println!("===> farmed_amount: {:?}", farmed_amount);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(ratio_reward_borrow))
    );

    //let asset = e.get_asset(&tokens.ndai);
    // println!("===> DAI borrowed: {:?}", &asset.borrowed.balance);
    // println!("===> DAI supplied: {:?}", &asset.supplied.balance);
    // claim the reward
    e.account_farm_claim_all(&users.alice).assert_success();
    let nel_token_after_skip_10_days = e.nel_balance_of(&users.alice).0;

    let asset = e.get_asset(&e.booster_contract.user_account);
    println!("{:?}", asset);
    assert_eq!(
        nel_token_after_skip_10_days - nel_token_before,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(ratio_reward_borrow))
    );
    //assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    //assert_eq!(asset.borrowed.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);

    println!(
        "===> Account NEL token after claim: {:?}",
        &e.nel_balance_of(&users.alice)
    );

    let asset = e.get_asset(&tokens.ndai);
    println!("===> DAI borrowed: {:?}", &asset.borrowed.balance);
    println!("===> DAI supplied: {:?}", &asset.supplied.balance);

    // Repay 500 USDT
    let repay_amount = d(500, 18);
    e.deposit_and_repay(&users.alice, &tokens.ndai, repay_amount);
    let asset = e.get_asset(&tokens.ndai);
    println!("===> DAI borrowed: {:?}", &asset.borrowed.balance);
    println!("===> DAI supplied: {:?}", &asset.supplied.balance);
    // Update Ratio reward:
    let ratio_reward_borrow = (amount.saturating_sub(repay_amount) * 10_000
        / (dai_amount + amount.saturating_sub(repay_amount))) as u32;

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 2 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 10);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    //assert_eq!(asset.borrowed.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - reward_per_day * 20
    );

    let account = e.get_account(&users.alice);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(
            &BigDecimal::from(reward_per_day * 10).mul_ratio(ratio_reward_borrow)
        )
    );

    // Claim all rewards
    e.account_farm_claim_all(&users.alice).assert_success();
    let nel_token_after_skip_10_days = e.nel_balance_of(&users.alice).0;

    assert_eq!(
        nel_token_after_skip_10_days - nel_token_before,
        BigDecimal::round_u128(
            &BigDecimal::from(reward_per_day * 10).mul_ratio(ratio_reward_borrow)
        )
    );
}

///  Test Reward farm Borrow with only 1 user Alice Deposit:
///  1. Add farm for Deposit USDT token with config:
///  - 100 NEL/day
///  - Total reward 3000 NEL
///  2. Deposit 4000 USDT to Pool
///  3. Borrow 1000 USDT
///  4. Check reward amount for 10 days,
///  5. Borrow more 1000 USDT
///  6. Check reward 10 days next
#[test]
fn test_farm_borrowed_3() {
    let (e, tokens, users) = basic_setup();

    let reward_per_day = d(100, BOOSTER_TOKEN_DECIMALS);
    let total_reward = d(3000, BOOSTER_TOKEN_DECIMALS);

    let farm_usdt_id = FarmId::Supplied(tokens.nusdt.account_id());
    println!("==========> Farm USDT Id: {:?}", farm_usdt_id);
    // add farm reward DAI
    e.add_farm(
        farm_usdt_id.clone(),
        &e.booster_contract.user_account,
        reward_per_day,
        d(100, BOOSTER_TOKEN_DECIMALS),
        total_reward,
    );

    let asset = e.get_asset(&tokens.nusdt);
    println!("=====> Assets Before deposit: {:?}", asset);
    assert_eq!(asset.farms.len(), 1);
    assert_eq!(asset.farms[0].farm_id, farm_usdt_id);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let usdt_amount = d(4000, 6);
    let borrow_amount = d(1000, 6);

    let ratio_reward_borrow = (borrow_amount * 10_000 / (usdt_amount + borrow_amount)) as u32;

    // deposit 4000 nUSDT to the farm
    e.contract_ft_transfer_call(&tokens.nusdt, &users.alice, usdt_amount, "")
        .assert_success();

    // deposit 4000 nDAI to the farm
    e.contract_ft_transfer_call(&tokens.ndai, &users.bob, usdt_amount, "")
        .assert_success();

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.borrowed.balance, 0);

    e.borrow(
        &users.bob,
        &tokens.nusdt,
        price_data(&tokens, None, None, None),
        borrow_amount,
    );

    let asset = e.get_asset(&tokens.nusdt);
    println!("=====> Assets After deposit: {:?}", asset);

    // assert_eq!(asset.borrowed.balance, borrow_amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    let account = e.get_account(&users.bob);
    println!("=====> Bob account: {:?}", account);
    assert_balances(
        &account.borrowed,
        &[av(tokens.nusdt.account_id(), borrow_amount)],
    );
    let nel_token_before = e.nel_balance_of(&users.bob).0;
    println!("===> Account before: {:?}", account);

    assert_eq!(account.farms[0].farm_id, farm_usdt_id);
    assert_eq!(
        account.farms[0].rewards[0].reward_token_id,
        e.booster_contract.user_account.account_id()
    );
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.nusdt.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 10 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 10);

    let farmed_amount = reward_per_day * 10;

    println!("===> farmed_amount: {:?}", farmed_amount);

    //let asset = e.get_asset(&e.booster_contract.user_account);
    //assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.nusdt);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.bob);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.nusdt.account_id())
            .shares
            .0,
    );
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(ratio_reward_borrow))
    );

    // claim the reward
    e.account_farm_claim_all(&users.bob).assert_success();
    let nel_token_after_skip_10_days = e.nel_balance_of(&users.bob).0;

    let asset = e.get_asset(&e.booster_contract.user_account);
    println!("{:?}", asset);
    assert_eq!(
        nel_token_after_skip_10_days - nel_token_before,
        BigDecimal::round_u128(&BigDecimal::from(farmed_amount).mul_ratio(ratio_reward_borrow))
    );
    assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.nusdt);
    //assert_eq!(asset.borrowed.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.bob);

    println!(
        "===> Account NEL token after claim: {:?}",
        &e.nel_balance_of(&users.bob)
    );

    // Continue borrow 3000 USDT
    let borrow_amount_bonus = d(3000, 6);
    e.borrow(
        &users.bob,
        &tokens.nusdt,
        price_data(&tokens, None, None, None),
        borrow_amount,
    );
    // Update Ratio reward:
    let ratio_reward_borrow = (borrow_amount.saturating_add(borrow_amount_bonus) * 10_000
        / (usdt_amount + borrow_amount.saturating_add(borrow_amount_bonus)))
        as u32;

    assert_eq!(account.farms[0].farm_id, farm_usdt_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.borrowed, &tokens.nusdt.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    // next 2 days, the farm should get rewards
    e.skip_time(ONE_DAY_SEC * 10);

    let asset = e.get_asset(&e.booster_contract.user_account);
    assert_eq!(asset.borrowed.balance, 0);

    let asset = e.get_asset(&tokens.nusdt);
    //assert_eq!(asset.borrowed.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_contract.user_account.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - reward_per_day * 20
    );

    let account = e.get_account(&users.bob);

    assert_eq!(account.farms[0].farm_id, farm_usdt_id);
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        BigDecimal::round_u128(
            &BigDecimal::from(reward_per_day * 10).mul_ratio(ratio_reward_borrow)
        )
    );

    // Claim all rewards
    e.account_farm_claim_all(&users.bob).assert_success();
    let nel_token_after_skip_10_days = e.nel_balance_of(&users.bob).0;

    assert_eq!(
        nel_token_after_skip_10_days - nel_token_before,
        BigDecimal::round_u128(
            &BigDecimal::from(reward_per_day * 10).mul_ratio(ratio_reward_borrow)
        )
    );
}

///  Test Reward farm Borrow with only 1 user Alice Deposit:
///  1. Add farm for Borrow & Deposit USDT & DAI token with config:
///  - 100 NEL/day for DAI
///  - 100 NEL/day for USDT
///  - Total reward 3000 NEL
///  2. Deposit 1000 USDT to Pool
///  3. Borrow 1000 DAI to Pool
///  4. Check reward amount for 10 days,
#[test]
fn test_farm_supplied_and_borrowed() {}

#[test]
fn test_has_potential_farms() {
    let (e, tokens, users) = basic_setup();

    let amount = d(100, 18);
    e.contract_ft_transfer_call(&tokens.ndai, &users.alice, amount, "")
        .assert_success();

    let account = e.get_account(&users.alice);
    assert!(!account.has_non_farmed_assets);

    let reward_per_day = d(100, 18);
    let total_reward = d(3000, 18);

    let farm_id = FarmId::Supplied(tokens.ndai.account_id());
    e.add_farm(
        farm_id,
        &e.booster_contract.user_account,
        reward_per_day,
        d(100, 18),
        total_reward,
    );

    let account = e.get_account(&users.alice);
    assert_eq!(account.farms.len(), 0);
    assert!(account.has_non_farmed_assets);

    e.account_farm_claim_all(&users.alice).assert_success();

    let account = e.get_account(&users.alice);
    assert_eq!(account.farms.len(), 1);
    assert!(!account.has_non_farmed_assets);
}

#[test]
fn test_farm_supplied_xbooster() {
    let (e, tokens, users) = basic_setup();

    let reward_per_day = d(100, 18);
    let total_reward = d(3000, 18);
    let booster_base = d(20, 18);

    let farm_id = FarmId::Supplied(tokens.ndai.account_id());
    e.add_farm(
        farm_id,
        &tokens.nusdc,
        reward_per_day,
        booster_base,
        total_reward,
    );

    let booster_amount = d(5, 18);
    e.contract_ft_transfer_call(
        &e.booster_contract.user_account,
        &users.alice,
        booster_amount,
        "",
    )
    .assert_success();

    e.account_stake_booster(&users.alice, booster_amount, MAX_DURATION_SEC)
        .assert_success();

    let amount = d(100, 18);
    e.contract_ft_transfer_call(&tokens.ndai, &users.alice, amount, "")
        .assert_success();

    let asset = e.get_asset(&tokens.nusdc);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);
    assert_eq!(booster_reward.boosted_shares, asset.supplied.shares.0 * 2);

    let account = e.get_account(&users.alice);
    assert_balances(&account.supplied, &[av(tokens.ndai.account_id(), amount)]);

    let booster_staking = account.booster_staking.unwrap();
    assert_eq!(booster_staking.staked_booster_amount, booster_amount);
    assert_eq!(booster_staking.x_booster_amount, booster_amount * 4);

    // The amount of boosted shares should be 2X due to the log base.
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0
            * 2,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    e.skip_time(ONE_DAY_SEC * 3);

    let farmed_amount = reward_per_day * 3;
    let asset = e.get_asset(&tokens.ndai);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, farmed_amount);

    let booster_amount = d(95, 18);
    e.contract_ft_transfer_call(
        &e.booster_contract.user_account,
        &users.alice,
        booster_amount,
        "",
    )
    .assert_success();

    // Increasing booster stake updates all farms.
    e.account_stake_booster(&users.alice, booster_amount, MAX_DURATION_SEC)
        .assert_success();

    let asset = e.get_asset(&tokens.nusdc);
    // assert_eq!(asset.supplied.balance, farmed_amount);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );
    assert_eq!(booster_reward.boosted_shares, asset.supplied.shares.0 * 3);

    let account = e.get_account(&users.alice);
    assert_balances(
        &account.supplied,
        &[
            av(tokens.ndai.account_id(), amount),
            // av(tokens.nusdc.account_id(), farmed_amount),
        ],
    );

    // The boosted amount should 3X because the xBooster is 400.
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0
            * 3,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);
    let booster_staking = account.booster_staking.unwrap();
    assert_eq!(booster_staking.staked_booster_amount, d(100, 18));
    assert_eq!(booster_staking.x_booster_amount, d(400, 18));
}

#[test]
fn test_farm_supplied_xbooster_unstake() {
    let (e, tokens, users) = basic_setup();

    let booster_amount = d(5, 18);
    e.contract_ft_transfer_call(
        &e.booster_contract.user_account,
        &users.alice,
        booster_amount,
        "",
    )
    .assert_success();

    e.account_stake_booster(&users.alice, booster_amount, MAX_DURATION_SEC)
        .assert_success();

    e.skip_time(MAX_DURATION_SEC);

    let reward_per_day = d(100, 18);
    let total_reward = d(3000, 18);
    let booster_base = d(20, 18);

    let farm_id = FarmId::Supplied(tokens.ndai.account_id());
    e.add_farm(
        farm_id,
        &tokens.nusdc,
        reward_per_day,
        booster_base,
        total_reward,
    );

    let amount = d(100, 18);
    e.contract_ft_transfer_call(&tokens.ndai, &users.alice, amount, "")
        .assert_success();

    let asset = e.get_asset(&tokens.nusdc);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, amount);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);
    assert_eq!(booster_reward.boosted_shares, asset.supplied.shares.0 * 2);

    let account = e.get_account(&users.alice);

    // The amount of boosted shares should be 2X due to the log base.
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0
            * 2,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    e.skip_time(ONE_DAY_SEC * 3);

    let farmed_amount = reward_per_day * 3;
    let asset = e.get_asset(&tokens.ndai);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, farmed_amount);

    // Unstaking booster updates all farms.
    e.account_unstake_booster(&users.alice).assert_success();

    let asset = e.get_asset(&tokens.nusdc);
    // assert_eq!(asset.supplied.balance, farmed_amount);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );
    // The boosted amount should 1X because of xBooster unstaking.
    assert_eq!(booster_reward.boosted_shares, asset.supplied.shares.0);

    let account = e.get_account(&users.alice);
    assert_balances(
        &account.supplied,
        &[
            av(tokens.ndai.account_id(), amount),
            av(e.booster_contract.user_account.account_id(), booster_amount),
            // av(tokens.nusdc.account_id(), farmed_amount),
        ],
    );

    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);
    assert!(account.booster_staking.is_none());
}
#[test]
fn test_farm_supplied_two_users() {
    let (e, tokens, users) = basic_setup();

    let booster_amount_alice = d(5, 18);
    e.contract_ft_transfer_call(
        &e.booster_contract.user_account,
        &users.alice,
        booster_amount_alice,
        "",
    )
    .assert_success();

    e.account_stake_booster(&users.alice, booster_amount_alice, MAX_DURATION_SEC)
        .assert_success();

    let booster_amount_bob = d(100, 18);
    e.contract_ft_transfer_call(
        &e.booster_contract.user_account,
        &users.bob,
        booster_amount_bob,
        "",
    )
    .assert_success();

    e.account_stake_booster(&users.bob, booster_amount_bob, MAX_DURATION_SEC)
        .assert_success();

    let reward_per_day = d(100, 18);
    let total_reward = d(3000, 18);
    let booster_base = d(20, 18);

    let farm_id = FarmId::Supplied(tokens.ndai.account_id());
    e.add_farm(
        farm_id,
        &tokens.nusdc,
        reward_per_day,
        booster_base,
        total_reward,
    );

    let amount = d(100, 18);
    e.contract_ft_transfer_call(&tokens.ndai, &users.alice, amount, "")
        .assert_success();

    e.contract_ft_transfer_call(&tokens.ndai, &users.bob, amount, "")
        .assert_success();

    let asset = e.get_asset(&tokens.nusdc);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    assert_eq!(asset.supplied.balance, amount * 2);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);
    // 2.5X (Alice 2X, Bob 3X)
    assert_eq!(
        booster_reward.boosted_shares,
        asset.supplied.shares.0 * 5 / 2
    );

    let account = e.get_account(&users.alice);

    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0
            * 2,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    let account = e.get_account(&users.bob);

    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0
            * 3,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    e.skip_time(ONE_DAY_SEC * 3);

    let farmed_amount = reward_per_day * 3;
    let asset = e.get_asset(&tokens.ndai);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        farmed_amount * 2 / 5
    );

    let account = e.get_account(&users.bob);
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        farmed_amount * 3 / 5
    );

    let extra_booster_amount = d(95, 18);
    e.contract_ft_transfer_call(
        &e.booster_contract.user_account,
        &users.alice,
        extra_booster_amount,
        "",
    )
    .assert_success();

    // Increasing booster stake updates all farms.
    e.account_stake_booster(&users.alice, extra_booster_amount, MAX_DURATION_SEC)
        .assert_success();

    let asset = e.get_asset(&tokens.nusdc);
    // The amount of only for Alice, but Bob still unclaimed
    // assert_eq!(asset.supplied.balance, farmed_amount * 2 / 5);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    // Both Alice and Bob now have 3X booster
    assert_eq!(booster_reward.boosted_shares, asset.supplied.shares.0 * 3);

    let account = e.get_account(&users.alice);
    assert_balances(&account.supplied, &[av(tokens.ndai.account_id(), amount)]);

    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        find_asset(&account.supplied, &tokens.ndai.account_id())
            .shares
            .0
            * 3,
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    let account = e.get_account(&users.bob);
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        farmed_amount * 3 / 5
    );

    e.skip_time(ONE_DAY_SEC * 2);

    let asset = e.get_asset(&tokens.nusdc);
    // assert_eq!(asset.supplied.balance, farmed_amount * 2 / 5);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&tokens.ndai);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&tokens.nusdc.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - reward_per_day * 5
    );

    let account = e.get_account(&users.alice);
    // Unclaimed half of the rewards for 2 days
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        reward_per_day * 2 / 2
    );

    let account = e.get_account(&users.bob);
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        farmed_amount * 3 / 5 + reward_per_day * 2 / 2
    );
}
