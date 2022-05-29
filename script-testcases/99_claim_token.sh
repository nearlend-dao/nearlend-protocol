#!/bin/bash
export MAIN_ACCOUNT=lam-test50.testnet
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
export ACCOUNT_TEST=lam-test51.testnet


######################### B3: Deposit storage #########################

# Deposit CONTRACT_ID
near call $CONTRACT_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.1
near call $CONTRACT_ID --accountId=$OWNER_ID storage_deposit '' --amount=0.1

# Deposit WETH_TOKEN_ID
# near call $WETH_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.00125
# near call $WETH_TOKEN_ID --accountId=$OWNER_ID storage_deposit '' --amount=0.00125

# Deposit DAI_TOKEN_ID
near call $DAI_TOKEN_ID --accountId=$CONTRACT_ID storage_deposit '' --amount=0.00125
near call $DAI_TOKEN_ID --accountId=$OWNER_ID storage_deposit '' --amount=0.00125


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

## Mint WETH_TOKEN_ID
# near call $WETH_TOKEN_ID --accountId=$ACCOUNT_ID mint '{
#  "account_id": "'$ACCOUNT_ID'",
#  "amount": "10000000000000000000"
# }'

## Mint DAI_TOKEN_ID
# near call $DAI_TOKEN_ID --accountId=$ACCOUNT_ID mint '{
#   "account_id": "'$ACCOUNT_ID'",
#   "amount": "100000000000000000000000"
# }'

# near call $DAI_TOKEN_ID --accountId=$ACCOUNT_TEST mint '{
#   "account_id": "'$ACCOUNT_TEST'",
#   "amount": "100000000000000000000000"
# }'

# Mint USDT_TOKEN_ID
# near call $USDT_TOKEN_ID --accountId=$ACCOUNT_ID mint '{
#   "account_id": "'$ACCOUNT_ID'",
#   "amount": "10000000000"
# }'

## Mint WNEAR_TOKEN_ID
# near call $WNEAR_TOKEN_ID --accountId=$ACCOUNT_ID de '{
#   "account_id": "'$ACCOUNT_ID'",
#   "amount": "10000000000"
# }'
near call $WNEAR_TOKEN_ID --accountId=$ACCOUNT_ID near_deposit '{}' --amount=10

###################### End B4: Mint tokens #####################
