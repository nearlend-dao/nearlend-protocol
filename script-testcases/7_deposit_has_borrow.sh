#!/bin/bash

export MAIN_ACCOUNT=lam-test01.testnet
export NEAR_ENV=testnet
export OWNER_ID=$MAIN_ACCOUNT
export ORACLE_ID=price-oracle.lam-test50.testnet
# export ORACLE_ID=priceoracle.$MAIN_ACCOUNT
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
export ACCOUNT_TEST=lam-test02.testnet

# Chạy test deposit:
# Ví dụ: Deposit vs 10 DAI  và withdraw 5 DAI
    # B1: Chạy file deploy
    # B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID
    # B3: Thực hiện IncreaseCollateral vs 5 DAI
    # B4: Thực hiện Borrow 1 DAI


###################### B1: Chạy file deploy #####################
./0_deploy.sh
###################### End B1: Chạy file deploy #####################

near call $CONTRACT_ID set_diff_time '{"seconds": 1}' --accountId=$OWNER_ID
near view $CONTRACT_ID get_diff_time ''
###################### B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID #####################

near call $DAI_TOKEN_ID --accountId=$ACCOUNT_ID ft_transfer_call '{
  "receiver_id": "'$CONTRACT_ID'",
  "amount": "155'$DECIMAL_18'",
  "msg": ""
}' --amount=$ONE_YOCTO --gas=$GAS

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}' 
near view $CONTRACT_ID get_assets_paged_detailed '{"from_index": 0, "limit": 10}'

###################### End B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID #####################


###################### B3: Thực hiện IncreaseCollateral vs 5 DAI #####################
near call $CONTRACT_ID --accountId=$ACCOUNT_ID --gas=$GAS --amount=$ONE_YOCTO execute '{
  "actions": [
    {
      "IncreaseCollateral": {
        "token_id": "'$DAI_TOKEN_ID'",
        "amount": "100'$DECIMAL_18'"
      }
    }
  ]
}'

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}' 
near view $CONTRACT_ID get_assets_paged_detailed '{"from_index": 0, "limit": 10}'
###################### End B3: Thực hiện IncreaseCollateral vs 5 DAI #####################


###################### B4: Thực hiện Borrow 1 DAI #####################
near call $ORACLE_ID --accountId=$OWNER_ID oracle_call '{
  "receiver_id": "'$CONTRACT_ID'",
  "asset_ids": [
    "'$USDT_TOKEN_ID'",
    "'$DAI_TOKEN_ID'"
  ],
  "msg": "{\"Execute\": {\"actions\": [{\"Borrow\": {\"token_id\": \"'$DAI_TOKEN_ID'\", \"amount\": \"9'$DECIMAL_18'\"}},{\"Withdraw\":{\"token_id\":\"'$DAI_TOKEN_ID'\",\"amount\":\"9'$DECIMAL_18'\"}}]}}"
}' --amount=$ONE_YOCTO --gas=$GAS

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}'
near view $CONTRACT_ID get_assets_paged_detailed '{"from_index": 0, "limit": 10}'

###################### End B4: Thực hiện Borrow 1 DAI #####################


near call $CONTRACT_ID set_diff_time '{"seconds": 2}' --accountId=$OWNER_ID
near view $CONTRACT_ID get_diff_time ''
###################### B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID #####################

near call $DAI_TOKEN_ID --accountId=$ACCOUNT_TEST ft_transfer_call '{
  "receiver_id": "'$CONTRACT_ID'",
  "amount": "100'$DECIMAL_18'",
  "msg": ""
}' --amount=$ONE_YOCTO --gas=$GAS

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_TEST'"}' 
near view $CONTRACT_ID get_assets_paged_detailed '{"from_index": 0, "limit": 10}'

###################### End B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID #####################


# near view nearlend.lam-test6.testnet get_account '{"account_id": "lam-test6.testnet"}' 
# near view nearlend.lam-test6.testnet get_assets_paged '{"from_index": 0, "limit": 10}'


# B1: A deposit 155 DAI
# B2: A thế chấp 100 DAI
# B3: A vay 9 DAI + Withdraw 9 DAI
# => khi tính lãi suất: 155 DAI 


# B4: B deposit 10 DAI




# near view contract.main.burrow.near get_account '{"account_id": "oskarlee.near"}'

