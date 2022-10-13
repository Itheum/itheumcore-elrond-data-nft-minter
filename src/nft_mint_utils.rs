elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait NftMintUtils: crate::storage::StorageModule {
    fn crate_hash_buffer(
        &self,
        data_marchal: &ManagedBuffer,
        data_stream: &ManagedBuffer,
    ) -> ManagedBuffer {
        let mut new_data = data_marchal.clone();
        new_data.append(data_stream);
        let hash_buffer = self.crypto().sha256(new_data).as_managed_buffer().clone();
        hash_buffer
    }

    fn create_uris(&self, media: ManagedBuffer) -> ManagedVec<ManagedBuffer> {
        let mut uris = ManagedVec::new();
        uris.push(media);
        uris
    }
}
