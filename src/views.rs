elrond_wasm::imports!();
elrond_wasm::derive_imports!();

//Module that implements views (read-only endpoint)
#[elrond_wasm::module]
pub trait ViewsModule: crate::storage::StorageModule {}
