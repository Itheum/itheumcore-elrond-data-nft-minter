use datanftmint::storage::DataNftAttributes;
use multiversx_sc_scenario::{
    api::SingleTxApi,
    managed_address, managed_buffer,
    scenario_model::{CheckAccount, CheckStateStep, SetStateStep, TxExpect},
};

use crate::minter_state::minter_state::{
    ContractsState, ANOTHER_TOKEN_IDENTIFIER, BONDING_OWNER_ADDRESS_EXPR, DATA_NFT_IDENTIFIER,
    DATA_NFT_IDENTIFIER_EXPR, FIRST_USER_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER,
    MINTER_OWNER_ADDRESS_EXPR,
};

#[test]
fn burn_token_test() {
    let mut state = ContractsState::new();
    let first_user_address = state.first_user.clone();

    state
        .mock_minter_initialized(ITHEUM_TOKEN_IDENTIFIER, 100u64, 10u64)
        .unpause_minter_contract(MINTER_OWNER_ADDRESS_EXPR, None)
        .bond_contract_default_deploy_and_set(10u64, 100u64)
        .bond_unpause_contract(BONDING_OWNER_ADDRESS_EXPR, None);

    state.minter_disable_whitelist(MINTER_OWNER_ADDRESS_EXPR, None);

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(11u64));

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "https://test.com/test",
        "https://test.com/test",
        "random-url-encoded-here",
        "https://test.com/test",
        1000u64,
        5u64,
        &"Test title".repeat(1),
        &"Test description".repeat(1),
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        100u64 + 100u64,
        None,
    );

    state.pause_minter_contract(MINTER_OWNER_ADDRESS_EXPR, None);

    state.minter_burn(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        2u64,
        Some(TxExpect::user_error("str:Minting and burning not allowed")),
    );

    state.unpause_minter_contract(MINTER_OWNER_ADDRESS_EXPR, None);

    state.minter_burn(
        FIRST_USER_ADDRESS_EXPR,
        ANOTHER_TOKEN_IDENTIFIER,
        0,
        2u64,
        Some(TxExpect::user_error("str:Invalid payment token")),
    );

    state.minter_burn(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        0u64,
        Some(TxExpect::user_error("str:Value must be higher than zero")),
    );

    state.minter_burn(
        FIRST_USER_ADDRESS_EXPR,
        DATA_NFT_IDENTIFIER,
        1u64,
        3u64,
        None,
    );

    let data_nft_attributes: DataNftAttributes<SingleTxApi> = DataNftAttributes {
        data_stream_url: managed_buffer!(b"random-url-encoded-here"),
        data_preview_url: managed_buffer!(b"https://test.com/test"),
        data_marshal_url: managed_buffer!(b"https://test.com/test"),
        creator: managed_address!(&first_user_address),
        creation_time: 11u64,
        title: managed_buffer!(b"Test title"),
        description: managed_buffer!(b"Test description"),
    };

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            FIRST_USER_ADDRESS_EXPR,
            CheckAccount::new().esdt_nft_balance_and_attributes(
                DATA_NFT_IDENTIFIER_EXPR,
                1u64,
                "2",
                Some(data_nft_attributes),
            ),
        ));
}
