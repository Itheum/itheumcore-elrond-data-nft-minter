use crate::errors::{ERR_MAX_ROYALTIES_TOO_HIGH, ERR_MIN_ROYALTIES_BIGGER_THAN_MAX_ROYALTIES};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

// Module that handles generic (commonly used, which are not specific to one function) requirements which should stop execution and rollback if not met
#[multiversx_sc::module]
pub trait RequirementsModule: crate::storage::StorageModule {
    // Checks whether the owner of the smart contract did all the prerequisites for the minting process to start and contract is not paused
    fn require_ready_for_minting_and_burning(&self) {
        let mut is_mint_ready = true;
        if self.is_paused().get() {
            is_mint_ready = false;
        }
        if self.token_id().is_empty() {
            is_mint_ready = false;
        }
        if self.treasury_address().is_empty() {
            is_mint_ready = false;
        }
        require!(is_mint_ready, "Minting is not ready");
    }

    // Checks whether the address trying to mint is allowed to do so
    fn require_minting_is_allowed(&self, address: &ManagedAddress, current_time: u64) {
        let last_mint_time = self.last_mint_time(address).get();
        let mint_time_limit = self.mint_time_limit().get();
        require!(
            current_time - last_mint_time >= mint_time_limit,
            "You need to wait more time before minting again"
        );

        let whitelist_enabled = self.whitelist_enabled().get();
        if whitelist_enabled {
            require!(
                self.whitelist().contains(address),
                "You are not whitelisted"
            );
        }
    }

    // Checks whether a value is bigger than zero
    fn require_value_is_positive(&self, value: &BigUint) {
        require!(value > &BigUint::zero(), "Value must be higher than zero");
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
        require!(supply > &BigUint::zero(), "Supply must be higher than zero");
    }

    // Checks whether address is privileged
    fn require_is_privileged(&self, address: &ManagedAddress) {
        if &self.blockchain().get_owner_address() != address {
            require!(
                !&self.administrator().is_empty(),
                "Address is not privileged"
            );
            require!(
                &self.administrator().get() == address,
                "Address is not privileged"
            );
        }
    }

    // Checks whether the uris are valid
    fn require_url_is_valid(&self, url: &ManagedBuffer) {
        let url_length = url.len();
        let starts_with: &[u8] = b"https://";
        self.require_url_is_adequate_length(url);
        let url_vec = url.to_boxed_bytes().into_vec();
        for i in 0..starts_with.len() {
            require!(url_vec[i] == starts_with[i], "URL must start with https://");
        }
        for i in 0..url_length {
            if url_vec[i] == 32 || url_vec[i] == 10 || url_vec[i] == 13 {
                sc_panic!("URL contains invalid characters");
            }
        }
    }

    fn require_url_is_adequate_length(&self, url: &ManagedBuffer) {
        let url_length = url.len();
        require!(!url.is_empty(), "URL is empty");
        require!(url_length <= 300, "URL length is too big");
        require!(url_length >= 20, "URL length is too small");
    }

    fn require_royalties_are_valid(&self, min_royalties: &BigUint, max_royalties: &BigUint) {
        require!(
            min_royalties <= max_royalties,
            ERR_MIN_ROYALTIES_BIGGER_THAN_MAX_ROYALTIES
        );
        require!(
            max_royalties <= &BigUint::from(10000u64),
            ERR_MAX_ROYALTIES_TOO_HIGH
        );
    }
}
