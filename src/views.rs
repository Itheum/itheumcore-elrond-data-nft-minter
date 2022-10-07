elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// Structure used in order to get all the necesary data for the average user in one call to the smart contract
#[derive(
    Clone, NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Debug, PartialEq, Eq,
)]
pub struct UserDataOut<M: ManagedTypeApi> {
    pub how_many_can_mint: BigUint<M>,
    pub public_egld_price: BigUint<M>,
    pub private_egld_price: BigUint<M>,
    pub public_prices: ManagedVec<M, EsdtTokenPayment<M>>,
    pub private_prices: ManagedVec<M, EsdtTokenPayment<M>>,
    pub collection_size: BigUint<M>,
    pub minted_for_address: BigUint<M>,
    pub minted_in_total: BigUint<M>,
    pub can_mint: bool,
    pub max_per_tx: BigUint<M>,
}

//Module that implements views (read-only endpoint)
#[elrond_wasm::module]
pub trait ViewsModule: crate::storage::StorageModule {
    // View that returns how many NFTs can still be minted
    #[view(getSftsLeftToMint)]
    fn sfts_left_to_mint(&self) -> BigUint {
        self.collection_size().get() - self.minted_tokens().get()
    }

    // View that returns the above mentioned all-in-one structure for viewing data through one call
    #[view(getUserDataOutFromContract)]
    fn get_user_data_out_from_contract(&self, address: &ManagedAddress) -> UserDataOut<Self::Api> {
        let max_per_tx = self.max_per_tx().get();
        let minted_in_total = self.minted_tokens().get();
        let minted_for_address = self.minted_per_address(address).get();
        let collection_size = self.collection_size().get();

        let mut can_mint = true;
        if self.is_paused().get() {
            can_mint = false;
        }
        if self.token_id().is_empty() {
            can_mint = false;
        }
        if self.token_created_nonce().is_empty() {
            can_mint = false;
        }

        let mut public_egld_price = BigUint::zero();
        let mut public_prices: ManagedVec<EsdtTokenPayment<Self::Api>> = ManagedVec::new();
        let mut private_egld_price = BigUint::zero();
        let mut private_prices: ManagedVec<EsdtTokenPayment<Self::Api>> = ManagedVec::new();
        for (token_id, price) in self.token_public_price().iter() {
            if token_id.is_egld() {
                public_egld_price = price;
            } else {
                let payment = EsdtTokenPayment::new(token_id.unwrap_esdt(), 0, price);
                public_prices.push(payment);
            }
        }
        for (token_id, price) in self.token_private_price().iter() {
            if token_id.is_egld() {
                private_egld_price = price;
            } else {
                let payment = EsdtTokenPayment::new(token_id.unwrap_esdt(), 0, price);
                private_prices.push(payment);
            }
        }

        let max_per_address = self.max_per_address().get();
        let mut how_many_can_mint = &max_per_address - &minted_for_address;
        if self.white_list_enabled().get() {
            let white_list_mintable = self.white_list(address).get();
            if white_list_mintable < how_many_can_mint {
                how_many_can_mint = white_list_mintable;
            }
        }
        let user_data = UserDataOut {
            how_many_can_mint,
            public_egld_price,
            private_egld_price,
            public_prices,
            private_prices,
            collection_size,
            minted_for_address,
            minted_in_total,
            can_mint,
            max_per_tx,
        };
        user_data
    }
}
