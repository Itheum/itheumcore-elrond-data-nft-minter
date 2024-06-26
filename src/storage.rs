multiversx_sc::imports!();
multiversx_sc::derive_imports!();

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
#[multiversx_sc::module]
pub trait StorageModule {
    // Stores the token identifier of the SFT to be minted
    #[view(getTokenId)]
    #[storage_mapper("sft_token_id")]
    fn token_id(&self) -> NonFungibleTokenMapper<Self::Api>;

    // Stores the treasury address
    #[view(getTreasuryAddress)]
    #[storage_mapper("treasury_address")]
    fn treasury_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getDonationTreasuryAddress)]
    #[storage_mapper("donation_treasury_address")]
    fn donation_treasury_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getMaxDonationPercentage)]
    #[storage_mapper("max_donation_percentage")]
    fn max_donation_percentage(&self) -> SingleValueMapper<u64>;

    #[view(getWithdrawalAddress)]
    #[storage_mapper("withdrawal_address")]
    fn withdrawal_address(&self) -> SingleValueMapper<ManagedAddress>;

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

    // Stores max SFT supply a user can mint
    #[view(getMaxSupply)]
    #[storage_mapper("max_supply")]
    fn max_supply(&self) -> SingleValueMapper<BigUint>;

    // Stores how many SFTs have been minted per address
    #[view(getMintedPerAddress)]
    #[storage_mapper("minted_per_address")]
    fn minted_per_address(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    // Stores the amount of time in seconds that an address has to wait before minting again
    #[view(mintTimeLimit)]
    #[storage_mapper("mint_time_limit")]
    fn mint_time_limit(&self) -> SingleValueMapper<u64>;

    // Stores the moment when an address last minted
    #[view(lastMintTime)]
    #[storage_mapper("last_mint_time")]
    fn last_mint_time(&self, address: &ManagedAddress) -> SingleValueMapper<u64>;

    // Stores the addresses that are whitelisted to mint
    #[view(getWhiteList)]
    #[storage_mapper("whitelist")]
    fn whitelist(&self) -> SetMapper<ManagedAddress>;

    // Stores the addresses that have been frozen for the entire collection
    #[view(getCollectionFrozenList)]
    #[storage_mapper("collection_frozen_list")]
    fn frozen_addresses_for_collection(&self) -> SetMapper<ManagedAddress>;

    // Stores the actual nonces that have been frozen for an address in a vector
    #[view(getSftsFrozenForAddress)]
    #[storage_mapper("sfts_frozen_list_per_address")]
    fn frozen_sfts_per_address(&self, address: &ManagedAddress) -> SetMapper<u64>;

    // stores the total number of nonces frozen for an address
    #[view(getFrozenCount)]
    #[storage_mapper("frozen_count_per_address")]
    fn frozen_count(&self, address: &ManagedAddress) -> SingleValueMapper<usize>;

    // Stores whether the contract is in private sale mode or not
    #[view(isWhiteListEnabled)]
    #[storage_mapper("whitelist_enabled")]
    fn whitelist_enabled(&self) -> SingleValueMapper<bool>;

    #[view(rolesAreSet)]
    #[storage_mapper("roles_are_set")]
    fn roles_are_set(&self) -> SingleValueMapper<bool>;

    // Stores admin address
    #[view(getAdministrator)]
    #[storage_mapper("administrator")]
    fn administrator(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getBondContractAddress)]
    #[storage_mapper("bond_contract_address")]
    fn bond_contract_address(&self) -> SingleValueMapper<ManagedAddress>;
}
