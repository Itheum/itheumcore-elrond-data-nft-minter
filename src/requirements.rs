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
            "You need to wait more time before minting again"
        );

        let whitelist_enabled = self.white_list_enabled().get();
        if whitelist_enabled {
            require!(
                self.white_list().contains(address),
                "You are not whitelisted"
            );
        }
    }

    // Checks whether a value is bigger than zero
    fn require_value_is_positive(&self, value: &BigUint) {
        require!(value > &BigUint::zero(), "Value must be positive");
    }

    // Checks whether SFT creation conditions are met
    fn require_sft_is_valid(&self, royalties: &BigUint, supply: &BigUint) {
        let max_royalties = self.max_royalties().get();
        let min_royalties = self.min_royalties().get();
        let max_supply = self.max_supply().get();
        require!(
            royalties <= &max_royalties,
            "Royalties are bigger than max royalties"
        );
        require!(
            royalties >= &min_royalties,
            "Royalties are smaller than min royalties"
        );
        require!(supply <= &max_supply, "Max supply exceeded");
        require!(supply > &BigUint::zero(), "Supply must be positive");
    }
}
