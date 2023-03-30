use crate::errors::{
    ERR_FIELD_IS_EMPTY, ERR_MAX_ROYALTIES_TOO_HIGH, ERR_MAX_SUPPLY_EXCEEDED,
    ERR_MINTING_AND_BURNING_NOT_ALLOWED, ERR_MIN_ROYALTIES_BIGGER_THAN_MAX_ROYALTIES,
    ERR_NOT_PRIVILEGED, ERR_NOT_URL, ERR_NOT_WHITELISTED,
    ERR_ROYALTIES_ARE_BIGGER_THAN_MAX_ROYALTIES, ERR_ROYALTIES_ARE_SMALLER_THAN_MIN_ROYALTIES,
    ERR_SUPPLY_HIGHER_THAN_ZERO, ERR_TOKEN_NOT_ISSUED, ERR_TOO_MANY_CHARS,
    ERR_URL_INVALID_CHARACTERS, ERR_URL_IS_EMPTY, ERR_URL_TOO_BIG, ERR_URL_TOO_SMALL,
    ERR_VALUE_MUST_BE_POSITIVE, ERR_WAIT_MORE_TIME,
};

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
        if self.roles_are_set().is_empty() || self.roles_are_set().get() == false {
            is_mint_ready = false;
        }
        require!(is_mint_ready, ERR_MINTING_AND_BURNING_NOT_ALLOWED);
    }

    // Checks whether the address trying to mint is allowed to do so
    fn require_minting_is_allowed(&self, address: &ManagedAddress, current_time: u64) {
        let last_mint_time = self.last_mint_time(address).get();
        let mint_time_limit = self.mint_time_limit().get();
        require!(
            current_time - last_mint_time >= mint_time_limit,
            ERR_WAIT_MORE_TIME
        );

        let whitelist_enabled = self.whitelist_enabled().get();
        if whitelist_enabled {
            require!(self.whitelist().contains(address), ERR_NOT_WHITELISTED);
        }
    }

    // Checks whether a value is bigger than zero
    fn require_value_is_positive(&self, value: &BigUint) {
        require!(value > &BigUint::zero(), ERR_VALUE_MUST_BE_POSITIVE);
    }

    // Checks whether SFT creation conditions are met
    fn require_sft_is_valid(&self, royalties: &BigUint, supply: &BigUint) {
        let max_royalties = self.max_royalties().get();
        let min_royalties = self.min_royalties().get();
        let max_supply = self.max_supply().get();
        require!(
            royalties <= &max_royalties,
            ERR_ROYALTIES_ARE_BIGGER_THAN_MAX_ROYALTIES
        );
        require!(
            royalties >= &min_royalties,
            ERR_ROYALTIES_ARE_SMALLER_THAN_MIN_ROYALTIES
        );
        require!(supply <= &max_supply, ERR_MAX_SUPPLY_EXCEEDED);
        require!(supply > &BigUint::zero(), ERR_SUPPLY_HIGHER_THAN_ZERO);
    }

    // Checks whether address is privileged
    fn require_is_privileged(&self, address: &ManagedAddress) {
        if &self.blockchain().get_owner_address() != address {
            require!(!&self.administrator().is_empty(), ERR_NOT_PRIVILEGED);
            require!(&self.administrator().get() == address, ERR_NOT_PRIVILEGED);
        }
    }

    fn require_title_description_are_valid(
        &self,
        title: &ManagedBuffer,
        description: &ManagedBuffer,
    ) {
        require!(!title.is_empty(), ERR_FIELD_IS_EMPTY);
        require!(title.len() <= 30, ERR_TOO_MANY_CHARS);
        require!(!description.is_empty(), ERR_FIELD_IS_EMPTY);
        require!(description.len() <= 400, ERR_TOO_MANY_CHARS);
    }

    // Checks whether the URL passed is valid (characters, starts with https://)
    fn require_url_is_valid(&self, url: &ManagedBuffer) {
        let url_length = url.len();
        let starts_with: &[u8] = b"https://";
        self.require_url_is_adequate_length(url);
        let url_vec = url.to_boxed_bytes().into_vec();
        for i in 0..starts_with.len() {
            require!(url_vec[i] == starts_with[i], ERR_NOT_URL);
        }
        for i in 0..url_length {
            require!(
                url_vec[i] > 32 && url_vec[i] < 127,
                ERR_URL_INVALID_CHARACTERS
            )
        }
    }

    // Checks whether the URL passed has a valid length
    fn require_url_is_adequate_length(&self, url: &ManagedBuffer) {
        let url_length = url.len();
        require!(!url.is_empty(), ERR_URL_IS_EMPTY);
        require!(url_length <= 400, ERR_URL_TOO_BIG);
        require!(url_length >= 15, ERR_URL_TOO_SMALL);
    }

    // Checks whether the royalties passed are valid
    fn require_royalties_are_valid(&self, min_royalties: &BigUint, max_royalties: &BigUint) {
        require!(
            min_royalties < max_royalties,
            ERR_MIN_ROYALTIES_BIGGER_THAN_MAX_ROYALTIES
        );
        require!(
            max_royalties < &BigUint::from(10000u64),
            ERR_MAX_ROYALTIES_TOO_HIGH
        );
    }

    // Checks whether the token is issued
    fn require_token_issued(&self) {
        require!(!self.token_id().is_empty(), ERR_TOKEN_NOT_ISSUED);
    }
}
