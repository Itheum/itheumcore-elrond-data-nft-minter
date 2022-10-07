elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// Module that handles the common storage of the smart contract
#[elrond_wasm::module]
pub trait StorageModule {
    // Stores the token identifier of the SFT to be minted
    #[view(getTokenId)]
    #[storage_mapper("sft_token_id")]
    fn token_id(&self) -> NonFungibleTokenMapper<Self::Api>;

    // Stores the CID of the media file used for the SFT
    #[view(getTokenMediaCid)]
    #[storage_mapper("sft_token_media_cid")]
    fn token_media_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    // Stores the CID of the metadata file used for the SFT
    #[view(getTokenMetadataCid)]
    #[storage_mapper("sft_token_metadata_cid")]
    fn token_metadata_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    // Stores the royalties percentage of the created SFT
    #[view(getTokenRoyalties)]
    #[storage_mapper("sft_token_royalties")]
    fn token_royalties(&self) -> SingleValueMapper<BigUint>;

    // Stores the maximum amount that can be created from the given SFT
    #[view(getCollectionSize)]
    #[storage_mapper("collection_size")]
    fn collection_size(&self) -> SingleValueMapper<BigUint>;

    // Stores the amount of SFTs that have been created
    #[view(getMintedTokens)]
    #[storage_mapper("minted_tokens")]
    fn minted_tokens(&self) -> SingleValueMapper<BigUint>;

    // Stores whether minting is paused or not
    #[view(getIsPaused)]
    #[storage_mapper("is_paused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;

    // Stores the prices of the SFT for the public sale
    #[view(getTokenPublicPrice)]
    #[storage_mapper("token_public_price")]
    fn token_public_price(&self) -> MapMapper<EgldOrEsdtTokenIdentifier, BigUint>;

    // Stores the prices of the SFT for the private sale
    #[view(getTokenPrivatePrice)]
    #[storage_mapper("token_private_price")]
    fn token_private_price(&self) -> MapMapper<EgldOrEsdtTokenIdentifier, BigUint>;

    // Stores the max per transaction minting limit
    #[view(getMaxPerTx)]
    #[storage_mapper("max_per_tx")]
    fn max_per_tx(&self) -> SingleValueMapper<BigUint>;

    // Stores the max per address minting limit
    #[view(getMaxPerAddress)]
    #[storage_mapper("max_per_address")]
    fn max_per_address(&self) -> SingleValueMapper<BigUint>;

    // Stores how many SFTs have been minted per address
    #[view(getMintedPerAddress)]
    #[storage_mapper("minted_per_address")]
    fn minted_per_address(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // Stores how many SFTs each address can mint during private sale
    #[view(getWhiteList)]
    #[storage_mapper("white_list")]
    fn white_list(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // Stores whether the contract is in private sale mode or not
    #[view(isWhiteListEnabled)]
    #[storage_mapper("white_list_enabled")]
    fn white_list_enabled(&self) -> SingleValueMapper<bool>;

    // Stores the nonce of the SFT that is sold (in happy workflow should always be 1)
    #[view(getTokenCreatedNonce)]
    #[storage_mapper("token_created_nonce")]
    fn token_created_nonce(&self) -> SingleValueMapper<u64>;

    // Stores whether the contract has been initialized or not
    #[view(getContractInitialized)]
    #[storage_mapper("contract_initialized")]
    fn contract_initialized(&self) -> SingleValueMapper<bool>;
}
