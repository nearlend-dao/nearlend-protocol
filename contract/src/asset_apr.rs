use crate::BigDecimal;
use crate::*;

#[derive(Serialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, Deserialize))]
#[serde(crate = "near_sdk::serde")]
pub struct AssetAprView {
    pub token_id: TokenId,
    /// Current APR excluding farms for supplying the asset.
    pub supply_apr: BigDecimal,
    /// Current APR excluding farms for borrowing the asset.
    pub borrow_apr: BigDecimal,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AssetAprViewJSon {
    pub token_id: String,
    /// Current APR excluding farms for supplying the asset.
    pub supply_apr: f64,
    /// Current APR excluding farms for borrowing the asset.
    pub borrow_apr: f64,
}

impl From<AssetAprView> for AssetAprViewJSon {
    fn from(v: AssetAprView) -> Self {
        AssetAprViewJSon {
            token_id: v.token_id.to_string(),
            supply_apr: v.supply_apr.f64(),
            borrow_apr: v.borrow_apr.f64(),
        }
    }
}

impl Contract {
    pub fn asset_into_apr_view(&self, token_id: TokenId, asset: Asset) -> AssetAprViewJSon {
        let supply_apr = asset.get_supply_apr();
        let borrow_apr = asset.get_borrow_apr();
        let asset_apr_view = AssetAprView {
            token_id,
            supply_apr,
            borrow_apr,
        };

        AssetAprViewJSon::from(asset_apr_view)
    }
}
