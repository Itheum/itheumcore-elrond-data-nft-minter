elrond_wasm::imports!();
elrond_wasm::derive_imports!();

//Module that handles generic (commonly used, which are not specific to one function) requirements which should stop execution and rollback if not met
#[elrond_wasm::module]
pub trait RequirementsModule: crate::storage::StorageModule {
    //Checks whether the owner of the smart contract did all the prerequisites for the minting process to start and contract is not paused
    fn require_minting_is_ready(&self) {
        let mut is_mint_ready = true;
        if self.is_paused().get() {
            is_mint_ready = false;
        }
        if self.token_id().is_empty() {
            is_mint_ready = false;
        }
        require!(is_mint_ready, "Minting is not ready");
    }

    //Checks whether a value is higher than zero
    fn require_value_higher_than_zero(&self, value: &BigUint) {
        require!(value > &BigUint::zero(), "Value must be greater than 0");
    }

    //Checks whether a value is lower than or equal to max per address
    fn require_value_lower_or_equal_max_per_address(&self, value: &BigUint) {
        require!(
            value <= &self.max_per_address().get(),
            "Value must be lower than or equal to max per address"
        );
    }
}
