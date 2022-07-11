#!/bin/bash
export MAIN_ACCOUNT=lam-test11.testnet
export NEAR_ENV=testnet
export OWNER_ID=$MAIN_ACCOUNT
# export ORACLE_ID=priceoracle.testnet
# export ORACLE_ID=price-oracle.lam-test50.testnet
export ORACLE_ID=price-oracle.duonghb3.testnet
export ACCOUNT_ID=$MAIN_ACCOUNT
export NFT_CONTRACT_ID=nft.$MAIN_ACCOUNT
export CONTRACT_ID=nearlend.$MAIN_ACCOUNT
export BOOSTER_TOKEN_ID=ref.fakes.testnet
export WETH_TOKEN_ID=weth.fakes.testnet
export DAI_TOKEN_ID=dai.fakes.testnet
export USDT_TOKEN_ID=usdt.fakes.testnet
export USDC_TOKEN_ID=usdc.testnet
export AURORAX_TOKEN_ID=aurorax.$OWNER_ID
export NEL_TOKEN_ID=nearlendtest.testnet
export WNEAR_TOKEN_ID=wrap.testnet
export ONE_YOCTO=0.000000000000000000000001
export GAS=200000000000000
export DECIMAL_18=000000000000000000
export ACCOUNT_TEST=lam-test04.testnet 

# nearlend.lam-test01.testnet

#####################################
# - Sử dụng đồng DAI cho việc test, nếu muốn test vs các token khác thì tương tự
# - Ví dụ:
#     B1: Deploy Nearland contract 
#     B2: Init Nearland contract
#     B3: Deposit storage cho CONTRACT_ID, OWNER_ID với token DAI_TOKEN_ID và USDT_TOKEN_ID
#     B4: Thực hiện mint để token về ACCOUNT_ID cho việc test 
#     B5: Thực hiện add asset DAI_TOKEN_ID và USDT_TOKEN_ID vào Nearland 




################## B1: Deploy Nearland contract ##################
echo "###################### Build Contract #####################"
../build.sh

echo "################### DELETE ACCOUNT ###################"
near delete $CONTRACT_ID $ACCOUNT_ID
# near delete $ACCOUNT_TEST $ACCOUNT_ID


echo "################### CREATE ACCOUNT ###################"

near create-account $CONTRACT_ID --masterAccount $ACCOUNT_ID --initialBalance 10
# near create-account $ACCOUNT_TEST --masterAccount $ACCOUNT_ID --initialBalance 10

echo "################### CREATE CONTRACT ###################"
near deploy $CONTRACT_ID --accountId $ACCOUNT_ID --wasmFile ../res/nearlend_protocol.wasm

###################### End B1: Deploy Nearland contract #####################




######################### B2: Init Nearland contract #########################

echo "################### INIT CONTRACT ###################"
near call $CONTRACT_ID --accountId=$CONTRACT_ID new '{
  "config" : {
    "oracle_account_id": "'$ORACLE_ID'", 
    "owner_id": "'$ACCOUNT_ID'", 
    "booster_token_id": "'$BOOSTER_TOKEN_ID'", 
    "booster_decimals": 18,
    "max_num_assets": 10,
    "maximum_recency_duration_sec": 90,
    "maximum_staleness_duration_sec": 15,
    "minimum_staking_duration_sec": 2678400,
    "maximum_staking_duration_sec": 31536000,
    "x_booster_multiplier_at_maximum_staking_duration": 40000,
    "force_closing_enabled": true
  }
}'

###################### End B2: Init Nearland contract #####################



######################### B3: Deposit storage #########################

# # Deposit BOOSTER_TOKEN_ID
# near call $BOOSTER_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.00125
# near call $BOOSTER_TOKEN_ID --accountId=$OWNER_ID storage_deposit '' --amount=0.00125

# Deposit CONTRACT_ID
near call $CONTRACT_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.1
near call $CONTRACT_ID --accountId=$OWNER_ID storage_deposit '' --amount=0.1
near call $CONTRACT_ID --accountId=$ACCOUNT_TEST storage_deposit '' --amount=0.1

# Deposit WETH_TOKEN_ID
near call $WETH_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.00125
near call $WETH_TOKEN_ID --accountId=$OWNER_ID storage_deposit '' --amount=0.00125

# Deposit DAI_TOKEN_ID
near call $DAI_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.00125
near call $DAI_TOKEN_ID --accountId=$OWNER_ID storage_deposit '' --amount=0.00125
near call $DAI_TOKEN_ID --accountId=$ACCOUNT_TEST storage_deposit '' --amount=0.00125


# Deposit USDT_TOKEN_ID
near call $USDT_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.00125
near call $USDT_TOKEN_ID --accountId=$OWNER_ID storage_deposit '' --amount=0.00125

# Deposit WNEAR_TOKEN_ID
near call $WNEAR_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.00125
near call $WNEAR_TOKEN_ID --accountId=$OWNER_ID storage_deposit '' --amount=0.00125

###################### End B3: Deposit storage #####################

######################### B4: Mint tokens #########################

# Mint BOOTER_TOKEN_ID
# near call $BOOSTER_TOKEN_ID --accountId=$ACCOUNT_ID mint '{
#  "account_id": "'$ACCOUNT_ID'",
#  "amount": "100000000000000000000000"
# }'

# Mint WETH_TOKEN_ID
near call $WETH_TOKEN_ID --accountId=$ACCOUNT_ID mint '{
 "account_id": "'$ACCOUNT_ID'",
 "amount": "10000000000000000000"
}'

## Mint DAI_TOKEN_ID
near call $DAI_TOKEN_ID --accountId=$ACCOUNT_ID mint '{
  "account_id": "'$ACCOUNT_ID'",
  "amount": "100000000000000000000000"
}'

near call $DAI_TOKEN_ID --accountId=$ACCOUNT_TEST mint '{
  "account_id": "'$ACCOUNT_TEST'",
  "amount": "100000000000000000000000"
}'

# Mint USDT_TOKEN_ID
near call $USDT_TOKEN_ID --accountId=$ACCOUNT_ID mint '{
  "account_id": "'$ACCOUNT_ID'",
  "amount": "10000000000"
}'

# ## Mint WNEAR_TOKEN_ID
# near call $WNEAR_TOKEN_ID --accountId=$ACCOUNT_ID de '{
#   "account_id": "'$ACCOUNT_ID'",
#   "amount": "10000000000"
# }'
# near call $WNEAR_TOKEN_ID --accountId=$ACCOUNT_ID near_deposit '{}' --amount=10

###################### End B4: Mint tokens #####################


###################### B5: Add asset #####################

# Booster APR is 30%, to verify run ./scripts/apr_to_rate.py 30
# Max APR for all assets is 250%
# Booster can't be used as a collateral or borrowed (for now), so APR doesn't matter.
# near call $CONTRACT_ID --accountId=$OWNER_ID add_asset '{
#   "token_id": "'$BOOSTER_TOKEN_ID'",
#   "asset_config": {
#     "reserve_ratio": 2500,
#     "target_utilization": 8000,
#     "target_utilization_rate": "1000000000008319516250272147",
#     "max_utilization_rate": "1000000000039724853136740579",
#     "volatility_ratio": 2000,
#     "extra_decimals": 18,
#     "can_deposit": true,
#     "can_withdraw": true,
#     "can_use_as_collateral": true,
#     "can_borrow": true
#   }
# }' --amount=$ONE_YOCTO --gas=$GAS



# wETH APR is 6%, to verify run ./scripts/apr_to_rate.py 5
# Volatility ratio is 60%, since it's somewhat liquid on NEAR
# near call $CONTRACT_ID --accountId=$OWNER_ID add_asset '{
#   "token_id": "'$WETH_TOKEN_ID'",
#   "asset_config": {
#     "reserve_ratio": 2500,
#     "target_utilization": 8000,
#     "target_utilization_rate": "1000000000001547125956667610",
#     "max_utilization_rate": "1000000000039724853136740579",
#     "volatility_ratio": 6000,
#     "extra_decimals": 0,
#     "can_deposit": true,
#     "can_withdraw": true,
#     "can_use_as_collateral": true,
#     "can_borrow": true
#   }
# }' --amount=$ONE_YOCTO --gas=$GAS


# Ví dụ: 
#     thế chấp 100 DAI = $100, đi vay 1ETH = $10 và volatility_ratio = 60%
#     tính max có thể descrease collateral: $100 - $10/0.6 = $84   => $16 -> thế chấp
#         => max có thể descrease collateral => $90  
    


# DAI APR is 4%, to verify run ./scripts/apr_to_rate.py 4
# max_utilization_rate is 75%, to verify run ./scripts/apr_to_rate.py 75
# Volatility ratio is 95%, since it's stable and liquid on NEAR
near call $CONTRACT_ID --accountId=$OWNER_ID add_asset '{
  "token_id": "'$DAI_TOKEN_ID'",
  "asset_config": {
    "reserve_ratio": 1000,
    "target_utilization": 8000,
    "target_utilization_rate": "1000000001243680656318820313",
    "max_utilization_rate": "1000000017745300383710610089",
    "volatility_ratio": 9500,
    "extra_decimals": 0,
    "can_deposit": true,
    "can_withdraw": true,
    "can_use_as_collateral": true,
    "can_borrow": true
  }
}' --amount=$ONE_YOCTO --gas=$GAS

# USDT APR is 8%, to verify run ./scripts/apr_to_rate.py 8
# Volatility ratio is 95%, since it's stable and liquid on NEAR
# USDT by default has 6 decimals, the config adds extra 12 decimals, to bring the total to 18
near call $CONTRACT_ID --accountId=$OWNER_ID add_asset '{
  "token_id": "'$USDT_TOKEN_ID'",
  "asset_config": {
    "reserve_ratio": 2500,
    "target_utilization": 8000,
    "target_utilization_rate": "1000000000002440418605283556",
    "max_utilization_rate": "1000000000039724853136740579",
    "volatility_ratio": 9500,
    "extra_decimals": 12,
    "can_deposit": true,
    "can_withdraw": true,
    "can_use_as_collateral": true,
    "can_borrow": true
  }
}' --amount=$ONE_YOCTO --gas=$GAS

# wNEAR APR is 12%, to verify run ./scripts/apr_to_rate.py 12
# Target utilization is 60% (for some reason)
# Volatility ratio is 60%
# near call $CONTRACT_ID --accountId=$OWNER_ID add_asset '{
#   "token_id": "'$WNEAR_TOKEN_ID'",
#   "asset_config": {
#     "reserve_ratio": 2500,
#     "target_utilization": 6000,
#     "target_utilization_rate": "1000000000003593629036885046",
#     "max_utilization_rate": "1000000000039724853136740579",
#     "volatility_ratio": 6000,
#     "extra_decimals": 0,
#     "can_deposit": true,
#     "can_withdraw": true,
#     "can_use_as_collateral": true,
#     "can_borrow": true
#   }
# }' --amount=$ONE_YOCTO --gas=$GAS

# near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'
# near view $CONTRACT_ID get_assets_paged_detailed '{"from_index": 0, "limit": 10}'


near call $CONTRACT_ID --accountId=$OWNER_ID add_asset '{
  "token_id": "'$NFT_CONTRACT_ID'",
  "asset_config": {
    "reserve_ratio": 2500,
    "target_utilization": 0,
    "target_utilization_rate": "0",
    "max_utilization_rate": "0",
    "volatility_ratio": 6000,
    "extra_decimals": 0,
    "can_deposit": true,
    "can_withdraw": true,
    "can_use_as_collateral": true,
    "can_borrow": true
  }
}' --amount=$ONE_YOCTO --gas=$GAS


near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'
near view $CONTRACT_ID get_assets_paged_detailed '{"from_index": 0, "limit": 10}'
###################### End B5: Add asset #####################