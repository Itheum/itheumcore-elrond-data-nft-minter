#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{
    callbacks::CallbackProxy,
    errors::{
        ERR_ALREADY_IN_WHITELIST, ERR_CONTRACT_ALREADY_INITIALIZED, ERR_DATA_STREAM_IS_EMPTY,
        ERR_INVALID_URL_START_CHARS, ERR_ISSUE_COST, ERR_NOT_ENOUGH_FUNDS, ERR_NOT_IN_WHITELIST,
        ERR_WHITELIST_IS_EMPTY,
    },
    storage::DataNftAttributes,
};

pub mod callbacks;
pub mod collection_management;
pub mod errors;
pub mod events;
pub mod nft_mint_utils;
pub mod requirements;
pub mod storage;
pub mod views;
#[multiversx_sc::contract]
pub trait DataNftMint:
    storage::StorageModule
    + events::EventsModule
    + requirements::RequirementsModule
    + nft_mint_utils::NftMintUtils
    + views::ViewsModule
    + callbacks::Callbacks
    + collection_management::CollectionManagement
{
    // When the smart contract is deployed or upgraded, minting is automatically paused, whitelisting is enabled and default values are set
    #[init]
    fn init(&self) {
        self.is_paused().set(true);
        self.mint_pause_toggle_event(&true);

        self.whitelist_enabled().set(true);
        self.whitelist_enable_toggle_event(&true);

        self.min_royalties().set_if_empty(BigUint::from(0u64));
        self.max_royalties().set_if_empty(BigUint::from(8000u64));

        self.set_royalties_limits_event(&self.min_royalties().get(), &self.max_royalties().get());

        self.max_supply().set_if_empty(&BigUint::from(20u64));

        self.set_max_supply_event(&self.max_supply().get());
    }

    #[upgrade]
    fn upgrade(&self) {
        self.is_paused().set(true);
    }

    // Endpoint used by the owner in the first place to initialize the contract with all the data needed for the SFT token creation
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(initializeContract)]
    fn initialize_contract(
        &self,
        collection_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        mint_time_limit: u64,
    ) {
        require!(self.token_id().is_empty(), ERR_CONTRACT_ALREADY_INITIALIZED);
        let issue_cost = self.call_value().egld_value().clone_value();
        require!(
            issue_cost == BigUint::from(5u64) * BigUint::from(10u64).pow(16u32),
            ERR_ISSUE_COST
        );

        self.set_mint_time_limit_event(&mint_time_limit);
        self.mint_time_limit().set(mint_time_limit);

        // Collection issuing
        self.send()
            .esdt_system_sc_proxy()
            .issue_semi_fungible(
                issue_cost,
                &collection_name,
                &token_ticker,
                SemiFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                    can_transfer_create_role: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().issue_callback())
            .call_and_exit();
    }

    // Endpoint used by the owner to set the special roles to the contract
    #[only_owner]
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) {
        self.require_token_issued();

        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.token_id().get_token_id(),
                [
                    EsdtLocalRole::NftCreate,
                    EsdtLocalRole::NftBurn,
                    EsdtLocalRole::NftAddQuantity,
                ][..]
                    .iter()
                    .cloned(),
            )
            .async_call()
            .with_callback(self.callbacks().set_local_roles_callback())
            .call_and_exit();
    }

    // Public endpoint used to mint Data NFT-FTs.
    #[payable("*")]
    #[endpoint(mint)]
    fn mint_token(
        &self,
        name: ManagedBuffer,
        media: ManagedBuffer,
        metadata: ManagedBuffer,
        data_marshal: ManagedBuffer,
        data_stream: ManagedBuffer,
        data_preview: ManagedBuffer,
        royalties: BigUint,
        supply: BigUint,
        title: ManagedBuffer,
        description: ManagedBuffer,
        lock_period: u64, // days
    ) -> DataNftAttributes<Self::Api> {
        self.require_ready_for_minting_and_burning();
        require!(!data_stream.is_empty(), ERR_DATA_STREAM_IS_EMPTY);

        self.require_url_is_valid(&data_marshal);

        require!(
            self.require_url_starts_with(&data_preview, b"ipfs/")
                || self.require_url_starts_with(&media, b"ipns/")
                || self.require_url_starts_with(&metadata, b"https://"),
            ERR_INVALID_URL_START_CHARS
        );

        self.require_url_is_valid(&media);
        self.require_url_is_valid(&metadata);

        self.require_title_description_are_valid(&title, &description);
        self.require_sft_is_valid(&royalties, &supply);

        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();
        self.require_minting_is_allowed(&caller, current_time);
        self.last_mint_time(&caller).set(current_time);

        let payment = self.call_value().single_esdt();

        let one_token = BigUint::from(1u64);
        self.minted_per_address(&caller)
            .update(|n| *n += &one_token);

        self.minted_tokens().update(|n| *n += &one_token);

        let attributes: DataNftAttributes<Self::Api> = DataNftAttributes {
            creation_time: current_time,
            creator: caller.clone(),
            data_marshal_url: data_marshal.clone(),
            data_stream_url: data_stream.clone(),
            data_preview_url: data_preview,
            title,
            description,
        };

        let token_identifier = self.token_id().get_token_id();

        // self.mint_event(
        //     &caller,
        //     &one_token,
        //     &payment.token_identifier,
        //     &payment.amount,
        // );

        let nonce = self.send().esdt_nft_create(
            &token_identifier,
            &supply,
            &name,
            &royalties,
            &self.create_hash_buffer(&data_marshal, &data_stream),
            &attributes,
            &self.create_uris(media, metadata),
        );

        // call bonding contract with token_identifier and nonce

        // await success

        let mut contract_call: ContractCallNoPayment<Self::Api, ()> = ContractCallNoPayment::new(
            self.bond_contract_address().get(),
            ManagedBuffer::new_from_bytes(b"bond"),
        );
        contract_call.proxy_arg(&caller);
        contract_call.proxy_arg(&token_identifier);
        contract_call.proxy_arg(&nonce);
        contract_call.proxy_arg(&lock_period);

        contract_call
            .with_esdt_transfer(payment.clone())
            .with_gas_limit(100_000_000u64)
            .execute_on_dest_context::<()>();

        self.send()
            .direct_esdt(&caller, &token_identifier, nonce, &supply);

        attributes
    }

    // Endpoint used to burn Data NFT-FTs.
    #[payable("*")]
    #[endpoint(burn)]
    fn burn_token(&self) {
        self.require_ready_for_minting_and_burning();
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();
        self.token_id()
            .require_same_token(&payment.token_identifier);
        self.require_value_is_positive(&payment.amount);
        self.token_id()
            .nft_burn(payment.token_nonce, &payment.amount);
        self.burn_event(
            &caller,
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }

    // Endpoint that will be used by privileged address to change the contract pause value.
    #[endpoint(setIsPaused)]
    fn set_is_paused(&self, is_paused: bool) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        self.mint_pause_toggle_event(&is_paused);
        self.is_paused().set(is_paused);
    }

    // Endpoint that will be used by the owner and privileged address to change the whitelist enable value.
    #[endpoint(setWhiteListEnabled)]
    fn set_whitelist_enabled(&self, is_enabled: bool) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        self.whitelist_enable_toggle_event(&is_enabled);
        self.whitelist_enabled().set(is_enabled);
    }

    // Endpoint that will be used by the owner and privileged address to set whitelist spots.
    #[endpoint(setWhiteListSpots)]
    fn set_whitelist_spots(&self, whitelist: MultiValueEncoded<ManagedAddress>) {
        require!(!whitelist.is_empty(), ERR_WHITELIST_IS_EMPTY);
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        for item in whitelist.into_iter() {
            if self.whitelist().insert(item.clone()) {
                self.set_whitelist_spot_event(&item);
            } else {
                sc_panic!(ERR_ALREADY_IN_WHITELIST);
            }
        }
    }

    // Endpoint that will be used by the owner privileged address to unset whitelist spots.
    #[endpoint(removeWhiteListSpots)]
    fn remove_whitelist_spots(&self, whitelist: MultiValueEncoded<ManagedAddress>) {
        require!(!whitelist.is_empty(), ERR_WHITELIST_IS_EMPTY);
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        for item in whitelist.into_iter() {
            if self.whitelist().remove(&item.clone()) {
                self.remove_whitelist_spot_event(&item);
            } else {
                sc_panic!(ERR_NOT_IN_WHITELIST);
            }
        }
    }

    // Endpoint that will be used by the owner to set mint time limit.
    #[only_owner]
    #[endpoint(setMintTimeLimit)]
    fn set_mint_time_limit(&self, mint_time_limit: u64) {
        self.set_mint_time_limit_event(&mint_time_limit);
        self.mint_time_limit().set(mint_time_limit);
    }

    // Endpoint that will be used by the owner and privileged address to set min and max royalties.
    #[endpoint(setRoyaltiesLimits)]
    fn set_royalties_limits(&self, min_royalties: BigUint, max_royalties: BigUint) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        self.require_royalties_are_valid(&min_royalties, &max_royalties);
        self.set_royalties_limits_event(&min_royalties, &max_royalties);
        self.min_royalties().set(min_royalties);
        self.max_royalties().set(max_royalties);
    }

    // Endpoint that will be used by the owner and privileged address to set max supply.
    #[endpoint(setMaxSupply)]
    fn set_max_supply(&self, max_supply: BigUint) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        self.set_max_supply_event(&max_supply);
        self.max_supply().set(max_supply);
    }

    // Endpoint that will be used by the owner to change the administrator (privileged) address.
    #[only_owner]
    #[endpoint(setAdministrator)]
    fn set_administrator(&self, administrator: ManagedAddress) {
        self.set_administrator_event(&administrator);
        self.administrator().set(&administrator);
    }

    #[endpoint(setBondContractAddress)]
    fn set_bond_contract_address(&self, bond_contract_address: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        // self.set_bond_contract_address_event(&bond_contract_address);
        self.bond_contract_address().set(&bond_contract_address);
    }

    #[only_owner]
    #[endpoint(setWithdrawalAddress)]
    fn set_withdrawal_address(&self, withdrawal_address: ManagedAddress) {
        self.set_withdrawal_address_event(&withdrawal_address);
        self.withdrawal_address().set(&withdrawal_address);
    }

    #[endpoint(withdraw)] // smart contract must be payable to receive royalties
    fn withdraw(&self, token_identifier: EgldOrEsdtTokenIdentifier, nonce: u64, amount: BigUint) {
        let caller = self.blockchain().get_caller();

        self.require_withdrawal_address_is_set();
        let withdrawal_address = self.withdrawal_address().get();
        self.require_is_withdrawal_address(&caller);

        let balance = self.blockchain().get_sc_balance(&token_identifier, nonce);

        self.require_value_is_positive(&amount);
        if balance > BigUint::zero() && amount <= balance {
            self.send()
                .direct(&withdrawal_address, &token_identifier, nonce, &amount);

            self.withdraw_tokens_event(&caller, &token_identifier, &amount);
        } else {
            sc_panic!(ERR_NOT_ENOUGH_FUNDS);
        }
    }
}
