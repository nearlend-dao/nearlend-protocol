use crate::*;

#[near_bindgen]
impl Contract {
    /// A method to migrate a state during the contract upgrade.
    /// Can only be called after upgrade method.
    #[private]
    #[init(ignore_state)]
    pub fn migrate_state() -> Self {
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 93b532d (Missing files)
        #[derive(BorshDeserialize, BorshSerialize)]
        pub struct OldConfig {
            /// The account ID of the oracle contract
            pub oracle_account_id: AccountId,

            /// The account ID of the contract owner that allows to modify config, assets and use reserves.
            pub owner_id: AccountId,

            /// The account ID of the booster token contract.
            pub booster_token_id: TokenId,

            /// The number of decimals of the booster fungible token.
            pub booster_decimals: u8,
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 899b7fd (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
>>>>>>> c3b16a5 (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
>>>>>>> bb5561c (Fix farm claim all, add potential farms into the account view, xBooster token)

            /// The total number of different assets
            pub max_num_assets: u32,

            /// The maximum number of seconds expected from the oracle price call.
            pub maximum_recency_duration_sec: DurationSec,

            /// Maximum staleness duration of the price data timestamp.
            /// Because NEAR protocol doesn't implement the gas auction right now, the only reason to
            /// delay the price updates are due to the shard congestion.
            /// This parameter can be updated in the future by the owner.
            pub maximum_staleness_duration_sec: DurationSec,
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 3879cfb (Missing files)
=======
>>>>>>> 899b7fd (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
>>>>>>> d2d0c2d (Missing files)
=======
>>>>>>> c3b16a5 (Fix farm claim all, add potential farms into the account view, xBooster token)
        }
=======
        let contract: Self = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
        contract
    }
>>>>>>> b9665e0 (Add remote upgrade functionality by owner)

    /// Returns semver of this contract.
    pub fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

mod upgrade {
    use near_sdk::{require, Gas};

<<<<<<< HEAD
=======
=======
>>>>>>> bb5561c (Fix farm claim all, add potential farms into the account view, xBooster token)
        }

        #[derive(BorshDeserialize)]
        pub struct OldContract {
            pub accounts: UnorderedMap<AccountId, VAccount>,
            pub storage: LookupMap<AccountId, VStorage>,
            pub assets: LookupMap<TokenId, VAsset>,
            pub asset_farms: LookupMap<FarmId, VAssetFarm>,
            pub asset_ids: UnorderedSet<TokenId>,
            pub config: LazyOption<OldConfig>,
        }

        let OldContract {
            accounts,
            storage,
            assets,
            asset_farms,
            asset_ids,
            config: old_config,
        } = env::state_read().expect("Failed to read old contract state");

>>>>>>> 93b532d (Missing files)
        let OldConfig {
            oracle_account_id,
            owner_id,
            booster_token_id,
            booster_decimals,
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
            max_num_assets,
            maximum_recency_duration_sec,
            maximum_staleness_duration_sec,
=======
>>>>>>> 3879cfb (Missing files)
=======
            max_num_assets,
            maximum_recency_duration_sec,
            maximum_staleness_duration_sec,
>>>>>>> 899b7fd (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
>>>>>>> d2d0c2d (Missing files)
=======
            max_num_assets,
            maximum_recency_duration_sec,
            maximum_staleness_duration_sec,
>>>>>>> c3b16a5 (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
>>>>>>> 93b532d (Missing files)
=======
            max_num_assets,
            maximum_recency_duration_sec,
            maximum_staleness_duration_sec,
>>>>>>> bb5561c (Fix farm claim all, add potential farms into the account view, xBooster token)
        } = old_config.get().expect("Failed to read old config");

        let new_config = Config {
            oracle_account_id,
            owner_id,
            booster_token_id,
            booster_decimals,
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 899b7fd (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
>>>>>>> c3b16a5 (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
>>>>>>> bb5561c (Fix farm claim all, add potential farms into the account view, xBooster token)
            max_num_assets,
            maximum_recency_duration_sec,
            maximum_staleness_duration_sec,
            minimum_staking_duration_sec: 2678400,
            maximum_staking_duration_sec: 31536000,
            x_booster_multiplier_at_maximum_staking_duration: 40000,
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
            max_num_assets: 10,
            maximum_recency_duration_sec: 90,
            maximum_staleness_duration_sec: 15,
>>>>>>> 3879cfb (Missing files)
=======
>>>>>>> 899b7fd (Fix farm claim all, add potential farms into the account view, xBooster token)
=======
            max_num_assets: 10,
            maximum_recency_duration_sec: 90,
            maximum_staleness_duration_sec: 15,
>>>>>>> d2d0c2d (Missing files)
=======
>>>>>>> c3b16a5 (Fix farm claim all, add potential farms into the account view, xBooster token)
        };
=======
    use super::*;
    use near_sys as sys;

    const GAS_TO_COMPLETE_UPGRADE_CALL: Gas = Gas(Gas::ONE_TERA.0 * 10);
    const GAS_FOR_GET_CONFIG_CALL: Gas = Gas(Gas::ONE_TERA.0 * 5);
    const MIN_GAS_FOR_MIGRATE_STATE_CALL: Gas = Gas(Gas::ONE_TERA.0 * 10);
>>>>>>> b9665e0 (Add remote upgrade functionality by owner)

    /// Self upgrade and call migrate, optimizes gas by not loading into memory the code.
    /// Takes as input non serialized set of bytes of the code.
    #[no_mangle]
    pub extern "C" fn upgrade() {
        env::setup_panic_hook();
        let contract: Contract = env::state_read().expect("ERR_CONTRACT_IS_NOT_INITIALIZED");
        contract.assert_owner();
        let current_account_id = env::current_account_id().as_bytes().to_vec();
        let migrate_method_name = b"migrate_state".to_vec();
        let get_config_method_name = b"get_config".to_vec();
        let empty_args = b"{}".to_vec();
        unsafe {
            sys::input(0);
            let promise_id = sys::promise_batch_create(
                current_account_id.len() as _,
                current_account_id.as_ptr() as _,
            );
            sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);
            // Gas required to complete this call.
            let required_gas =
                env::used_gas() + GAS_TO_COMPLETE_UPGRADE_CALL + GAS_FOR_GET_CONFIG_CALL;
            require!(
                env::prepaid_gas() >= required_gas + MIN_GAS_FOR_MIGRATE_STATE_CALL,
                "Not enough gas to complete state migration"
            );
            let migrate_state_attached_gas = env::prepaid_gas() - required_gas;
            // Scheduling state migration.
            sys::promise_batch_action_function_call(
                promise_id,
                migrate_method_name.len() as _,
                migrate_method_name.as_ptr() as _,
                empty_args.len() as _,
                empty_args.as_ptr() as _,
                0 as _,
                migrate_state_attached_gas.0,
            );
            // Scheduling to return config after the migration is completed.
            sys::promise_batch_action_function_call(
                promise_id,
                get_config_method_name.len() as _,
                get_config_method_name.as_ptr() as _,
                empty_args.len() as _,
                empty_args.as_ptr() as _,
                0 as _,
                GAS_FOR_GET_CONFIG_CALL.0,
            );
            sys::promise_return(promise_id);
        }
    }
=======
            max_num_assets: 10,
            maximum_recency_duration_sec: 90,
            maximum_staleness_duration_sec: 15,
=======
>>>>>>> bb5561c (Fix farm claim all, add potential farms into the account view, xBooster token)
        };

        Self {
            accounts,
            storage,
            assets,
            asset_farms,
            asset_ids,
            config: LazyOption::new(StorageKey::Config, Some(&new_config)),
        }
    }

    // TODO: Upgrade by owner.
>>>>>>> 93b532d (Missing files)
}
