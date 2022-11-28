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

    // Checks whether the address trying to mint is allowed to do so
    fn require_minting_is_allowed(&self, address: &ManagedAddress, current_time: u64) {
        let last_mint_time = self.last_mint_time(address).get();
        let mint_time_liimit = self.mint_time_limit().get();
        require!(
            current_time - last_mint_time >= mint_time_liimit,
            "Minting is not allowed"
        );
    }
}
