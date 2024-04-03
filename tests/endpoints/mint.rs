use datanftmint::storage::DataNftAttributes;
use multiversx_sc_scenario::{
    api::SingleTxApi,
    managed_address, managed_buffer,
    scenario_model::{CheckAccount, CheckStateStep, SetStateStep, TxExpect},
};

use crate::minter_state::minter_state::{
    ContractsState, BONDING_CONTRACT_ADDRESS_EXPR, BONDING_OWNER_ADDRESS_EXPR,
    DATA_NFT_IDENTIFIER_EXPR, FIRST_USER_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER,
    ITHEUM_TOKEN_IDENTIFIER_EXPR, MINTER_CONTRACT_ADDRESS_EXPR, MINTER_OWNER_ADDRESS_EXPR,
    TREAASURY_ADDRESS_EXPR,
};

#[test]
fn mint_test_without_anti_spam_tax_test() {
    let mut state = ContractsState::new();

    state.deploy_minter();

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "https://test.com/test",
        "https://test.com/test",
        "random-encoded-string",
        "https://test.com/test",
        1000u64,
        5u64,
        "Test title",
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:Minting and burning not allowed")),
    );

    state
        .mock_minter_initialized(ITHEUM_TOKEN_IDENTIFIER, 0u64, 10u64)
        .unpause_minter_contract(MINTER_OWNER_ADDRESS_EXPR, None);

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "https://test.com/test",
        "https://test.com/test",
        "",
        "https://test.com/test",
        1000u64,
        5u64,
        "Test title",
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:Data Stream is empty")),
    );

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "https://test.com/test",
        "http://test.com/test",
        "random-url-encoded-here",
        "https://test.com/test",
        1000u64,
        5u64,
        "Test title",
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:URL must start with https://")),
    );

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "https://test.com/test",
        "",
        "random-url-encoded-here",
        "https://test.com/test",
        1000u64,
        5u64,
        "Test title",
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:URL is empty")),
    );

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "https://test.com/test",
        "https://",
        "random-url-encoded-here",
        "https://test.com/test",
        1000u64,
        5u64,
        "Test title",
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:URL length is too small")),
    );

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "https://test.com/test",
        &"https://".repeat(52),
        "random-url-encoded-here",
        "https://test.com/test",
        1000u64,
        5u64,
        "Test title",
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:URL length is too big")),
    );

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "http://test.com/test",
        "https://test.com/test",
        "https://test.com/test",
        "random-url-encoded-here",
        "https://test.com/test",
        1000u64,
        5u64,
        "Test title",
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:URL must start with https://")),
    );

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "http://test.com/test",
        "https://test.com/test",
        "random-url-encoded-here",
        "https://test.com/test",
        1000u64,
        5u64,
        "Test title",
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:URL must start with https://")),
    );

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
        "Test title",
        "",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:Field is empty")),
    );

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
        "",
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:Field is empty")),
    );

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
        &"Test title".repeat(30),
        "Test description",
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:Too many characters")),
    );

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
        &"Test description".repeat(301),
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:Too many characters")),
    );

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "https://test.com/test",
        "https://test.com/test",
        "random-url-encoded-here",
        "https://test.com/test",
        10000u64,
        5u64,
        &"Test title".repeat(1),
        &"Test description".repeat(1),
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error(
            "str:Royalties are bigger than max royalties",
        )),
    );

    state.minter_mint(
        FIRST_USER_ADDRESS_EXPR,
        "Test",
        "https://test.com/test",
        "https://test.com/test",
        "https://test.com/test",
        "random-url-encoded-here",
        "https://test.com/test",
        1000u64,
        21u64,
        &"Test title".repeat(1),
        &"Test description".repeat(1),
        10u64,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        10u64 + 100u64,
        Some(TxExpect::user_error("str:Max supply exceeded")),
    );

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
        10u64 + 100u64,
        Some(TxExpect::user_error(
            "str:You need to wait more time before minting again",
        )),
    );

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
        10u64 + 100u64,
        Some(TxExpect::user_error("str:You are not whitelisted")),
    );

    state.minter_disable_whitelist(MINTER_OWNER_ADDRESS_EXPR, None);

    state.deploy_bonding();

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
        0u64,
        Some(TxExpect::user_error("str:Wrong bond period")),
    );

    state.bond_contract_default_deploy_and_set(10u64, 100u64);

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
        10 + 100u64,
        Some(TxExpect::user_error("str:Wrong amount of funds")),
    );

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
        100u64,
        None,
    );

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(
                FIRST_USER_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100"), // 200-100 = 100
            )
            .put_account(
                MINTER_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, ""),
            )
            .put_account(
                BONDING_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100"), // 100 for BOND
            ),
    );
}

#[test]
fn mint_with_anti_spam_tax_test_and_whitelist() {
    let mut state = ContractsState::new();
    let first_user_address = state.first_user.clone();

    state
        .mock_minter_initialized(ITHEUM_TOKEN_IDENTIFIER, 100u64, 10u64)
        .unpause_minter_contract(MINTER_OWNER_ADDRESS_EXPR, None)
        .bond_contract_default_deploy_and_set(10u64, 100u64)
        .bond_unpause_contract(BONDING_OWNER_ADDRESS_EXPR, None);

    state.minter_enable_whitelist(MINTER_OWNER_ADDRESS_EXPR, None);

    state
        .world
        .set_state_step(SetStateStep::new().block_timestamp(11u64));

    state.minter_add_to_whitelist(MINTER_OWNER_ADDRESS_EXPR, first_user_address.clone(), None);

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
        100u64,
        Some(TxExpect::user_error("str:Wrong amount of funds")),
    );

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
        99u64 + 100u64,
        Some(TxExpect::user_error("str:Wrong amount of funds")),
    );

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

    state.minter_remove_from_whitelist(MINTER_OWNER_ADDRESS_EXPR, first_user_address.clone(), None);

    let data_nft_attributes: DataNftAttributes<SingleTxApi> = DataNftAttributes {
        data_stream_url: managed_buffer!(b"random-url-encoded-here"),
        data_preview_url: managed_buffer!(b"https://test.com/test"),
        data_marshal_url: managed_buffer!(b"https://test.com/test"),
        creator: managed_address!(&first_user_address),
        creation_time: 11u64,
        title: managed_buffer!(b"Test title"),
        description: managed_buffer!(b"Test description"),
    };

    state.world.check_state_step(
        CheckStateStep::new()
            .put_account(
                FIRST_USER_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "0"), // 200-100 (bond) - 100 (spam tax) = 0
            )
            .put_account(
                MINTER_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "0"),
            )
            .put_account(
                BONDING_CONTRACT_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100"), // 100 for BOND
            )
            .put_account(
                TREAASURY_ADDRESS_EXPR,
                CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100"),
            )
            .put_account(
                FIRST_USER_ADDRESS_EXPR,
                CheckAccount::new().esdt_nft_balance_and_attributes(
                    DATA_NFT_IDENTIFIER_EXPR,
                    1u64,
                    "5",
                    Some(data_nft_attributes),
                ),
            ),
    );
}
