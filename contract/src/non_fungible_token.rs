use crate::*;
use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, is_promise_success, log, near_bindgen, require, AccountId, Gas, PanicOnDefault,
    PromiseOrValue,
};

// const GAS_FOR_NFT_TRANSFER: Gas = Gas(Gas::ONE_TERA.0 * 20);
const GAS_FOR_NFT_TRANSFER: Gas = Gas(10_000_000_000_000); // Gas(BASE_GAS) + Gas(PROMISE_CALL);

#[ext_contract(ext_nft_contract)]
pub trait NonFungibleTokenCore {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn after_nft_transfer(
        &mut self,
        account_id: AccountId,
        nft_contract_id: NFTContractId,
        token_id: TokenId,
    ) -> bool;
}

trait ExtSelf {
    fn after_nft_transfer(
        &mut self,
        account_id: AccountId,
        nft_contract_id: NFTContractId,
        token_id: TokenId,
    ) -> bool;
}

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        let nft_contract_id = env::predecessor_account_id();
        let asset = self.internal_unwrap_asset(&nft_contract_id);
        assert!(
            asset.config.can_deposit,
            "Deposits for this asset are not enabled"
        );

        let mut account = self.internal_unwrap_account(&sender_id);
        account.add_affected_farm(FarmId::Supplied(nft_contract_id.clone()));

        // Add NFT to the account assets
        self.internal_nft_deposit(&mut account, &nft_contract_id, &token_id);

        // Save all change to the account
        self.internal_set_account(&sender_id, account);

        log!(
            "in nft_on_transfer; sender_id={}, previous_owner_id={}, token_id={}, msg={}",
            &sender_id,
            &previous_owner_id,
            &token_id,
            msg
        );
        log!("====> message from sender: {:?}", msg);
        // Add NFT to asset
        self.internal_set_nft_asset(&nft_contract_id, sender_id.clone(), token_id.clone(), asset);
        PromiseOrValue::Value(false)
    }
}

#[near_bindgen]
impl Contract {
    pub fn internal_nft_transfer(
        &mut self,
        account_id: &AccountId,
        nft_contract_id: &NFTContractId,
        token_id: &NFTTokenId,
    ) -> Promise {
        log!(
            "======> nft_transfer: nft_contract_id: {:?}, token_id: {:?}, receiver_id {:?}",
            nft_contract_id.clone(),
            token_id.clone(),
            account_id.clone()
        );

        ext_nft_contract::nft_transfer(
            account_id.clone(),
            token_id.clone(),
            Some(0u64),
            Some("memo".to_string()),
            nft_contract_id.clone(),
            1,
            GAS_FOR_NFT_TRANSFER,
        )
        .then(ext_self::after_nft_transfer(
            account_id.clone(),
            nft_contract_id.clone(),
            token_id.clone(),
            env::current_account_id(),
            0,
            GAS_FOR_NFT_TRANSFER,
        ))
    }
}

#[near_bindgen]
// impl Contract {
impl ExtSelf for Contract {
    #[private]
    fn after_nft_transfer(
        &mut self,
        account_id: AccountId,
        nft_contract_id: NFTContractId,
        token_id: NFTTokenId,
    ) -> bool {
        let promise_success = is_promise_success();
        if !promise_success {
            let mut account = self.internal_unwrap_account(&account_id);
            // account.add_affected_farm(FarmId::Supplied(nft_contract_id.clone()));
            self.internal_nft_deposit(&mut account, &nft_contract_id, &token_id);
            events::emit::withdraw_nft_failed(&account_id, &nft_contract_id, &token_id);
            self.internal_set_account(&account_id, account);
        } else {
            events::emit::withdraw_nft_succeeded(&account_id, &nft_contract_id, &token_id);
        }
        promise_success
    }
}
