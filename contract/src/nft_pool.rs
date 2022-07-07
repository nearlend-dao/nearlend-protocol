use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct NftPool {
    pub owner_id: AccountId,
    pub token_id: NFTTokenId,
    #[serde(with = "u64_dec_format")]
    pub deposit_timestamp: Timestamp,
}

impl NftPool {
    pub fn new() -> Self {
        Self {
            owner_id: env::current_account_id(),
            token_id: String::from('0'),
            deposit_timestamp: env::block_timestamp(),
        }
    }

    pub fn deposit(&mut self, owner_id: AccountId, token_id: NFTTokenId) {}

    pub fn withdraw(&mut self, owner_id: AccountId, token_id: NFTTokenId) {}
}
