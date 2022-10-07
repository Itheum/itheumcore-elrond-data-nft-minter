elrond_wasm::imports!();
elrond_wasm::derive_imports!();
//Module that handles event emitting for important smart contract events in order to facilitate logging, debugging and monitoring with ease
#[elrond_wasm::module]
pub trait EventsModule {
    //Emitted whenever minting pause changes value
    #[event("mintPauseToggle")]
    fn mint_pause_toggle_event(&self, #[indexed] pause_value: &bool);

    //Emitted whenever whitelist enabling changes value
    #[event("whitelistEnableToggle")]
    fn whitelist_enable_toggle_event(&self, #[indexed] enable_value: &bool);

    //Emitted whenever a price for the whitelist sale is set
    #[event("privateSalePriceSet")]
    fn set_private_sale_price_event(
        &self,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    //Emitted whenever a price for the public sale is set
    #[event("publicSalePriceSet")]
    fn set_public_sale_price_event(
        &self,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    //Emitted whenever the max mints per address limit is set
    #[event("maxPerAddressSet")]
    fn set_max_per_address_event(&self, #[indexed] amount: &BigUint);

    //Emitted whenever the max mints per transaction limit is set
    #[event("maxPerTxSet")]
    fn set_max_per_tx_event(&self, #[indexed] amount: &BigUint);

    //Emitted whenever a whitelist spot is set
    #[event("whitelistSpotSet")]
    fn set_whitelist_spot_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] amount: &BigUint,
    );

    //Emitted whenever a mint is performed
    #[event("mint")]
    fn mint_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] amount: &BigUint,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] price: &BigUint,
    );
}
