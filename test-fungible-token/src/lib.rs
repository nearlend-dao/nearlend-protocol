/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::Base64VecU8;
use near_sdk::json_types::U128;
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEARLEND_ICON: &str = "data:image/png;base64,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";

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Nearlend Dao".to_string(),
                symbol: "NEL".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEARLEND_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();
        this
    }

    pub fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        self.token.accounts.get(&account_id).unwrap_or(0).into()
    }
}
