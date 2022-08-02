use crate::*;

/// V0 legacy version of Account structure, before staking of the nearlend token was introduced.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccountV0 {
    /// A copy of an account ID. Saves one storage_read when iterating on accounts.
    pub account_id: AccountId,
    /// A list of assets that are supplied by the account (but not used a collateral).
    /// It's not returned for account pagination.
    pub supplied: HashMap<TokenId, Shares>,
    /// A list of borrowed assets.
    pub borrowed: HashMap<TokenId, Shares>,
    /// A list of nft supplied assets.
    pub nft_supplied: HashMap<NFTContractTokenId, AccountNFTAsset>,
    /// Keeping track of data required for farms for this account.
    pub farms: HashMap<FarmId, AccountFarm>,
}

impl From<AccountV0> for Account {
    fn from(a: AccountV0) -> Self {
        let AccountV0 {
            account_id,
            supplied,
            borrowed,
            nft_supplied,
            farms,
        } = a;
        Self {
            account_id,
            supplied,
            borrowed,
            nft_supplied,
            farms,
            affected_farms: Default::default(),
            storage_tracker: Default::default(),
            booster_staking: None,
        }
    }
}
