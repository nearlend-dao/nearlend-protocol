#!/bin/bash
export CONTRACT_ID=contract.main.burrow.near
export NEAR_ENV=mainnet

near view $CONTRACT_ID get_assets_paged '{"from_index": 0, "limit": 10}'