multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait NftMintUtils: crate::storage::StorageModule {
    fn create_hash_buffer(
        &self,
        data_marshal: &ManagedBuffer,
        data_stream: &ManagedBuffer,
    ) -> ManagedBuffer {
        let mut new_data = data_marshal.clone();
        new_data.append(data_stream);
        let hash_buffer = self.crypto().sha256(new_data).as_managed_buffer().clone();
        hash_buffer
    }

    fn create_uris(
        &self,
        media: ManagedBuffer,
        metadata: ManagedBuffer,
        extra_assets: ManagedVec<ManagedBuffer>,
    ) -> ManagedVec<ManagedBuffer> {
        let mut uris = ManagedVec::new();
        uris.push(media);
        uris.push(metadata);
        uris.append_vec(extra_assets);
        uris
    }
}
