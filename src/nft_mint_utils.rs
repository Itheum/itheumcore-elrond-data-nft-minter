elrond_wasm::imports!();
elrond_wasm::derive_imports!();

//Module that handles string parsing and creation for the minting process
#[elrond_wasm::module]
pub trait UtilsModule: crate::storage::StorageModule {
    //Function that is used in order to create the attributes string in the Elrond specific format when using IPFS
    fn create_attributes(&self) -> ManagedBuffer {
        let cid = self.token_metadata_cid().get();
        let mut attributes = ManagedBuffer::new_from_bytes("metadata:".as_bytes());
        attributes.append(&cid);
        attributes
    }

    //Function that is used in order to create URIs that will be attached as assets to the SFT, media first, metadata second
    fn create_uris(&self) -> ManagedVec<ManagedBuffer> {
        let mut uris = ManagedVec::new();

        let media_cid = self.token_media_cid().get();
        let mut media_uri = ManagedBuffer::new_from_bytes("https://ipfs.io/ipfs/".as_bytes());
        media_uri.append(&media_cid);

        let metadata_cid = self.token_metadata_cid().get();
        let mut metadata_uri = ManagedBuffer::new_from_bytes("https://ipfs.io/ipfs/".as_bytes());
        metadata_uri.append(&metadata_cid);

        uris.push(media_uri);
        uris.push(metadata_uri);

        uris
    }
}
