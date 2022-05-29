#!/bin/bash
export MAIN_ACCOUNT=lam-test01.testnet
export NEAR_ENV=testnet
export OWNER_ID=$MAIN_ACCOUNT
# export ORACLE_ID=priceoracle.testnet
export ORACLE_ID=price-oracle.lam-test50.testnet
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
near deploy $CONTRACT_ID --accountId $ACCOUNT_ID --wasmFile ../res/nearlend.wasm

###################### End B1: Deploy Nearland contract #####################




######################### B2: Init Nearland contract #########################

echo "################### INIT CONTRACT ###################"
near call $CONTRACT_ID --accountId=$CONTRACT_ID new '{"config" : {"oracle_account_id": "'$ORACLE_ID'", "owner_id": "'$ACCOUNT_ID'", "booster_token_id": "'$BOOSTER_TOKEN_ID'", "booster_decimals": 18}}'

###################### End B2: Init Nearland contract #####################

near call $CONTRACT_ID set_diff_time '{"seconds": 10}' --accountId=$OWNER_ID
near view $CONTRACT_ID get_diff_time ''
