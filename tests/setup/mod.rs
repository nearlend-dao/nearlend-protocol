#![allow(dead_code)]

use common::{AssetOptionalPrice, DurationSec, Price, PriceData, ONE_YOCTO};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FT_METADATA_SPEC};
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{env, serde_json, AccountId, Balance, Gas, Timestamp};
use near_sdk_sim::runtime::GenesisConfig;
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, ExecutionResult, UserAccount, STORAGE_AMOUNT,
};

pub use contract::{
    AccountDetailedView, Action, AssetAmount, AssetConfig, AssetDetailedView, Config,
    ContractContract as NearlendContract, PriceReceiverMsg, TokenReceiverMsg,
};
use contract::{AssetFarmView, AssetView, FarmId, NFTAsset};
use near_sdk_sim::runtime::RuntimeStandalone;
use test_oracle::ContractContract as OracleContract;

use near_contract_standards::non_fungible_token::metadata::TokenMetadata;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NEARLEND_WASM_BYTES => "res/nearlend_protocol.wasm",
    NEARLEND_0_3_0_WASM_BYTES => "res/nearlend_protocol_0.3.0.wasm",
    NEARLEND_0_4_0_WASM_BYTES => "res/nearlend_protocol_0.4.0.wasm",
    NEARLEND_PREVIOUS_WASM_BYTES => "res/nearlend_protocol_0.5.1.wasm",
    TEST_ORACLE_WASM_BYTES => "res/test_oracle.wasm",
    FUNGIBLE_TOKEN_WASM_BYTES => "res/fungible_token.wasm",
    NON_FUNGIBLE_TOKEN_WASM_BYTES => "res/nft.wasm",
}

pub fn nearlend_0_3_0_wasm_bytes() -> &'static [u8] {
    &NEARLEND_0_3_0_WASM_BYTES
}

pub fn nearlend_0_4_0_wasm_bytes() -> &'static [u8] {
    &NEARLEND_0_4_0_WASM_BYTES
}

pub fn nearlend_previous_wasm_bytes() -> &'static [u8] {
    &NEARLEND_PREVIOUS_WASM_BYTES
}

pub fn nearlend_wasm_bytes() -> &'static [u8] {
    &NEARLEND_WASM_BYTES
}

pub const NEAR: &str = "near";
pub const ORACLE_ID: &str = "oracle.near";
pub const NEARLEND_ID: &str = "nearlend.near";
pub const BOOSTER_TOKEN_ID: &str = "ft.nearlend.near";
pub const OWNER_ID: &str = "owner.near";
pub const NFT_ID: &str = "nft-nearlend.near";

pub const DEFAULT_GAS: Gas = Gas(Gas::ONE_TERA.0 * 100);
pub const MAX_GAS: Gas = Gas(Gas::ONE_TERA.0 * 300);
pub const BOOSTER_TOKEN_DECIMALS: u8 = 18;
pub const BOOSTER_TOKEN_TOTAL_SUPPLY: Balance =
    1_000_000_000 * 10u128.pow(BOOSTER_TOKEN_DECIMALS as _);

pub const DEPOSIT_TO_RESERVE: &str = "\"DepositToReserve\"";

pub const GENESIS_TIMESTAMP: u64 = 1_600_000_000 * 10u64.pow(9);

pub const ONE_DAY_SEC: DurationSec = 24 * 60 * 60;
pub const MIN_DURATION_SEC: DurationSec = 2678400;
pub const MAX_DURATION_SEC: DurationSec = 31536000;

pub const MINT_STORAGE_COST: u128 = 11280000000000000000000;

pub struct Env {
    pub root: UserAccount,
    pub near: UserAccount,
    pub owner: UserAccount,
    pub oracle: ContractAccount<OracleContract>,
    pub contract: ContractAccount<NearlendContract>,
    pub booster_token: UserAccount,
    pub nft_contract: UserAccount,
}

#[derive(Debug)]
pub struct Tokens {
    pub wnear: UserAccount,
    pub neth: UserAccount,
    pub ndai: UserAccount,
    pub nusdt: UserAccount,
    pub nusdc: UserAccount,
}

#[derive(Debug)]
pub struct Users {
    pub alice: UserAccount,
    pub bob: UserAccount,
    pub charlie: UserAccount,
    pub dude: UserAccount,
    pub eve: UserAccount,
}

pub type NFTContractId = AccountId;
pub type NFTTokenId = String;
pub type NFTContractTokenId = String;

pub fn storage_deposit(
    user: &UserAccount,
    contract_id: &AccountId,
    account_id: &AccountId,
    attached_deposit: Balance,
) {
    user.call(
        contract_id.clone(),
        "storage_deposit",
        &json!({ "account_id": account_id }).to_string().into_bytes(),
        DEFAULT_GAS.0,
        attached_deposit,
    )
    .assert_success();
}

pub fn ft_storage_deposit(
    user: &UserAccount,
    token_account_id: &AccountId,
    account_id: &AccountId,
) {
    storage_deposit(
        user,
        token_account_id,
        account_id,
        125 * env::STORAGE_PRICE_PER_BYTE,
    );
}

pub fn to_nano(timestamp: u32) -> Timestamp {
    Timestamp::from(timestamp) * 10u64.pow(9)
}

impl Env {
    pub fn init_with_contract(contract_bytes: &[u8]) -> Self {
        let mut genesis_config = GenesisConfig::default();
        genesis_config.genesis_time = GENESIS_TIMESTAMP;
        genesis_config.block_prod_time = 0;
        let root = init_simulator(Some(genesis_config));
        let near = root.create_user(
            AccountId::new_unchecked(NEAR.to_string()),
            to_yocto("1000000"),
        );
        let owner = near.create_user(
            AccountId::new_unchecked(OWNER_ID.to_string()),
            to_yocto("10000"),
        );

        let oracle = deploy!(
            contract: OracleContract,
            contract_id: ORACLE_ID.to_string(),
            bytes: &TEST_ORACLE_WASM_BYTES,
            signer_account: near,
            deposit: to_yocto("10")
        );

        let contract = deploy!(
            contract: NearlendContract,
            contract_id: NEARLEND_ID.to_string(),
            bytes: contract_bytes,
            signer_account: near,
            deposit: to_yocto("20"),
            gas: DEFAULT_GAS.0,
            init_method: new(
                Config {
                    oracle_account_id: a(ORACLE_ID),
                    owner_id: owner.account_id(),
                    booster_token_id: a(BOOSTER_TOKEN_ID),
                    booster_decimals: BOOSTER_TOKEN_DECIMALS,
                    max_num_assets: 10,
                    maximum_recency_duration_sec: 90,
                    maximum_staleness_duration_sec: 15,
                    minimum_staking_duration_sec: 2678400,
                    maximum_staking_duration_sec: 31536000,
                    x_booster_multiplier_at_maximum_staking_duration: 40000,
                    force_closing_enabled: true,
                }
            )
        );

        let booster_token = contract.user_account.deploy_and_init(
            &FUNGIBLE_TOKEN_WASM_BYTES,
            a(BOOSTER_TOKEN_ID),
            "new",
            &json!({
                "owner_id": owner.account_id(),
                "total_supply": U128::from(BOOSTER_TOKEN_TOTAL_SUPPLY),
                "metadata": FungibleTokenMetadata {
                    spec: FT_METADATA_SPEC.to_string(),
                    name: "Booster Token".to_string(),
                    symbol: "BOOSTER".to_string(),
                    icon: None,
                    reference: None,
                    reference_hash: None,
                    decimals: BOOSTER_TOKEN_DECIMALS,
                }
            })
            .to_string()
            .into_bytes(),
            to_yocto("10"),
            DEFAULT_GAS.0,
        );

        ft_storage_deposit(&owner, &a(BOOSTER_TOKEN_ID), &a(NEARLEND_ID));

        // Deploy NFT contract
        let nft_account_id = AccountId::new_unchecked(NFT_ID.to_string());
        let nft_contract = near.deploy(
            &NON_FUNGIBLE_TOKEN_WASM_BYTES,
            nft_account_id.clone(),
            STORAGE_AMOUNT,
        );

        // Init NFT contract
        nft_contract.call(
            nft_account_id,
            "new_default_meta",
            &json!({
                "owner_id": owner.account_id(),
            })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS.0,
            0,
        );

        Self {
            root,
            near,
            owner,
            contract,
            oracle,
            booster_token,
            nft_contract,
        }
    }

    pub fn init() -> Self {
        Self::init_with_contract(&NEARLEND_WASM_BYTES)
    }

    pub fn deploy_contract_by_key(&self, contract_bytes: &[u8]) -> ExecutionResult {
        self.contract
            .user_account
            .create_transaction(a(NEARLEND_ID))
            .deploy_contract(contract_bytes.to_vec())
            .function_call(
                "migrate_state".to_string(),
                b"{}".to_vec(),
                DEFAULT_GAS.0,
                0,
            )
            .submit()
    }

    pub fn deploy_contract_by_owner(&self, contract_bytes: &[u8]) -> ExecutionResult {
        self.owner
            .create_transaction(a(NEARLEND_ID))
            .function_call("upgrade".to_string(), contract_bytes.to_vec(), MAX_GAS.0, 0)
            .submit()
    }

    pub fn update_asset(&self, token_id: AccountId, asset_config: AssetConfig) {
        self.owner
            .function_call(
                self.contract.contract.update_asset(token_id, asset_config),
                DEFAULT_GAS.0,
                ONE_YOCTO,
            )
            .assert_success()
    }

    pub fn setup_assets(&self, tokens: &Tokens) {
        self.owner
            .function_call(
                self.contract.contract.add_asset(
                    self.booster_token.account_id(),
                    AssetConfig {
                        reserve_ratio: 2500,
                        target_utilization: 8000,
                        target_utilization_rate: U128(1000000000008319516250272147),
                        max_utilization_rate: U128(1000000000039724853136740579),
                        volatility_ratio: 2000,
                        extra_decimals: 0,
                        can_deposit: true,
                        can_withdraw: true,
                        can_use_as_collateral: false,
                        can_borrow: false,
                    },
                ),
                DEFAULT_GAS.0,
                ONE_YOCTO,
            )
            .assert_success();

        self.owner
            .function_call(
                self.contract.contract.add_asset(
                    tokens.neth.account_id(),
                    AssetConfig {
                        reserve_ratio: 2500,
                        target_utilization: 8000,
                        target_utilization_rate: U128(1000000000001547125956667610),
                        max_utilization_rate: U128(1000000000039724853136740579),
                        volatility_ratio: 6000,
                        extra_decimals: 0,
                        can_deposit: true,
                        can_withdraw: true,
                        can_use_as_collateral: true,
                        can_borrow: true,
                    },
                ),
                DEFAULT_GAS.0,
                ONE_YOCTO,
            )
            .assert_success();

        self.owner
            .function_call(
                self.contract.contract.add_asset(
                    tokens.ndai.account_id(),
                    AssetConfig {
                        reserve_ratio: 2500,
                        target_utilization: 8000,
                        target_utilization_rate: U128(1000000000002440418605283556),
                        max_utilization_rate: U128(1000000000039724853136740579),
                        volatility_ratio: 9500,
                        extra_decimals: 0,
                        can_deposit: true,
                        can_withdraw: true,
                        can_use_as_collateral: true,
                        can_borrow: true,
                    },
                ),
                DEFAULT_GAS.0,
                ONE_YOCTO,
            )
            .assert_success();

        self.owner
            .function_call(
                self.contract.contract.add_asset(
                    tokens.nusdt.account_id(),
                    AssetConfig {
                        reserve_ratio: 2500,
                        target_utilization: 8000,
                        target_utilization_rate: U128(1000000000002440418605283556),
                        max_utilization_rate: U128(1000000000039724853136740579),
                        volatility_ratio: 9500,
                        extra_decimals: 12,
                        can_deposit: true,
                        can_withdraw: true,
                        can_use_as_collateral: true,
                        can_borrow: true,
                    },
                ),
                DEFAULT_GAS.0,
                ONE_YOCTO,
            )
            .assert_success();

        self.owner
            .function_call(
                self.contract.contract.add_asset(
                    tokens.nusdc.account_id(),
                    AssetConfig {
                        reserve_ratio: 2500,
                        target_utilization: 8000,
                        target_utilization_rate: U128(1000000000002440418605283556),
                        max_utilization_rate: U128(1000000000039724853136740579),
                        volatility_ratio: 9500,
                        extra_decimals: 12,
                        can_deposit: true,
                        can_withdraw: true,
                        can_use_as_collateral: true,
                        can_borrow: true,
                    },
                ),
                DEFAULT_GAS.0,
                ONE_YOCTO,
            )
            .assert_success();

        self.owner
            .function_call(
                self.contract.contract.add_asset(
                    tokens.wnear.account_id(),
                    AssetConfig {
                        reserve_ratio: 2500,
                        target_utilization: 8000,
                        target_utilization_rate: U128(1000000000003593629036885046),
                        max_utilization_rate: U128(1000000000039724853136740579),
                        volatility_ratio: 6000,
                        extra_decimals: 0,
                        can_deposit: true,
                        can_withdraw: true,
                        can_use_as_collateral: true,
                        can_borrow: true,
                    },
                ),
                DEFAULT_GAS.0,
                ONE_YOCTO,
            )
            .assert_success();

        self.owner
            .function_call(
                self.contract.contract.add_asset(
                    self.nft_contract.account_id(),
                    AssetConfig {
                        reserve_ratio: 2500,
                        target_utilization: 8000,
                        target_utilization_rate: U128(0),
                        max_utilization_rate: U128(0),
                        volatility_ratio: 3000,
                        extra_decimals: 0,
                        can_deposit: true,
                        can_withdraw: true,
                        can_use_as_collateral: true,
                        can_borrow: false,
                    },
                ),
                DEFAULT_GAS.0,
                ONE_YOCTO,
            )
            .assert_success();
    }

    pub fn deposit_reserves(&self, tokens: &Tokens) {
        self.contract_ft_transfer_call(
            &tokens.wnear,
            &self.owner,
            d(10000, 24),
            DEPOSIT_TO_RESERVE,
        );
        self.contract_ft_transfer_call(&tokens.neth, &self.owner, d(10000, 18), DEPOSIT_TO_RESERVE);
        self.contract_ft_transfer_call(&tokens.ndai, &self.owner, d(10000, 18), DEPOSIT_TO_RESERVE);
        self.contract_ft_transfer_call(&tokens.nusdt, &self.owner, d(10000, 6), DEPOSIT_TO_RESERVE);
        self.contract_ft_transfer_call(&tokens.nusdc, &self.owner, d(10000, 6), DEPOSIT_TO_RESERVE);
        self.contract_ft_transfer_call(
            &self.booster_token,
            &self.owner,
            d(10000, BOOSTER_TOKEN_DECIMALS),
            DEPOSIT_TO_RESERVE,
        );
    }

    pub fn contract_ft_transfer_call(
        &self,
        token: &UserAccount,
        user: &UserAccount,
        amount: Balance,
        msg: &str,
    ) -> ExecutionResult {
        user.call(
            token.account_id.clone(),
            "ft_transfer_call",
            &json!({
                "receiver_id": self.contract.user_account.account_id(),
                "amount": U128::from(amount),
                "msg": msg,
            })
            .to_string()
            .into_bytes(),
            MAX_GAS.0,
            1,
        )
    }

    pub fn contract_nft_transfer_call(
        &self,
        user: &UserAccount,
        nft_contract_id: NFTContractId,
        nft_token_id: NFTTokenId,
        msg: &str,
    ) -> ExecutionResult {
        user.call(
            nft_contract_id,
            "nft_transfer_call",
            &json!({
                "token_id": nft_token_id,
                "receiver_id": self.contract.user_account.account_id(),
                "approval_id": 0,
                "memo": "memo",
                "msg": msg,
            })
            .to_string()
            .into_bytes(),
            MAX_GAS.0,
            1,
        )
    }

    pub fn mint_ft(&self, token: &UserAccount, receiver: &UserAccount, amount: Balance) {
        self.owner
            .call(
                token.account_id.clone(),
                "ft_transfer",
                &json!({
                    "receiver_id": receiver.account_id(),
                    "amount": U128::from(amount),
                })
                .to_string()
                .into_bytes(),
                DEFAULT_GAS.0,
                1,
            )
            .assert_success();
    }

    pub fn get_balance(&self, token: &UserAccount, account: &UserAccount) -> U128 {
        let balance: U128 = account
            .view(
                token.account_id.clone(),
                "ft_balance_of",
                &json!({
                    "account_id": account.account_id(),
                })
                .to_string()
                .into_bytes(),
            )
            .unwrap_json();
        balance
    }

    fn sample_token_metadata(&self) -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    pub fn mint_nft(&self, receiver: &UserAccount, token_id: NFTTokenId) {
        self.owner
            .call(
                self.nft_contract.account_id.clone(),
                "nft_mint",
                &json!({
                    "token_id": token_id,
                    "token_owner_id": receiver.account_id(),
                    "receiver_id": receiver.account_id(),
                    "token_metadata": self.sample_token_metadata(),
                })
                .to_string()
                .into_bytes(),
                DEFAULT_GAS.0,
                MINT_STORAGE_COST,
            )
            .assert_success();
    }

    pub fn mint_tokens(&self, tokens: &Tokens, user: &UserAccount) {
        ft_storage_deposit(user, &tokens.wnear.account_id(), &user.account_id());
        ft_storage_deposit(user, &tokens.neth.account_id(), &user.account_id());
        ft_storage_deposit(user, &tokens.ndai.account_id(), &user.account_id());
        ft_storage_deposit(user, &tokens.nusdt.account_id(), &user.account_id());
        ft_storage_deposit(user, &tokens.nusdc.account_id(), &user.account_id());
        ft_storage_deposit(user, &self.booster_token.account_id(), &user.account_id());

        let amount = 1_000_000;
        self.mint_ft(&tokens.wnear, user, d(amount, 24));
        self.mint_ft(&tokens.neth, user, d(amount, 18));
        self.mint_ft(&tokens.ndai, user, d(amount, 18));
        self.mint_ft(&tokens.nusdt, user, d(amount, 6));
        self.mint_ft(&tokens.nusdc, user, d(amount, 6));
        self.mint_ft(&self.booster_token, user, d(amount, BOOSTER_TOKEN_DECIMALS));
    }

    pub fn get_asset(&self, token: &UserAccount) -> AssetDetailedView {
        let asset: Option<AssetDetailedView> = self
            .near
            .view_method_call(self.contract.contract.get_asset(token.account_id()))
            .unwrap_json();
        asset.unwrap()
    }

    pub fn get_asset_farm(&self, farm_id: FarmId) -> AssetFarmView {
        let asset_farm: Option<serde_json::value::Value> = self
            .near
            .view_method_call(self.contract.contract.get_asset_farm(farm_id.clone()))
            .unwrap_json();
        let asset_farm = asset_farm.unwrap();
        AssetFarmView {
            farm_id,
            rewards: serde_json::from_value(asset_farm["rewards"].clone()).unwrap(),
        }
    }

    pub fn get_account(&self, user: &UserAccount) -> AccountDetailedView {
        let account: Option<AccountDetailedView> = self
            .near
            .view_method_call(self.contract.contract.get_account(user.account_id()))
            .unwrap_json();
        account.unwrap()
    }

    pub fn storage_balance_of(&self, user: &UserAccount) -> Option<StorageBalance> {
        self.near
            .view_method_call(self.contract.contract.storage_balance_of(user.account_id()))
            .unwrap_json()
    }

    pub fn debug_storage_balance_of(&self, user: &UserAccount) -> Option<StorageBalance> {
        self.near
            .view_method_call(
                self.contract
                    .contract
                    .debug_storage_balance_of(user.account_id()),
            )
            .unwrap_json()
    }

    pub fn supply_to_collateral(
        &self,
        user: &UserAccount,
        token: &UserAccount,
        amount: Balance,
    ) -> ExecutionResult {
        self.contract_ft_transfer_call(token, user, amount, "")
    }

    pub fn supply_nft_to_collateral(
        &self,
        user: &UserAccount,
        nft_contract_id: NFTContractId,
        nft_token_id: NFTTokenId,
    ) -> ExecutionResult {
        self.contract_nft_transfer_call(user, nft_contract_id, nft_token_id, "")
    }

    pub fn deposit_and_repay(
        &self,
        user: &UserAccount,
        token: &UserAccount,
        amount: Balance,
    ) -> ExecutionResult {
        let msg = serde_json::to_string(&TokenReceiverMsg::Execute {
            actions: vec![Action::Repay(asset_amount(token, amount))],
        })
        .unwrap();
        self.contract_ft_transfer_call(token, user, amount, &msg)
    }

    pub fn oracle_call(
        &self,
        user: &UserAccount,
        price_data: PriceData,
        msg: PriceReceiverMsg,
    ) -> ExecutionResult {
        user.function_call(
            self.oracle.contract.oracle_call(
                self.contract.user_account.account_id(),
                price_data,
                serde_json::to_string(&msg).unwrap(),
            ),
            MAX_GAS.0,
            ONE_YOCTO,
        )
    }

    pub fn borrow(
        &self,
        user: &UserAccount,
        token: &UserAccount,
        price_data: PriceData,
        amount: Balance,
    ) -> ExecutionResult {
        self.oracle_call(
            user,
            price_data,
            PriceReceiverMsg::Execute {
                actions: vec![Action::Borrow(asset_amount(token, amount))],
            },
        )
    }

    pub fn withdraw(
        &self,
        user: &UserAccount,
        token: &UserAccount,
        price_data: PriceData,
        amount: Balance,
    ) -> ExecutionResult {
        self.oracle_call(
            user,
            price_data,
            PriceReceiverMsg::Execute {
                actions: vec![Action::Withdraw(asset_amount(token, amount))],
            },
        )
    }

    pub fn withdraw_nft(
        &self,
        user: &UserAccount,
        price_data: PriceData,
        nft_conntract_id: NFTContractId,
        nft_token_id: NFTTokenId,
    ) -> ExecutionResult {
        self.oracle_call(
            user,
            price_data,
            PriceReceiverMsg::Execute {
                actions: vec![Action::WithdrawNFT(nft_asset(
                    nft_conntract_id,
                    nft_token_id,
                ))],
            },
        )
    }

    pub fn borrow_and_withdraw(
        &self,
        user: &UserAccount,
        token: &UserAccount,
        price_data: PriceData,
        amount: Balance,
    ) -> ExecutionResult {
        self.oracle_call(
            user,
            price_data,
            PriceReceiverMsg::Execute {
                actions: vec![
                    Action::Borrow(asset_amount(token, amount)),
                    Action::Withdraw(asset_amount(token, amount)),
                ],
            },
        )
    }

    pub fn borrow_and_withdraw_nft(
        &self,
        user: &UserAccount,
        token: &UserAccount,
        price_data: PriceData,
        amount: Balance,
        nft_conntract_id: NFTContractId,
        nft_token_id: NFTTokenId,
    ) -> ExecutionResult {
        self.oracle_call(
            user,
            price_data,
            PriceReceiverMsg::Execute {
                actions: vec![
                    Action::Borrow(asset_amount(token, amount)),
                    Action::Withdraw(asset_amount(token, amount)),
                    Action::WithdrawNFT(nft_asset(nft_conntract_id, nft_token_id)),
                ],
            },
        )
    }

    pub fn liquidate(
        &self,
        user: &UserAccount,
        liquidation_user: &UserAccount,
        price_data: PriceData,
        in_assets: Vec<AssetAmount>,
        out_assets: Vec<AssetAmount>,
    ) -> ExecutionResult {
        self.oracle_call(
            user,
            price_data,
            PriceReceiverMsg::Execute {
                actions: vec![Action::Liquidate {
                    account_id: liquidation_user.account_id(),
                    in_assets,
                    out_assets,
                }],
            },
        )
    }

    pub fn liquidate_nft(
        &self,
        user: &UserAccount,
        liquidation_user: &UserAccount,
        price_data: PriceData,
        in_assets: Vec<AssetAmount>,
        out_nft_assets: Vec<NFTAsset>,
    ) -> ExecutionResult {
        self.oracle_call(
            user,
            price_data,
            PriceReceiverMsg::Execute {
                actions: vec![Action::LiquidateNFT {
                    account_id: liquidation_user.account_id(),
                    in_assets,
                    out_nft_assets,
                }],
            },
        )
    }

    pub fn force_close(
        &self,
        user: &UserAccount,
        liquidation_user: &UserAccount,
        price_data: PriceData,
    ) -> ExecutionResult {
        self.oracle_call(
            user,
            price_data,
            PriceReceiverMsg::Execute {
                actions: vec![Action::ForceClose {
                    account_id: liquidation_user.account_id(),
                }],
            },
        )
    }

    pub fn skip_time(&self, seconds: u32) {
        self.near.borrow_runtime_mut().cur_block.block_timestamp += to_nano(seconds);
    }

    pub fn account_stake_booster(
        &self,
        user: &UserAccount,
        amount: Balance,
        duration: DurationSec,
    ) -> ExecutionResult {
        user.function_call(
            self.contract
                .contract
                .account_stake_booster(Some(U128::from(amount)), duration),
            DEFAULT_GAS.0,
            1,
        )
    }

    pub fn account_unstake_booster(&self, user: &UserAccount) -> ExecutionResult {
        user.function_call(
            self.contract.contract.account_unstake_booster(),
            DEFAULT_GAS.0,
            1,
        )
    }

    pub fn add_farm(
        &self,
        farm_id: FarmId,
        reward_token: &UserAccount,
        new_reward_per_day: Balance,
        new_booster_log_base: Balance,
        reward_amount: Balance,
    ) {
        self.owner
            .function_call(
                self.contract.contract.add_asset_farm_reward(
                    farm_id,
                    reward_token.account_id(),
                    U128::from(new_reward_per_day),
                    U128::from(new_booster_log_base),
                    U128::from(reward_amount),
                ),
                DEFAULT_GAS.0,
                1,
            )
            .assert_success();
    }

    pub fn account_farm_claim_all(&self, user: &UserAccount) -> ExecutionResult {
        user.function_call(
            self.contract.contract.account_farm_claim_all(None),
            MAX_GAS.0,
            0,
        )
    }

    pub fn account_farm_claim_all_on_behalf(
        &self,
        caller: &UserAccount,
        user: &UserAccount,
    ) -> ExecutionResult {
        caller.function_call(
            self.contract
                .contract
                .account_farm_claim_all(Some(user.account_id())),
            MAX_GAS.0,
            0,
        )
    }
}

pub fn init_token(e: &Env, token_account_id: &AccountId, decimals: u8) -> UserAccount {
    let token = e.near.deploy_and_init(
        &FUNGIBLE_TOKEN_WASM_BYTES,
        token_account_id.clone(),
        "new",
        &json!({
            "owner_id": e.owner.account_id(),
            "total_supply": U128::from(10u128.pow((9 + decimals) as _)),
            "metadata": FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: token_account_id.to_string(),
                symbol: token_account_id.to_string(),
                icon: None,
                reference: None,
                reference_hash: None,
                decimals,
            }
        })
        .to_string()
        .into_bytes(),
        to_yocto("10"),
        DEFAULT_GAS.0,
    );

    ft_storage_deposit(&e.owner, token_account_id, &e.contract.account_id());
    token
}

impl Tokens {
    pub fn init(e: &Env) -> Self {
        Self {
            wnear: init_token(e, &a("wrap.near"), 24),
            neth: init_token(e, &a("neth.near"), 18),
            ndai: init_token(e, &a("dai.near"), 18),
            nusdt: init_token(e, &a("nusdt.near"), 6),
            nusdc: init_token(e, &a("nusdc.near"), 6),
        }
    }
}

impl Users {
    pub fn init(e: &Env) -> Self {
        Self {
            alice: e.near.create_user(a("alice.near"), to_yocto("10000")),
            bob: e.near.create_user(a("bob.near"), to_yocto("10000")),
            charlie: e.near.create_user(a("charlie.near"), to_yocto("10000")),
            dude: e.near.create_user(a("dude.near"), to_yocto("10000")),
            eve: e.near.create_user(a("eve.near"), to_yocto("10000")),
        }
    }
}

pub fn d(value: Balance, decimals: u8) -> Balance {
    value * 10u128.pow(decimals as _)
}
pub fn a(account_id: &str) -> AccountId {
    AccountId::new_unchecked(account_id.to_string())
}

pub fn price_data(
    tokens: &Tokens,
    wnear_mul: Option<Balance>,
    neth_mul: Option<Balance>,
    nft_mul: Option<Balance>,
) -> PriceData {
    let mut prices = vec![
        AssetOptionalPrice {
            asset_id: tokens.ndai.account_id().to_string(),
            price: Some(Price {
                multiplier: 10000,
                decimals: 22,
            }),
        },
        AssetOptionalPrice {
            asset_id: tokens.nusdc.account_id().to_string(),
            price: Some(Price {
                multiplier: 10000,
                decimals: 10,
            }),
        },
        AssetOptionalPrice {
            asset_id: tokens.nusdt.account_id().to_string(),
            price: Some(Price {
                multiplier: 10000,
                decimals: 10,
            }),
        },
    ];
    if let Some(wnear_mul) = wnear_mul {
        prices.push(AssetOptionalPrice {
            asset_id: tokens.wnear.account_id().to_string(),
            price: Some(Price {
                multiplier: wnear_mul,
                decimals: 28,
            }),
        })
    }
    if let Some(neth_mul) = neth_mul {
        prices.push(AssetOptionalPrice {
            asset_id: tokens.neth.account_id().to_string(),
            price: Some(Price {
                multiplier: neth_mul,
                decimals: 22,
            }),
        })
    }
    if let Some(nft_mul) = nft_mul {
        prices.push(AssetOptionalPrice {
            asset_id: NFT_ID.to_string(),
            price: Some(Price {
                multiplier: nft_mul,
                decimals: 28,
            }),
        })
    }

    PriceData {
        timestamp: tokens.wnear.borrow_runtime().cur_block.block_timestamp,
        recency_duration_sec: 90,
        prices,
    }
}

pub fn basic_setup_with_contract(contract_bytes: &[u8]) -> (Env, Tokens, Users) {
    let e = Env::init_with_contract(contract_bytes);
    let tokens = Tokens::init(&e);
    e.setup_assets(&tokens);
    e.deposit_reserves(&tokens);

    let users = Users::init(&e);
    e.mint_tokens(&tokens, &users.alice);
    storage_deposit(
        &users.alice,
        &e.contract.account_id(),
        &users.alice.account_id(),
        d(1, 23),
    );
    e.mint_tokens(&tokens, &users.bob);
    storage_deposit(
        &users.bob,
        &e.contract.account_id(),
        &users.bob.account_id(),
        d(1, 23),
    );
    e.mint_tokens(&tokens, &users.charlie);
    storage_deposit(
        &users.charlie,
        &e.contract.account_id(),
        &users.charlie.account_id(),
        d(1, 23),
    );

    (e, tokens, users)
}

pub fn basic_setup() -> (Env, Tokens, Users) {
    basic_setup_with_contract(&NEARLEND_WASM_BYTES)
}

pub fn sec_to_nano(sec: u32) -> u64 {
    u64::from(sec) * 10u64.pow(9)
}

pub fn asset_amount(token: &UserAccount, amount: Balance) -> AssetAmount {
    AssetAmount {
        token_id: token.account_id(),
        amount: Some(amount.into()),
        max_amount: None,
    }
}

pub fn nft_asset(nft_contract_id: NFTContractId, token_id: NFTTokenId) -> NFTAsset {
    NFTAsset {
        nft_contract_id,
        token_id,
    }
}

pub const EVENT_JSON: &str = "EVENT_JSON:";

pub fn get_logs(runtime: &RuntimeStandalone) -> Vec<String> {
    runtime
        .last_outcomes
        .iter()
        .flat_map(|hash| runtime.outcome(hash).map(|o| o.logs).unwrap_or_default())
        .collect()
}

pub fn find_asset<'a>(assets: &'a [AssetView], token_id: &AccountId) -> &'a AssetView {
    let msg = format!("Missing asset: {:?}", token_id);
    assets
        .iter()
        .find(|e| &e.token_id == token_id)
        .unwrap_or_else(|| panic!("{}", msg))
}

pub fn assert_balances(actual: &[AssetView], expected: &[AssetView]) {
    assert_eq!(actual.len(), expected.len());
    for asset in actual {
        assert_eq!(asset.balance, find_asset(expected, &asset.token_id).balance);
    }
}

pub fn av(token_id: AccountId, balance: Balance) -> AssetView {
    AssetView {
        token_id,
        balance,
        shares: U128(0),
        apr: Default::default(),
    }
}

pub fn almost_eq(a: u128, b: u128, prec: u32) {
    let p = 10u128.pow(27 - prec);
    let ap = (a + p / 2) / p;
    let bp = (b + p / 2) / p;
    assert_eq!(
        ap,
        bp,
        "{}",
        format!("Expected {} to eq {}, with precision {}", a, b, prec)
    );
}
