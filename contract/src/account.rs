use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
    /// A copy of an account ID. Saves one storage_read when iterating on accounts.
    pub account_id: AccountId,
    /// A list of assets that are supplied by the account (but not used a collateral).
    /// It's not returned for account pagination.
    #[serde(skip_serializing)]
    pub supplied: UnorderedMap<TokenId, VAccountAsset>,
    /// A list of collateral assets.
    pub collateral: Vec<CollateralAsset>,
    /// A list of borrowed assets.
    pub borrowed: Vec<BorrowedAsset>,

    // A list of NFT assets that are supplied by the account used as a collateral.
    #[serde(skip_serializing)]
    pub nft_supplied: UnorderedMap<NFTContractTokenId, AccountNFTAsset>,

    /// Keeping track of data required for farms for this account.
    #[serde(skip_serializing)]
    pub farms: UnorderedMap<FarmId, VAccountFarm>,
    #[borsh_skip]
    #[serde(skip_serializing)]
    pub affected_farms: Vec<FarmId>,

    /// Tracks changes in storage usage by persistent collections in this account.
    #[borsh_skip]
    #[serde(skip)]
    pub storage_tracker: StorageTracker,

    /// Staking of booster token.
    pub booster_staking: Option<BoosterStaking>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAccount {
    V0(AccountV0),
    Current(Account),
}

impl From<VAccount> for Account {
    fn from(v: VAccount) -> Self {
        match v {
            VAccount::V0(c) => c.into(),
            VAccount::Current(c) => c,
        }
    }
}

impl From<Account> for VAccount {
    fn from(c: Account) -> Self {
        VAccount::Current(c)
    }
}

impl Account {
    pub fn new(account_id: &AccountId) -> Self {
        Self {
            account_id: account_id.clone(),
            supplied: UnorderedMap::new(StorageKey::AccountAssets {
                account_id: account_id.clone(),
            }),
            nft_supplied: UnorderedMap::new(StorageKey::AccountNftAssets {
                account_id: account_id.clone(),
            }),
            collateral: vec![],
            borrowed: vec![],
            farms: UnorderedMap::new(StorageKey::AccountFarms {
                account_id: account_id.clone(),
            }),
            affected_farms: vec![],
            storage_tracker: Default::default(),
            booster_staking: None,
        }
    }

    pub fn increase_collateral(&mut self, token_id: &TokenId, shares: Shares) {
        if let Some(collateral) = self.collateral.iter_mut().find(|c| &c.token_id == token_id) {
            collateral.shares.0 += shares.0;
        } else {
            self.collateral.push(CollateralAsset {
                token_id: token_id.clone(),
                shares,
            })
        }
    }

    pub fn decrease_collateral(&mut self, token_id: &TokenId, shares: Shares) {
        let index = self
            .collateral
            .iter()
            .position(|c| &c.token_id == token_id)
            .expect("Collateral not found");
        if let Some(new_balance) = self.collateral[index].shares.0.checked_sub(shares.0) {
            if new_balance > 0 {
                self.collateral[index].shares.0 = new_balance;
            } else {
                self.collateral.swap_remove(index);
            }
        } else {
            env::panic_str("Not enough collateral balance");
        }
    }

    pub fn increase_borrowed(&mut self, token_id: &TokenId, shares: Shares) {
        if let Some(borrowed) = self.borrowed.iter_mut().find(|c| &c.token_id == token_id) {
            borrowed.shares.0 += shares.0;
        } else {
            self.borrowed.push(BorrowedAsset {
                token_id: token_id.clone(),
                shares,
            })
        }
    }

    pub fn decrease_borrowed(&mut self, token_id: &TokenId, shares: Shares) {
        let index = self
            .borrowed
            .iter()
            .position(|c| &c.token_id == token_id)
            .expect("Borrowed asset not found");
        if let Some(new_balance) = self.borrowed[index].shares.0.checked_sub(shares.0) {
            if new_balance > 0 {
                self.borrowed[index].shares.0 = new_balance;
            } else {
                self.borrowed.swap_remove(index);
            }
        } else {
            env::panic_str("Not enough borrowed balance");
        }
    }

    pub fn internal_unwrap_collateral(&mut self, token_id: &TokenId) -> Shares {
        self.collateral
            .iter()
            .find(|c| &c.token_id == token_id)
            .expect("Collateral not found")
            .shares
    }

    pub fn internal_unwrap_borrowed(&mut self, token_id: &TokenId) -> Shares {
        self.borrowed
            .iter()
            .find(|c| &c.token_id == token_id)
            .expect("Borrowed asset not found")
            .shares
    }

    pub fn add_affected_farm(&mut self, farm_id: FarmId) {
        if !self.affected_farms.contains(&farm_id) {
            self.affected_farms.push(farm_id);
        }
    }

    /// Returns all assets that can be potentially farmed.
    pub fn get_all_potential_farms(&self) -> Vec<FarmId> {
        let mut potential_farms = vec![];
        for token_id in self.supplied.keys() {
            potential_farms.push(FarmId::Supplied(token_id));
        }
        for CollateralAsset { token_id, .. } in self.collateral.iter() {
            let farm_id = FarmId::Supplied(token_id.clone());
            if !potential_farms.contains(&farm_id) {
                potential_farms.push(farm_id);
            }
        }
        for BorrowedAsset { token_id, .. } in &self.borrowed {
            potential_farms.push(FarmId::Borrowed(token_id.clone()));
        }
        potential_farms
    }

    pub fn get_supplied_shares(&self, token_id: &TokenId) -> Shares {
        let collateral_shares = self
            .collateral
            .iter()
            .find(|c| &c.token_id == token_id)
            .map(|ca| ca.shares.0)
            .unwrap_or(0);
        let supplied_shares = self
            .internal_get_asset(token_id)
            .map(|asset| asset.shares.0)
            .unwrap_or(0);
        (supplied_shares + collateral_shares).into()
    }

    pub fn get_borrowed_shares(&self, token_id: &TokenId) -> Shares {
        self.borrowed
            .iter()
            .find(|b| &b.token_id == token_id)
            .map(|ba| ba.shares)
            .unwrap_or(0.into())
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CollateralAsset {
    pub token_id: TokenId,
    pub shares: Shares,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BorrowedAsset {
    pub token_id: TokenId,
    pub shares: Shares,
}

impl Contract {
    pub fn internal_get_account(&self, account_id: &AccountId) -> Option<Account> {
        self.accounts.get(account_id).map(|o| o.into())
    }

    pub fn internal_unwrap_account(&self, account_id: &AccountId) -> Account {
        self.internal_get_account(account_id)
            .expect("Account is not registered")
    }

    pub fn internal_set_account(&mut self, account_id: &AccountId, mut account: Account) {
        let mut storage = self.internal_unwrap_storage(account_id);
        storage
            .storage_tracker
            .consume(&mut account.storage_tracker);
        storage.storage_tracker.start();
        self.accounts.insert(account_id, &account.into());
        storage.storage_tracker.stop();
        self.internal_set_storage(account_id, storage);
    }
}

#[near_bindgen]
impl Contract {
    /// Returns detailed information about an account for a given account_id.
    /// The information includes all supplied assets, collateral and borrowed.
    /// Each asset includes the current balance and the number of shares.
    pub fn get_account(&self, account_id: AccountId) -> Option<AccountDetailedView> {
        self.internal_get_account(&account_id)
            .map(|account| self.account_into_detailed_view(account))
    }

    /// Returns limited account information for accounts from a given index up to a given limit.
    /// The information includes number of shares for collateral and borrowed assets.
    /// This method can be used to iterate on the accounts for liquidation.
    pub fn get_accounts_paged(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Account> {
        let values = self.accounts.values_as_vector();
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(values.len());
        (from_index..std::cmp::min(values.len(), limit))
            .map(|index| values.get(index).unwrap().into())
            .collect()
    }

    /// Returns the number of accounts
    pub fn get_num_accounts(&self) -> u32 {
        self.accounts.len() as _
    }
}
