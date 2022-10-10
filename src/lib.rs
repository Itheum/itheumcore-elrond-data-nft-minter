#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod events;
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
        token_identifier: &EgldOrEsdtTokenIdentifier,
        mint_price: BigUint,
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

        self.mint_price(token_identifier).set(mint_price);

        // Collection issuing and giving NFT creation rights to the contract.
        self.token_id().issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            issue_cost,
            collection_name,
            token_ticker,
            0usize,
            None,
        )
    }

    // Public endpoint used to mint and buy SFTs.
    #[payable("*")]
    #[endpoint(mint)]
    fn mint_token(&self) {
        self.require_minting_is_ready();
        let payment = self.call_value().egld_or_single_esdt();

        let whitelist_enabled = self.white_list_enabled().get();
        let price = self.mint_price(&payment.token_identifier).get();

        // The contract will panic if the user tries to use a token which is has not been set as buyable by the owner.
        require!(price > BigUint::zero(), "Cannot buy with this token");
        require!(&payment.amount == &price, "Wrong amount of payment sent");

        let caller = self.blockchain().get_caller();
        let one_token = BigUint::from(1u64);
        self.minted_per_address(&caller)
            .update(|n| *n += &one_token);

        self.minted_tokens().update(|n| *n += &one_token);

        // Check if user is allowed to mint in case of private sale.
        if whitelist_enabled {
            require!(
                self.white_list().contains(&caller),
                "Caller is not whitelisted"
            );
        }

        self.mint_event(&caller, &one_token, &payment.token_identifier, &price);

        // Create the SFT quantity paid by the user and send it.
        // self.token_id().nft_add_quantity_and_send(
        //     &caller,
        //     self.token_created_nonce().get(),
        //     number_of_tokens_to_mint,
        // );
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

    // Endpoint that will be used by the owner to set public sale prices.
    #[only_owner]
    #[endpoint(setMintPrice)]
    fn set_mint_price(&self, token_id: EgldOrEsdtTokenIdentifier, price: BigUint) {
        self.mint_price(&token_id).set(price);
    }

    // Endpoint that will be used by the owner to set private sale whitelist spots.
    #[only_owner]
    #[endpoint(setWhiteListSpots)]
    fn set_whitelist_spots(&self, whitelist: MultiValueEncoded<ManagedAddress>) {
        require!(!whitelist.is_empty(), "Given whitelist is empty");
        for item in whitelist.into_iter() {
            self.white_list().insert(item);
        }
    }
}
