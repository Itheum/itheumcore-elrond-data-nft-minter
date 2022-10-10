elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// Module that handles the common storage of the smart contract
#[elrond_wasm::module]
pub trait StorageModule {
    // Stores the token identifier of the SFT to be minted
    #[view(getTokenId)]
    #[storage_mapper("sft_token_id")]
    fn token_id(&self) -> NonFungibleTokenMapper<Self::Api>;

    // Stores the amount of SFTs that have been created
    #[view(getMintedTokens)]
    #[storage_mapper("minted_tokens")]
    fn minted_tokens(&self) -> SingleValueMapper<BigUint>;

    // Stores the price for minting an NFT
    #[view(getMintPrice)]
    #[storage_mapper("mint_price")]
    fn mint_price(&self, token: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint>;

    // Stores whether minting is paused or not
    #[view(getIsPaused)]
    #[storage_mapper("is_paused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;

    // Stores how many SFTs have been minted per address
    #[view(getMintedPerAddress)]
    #[storage_mapper("minted_per_address")]
    fn minted_per_address(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // Stores how many SFTs each address can mint during private sale
    #[view(getWhiteList)]
    #[storage_mapper("white_list")]
    fn white_list(&self) -> SetMapper<ManagedAddress>;

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
