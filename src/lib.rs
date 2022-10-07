#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod events;
pub mod nft_mint_utils;
pub mod requirements;
pub mod storage;
pub mod views;

#[elrond_wasm::contract]
pub trait SftMint:
    elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + storage::StorageModule
    + events::EventsModule
    + views::ViewsModule
    + requirements::RequirementsModule
    + nft_mint_utils::UtilsModule
{
    // When the smart contract is deployed or upgraded, minting is automatically paused and sale is set to private.
    #[init]
    fn init(&self) {
        self.is_paused().set(true);
        self.mint_pause_toggle_event(&true);

        self.white_list_enabled().set(true);
        self.whitelist_enable_toggle_event(&true);

        self.contract_initialized().set(false);
    }

    // Endpoint used by the owner in the first place to initialize the contract with all the data needed for the token creation to begin.
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(initializeContract)]
    fn initialize_contract(
        &self,
        collection_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_royalties: BigUint,
        token_media_cid: ManagedBuffer,
        token_metadata_cid: ManagedBuffer,
        collection_size: BigUint,
        max_per_tx: BigUint,
        max_per_address: BigUint,
    ) {
        self.require_value_higher_than_zero(&max_per_tx);
        require!(
            max_per_tx <= max_per_address,
            "Max per tx must be lower or equal to max per address"
        );
        require!(
            collection_size >= max_per_address,
            "Collection size must be greater than or equal to max per address"
        );
        require!(
            self.token_id().is_empty(),
            "Contract was already initialized"
        );
        let issue_cost = self.call_value().egld_value();
        require!(
            issue_cost == BigUint::from(5u64) * BigUint::from(10u64).pow(16u32),
            "Issue cost is 0.05 eGLD"
        );
        self.token_royalties().set(token_royalties);
        self.token_media_cid().set(token_media_cid);
        self.max_per_tx().set(max_per_tx);
        self.max_per_address().set(max_per_address);
        self.token_metadata_cid().set(token_metadata_cid);
        self.collection_size().set(collection_size);

        // Collection issuing and giving SFT cration rights to the contract.
        self.token_id().issue_and_set_all_roles(
            EsdtTokenType::SemiFungible,
            issue_cost,
            collection_name,
            token_ticker,
            0usize,
            None,
        )
    }

    // Endpoint that will be used by the owner to create the SFT with a quantity of 1.
    #[only_owner]
    #[endpoint(createToken)]
    fn create_token(&self, token_name: ManagedBuffer) {
        require!(!self.token_id().is_empty(), "Token id must not be empty");
        let attributes = self.create_attributes();
        let token_id = self.token_id().get_token_id();
        let uris = self.create_uris();
        let hash = self.crypto().sha256(&attributes);
        let token_amount = BigUint::from(1u64);

        // Minting the SFT with a quantity of 1 and with the required attributes and URIs.
        let token_created = self.send().esdt_nft_create(
            &token_id,
            &token_amount,
            &token_name,
            &self.token_royalties().get(),
            &hash.as_managed_buffer(),
            &attributes,
            &uris,
        );
        self.contract_initialized().set(true);
        self.token_created_nonce().set(token_created);
        self.minted_tokens().update(|n| *n += &token_amount);
    }

    // Public endpoint used to mint and buy SFTs.
    #[payable("*")]
    #[endpoint(mint)]
    fn mint_token(&self) {
        self.require_minting_is_ready();
        let payment = self.call_value().egld_or_single_esdt();

        let whitelist_enabled = self.white_list_enabled().get();
        let option_price;

        // If whitelist is enabled, the private price will be used, else the public price will be used.
        if whitelist_enabled {
            option_price = self.token_private_price().get(&payment.token_identifier);
        } else {
            option_price = self.token_public_price().get(&payment.token_identifier);
        }

        // The contract will panic if the user tries to use a token which is has not been set as buyable by the owner.
        if let Some(price) = option_price {
            require!(price > BigUint::zero(), "Cannot buy with this token");
            require!(
                &payment.amount % &price == BigUint::zero(),
                "Wrong amount of payment sent"
            );

            let number_of_tokens_to_mint = &payment.amount / &price;
            require!(
                number_of_tokens_to_mint >= BigUint::from(1u64),
                "Payment too low"
            );
            self.require_value_lower_or_equal_max_per_tx(&number_of_tokens_to_mint);

            let caller = self.blockchain().get_caller();
            self.minted_per_address(&caller)
                .update(|n| *n += &number_of_tokens_to_mint);
            let already_minted_for_address = self.minted_per_address(&caller).get();
            self.require_value_lower_or_equal_max_per_address(&already_minted_for_address);

            self.minted_tokens()
                .update(|n| *n += &number_of_tokens_to_mint);

            // Check if there are enough tokens left to mint.
            let already_minted = self.minted_tokens().get();
            require!(
                already_minted <= self.collection_size().get(),
                "Collection size exceeded"
            );

            // Check if user is allowed to mint in case of private sale.
            if whitelist_enabled {
                let whitelist_mints_allowed = self.white_list(&caller);
                require!(
                    number_of_tokens_to_mint <= whitelist_mints_allowed.get(),
                    "Maximum number of private sale mints for this address exceeded"
                );
                whitelist_mints_allowed.update(|n| *n -= &number_of_tokens_to_mint);
            }

            self.mint_event(
                &caller,
                &number_of_tokens_to_mint,
                &payment.token_identifier,
                &price,
            );

            // Create the SFT quantity paid by the user and send it.
            self.token_id().nft_add_quantity_and_send(
                &caller,
                self.token_created_nonce().get(),
                number_of_tokens_to_mint,
            );
        } else {
            sc_panic!("Cannot buy with this token");
        }
    }

    // Endpoint that will be used by the owner to change the mint pause value.
    #[only_owner]
    #[endpoint(setIsPaused)]
    fn set_is_paused(&self, is_paused: bool) {
        self.mint_pause_toggle_event(&is_paused);
        self.is_paused().set(is_paused);
    }

    // Endpoint that will be used by the owner to change the whitelist enable value.
    #[only_owner]
    #[endpoint(setWhiteListEnabled)]
    fn set_white_list_enabled(&self, is_enabled: bool) {
        self.whitelist_enable_toggle_event(&is_enabled);
        self.white_list_enabled().set(is_enabled);
    }

    // Endpoint that will be used by the owner to set private sale prices.
    #[only_owner]
    #[endpoint(setPrivatePrice)]
    fn set_private_price(&self, token_id: EgldOrEsdtTokenIdentifier, price: BigUint) {
        self.require_price_set_is_valid(&token_id, &price);
        self.set_private_sale_price_event(&token_id, &price);
        self.token_private_price().insert(token_id, price);
    }

    // Endpoint that will be used by the owner to set public sale prices.
    #[only_owner]
    #[endpoint(setPublicPrice)]
    fn set_public_price(&self, token_id: EgldOrEsdtTokenIdentifier, price: BigUint) {
        self.require_price_set_is_valid(&token_id, &price);
        self.set_public_sale_price_event(&token_id, &price);
        self.token_public_price().insert(token_id, price);
    }

    // Endpoint that will be used by the owner to set max per address mint limit.
    #[only_owner]
    #[endpoint(setMaxPerAddress)]
    fn set_max_per_address(&self, max_per_address: BigUint) {
        self.require_value_higher_than_zero(&max_per_address);
        self.require_value_higher_or_equal_max_per_tx(&max_per_address);
        self.set_max_per_address_event(&max_per_address);
        self.max_per_address().set(max_per_address);
    }

    // Endpoint that will be used by the owner to set max per transaction mint limit.
    #[only_owner]
    #[endpoint(setMaxPerTx)]
    fn set_max_per_tx(&self, max_per_tx: BigUint) {
        self.require_value_higher_than_zero(&max_per_tx);
        self.require_value_lower_or_equal_max_per_address(&max_per_tx);
        self.set_max_per_tx_event(&max_per_tx);
        self.max_per_tx().set(max_per_tx);
    }

    // Endpoint that will be used by the owner to set private sale whitelist spots.
    #[only_owner]
    #[endpoint(setWhiteListSpots)]
    fn set_whitelist_spots(
        &self,
        whitelist: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    ) {
        require!(!whitelist.is_empty(), "Given whitelist is empty");
        for item in whitelist.into_iter() {
            let (address, value) = item.into_tuple();
            self.set_whitelist_spot_event(&address, &value);
            self.white_list(&address).set(value);
        }
    }
}
