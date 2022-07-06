#!/bin/bash
export NEAR_ENV=testnet
export ACCOUNT_ID=lamns3.testnet
export CONTRACT_ID=nft.$ACCOUNT_ID
export DECIMAL_18=000000000000000000
export ACCOUNT_TEST=lamns4.testnet 



################### B1: Deploy Nearland contract ##################

echo "################### DELETE ACCOUNT ###################"
near delete $CONTRACT_ID $ACCOUNT_ID


echo "################### CREATE ACCOUNT ###################"

near create-account $CONTRACT_ID --masterAccount $ACCOUNT_ID --initialBalance 4
# near create-account $ACCOUNT_TEST --masterAccount $ACCOUNT_ID --initialBalance 10

echo "################### CREATE CONTRACT ###################"
near deploy $CONTRACT_ID --accountId $ACCOUNT_ID --wasmFile ../res/nft.wasm 

echo "################### Init Contract ###################"
near call $CONTRACT_ID --accountId=$ACCOUNT_ID new_default_meta '{"owner_id" : "'$ACCOUNT_ID'"}'

near view $CONTRACT_ID nft_metadata

echo "################### Mint NFT ###################"
###### mint vào account test: lam-test04.testnet
near call $CONTRACT_ID nft_mint '{"token_id": "1", "token_owner_id": "'$ACCOUNT_TEST'", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://nearlend.web.app/static/media/nft_circle.7266257755e818ec83f1.jpeg", "copies": 1}}' --accountId $ACCOUNT_ID --deposit 0.1
near call $CONTRACT_ID nft_mint '{"token_id": "2", "token_owner_id": "'$ACCOUNT_TEST'", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://nearlend.web.app/static/media/nft_samurai.904528e4d6cfc1563126.jpeg", "copies": 1}}' --accountId $ACCOUNT_ID --deposit 0.1
near call $CONTRACT_ID nft_mint '{"token_id": "3", "token_owner_id": "'$ACCOUNT_TEST'", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://public.nftstatic.com/static/nft/zipped/38be5d8e2eac425cb87e714380cc91f1_zipped.gif", "copies": 1}}' --accountId $ACCOUNT_ID --deposit 0.1


echo "########## List NFT ################"
near view $CONTRACT_ID nft_tokens '{"from_index": "0", "limit": 10}'

