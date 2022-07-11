use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccountNFTAsset {
    pub nft_contract_id: NFTContractId,
    pub nft_token_id: NFTTokenId,
    pub deposit_timestamp: Timestamp,
}

impl AccountNFTAsset {
    pub fn new() -> Self {
        Self {
            nft_contract_id: env::current_account_id(),
            nft_token_id: String::from("0"),
            deposit_timestamp: env::block_timestamp(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nft_token_id == String::from("0")
    }
}
impl Account {
    pub fn internal_unwrap_nft_asset(
        &self,
        nft_contract_token_id: &NFTContractTokenId,
    ) -> AccountNFTAsset {
        self.internal_get_nft_asset(nft_contract_token_id)
            .expect("NFT asset not found")
    }

    pub fn internal_get_nft_asset(
        &self,
        nft_contract_token_id: &NFTContractTokenId,
    ) -> Option<AccountNFTAsset> {
        self.nft_supplied
            .get(nft_contract_token_id)
            .map(|o| o.into())
    }

    pub fn internal_get_nft_asset_or_default(
        &mut self,
        nft_contract_token_id: &NFTContractTokenId,
    ) -> AccountNFTAsset {
        self.internal_get_nft_asset(nft_contract_token_id)
            .unwrap_or_else(AccountNFTAsset::new)
    }

    pub fn internal_set_nft_asset(
        &mut self,
        nft_contract_token_id: &NFTContractTokenId,
        account_nft_asset: AccountNFTAsset,
    ) {
        self.storage_tracker.start();
        if account_nft_asset.is_empty() {
            self.nft_supplied.remove(nft_contract_token_id);
        } else {
            self.nft_supplied
                .insert(nft_contract_token_id, &account_nft_asset.into());
        }
        self.storage_tracker.stop();
        // self.add_affected_farm(FarmId::Supplied(nft_contract_token_id.clone()));
    }
}
