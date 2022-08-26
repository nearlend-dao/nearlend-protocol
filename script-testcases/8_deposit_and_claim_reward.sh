#!/bin/bash
export MAIN_ACCOUNT=nearlend-official
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
export ACCOUNT_TEST=lamns3.testnet



##################### B1: Chạy file deploy #####################
# ./0_deploy.sh
##################### End B1: Chạy file deploy #####################


##################### B2: Deposit 10 DAI #####################
# near call $DAI_TOKEN_ID --accountId=$ACCOUNT_TEST ft_transfer_call '{
#   "receiver_id": "'$CONTRACT_ID'",
#   "amount": "10'$DECIMAL_18'",
#   "msg": ""
# }' --amount=$ONE_YOCTO --gas=$GAS

near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_TEST'"}' 
# near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

##################### End B2: Deposit 10 DAI #####################

###################### B3: Claim token NEL #####################

# Claim Reward token
near call $CONTRACT_ID --accountId=$MAIN_ACCOUNT account_farm_claim_all '{"account_id":  "'$ACCOUNT_TEST'"}'  --gas=$GAS
near call $CONTRACT_ID  --accountId=$ACCOUNT_TEST get_account '{"account_id": "'$ACCOUNT_TEST'"}' 

###################### End B3: Claim token NEL ##################### 991,158.02497 991,159.28991  1234975859965277777