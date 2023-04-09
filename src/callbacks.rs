use crate::errors::ERR_TOKEN_ISSUED;

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
                require!(self.token_id().is_empty(), ERR_TOKEN_ISSUED);
                self.token_id().set_token_id(token_id.unwrap_esdt());
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

    #[callback]
    fn set_local_roles_callback(&self, #[call_result] result: ManagedAsyncCallResult<()>) {
        match result {
            ManagedAsyncCallResult::Ok(_) => {
                self.roles_are_set().set(true);
            }
            ManagedAsyncCallResult::Err(_) => {}
        }
    }
}
