#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::storage::DataNftAttributes;

pub mod events;
pub mod nft_mint_utils;
pub mod requirements;
pub mod storage;
pub mod views;

#[elrond_wasm::contract]
pub trait DataNftMint:
    elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + storage::StorageModule
    + events::EventsModule
    + views::ViewsModule
    + requirements::RequirementsModule
    + nft_mint_utils::NftMintUtils
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
            EsdtTokenType::SemiFungible,
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
    fn mint_token(
        &self,
        name: ManagedBuffer,
        media: ManagedBuffer,
        data_marchal: ManagedBuffer,
        data_stream: ManagedBuffer,
        data_preview: ManagedBuffer,
        royalties: BigUint,
        amount: BigUint,
    ) {
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

        let attributes: DataNftAttributes<Self::Api> = DataNftAttributes {
            creation_time: self.blockchain().get_block_timestamp(),
            creator: caller.clone(),
            data_marchal_url: data_marchal.clone(),
            data_stream_url: data_stream.clone(),
            data_preview_url: data_preview.clone(),
        };

        let token_identifier = self.token_id().get_token_id();

        self.mint_event(&caller, &one_token, &payment.token_identifier, &price);

        let nonce = self.send().esdt_nft_create(
            &token_identifier,
            &amount,
            &name,
            &royalties,
            &self.crate_hash_buffer(&data_marchal, &data_stream),
            &attributes,
            &self.create_uris(media),
        );

        self.send()
            .direct_esdt(&caller, &token_identifier, nonce, &amount);
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
