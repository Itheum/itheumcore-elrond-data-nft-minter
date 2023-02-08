multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait Callbacks: crate::storage::StorageModule {
    // Callback used to set the Token ID and the special roles for the SFT token.
    #[callback]
    fn issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.token_id().set_token_id(token_id.unwrap_esdt());
                self.send()
                    .esdt_system_sc_proxy()
                    .set_special_roles(
                        &self.blockchain().get_sc_address(),
                        &self.token_id().get_token_id(),
                        [
                            EsdtLocalRole::NftCreate,
                            EsdtLocalRole::NftBurn,
                            EsdtLocalRole::NftAddQuantity,
                        ][..]
                            .iter()
                            .cloned(),
                    )
                    .async_call()
                    .call_and_exit()
            }
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.send()
                        .direct(&caller, &returned.token_identifier, 0, &returned.amount);
                }
            }
        }
    }
}
