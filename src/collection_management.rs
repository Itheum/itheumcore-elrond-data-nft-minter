elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const FREEZE_SINGLE_NFT: &[u8] = b"freezeSingleNFT";
const UNFREEZE_SINGLE_NFT: &[u8] = b"unFreezeSingleNFT";
const WIPE_SINGLE_NFT: &[u8] = b"wipeSingleNFT";

#[elrond_wasm::module]
pub trait CollectionManagement:
    crate::storage::StorageModule
    + crate::events::EventsModule
    + crate::requirements::RequirementsModule
{
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

    // Endpoint used by the owner to pause the collection
    #[only_owner]
    #[endpoint(pause)]
    fn pause_collection(&self) {
        let token_identifier = self.token_id().get_token_id();
        self.pause_collection_event(&token_identifier);
        self.send()
            .esdt_system_sc_proxy()
            .pause(&token_identifier)
            .async_call()
            .call_and_exit();
    }

    // Endpoint used by the owner to unpause the collection
    #[only_owner]
    #[endpoint(unpause)]
    fn unpause_collection(&self) {
        let token_identifier = self.token_id().get_token_id();
        self.unpause_collection_event(&token_identifier);
        self.send()
            .esdt_system_sc_proxy()
            .unpause(&token_identifier)
            .async_call()
            .call_and_exit();
    }

    // Endpoint used by the owner to freeze entire collection for specific address
    #[only_owner]
    #[endpoint(freeze)]
    fn freeze_collection_for_address(&self, address: &ManagedAddress) {
        if self
            .freezed_addresses_for_collection()
            .insert(address.clone())
        {
            let token_identifier = self.token_id().get_token_id();
            self.set_collection_freeze_list_spot_event(&address);
            self.send()
                .esdt_system_sc_proxy()
                .freeze(&token_identifier, &address)
                .async_call()
                .call_and_exit();
        } else {
            sc_panic!("Address is in collection freeze list");
        }
    }

    // Endpoint used by the owner to unFreeze entire collection for specific address
    #[only_owner]
    #[endpoint(unfreeze)]
    fn unfreeze_collection_for_address(&self, address: &ManagedAddress) {
        if self.freezed_addresses_for_collection().remove(address) {
            let token_identifier = self.token_id().get_token_id();
            self.remove_collection_freeze_list_spot_event(&address);
            self.send()
                .esdt_system_sc_proxy()
                .unfreeze(&token_identifier, &address)
                .async_call()
                .call_and_exit();
        } else {
            sc_panic!("Address is not in collection freeze list");
        }
    }

    // Endpoint used by the owner and the administrator to freeze address
    #[endpoint(freezeSingleNFT)]
    fn freeze_single_token_for_address(&self, nonce: u64, address: &ManagedAddress) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        if !self.freezed_addresses_for_collection().contains(&address) {
            if self.freezed_sfts_per_address(&address).insert(nonce) {
                let total_freezed = self.freezed_sfts_per_address(&address).len();
                self.freezed_count(&address).set(&total_freezed);
                self.set_freezed_sfts_per_address_event(&address, nonce);
                self.freeze_single_nft(nonce, &address)
                    .async_call()
                    .call_and_exit();
            } else {
                sc_panic!("Nonce not found in freeze list");
            }
        } else {
            sc_panic!("Address is in collection freeze list");
        }
    }

    // Endpoint used by the owner and the administrator to unfreeze address
    #[endpoint(unFreezeSingleNFT)]
    fn unfreeze_single_token_for_address(&self, nonce: u64, address: &ManagedAddress) {
        let caller = self.blockchain().get_caller();
        self.require_is_privileged(&caller);
        if !self.freezed_addresses_for_collection().contains(&address) {
            if self.freezed_sfts_per_address(&address).remove(&nonce) {
                let total_freezed = self.freezed_sfts_per_address(&address).len();
                self.freezed_count(&address).set(&total_freezed);
                self.remove_freezed_sfts_per_address_event(&address, nonce);
                self.unfreeze_single_nft(nonce, &address)
                    .async_call()
                    .call_and_exit();
            } else {
                sc_panic!("Nonce not found in freeze list");
            }
        } else {
            sc_panic!("Address is in collection freeze list");
        }
    }

    // Endpoint used by the owner and the administrator to wipe single nonce for data NFT-FTs
    #[endpoint(wipeSingleNFT)]
    fn wipe_single_token_for_address(&self, nonce: u64, address: &ManagedAddress) {
        let caller = self.blockchain().get_caller();
        let token_identifier = self.token_id().get_token_id();
        self.require_is_privileged(&caller);
        if !self.freezed_addresses_for_collection().contains(&address) {
            if self.freezed_sfts_per_address(&address).remove(&nonce) {
                let total_freezed = self.freezed_sfts_per_address(&address).len();
                self.freezed_count(&address).set(&total_freezed);
                self.wipe_event(&address, &token_identifier, nonce);
                self.wipe_single_nft(nonce, &address)
                    .async_call()
                    .call_and_exit();
            } else {
                sc_panic!("Nonce not found in freeze list");
            }
        } else {
            sc_panic!("Address is in collection blacklist");
        }
    }
}
