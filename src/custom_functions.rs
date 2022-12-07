elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const FREEZE_SINGLE_NFT: &[u8] = b"freezeSingleNFT";
const UNFREEZE_SINGLE_NFT: &[u8] = b"unFreezeSingleNFT";
const WIPE_SINGLE_NFT: &[u8] = b"wipeSingleNFT";

#[elrond_wasm::module]
pub trait CustomFunctions: crate::storage::StorageModule + crate::events::EventsModule {
    fn freeze_single_nft(
        &self,
        nonce: u64,
        address: &ManagedAddress,
    ) -> ContractCall<Self::Api, ()> {
        let token_identifier = self.token_id().get_token_id();
        let esdt_system_sc_address = self.send().esdt_system_sc_proxy().esdt_system_sc_address();
        let mut contract_call: ContractCall<Self::Api, ()> = ContractCall::new(
            esdt_system_sc_address,
            ManagedBuffer::new_from_bytes(FREEZE_SINGLE_NFT),
        );
        contract_call.push_endpoint_arg(&token_identifier);
        contract_call.push_endpoint_arg(&nonce);
        contract_call.push_endpoint_arg(&address);

        self.freeze_event(&address, &token_identifier, nonce);
        contract_call
    }

    fn unfreeze_single_nft(
        &self,
        nonce: u64,
        address: &ManagedAddress,
    ) -> ContractCall<Self::Api, ()> {
        let token_identifier = self.token_id().get_token_id();
        let esdt_system_sc_address = self.send().esdt_system_sc_proxy().esdt_system_sc_address();
        let mut contract_call: ContractCall<Self::Api, ()> = ContractCall::new(
            esdt_system_sc_address,
            ManagedBuffer::new_from_bytes(UNFREEZE_SINGLE_NFT),
        );
        contract_call.push_endpoint_arg(&token_identifier);
        contract_call.push_endpoint_arg(&nonce);
        contract_call.push_endpoint_arg(&address);

        self.unfreeze_event(&address, &token_identifier, nonce);
        contract_call
    }

    fn wipe_single_nft(&self, nonce: u64, address: &ManagedAddress) -> ContractCall<Self::Api, ()> {
        let token_identifier = self.token_id().get_token_id();
        let esdt_system_sc_address = self.send().esdt_system_sc_proxy().esdt_system_sc_address();
        let mut contract_call: ContractCall<Self::Api, ()> = ContractCall::new(
            esdt_system_sc_address,
            ManagedBuffer::new_from_bytes(WIPE_SINGLE_NFT),
        );
        contract_call.push_endpoint_arg(&token_identifier);
        contract_call.push_endpoint_arg(&nonce);
        contract_call.push_endpoint_arg(&address);

        self.wipe_event(&address, &token_identifier, nonce);
        contract_call
    }
}
