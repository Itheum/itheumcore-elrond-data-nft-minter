elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(
    Clone, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Debug, PartialEq, Eq,
)]
pub struct UserDataOut<M: ManagedTypeApi> {
    pub anti_spam_tax_value: BigUint<M>,
    pub is_paused: bool,
    pub max_royalties: BigUint<M>,
    pub min_royalties: BigUint<M>,
    pub max_supply: BigUint<M>,
    pub mint_time_limit: u64,
    pub last_mint_time: u64,
    pub whitelist_enabled: bool,
    pub is_whitelisted: bool,
}

//Module that handles read-only endpoints (views) for the smart contract
#[elrond_wasm::module]
pub trait ViewsModule: crate::storage::StorageModule {
    // View that returns the above mentioned all-in-one structure for viewing data through one call
    #[view(getUserDataOut)]
    fn get_user_data_out(
        &self,
        address: &ManagedAddress,
        tax_token: &EgldOrEsdtTokenIdentifier,
    ) -> UserDataOut<Self::Api> {
        {
            let anti_spam_tax_value = self.anti_spam_tax(tax_token).get(); //if it returns 0 the token is not supported
            let is_paused = self.is_paused().get();
            let max_royalties = self.max_royalties().get();
            let min_royalties = self.min_royalties().get();
            let max_supply = self.max_supply().get();
            let mint_time_limit = self.mint_time_limit().get();
            let last_mint_time = self.last_mint_time(&address).get();
            let whitelist_enabled = self.white_list_enabled().get();
            let is_whitelisted = self.white_list().contains(&address);

            let user_data = UserDataOut {
                anti_spam_tax_value,
                is_paused,
                max_royalties,
                min_royalties,
                max_supply,
                mint_time_limit,
                last_mint_time,
                whitelist_enabled,
                is_whitelisted,
            };
            user_data
        }
    }
}
