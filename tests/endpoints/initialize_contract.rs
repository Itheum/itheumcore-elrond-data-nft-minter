use multiversx_sc_scenario::scenario_model::TxExpect;

use crate::minter_state::minter_state::{
    ContractsState, COLLECTION_NAME, DATA_NFT_IDENTIFIER_EXPR, ITHEUM_TOKEN_IDENTIFIER,
    MINTER_OWNER_ADDRESS_EXPR,
};

#[test]
fn initialize_contract_test() {
    let mut state = ContractsState::new();
    let treasury_address = state.treasury.clone();

    state.deploy_minter();
    state.minter_initialize_contract(
        MINTER_OWNER_ADDRESS_EXPR,
        COLLECTION_NAME,
        DATA_NFT_IDENTIFIER_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        25u64,
        10u64,
        treasury_address.clone(),
        Some(1u64),
        Some(TxExpect::user_error("str:Issue cost is 0.05 eGLD")),
    );

    state.minter_initialize_contract(
        MINTER_OWNER_ADDRESS_EXPR,
        COLLECTION_NAME,
        DATA_NFT_IDENTIFIER_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        25u64,
        10u64,
        treasury_address.clone(),
        None,
        None,
    );

    state.mock_minter_initialized(ITHEUM_TOKEN_IDENTIFIER, 100u64, 10u64);

    state.minter_initialize_contract(
        MINTER_OWNER_ADDRESS_EXPR,
        COLLECTION_NAME,
        DATA_NFT_IDENTIFIER_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        25u64,
        10u64,
        treasury_address,
        None,
        Some(TxExpect::user_error("str:Contract was already initialized")),
    );
}
