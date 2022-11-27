#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::storage::DataNftAttributes;

pub mod events;
pub mod nft_mint_utils;
pub mod requirements;
pub mod storage;

#[elrond_wasm::contract]
pub trait DataNftMint:
    elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + storage::StorageModule
    + events::EventsModule
    + requirements::RequirementsModule
    + nft_mint_utils::NftMintUtils
{
    // When the smart contract is deployed or upgraded, minting is automatically paused and sale is set to private.
    #[init]
    fn init(&self) {
        self.is_paused().set(true);
        self.mint_pause_toggle_event(&true);
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
        anti_spam_tax: BigUint,
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

        self.set_anti_spam_tax_event(&token_identifier, &anti_spam_tax);

        self.anti_spam_tax(token_identifier).set(anti_spam_tax);

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
        data_marshal: ManagedBuffer,
        data_stream: ManagedBuffer,
        data_preview: ManagedBuffer,
        royalties: BigUint,
        amount: BigUint,
    ) -> DataNftAttributes<Self::Api> {
        self.require_minting_is_ready();
        let payment = self.call_value().egld_or_single_esdt();

        let price = self.anti_spam_tax(&payment.token_identifier).get();

        // The contract will panic if the user tries to use a token which is has not been set as buyable by the owner.
        require!(price > BigUint::zero(), "Cannot buy with this token");
        require!(&payment.amount == &price, "Wrong amount of payment sent");

        let caller = self.blockchain().get_caller();
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
        };

        let token_identifier = self.token_id().get_token_id();

        self.mint_event(&caller, &one_token, &payment.token_identifier, &price);

        let nonce = self.send().esdt_nft_create(
            &token_identifier,
            &amount,
            &name,
            &royalties,
            &self.crate_hash_buffer(&data_marshal, &data_stream),
            &attributes,
            &self.create_uris(media),
        );

        self.send()
            .direct_esdt(&caller, &token_identifier, nonce, &amount);

        attributes
    }

    // Endpoint that will be used by the owner to change the mint pause value.
    #[only_owner]
    #[endpoint(setIsPaused)]
    fn set_is_paused(&self, is_paused: bool) {
        self.mint_pause_toggle_event(&is_paused);
        self.is_paused().set(is_paused);
    }

    // Endpoint that will be used by the owner to set public sale prices.
    #[only_owner]
    #[endpoint(setAntiSpamTax)]
    fn set_anti_spam_tax(&self, token_id: EgldOrEsdtTokenIdentifier, tax: BigUint) {
        self.set_anti_spam_tax_event(&token_id, &tax);
        self.anti_spam_tax(&token_id).set(tax);
    }
}
