multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod bonding_contract_proxy {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[multiversx_sc::proxy]
    pub trait BondingProxy {
        #[view(getLockPeriodBondAmount)]
        fn lock_period_bond_amount(&self, lock_period: u64);

        #[payable("*")]
        #[endpoint(bond)]
        fn bond(
            &self,
            original_caller: &ManagedAddress,
            token_identifier: TokenIdentifier,
            nonce: u64,
            lock_period: u64,
        );
    }
}

#[multiversx_sc::module]
pub trait BondingContractProxyMethods: crate::storage::StorageModule {
    #[proxy]
    fn bonding_proxy(&self, sc_address: ManagedAddress)
        -> bonding_contract_proxy::Proxy<Self::Api>;

    #[endpoint]
    fn get_bond_amount_for_lock_period(&self, lock_period: u64) -> BigUint {
        let bonding_contract_address = self.bond_contract_address().get();
        self.bonding_proxy(bonding_contract_address)
            .lock_period_bond_amount(lock_period)
            .execute_on_dest_context::<BigUint>()
    }

    #[endpoint]
    fn send_bond(
        &self,
        original_caller: &ManagedAddress,
        token_identifier: TokenIdentifier,
        nonce: u64,
        lock_period: u64,
        payment: EgldOrEsdtTokenPayment,
    ) {
        let bonding_contract_address = self.bond_contract_address().get();
        self.bonding_proxy(bonding_contract_address)
            .bond(original_caller, token_identifier, nonce, lock_period)
            .with_egld_or_single_esdt_transfer(payment)
            .execute_on_dest_context::<()>();
    }
}
