elrond_wasm::imports!();
elrond_wasm::derive_imports!();

//Module that handles read-only endpoints (views) for the smart contract
#[elrond_wasm::module]
pub trait ViewsModule: crate::storage::StorageModule {}
