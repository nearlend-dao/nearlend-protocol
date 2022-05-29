#!/bin/bash

export MAIN_ACCOUNT=lam-test6.testnet
export NEAR_ENV=testnet
export OWNER_ID=$MAIN_ACCOUNT
export ORACLE_ID=priceoracle.testnet
export ACCOUNT_ID=$MAIN_ACCOUNT
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

# Chạy test deposit:
# Ví dụ: Deposit vs 10 DAI  và withdraw 5 DAI
    # B1: Chạy file deploy
    # B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID
    # B3: Thực hiện IncreaseCollateral vs 5 DAI
    # B4: Thực hiện DecreaseCollateral vs 3 DAI
    # B5: Thực hiện deposit and IncreaseCollateral vs 10 DAI
    # B6: Thực hiện DecreaseCollateral vs 6 DAI



###################### B1: Chạy file deploy #####################
./0_deploy.sh
###################### End B1: Chạy file deploy #####################


###################### B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID #####################

near call $DAI_TOKEN_ID --accountId=$ACCOUNT_ID ft_transfer_call '{
  "receiver_id": "'$CONTRACT_ID'",
  "amount": "10'$DECIMAL_18'",
  "msg": ""
}' --amount=$ONE_YOCTO --gas=$GAS

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}' 
near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

###################### End B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID #####################


###################### B3: Thực hiện IncreaseCollateral vs 5 DAI #####################
near call $CONTRACT_ID --accountId=$ACCOUNT_ID --gas=$GAS --amount=$ONE_YOCTO execute '{
  "actions": [
    {
      "IncreaseCollateral": {
        "token_id": "'$DAI_TOKEN_ID'",
        "amount": "5'$DECIMAL_18'"
      }
    }
  ]
}'

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}' 
near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'
###################### End B3: Thực hiện IncreaseCollateral vs 5 DAI #####################

###################### B4: Thực hiện DecreaseCollateral vs 3 DAI #####################
near call $CONTRACT_ID --accountId=$ACCOUNT_ID --gas=$GAS --amount=$ONE_YOCTO execute '{
  "actions": [
    {
      "DecreaseCollateral": {
        "token_id": "'$DAI_TOKEN_ID'",
        "amount": "3'$DECIMAL_18'"
      }
    }
  ]
}'

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}' 
near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'
###################### End B4: Thực hiện DecreaseCollateral vs 3 DAI #####################

###################### B5: Thực hiện deposit and IncreaseCollateral vs 10 DAI #####################
near call $DAI_TOKEN_ID --accountId=$OWNER_ID ft_transfer_call '{
  "receiver_id": "'$CONTRACT_ID'",
  "amount": "10'$DECIMAL_18'",
  "msg": "{\"Execute\": {\"actions\": [{\"IncreaseCollateral\": {\"token_id\": \"'$DAI_TOKEN_ID'\", \"amount\": \"10'$DECIMAL_18'\"}}]}}"
}' --amount=$ONE_YOCTO --gas=$GAS

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}'
near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

###################### End B5: Thực hiện deposit and IncreaseCollateral vs 10 DAI #####################

###################### B6: Thực hiện DecreaseCollateral vs 6 DAI ########################
near call $CONTRACT_ID --accountId=$ACCOUNT_ID --gas=$GAS --amount=$ONE_YOCTO execute '{
  "actions": [
    {
      "DecreaseCollateral": {
        "token_id": "'$DAI_TOKEN_ID'",
        "amount": "6'$DECIMAL_18'"
      }
    }
  ]
}'

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}' 
near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'
###################### End B6: Thực hiện DecreaseCollateral vs 6 DAI ########################