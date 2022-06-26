use crate::*;
use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    env, log, near_bindgen, require, AccountId, Gas, PanicOnDefault, PromiseOrValue,
};


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
        self.internal_nft_deposit(&mut account, &nft_contract_id, &token_id);

        log!(
            "in nft_on_transfer; sender_id={}, previous_owner_id={}, token_id={}, msg={}",
            &sender_id,
            &previous_owner_id,
            &token_id,
            msg
        );
        log!("====> message from sender: {:?}", msg);
        self.internal_set_nft_asset(&nft_contract_id, sender_id, token_id, asset);
        PromiseOrValue::Value(false)
    }
}