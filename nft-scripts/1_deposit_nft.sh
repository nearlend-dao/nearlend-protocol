#!/bin/bash
export MAIN_ACCOUNT=lamns3.testnet
export NEAR_ENV=testnet
export OWNER_ID=$MAIN_ACCOUNT
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
export ACCOUNT_TEST=lamns4.testnet


################## B1: Deploy Nearland contract ##################
echo "###################### Deploy Contract #####################"
./0_nft_deploy.sh

# near call $DAI_TOKEN_ID --accountId=$OWNER_ID ft_transfer_call '{
#   "receiver_id": "'$CONTRACT_ID'",
#   "amount": "10'$DECIMAL_18'",
#   "msg": ""
# }' --amount=$ONE_YOCTO --gas=$GAS

# near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}' 
# near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'


near call $NFT_CONTRACT_ID nft_transfer_call '{
    "token_id": "1",
    "receiver_id": "'$CONTRACT_ID'",
    "approval_id": 0,
    "memo": "memo",
    "msg": "Action"
}' --accountId $ACCOUNT_TEST --depositYocto 1 --gas 300000000000000
near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

near call $NFT_CONTRACT_ID nft_transfer_call '{
    "token_id": "2",
    "receiver_id": "'$CONTRACT_ID'",
    "approval_id": 0,
    "memo": "memo",
    "msg": "Action"
}' --accountId $ACCOUNT_TEST --depositYocto 1 --gas 300000000000000
near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

near call $NFT_CONTRACT_ID nft_transfer_call '{
    "token_id": "3",
    "receiver_id": "'$CONTRACT_ID'",
    "approval_id": 0,
    "memo": "memo",
    "msg": "Action"
}' --accountId $ACCOUNT_TEST --depositYocto 1 --gas 300000000000000
near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'


# near call $CONTRACT_ID --accountId=$MAIN_ACCOUNT storage_deposit '' --amount=0.1
# near call $NFT_CONTRACT_ID nft_transfer '{
#     "token_id": "1",
#     "receiver_id": "'$MAIN_ACCOUNT'",
#     "approval_id": 0,
#     "memo": "memo",
#     "msg": "Action"
# }' --accountId $CONTRACT_ID --depositYocto 1 --gas 75000000000000

# near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'

# near call $NFT_CONTRACT_ID nft_transfer_call '{
#     "token_id": "1",
#     "receiver_id": "'$CONTRACT_ID'",
#     "approval_id": 0,
#     "memo": "memo",
#     "msg": "Action"
# }' --accountId $MAIN_ACCOUNT --depositYocto 1 --gas 75000000000000

# near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'
near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_ID'"}' 
near view $CONTRACT_ID get_account '{"account_id": "'$ACCOUNT_TEST'"}' 