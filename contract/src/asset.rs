use crate::*;

pub const MS_PER_YEAR: u64 = 31536000000;
pub const MAX_ITEMS: usize = 99999;

static ASSETS: Lazy<Mutex<HashMap<TokenId, Option<Asset>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Deserialize))]
#[serde(crate = "near_sdk::serde")]
pub struct Asset {
    /// Total supplied including collateral, but excluding reserved.
    pub supplied: Pool,
    /// Total borrowed.
    pub borrowed: Pool,
    pub nft_supplied: Vec<NftPool>,
    /// The amount reserved for the stability. This amount can also be borrowed and affects
    /// borrowing rate.
    #[serde(with = "u128_dec_format")]
    pub reserved: Balance,
    /// When the asset was last updated. It's always going to be the current block timestamp.
    #[serde(with = "u64_dec_format")]
    pub last_update_timestamp: Timestamp,
    /// The asset config.
    pub config: AssetConfig,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAsset {
    Current(Asset),
}

impl From<VAsset> for Asset {
    fn from(v: VAsset) -> Self {
        match v {
            VAsset::Current(c) => c,
        }
    }
}

impl From<Asset> for VAsset {
    fn from(c: Asset) -> Self {
        VAsset::Current(c)
    }
}

impl Asset {
    pub fn new(timestamp: Timestamp, config: AssetConfig) -> Self {
        Self {
            supplied: Pool::new(),
            borrowed: Pool::new(),
            nft_supplied: vec![],
            reserved: 0,
            last_update_timestamp: timestamp,
            config,
        }
    }

    pub fn get_rate(&self) -> BigDecimal {
        self.config
            .get_rate(self.borrowed.balance, self.supplied.balance + self.reserved)
    }

    pub fn get_borrow_apr(&self) -> BigDecimal {
        let rate = self.get_rate();
        rate.pow(MS_PER_YEAR) - BigDecimal::one()
    }

    pub fn get_supply_apr(&self) -> BigDecimal {
        if self.supplied.balance == 0 || self.borrowed.balance == 0 {
            return BigDecimal::zero();
        }

        let borrow_apr = self.get_borrow_apr();
        if borrow_apr == BigDecimal::zero() {
            return borrow_apr;
        }

        let interest = borrow_apr.round_mul_u128(self.borrowed.balance);
        let supply_interest = ratio(interest, MAX_RATIO - self.config.reserve_ratio);
        BigDecimal::from(supply_interest).div_u128(self.supplied.balance)
    }

    // n = 31536000000 ms in a year (365 days)
    //
    // Compute `r` from `X`. `X` is desired APY
    // (1 + r / n) ** n = X (2 == 200%)
    // n * log(1 + r / n) = log(x)
    // log(1 + r / n) = log(x) / n
    // log(1 + r  / n) = log( x ** (1 / n))
    // 1 + r / n = x ** (1 / n)
    // r / n = (x ** (1 / n)) - 1
    // r = n * ((x ** (1 / n)) - 1)
    // n = in millis
    fn compound(&mut self, time_diff_ms: Duration) {
        let rate = self.get_rate();
        let interest =
            rate.pow(time_diff_ms).round_mul_u128(self.borrowed.balance) - self.borrowed.balance;
        // TODO: Split interest based on ratio between reserved and supplied?
        let reserved = ratio(interest, self.config.reserve_ratio);
        if self.supplied.shares.0 > 0 {
            self.supplied.balance += interest - reserved;
            self.reserved += reserved;
        } else {
            self.reserved += interest;
        }
        self.borrowed.balance += interest;
    }

    pub fn update(&mut self) {
        let timestamp = env::block_timestamp();
        let time_diff_ms = nano_to_ms(timestamp - self.last_update_timestamp);
        if time_diff_ms > 0 {
            // update
            self.last_update_timestamp += ms_to_nano(time_diff_ms);
            self.compound(time_diff_ms);
        }
    }

    pub fn available_amount(&self) -> Balance {
        self.supplied.balance + self.reserved - self.borrowed.balance
    }

    /// Get the NFT owner from asset NFT pool
    pub fn get_owner_nft(&self, token_id: &NFTTokenId, asset: &Asset) -> Option<AccountId> {
        let index = asset
            .nft_supplied
            .iter()
            .position(|x| *x.token_id == token_id.clone())
            .unwrap_or(MAX_ITEMS);

        if index != MAX_ITEMS {
            let nft_pool = asset.nft_supplied.get(index).unwrap();
            return Some(nft_pool.owner_id.clone());
        }

        None
    }
}

impl Contract {
    pub fn internal_unwrap_asset(&self, token_id: &TokenId) -> Asset {
        self.internal_get_asset(token_id).expect("Asset not found")
    }

    pub fn internal_get_asset(&self, token_id: &TokenId) -> Option<Asset> {
        let mut cache = ASSETS.lock().unwrap();
        cache.get(token_id).cloned().unwrap_or_else(|| {
            let asset = self.assets.get(token_id).map(|o| {
                let mut asset: Asset = o.into();
                asset.update();
                asset
            });
            cache.insert(token_id.clone(), asset.clone());
            asset
        })
    }

    pub fn internal_set_nft_asset(
        &mut self,
        nft_contract_id: &NFTContractId,
        owner_id: AccountId,
        token_id: NFTTokenId,
        mut asset: Asset,
    ) {
        let index = asset
            .nft_supplied
            .iter()
            .position(|x| *x.token_id == token_id)
            .unwrap_or(MAX_ITEMS);

        let mut deposit_timestamp = env::block_timestamp();
        if index != MAX_ITEMS {
            let current_nft = asset.nft_supplied.get(index).expect("Can't find the nft");
            deposit_timestamp = current_nft.deposit_timestamp;
            asset.nft_supplied.remove(index);
        }

        asset.nft_supplied.push(NftPool {
            owner_id,
            token_id,
            deposit_timestamp,
        });

        ASSETS
            .lock()
            .unwrap()
            .insert(nft_contract_id.clone(), Some(asset.clone()));
        self.assets.insert(nft_contract_id, &asset.into());
    }

    pub fn internal_remove_nft_asset(
        &mut self,
        nft_contract_id: &NFTContractId,
        token_id: NFTTokenId,
        mut asset: Asset,
    ) {
        let index = asset
            .nft_supplied
            .iter()
            .position(|x| *x.token_id == token_id)
            .unwrap_or(MAX_ITEMS);
        if index != MAX_ITEMS {
            asset.nft_supplied.remove(index);
        }

        ASSETS
            .lock()
            .unwrap()
            .insert(nft_contract_id.clone(), Some(asset.clone()));
        self.assets.insert(nft_contract_id, &asset.into());
    }

    pub fn internal_set_asset(&mut self, token_id: &TokenId, mut asset: Asset) {
        if asset.supplied.shares.0 == 0 && asset.supplied.balance > 0 {
            asset.reserved += asset.supplied.balance;
            asset.supplied.balance = 0;
        }
        assert!(
            asset.borrowed.shares.0 > 0 || asset.borrowed.balance == 0,
            "Borrowed invariant broken"
        );
        asset.supplied.assert_invariant();
        asset.borrowed.assert_invariant();
        ASSETS
            .lock()
            .unwrap()
            .insert(token_id.clone(), Some(asset.clone()));
        self.assets.insert(token_id, &asset.into());
    }
}

#[near_bindgen]
impl Contract {
    /// Returns an asset for a given token_id.
    pub fn get_asset(&self, token_id: AccountId) -> Option<AssetDetailedView> {
        self.internal_get_asset(&token_id)
            .map(|asset| self.asset_into_detailed_view(token_id, asset))
    }

    /// Returns an list of pairs (token_id, asset) for assets a given list of token_id.
    /// Only returns pais for existing assets.
    pub fn get_assets(&self, token_ids: Vec<AccountId>) -> Vec<AssetDetailedView> {
        token_ids
            .into_iter()
            .filter_map(|token_id| {
                self.internal_get_asset(&token_id)
                    .map(|asset| self.asset_into_detailed_view(token_id, asset))
            })
            .collect()
    }

    /// Returns a list of pairs (token_id, asset) for assets from a given index up to a given limit.
    pub fn get_assets_paged(
        &self,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<(TokenId, Asset)> {
        let keys = self.asset_ids.as_vector();
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(keys.len());
        (from_index..std::cmp::min(keys.len(), from_index + limit))
            .map(|index| {
                let key = keys.get(index).unwrap();
                let mut asset: Asset = self.assets.get(&key).unwrap().into();
                asset.update();
                (key, asset)
            })
            .collect()
    }

    pub fn get_assets_paged_detailed(
        &self,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<AssetDetailedView> {
        let keys = self.asset_ids.as_vector();
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(keys.len());
        (from_index..std::cmp::min(keys.len(), from_index + limit))
            .map(|index| {
                let token_id = keys.get(index).unwrap();
                let mut asset: Asset = self.assets.get(&token_id).unwrap().into();
                asset.update();
                self.asset_into_detailed_view(token_id, asset)
            })
            .collect()
    }

    pub fn get_assets_apr(
        &self,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<AssetAprViewJSon> {
        let keys = self.asset_ids.as_vector();
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(keys.len());
        (from_index..std::cmp::min(keys.len(), from_index + limit))
            .map(|index| {
                let token_id = keys.get(index).unwrap();
                let mut asset: Asset = self.assets.get(&token_id).unwrap().into();
                asset.update();
                self.asset_into_apy_view(token_id, asset)
            })
            .collect()
    }

    /// Returns the NFTs from the asset
    pub fn get_nft_assets_paged(
        &self,
        nft_contract_id: NFTContractId,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<NftPool> {
        let mut nft_asset: Asset = self
            .assets
            .get(&nft_contract_id)
            .expect("Can't get the NFT asset")
            .into();

        nft_asset.nft_supplied.reverse();
        let values = nft_asset.nft_supplied;
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(values.len() as u64);
        (from_index..std::cmp::min(values.len() as u64, from_index + limit))
            .map(|index| values.get(index as usize).unwrap().clone())
            .collect()
    }

    /// Returns the number of NFTs
    pub fn get_num_nfts(&self, nft_contract_id: NFTContractId) -> u32 {
        let nft_asset: Asset = self
            .assets
            .get(&nft_contract_id)
            .expect("Can't get the NFT asset")
            .into();

        nft_asset.nft_supplied.len() as _
    }
}
