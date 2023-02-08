#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{callbacks::CallbackProxy, storage::DataNftAttributes};

pub mod callbacks;
pub mod collection_management;
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

        self.white_list_enabled().set(true);
        self.whitelist_enable_toggle_event(&true);

        self.min_royalties().set(BigUint::from(0u64));
        self.max_royalties().set(BigUint::from(8000u64));

        self.max_supply().set(&BigUint::from(20u64));
    }

    // Endpoint used by the owner in the first place to initialize the contract with all the data needed for the SFT token creation
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(initializeContract)]
    fn initialize_contract(
        &self,
        collection_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        anti_spam_tax_token: &EgldOrEsdtTokenIdentifier,
        anti_spam_tax_value: BigUint,
        mint_time_limit: u64,
        treasury_address: ManagedAddress,
    ) {
        require!(
            self.token_id().is_empty(),
            "Contract was already initialized"
        );
        let issue_cost = self.call_value().egld_value();
        require!(
            issue_cost == BigUint::from(5u64) * BigUint::from(10u64).pow(16u32),
            "Issue cost is 0.05 eGLD"
        );

        self.set_anti_spam_tax_event(&anti_spam_tax_token, &anti_spam_tax_value);
        self.anti_spam_tax(anti_spam_tax_token)
            .set(anti_spam_tax_value);

        self.set_mint_time_limit_event(&mint_time_limit);
        self.mint_time_limit().set(mint_time_limit);
        self.treasury_address().set(&treasury_address);
        // Collection issuing and giving NFT creation rights to the contract.
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

    // Public endpoint used to mint Data NFT-FTs.
    #[payable("*")]
    #[endpoint(mint)]
    fn mint_token(
        &self,
        name: ManagedBuffer,
        media: ManagedBuffer,
        data_marshal: ManagedBuffer,
        data_stream: ManagedBuffer,
        data_preview: ManagedBuffer,
        royalties: BigUint,
        supply: BigUint,
        title: ManagedBuffer,
        description: ManagedBuffer,
    ) -> DataNftAttributes<Self::Api> {
        self.require_minting_is_ready();
        self.require_url_is_valid(&data_marshal);
        self.require_url_is_adequate_length(&data_stream);
        self.require_url_is_valid(&data_preview);
        self.require_url_is_valid(&media);
        self.require_sft_is_valid(&royalties, &supply);
        let caller = self.blockchain().get_caller();
        let current_time = self.blockchain().get_block_timestamp();
        self.require_minting_is_allowed(&caller, current_time);
        self.last_mint_time(&caller).set(current_time);

        let payment = self.call_value().egld_or_single_esdt();
        let price = self.anti_spam_tax(&payment.token_identifier).get();
        // The contract will panic if the user tries to use a token which is has not been set as buyable by the owner.
        self.require_value_is_positive(&payment.amount);
        require!(&payment.amount == &price, "Wrong amount of payment sent");

        let one_token = BigUint::from(1u64);
        self.minted_per_address(&caller)
            .update(|n| *n += &one_token);

        self.minted_tokens().update(|n| *n += &one_token);

        let attributes: DataNftAttributes<Self::Api> = DataNftAttributes {
            creation_time: self.blockchain().get_block_timestamp(),
            creator: caller.clone(),
            data_marshal_url: data_marshal.clone(),
            data_stream_url: data_stream.clone(),
            data_preview_url: data_preview.clone(),
            title: title.clone(),
            description: description.clone(),
        };

        let token_identifier = self.token_id().get_token_id();

        self.mint_event(&caller, &one_token, &payment.token_identifier, &price);

        let nonce = self.send().esdt_nft_create(
            &token_identifier,
            &supply,
            &name,
            &royalties,
            &self.crate_hash_buffer(&data_marshal, &data_stream),
            &attributes,
            &self.create_uris(media),
        );

        self.send()
            .direct_esdt(&caller, &token_identifier, nonce, &supply);

        let treasury_address = self.treasury_address().get();

        if payment.token_identifier.is_egld() {
            self.send().direct_egld(&treasury_address, &payment.amount);
        } else {
            self.send().direct_esdt(
                &treasury_address,
                &payment.token_identifier.unwrap_esdt(),
                payment.token_nonce,
                &payment.amount,
            );
        }

        attributes
    }

    // Endpoint used to burn Data NFT-FTs.
    #[payable("*")]
    #[endpoint(burn)]
    fn burn_token(&self) {
        self.require_minting_is_ready();
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

    // Endpoint used to set the treasury address.
    #[only_owner]
    #[endpoint(setTreasuryAddress)]
    fn set_treasury_address(&self, address: ManagedAddress) {
        self.treasury_address_event(&address);
        self.treasury_address().set(&address);
    }

    // Endpoint that will be used by privileged address to change the contract pause value.
    #[endpoint(setIsPaused)]
    fn set_is_paused(&self, is_paused: bool) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        self.mint_pause_toggle_event(&is_paused);
        self.is_paused().set(is_paused);
    }

    // Endpoint that will be used by privileged address to set the anti spam tax for a specific token identifier.
    #[endpoint(setAntiSpamTax)]
    fn set_anti_spam_tax(&self, token_id: EgldOrEsdtTokenIdentifier, tax: BigUint) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        self.set_anti_spam_tax_event(&token_id, &tax);
        self.anti_spam_tax(&token_id).set(tax);
    }

    // Endpoint that will be used by the owner and privileged address to change the whitelist enable value.
    #[endpoint(setWhiteListEnabled)]
    fn set_white_list_enabled(&self, is_enabled: bool) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        self.whitelist_enable_toggle_event(&is_enabled);
        self.white_list_enabled().set(is_enabled);
    }

    // Endpoint that will be used by the owner and privileged address to set whitelist spots.
    #[endpoint(setWhiteListSpots)]
    fn set_whitelist_spots(&self, whitelist: MultiValueEncoded<ManagedAddress>) {
        require!(!whitelist.is_empty(), "Given whitelist is empty");
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        for item in whitelist.into_iter() {
            if self.white_list().insert(item.clone()) {
                self.set_whitelist_spot_event(&item);
            } else {
                sc_panic!("Address already in whitelist");
            }
        }
    }

    // Endpoint that will be used by the owner privileged address to unset whitelist spots.
    #[endpoint(removeWhiteListSpots)]
    fn remove_whitelist_spots(&self, whitelist: MultiValueEncoded<ManagedAddress>) {
        require!(!whitelist.is_empty(), "Given whitelist is empty");
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        for item in whitelist.into_iter() {
            if self.white_list().remove(&item.clone()) {
                self.remove_whitelist_spot_event(&item);
            } else {
                sc_panic!("Address not in whitelist");
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
}
