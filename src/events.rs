elrond_wasm::imports!();
elrond_wasm::derive_imports!();
// Module that handles event emitting for important smart contract events in order to facilitate logging, debugging and monitoring with ease
#[elrond_wasm::module]
pub trait EventsModule {
    // Emitted whenever minting pause changes value
    #[event("mintPauseToggle")]
    fn mint_pause_toggle_event(&self, #[indexed] pause_value: &bool);

    // Emitted whenever treasury address is set
    #[event("setTreasuryAddress")]
    fn treasury_address_event(&self, #[indexed] treasury_address: &ManagedAddress);

    // Emitted whenever whitelist enabling changes value
    #[event("whitelistEnableToggle")]
    fn whitelist_enable_toggle_event(&self, #[indexed] enable_value: &bool);

    // Emitted whenever a whitelist spot is set
    #[event("whitelistSpotSet")]
    fn set_whitelist_spot_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever a blacklist spot is set
    #[event("blacklistSpotSet")]
    fn set_blacklist_spot_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever a blacklist spot is removed
    #[event("blacklistSpotRemoved")]
    fn remove_blacklist_spot_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever a whitelist spot is removed
    #[event("whitelistSpotRemoved")]
    fn remove_whitelist_spot_event(&self, #[indexed] address: &ManagedAddress);

    // Emitted whenever the minimum and maximum royalties values changes
    #[event("setRoyaltiesLimits")]
    fn set_royalties_limits_event(
        &self,
        #[indexed] min_royalties: &BigUint,
        #[indexed] max_royalties: &BigUint,
    );

    // Emitted whenever max supply changes
    #[event("setMaxSupply")]
    fn set_max_supply_event(&self, #[indexed] max_supply: &BigUint);

    // Emitted whenever a price for the public sale is set
    #[event("antiSpamTaxSet")]
    fn set_anti_spam_tax_event(
        &self,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    // Emitted whenever the mint time limit changes
    #[event("mintTimeLimitSet")]
    fn set_mint_time_limit_event(&self, #[indexed] mint_time_limit: &u64);

    // Emitted whenever the administrator is set
    #[event("setAdministrator")]
    fn set_administrator_event(&self, #[indexed] administrator: &ManagedAddress);

    // Emitted whenever the collection is paused
    #[event("pauseCollection")]
    fn pause_collection_event(&self, #[indexed] token_identifier: &TokenIdentifier);

    // Emitted whenever the collection is unpaused
    #[event("unpauseCollection")]
    fn unpause_collection_event(&self, #[indexed] token_identifier: &TokenIdentifier);

    // Emitted whenever an address is freezed
    #[event("freeze")]
    fn freeze_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] nonce: u64,
    );

    // Emitted whenever an address is unfreezed
    #[event("unfreeze")]
    fn unfreeze_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] nonce: u64,
    );

    // Emitted whenever a token is wiped
    #[event("wipe")]
    fn wipe_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] nonce: u64,
    );

    // Emitted whenever a burn is performed
    #[event("burn")]
    fn burn_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] amount: &BigUint,
    );

    // Emitted whenever a mint is performed
    #[event("mint")]
    fn mint_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] amount: &BigUint,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] price: &BigUint,
    );
}
