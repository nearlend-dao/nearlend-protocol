mod setup;

use crate::setup::*;
use contract::FarmId;

#[test]
fn test_nft_farm_supplied() {
    let (e, _tokens, users) = basic_setup();

    let reward_per_day = d(100, 18);
    let total_reward = d(3000, 18);

    let farm_id = FarmId::SuppliedNFT(e.nft_contract.account_id());
    e.add_farm(
        farm_id.clone(),
        &e.booster_token,
        reward_per_day,
        d(100, 18),
        total_reward,
    );

    let asset = e.get_asset(&e.nft_contract);
    assert_eq!(asset.farms.len(), 1);
    assert_eq!(asset.farms[0].farm_id, farm_id);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_token.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    e.mint_nft(&users.alice, "1".to_string());
    e.mint_nft(&users.alice, "2".to_string());

    e.supply_nft_to_collateral(&users.alice, e.nft_contract.account_id(), "2".to_string())
        .assert_success();

    // let logs = get_logs(&e.near.borrow_runtime());
    // println!("logs {:?}", logs);

    let asset = e.get_asset(&e.booster_token);
    assert_eq!(asset.supplied.balance, 0);

    // Check if the NFT asset has been deposited.
    let asset = e.get_asset(&e.nft_contract);
    assert_eq!(asset.nft_supplied[0].owner_id, users.alice.account_id());
    assert_eq!(asset.nft_supplied[0].token_id, "2".to_string());

    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_token.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, total_reward);

    // Check Alice NFT supplied in contract
    let account = e.get_account(&users.alice);
    assert_eq!(
        account.nft_supplied[0].nft_contract_id,
        e.nft_contract.account_id()
    );
    assert_eq!(account.nft_supplied[0].nft_token_id, "2".to_string());

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].reward_token_id,
        e.booster_token.account_id()
    );
    let nft_supplied_count = 1;
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        nft_supplied_count * 10u128.pow(24)
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    e.skip_time(ONE_DAY_SEC * 3);

    let farmed_amount = reward_per_day * 3;

    let asset = e.get_asset(&e.booster_token);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&e.nft_contract);

    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_token.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);

    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        nft_supplied_count * 10u128.pow(24),
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, farmed_amount);

    e.account_farm_claim_all(&users.alice).assert_success();

    let asset = e.get_asset(&e.booster_token);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&e.nft_contract);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_token.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - farmed_amount
    );

    let account = e.get_account(&users.alice);
    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        nft_supplied_count * 10u128.pow(24),
    );
    assert_eq!(account.farms[0].rewards[0].unclaimed_amount, 0);

    e.skip_time(ONE_DAY_SEC * 2);

    let asset = e.get_asset(&e.booster_token);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&e.nft_contract);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_token.account_id())
        .cloned()
        .unwrap();
    assert_eq!(
        booster_reward.remaining_rewards,
        total_reward - reward_per_day * 5
    );

    let account = e.get_account(&users.alice);
    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        nft_supplied_count * 10u128.pow(24),
    );
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        reward_per_day * 2
    );

    e.skip_time(ONE_DAY_SEC * 30);

    let asset = e.get_asset(&e.nft_contract);
    let booster_reward = asset.farms[0]
        .rewards
        .get(&e.booster_token.account_id())
        .cloned()
        .unwrap();
    assert_eq!(booster_reward.remaining_rewards, 0);

    let account = e.get_account(&users.alice);
    assert_eq!(account.farms[0].farm_id, farm_id);
    assert_eq!(
        account.farms[0].rewards[0].boosted_shares,
        nft_supplied_count * 10u128.pow(24),
    );
    assert_eq!(
        account.farms[0].rewards[0].unclaimed_amount,
        total_reward - farmed_amount
    );

    // let booster_token_before = e.get_balance(&e.booster_token, &users.alice).0;
    e.account_farm_claim_all(&users.alice).assert_success();

    // let booster_token_after = e.get_balance(&e.booster_token, &users.alice).0;
    // assert_eq!(booster_token_before - booster_token_after, total_reward - farmed_amount);

    let asset = e.get_asset(&e.booster_token);
    assert_eq!(asset.supplied.balance, 0);

    let asset = e.get_asset(&e.nft_contract);
    assert!(asset.farms[0]
        .rewards
        .get(&e.booster_token.account_id())
        .is_none());

    let account = e.get_account(&users.alice);
    assert_eq!(account.farms[0].farm_id, farm_id);
    assert!(account.farms[0].rewards.is_empty());
}
