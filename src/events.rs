elrond_wasm::imports!();
elrond_wasm::derive_imports!();
//Module that handles event emitting for important smart contract events in order to facilitate logging, debugging and monitoring with ease
#[elrond_wasm::module]
pub trait EventsModule {
    //Emitted whenever minting pause changes value
    #[event("mintPauseToggle")]
    fn mint_pause_toggle_event(&self, #[indexed] pause_value: &bool);

    //Emitted whenever a price for the public sale is set
    #[event("antiSpamTaxSet")]
    fn set_anti_spam_tax_event(
        &self,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] amount: &BigUint,
    );

    #[event("mintTimeLimitSet")]
    fn set_mint_time_limit_event(&self, #[indexed] mint_time_limit: &u64);

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
