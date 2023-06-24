#!/bin/bash
export MAIN_ACCOUNT=nearlend-official.testnet
export NEAR_ENV=testnet
export ORACLE_ID=price-oracle.$MAIN_ACCOUNT
export NFT_CONTRACT_ID=nft.$MAIN_ACCOUNT
export CONTRACT_ID=main.$MAIN_ACCOUNT
export BOOSTER_TOKEN_ID=ft.$MAIN_ACCOUNT
export WETH_TOKEN_ID=weth.fakes.testnet
export DAI_TOKEN_ID=dai.fakes.testnet
export USDT_TOKEN_ID=usdt.fakes.testnet
export USDC_TOKEN_ID=usdc.testnet
export WNEAR_TOKEN_ID=wrap.testnet
export ONE_YOCTO=0.000000000000000000000001
export GAS=200000000000000
export DECIMAL_18=000000000000000000
export ACCOUNT_TEST=lamns1.testnet


# Chạy test deposit:
# Ví dụ: Deposit vs 10 DAI  và withdraw 5 DAI
    # B1: Chạy file deploy
    # B2: Thực hiện deposit vs 10 DAI vào MAIN_ACCOUNT
    # B3: Thực hiện Borrow 1 DAI
    # B4: Thực hiện Borrow and Withdraw 1 DAI


# ##################### B1: Chạy file deploy #####################
# ./0_deploy.sh
# ##################### End B1: Chạy file deploy #####################


# ###################### B2: Thực hiện deposit vs 10 DAI vào MAIN_ACCOUNT #####################

# near call $DAI_TOKEN_ID --accountId=$MAIN_ACCOUNT ft_transfer_call '{
#   "receiver_id": "'$CONTRACT_ID'",
#   "amount": "155'$DECIMAL_18'",
#   "msg": ""
# }' --amount=$ONE_YOCTO --gas=$GAS

# near view $CONTRACT_ID get_account '{"account_id": "'$MAIN_ACCOUNT'"}' 
# near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

###################### End B2: Thực hiện deposit vs 10 DAI vào MAIN_ACCOUNT #####################

###################### B3: Thực hiện Borrow 1 DAI #####################
near call $ORACLE_ID --accountId=$ACCOUNT_TEST oracle_call '{
  "receiver_id": "'$CONTRACT_ID'",
  "asset_ids": [
    "'$USDT_TOKEN_ID'",
    "'$DAI_TOKEN_ID'",
    "'$NFT_CONTRACT_ID'",
    "'$WETH_TOKEN_ID'",
    "'$WNEAR_TOKEN_ID'",
    "'$BOOSTER_TOKEN_ID'"
  ],
  "msg": "{\"Execute\": {\"actions\": [{\"Borrow\": {\"token_id\": \"'$DAI_TOKEN_ID'\", \"amount\": \"400'$DECIMAL_18'\"}}]}}"
}' --amount=$ONE_YOCTO --gas=$GAS

# near view $CONTRACT_ID get_account '{"account_id": "'$MAIN_ACCOUNT'"}'
# near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

###################### End B3: Thực hiện Borrow 1 DAI #####################

###################### B4: Thực hiện Borrow and Withdraw 1 DAI #####################
# near call $ORACLE_ID --accountId=$MAIN_ACCOUNT oracle_call '{
#   "receiver_id": "'$CONTRACT_ID'",
#   "asset_ids": [
#     "'$USDT_TOKEN_ID'",
#     "'$DAI_TOKEN_ID'"
#   ],
#   "msg": "{\"Execute\": {\"actions\": [{\"Borrow\": {\"token_id\": \"'$DAI_TOKEN_ID'\", \"amount\":\"10000'$DECIMAL_18'\"}},{\"Withdraw\":{\"token_id\":\"'$DAI_TOKEN_ID'\",\"amount\":\"1'$DECIMAL_18'\"}}]}}"
# }' --amount=$ONE_YOCTO --gas=$GAS

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_TEST'"}'
# near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

###################### End B4: Thực hiện Borrow and Withdraw 1 DAI #####################


# near view nearlend.lam-test6.testnet get_account '{"account_id": "lam-test6.testnet"}' 
# near view nearlend.lam-test6.testnet get_assets_paged '{"from_index": 0, "limit": 10}'