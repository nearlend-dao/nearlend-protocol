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
export ACCOUNT_ID_TEST_1=lam-test11.testnet
export ACCOUNT_ID_TEST_2=lam-test12.testnet
export ACCOUNT_ID_TEST_3=lam-test13.testnet

# Chạy test deposit:
# Ví dụ: Deposit vs 10 DAI 
    # B1: Chạy file deploy
    # B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID


###################### B1: Chạy file deploy #####################
./0_deploy.sh
###################### End B1: Chạy file deploy #####################


###################### B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID #####################

near call $DAI_TOKEN_ID --accountId=$OWNER_ID ft_transfer_call '{
  "receiver_id": "'$CONTRACT_ID'",
  "amount": "10'$DECIMAL_18'",
  "msg": ""
}' --amount=$ONE_YOCTO --gas=$GAS

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}' 
near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

###################### End B2: Thực hiện deposit vs 10 DAI vào ACCOUNT_ID #####################