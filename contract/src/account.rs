use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
    /// A copy of an account ID. Saves one storage_read when iterating on accounts.
    pub account_id: AccountId,
    #[serde(skip_serializing)]
    pub supplied: UnorderedMap<TokenId, VAccountAsset>,
    pub collateral: Vec<CollateralAsset>,
    pub borrowed: Vec<BorrowedAsset>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAccount {
    Current(Account),
}

impl From<VAccount> for Account {
    fn from(v: VAccount) -> Self {
        match v {
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
            collateral: vec![],
            borrowed: vec![],
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
            env::panic(b"Not enough collateral balance");
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
            env::panic(b"Not enough borrowed balance");
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

    pub fn internal_unwrap_account_with_storage(
        &self,
        account_id: &AccountId,
    ) -> (Account, Storage) {
        (
            self.internal_unwrap_account(account_id),
            self.internal_unwrap_storage(account_id),
        )
    }

    pub fn internal_unwrap_account(&self, account_id: &AccountId) -> Account {
        self.internal_get_account(account_id)
            .expect("Account is not registered")
    }

    pub fn internal_set_account(
        &mut self,
        account_id: &AccountId,
        account: Account,
        storage: Storage,
    ) {
        self.accounts.insert(account_id, &account.into());
        self.internal_set_storage(account_id, storage);
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_account(&self, account_id: ValidAccountId) -> Option<AccountDetailedView> {
        self.internal_get_account(account_id.as_ref())
            .map(|account| self.account_into_detailed_view(account))
    }

    /// This method is used to iterate on the accounts for liquidation
    pub fn get_accounts_paged(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<Account> {
        let values = self.accounts.values_as_vector();
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(values.len());
        (from_index..std::cmp::min(values.len(), limit))
            .map(|index| values.get(index).unwrap().into())
            .collect()
    }
}
