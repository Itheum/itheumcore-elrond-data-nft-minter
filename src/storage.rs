elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Debug, TypeAbi)]
pub struct DataNftAttributes<M: ManagedTypeApi> {
    pub data_stream_url: ManagedBuffer<M>,
    pub data_preview_url: ManagedBuffer<M>,
    pub data_marshal_url: ManagedBuffer<M>,
    pub creator: ManagedAddress<M>,
    pub creation_time: u64,
    pub title: ManagedBuffer<M>,
    pub description: ManagedBuffer<M>,
}

// Module that handles the common storage of the smart contract
#[elrond_wasm::module]
pub trait StorageModule {
    // Stores the token identifier of the SFT to be minted
    #[view(getTokenId)]
    #[storage_mapper("sft_token_id")]
    fn token_id(&self) -> NonFungibleTokenMapper<Self::Api>;

    // Stores the treasury address
    #[view(getTreasuryAddress)]
    #[storage_mapper("treasury_address")]
    fn treasury_address(&self) -> SingleValueMapper<ManagedAddress>;

    // Stores the amount of SFTs that have been created
    #[view(getMintedTokens)]
    #[storage_mapper("minted_tokens")]
    fn minted_tokens(&self) -> SingleValueMapper<BigUint>;

    // Stores the price for minting an NFT
    #[view(getAntiSpamTax)]
    #[storage_mapper("anti_spam_tax")]
    fn anti_spam_tax(&self, token: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint>;

    // Stores whether minting is paused or not
    #[view(getIsPaused)]
    #[storage_mapper("is_paused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;

    // Stores max royalties
    #[view(getMaxRoyalties)]
    #[storage_mapper("max_royalties")]
    fn max_royalties(&self) -> SingleValueMapper<BigUint>;

    // Stores min royalties
    #[view(getMinRoyalties)]
    #[storage_mapper("min_royalties")]
    fn min_royalties(&self) -> SingleValueMapper<BigUint>;

    // Stores max SFT supply
    #[view(getMaxSupply)]
    #[storage_mapper("max_supply")]
    fn max_supply(&self) -> SingleValueMapper<BigUint>;

    // Stores how many SFTs have been minted per address
    #[view(getMintedPerAddress)]
    #[storage_mapper("minted_per_address")]
    fn minted_per_address(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // Stores whether the contract has been initialized or not
    #[view(getContractInitialized)]
    #[storage_mapper("contract_initialized")]
    fn contract_initialized(&self) -> SingleValueMapper<bool>;

    // Stores the amount of time in seconds that an address has to wait before minting again
    #[view(mintTimeLimit)]
    #[storage_mapper("mint_time_limit")]
    fn mint_time_limit(&self) -> SingleValueMapper<u64>;

    // Stores the moment when an address last minted
    #[view(lastMintTime)]
    #[storage_mapper("last_mint_time")]
    fn last_mint_time(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    // Stores how many SFTs each address can mint during private sale
    #[view(getWhiteList)]
    #[storage_mapper("white_list")]
    fn white_list(&self) -> SetMapper<ManagedAddress>;

    // Stores the addresses that have been freezed for the entire collection
    #[view(getCollectionFreezedList)]
    #[storage_mapper("collection_freezed_list")]
    fn freezed_addresses_for_collection(&self) -> SetMapper<ManagedAddress>;

    // Stores the actual nonces that have been freezed for an address in a vector
    #[view(getSftsFreezedForAddress)]
    #[storage_mapper("sfts_freezed_list_per_address")]
    fn freezed_sfts_per_address(&self, address: &ManagedAddress) -> SetMapper<u64>;

    // stores the total number of nonces freezed for an address
    #[view(getFreezedCount)]
    #[storage_mapper("freezed_count_per_address")]
    fn freezed_count(&self, address: &ManagedAddress) -> SingleValueMapper<usize>;

    // Stores whether the contract is in private sale mode or not
    #[view(isWhiteListEnabled)]
    #[storage_mapper("white_list_enabled")]
    fn white_list_enabled(&self) -> SingleValueMapper<bool>;

    // Stores admin address
    #[view(getAdministrator)]
    #[storage_mapper("administrator")]
    fn administrator(&self) -> SingleValueMapper<ManagedAddress>;
}
