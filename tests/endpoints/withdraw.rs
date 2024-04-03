use multiversx_sc_scenario::scenario_model::{
    AddressValue, CheckAccount, CheckStateStep, TransferStep, TxExpect,
};

use crate::minter_state::minter_state::{
    ContractsState, BONDING_OWNER_ADDRESS_EXPR, ITHEUM_TOKEN_IDENTIFIER,
    ITHEUM_TOKEN_IDENTIFIER_EXPR, MINTER_CONTRACT_ADDRESS_EXPR, MINTER_OWNER_ADDRESS_EXPR,
    THIRD_USER_ADDRESS_EXPR, WITHDRAWAL_ADDRESS_EXPR,
};

#[test]
fn withdraw_test() {
    let mut state = ContractsState::new();

    state
        .mock_minter_initialized(ITHEUM_TOKEN_IDENTIFIER, 100u64, 10u64)
        .unpause_minter_contract(MINTER_OWNER_ADDRESS_EXPR, None)
        .bond_contract_default_deploy_and_set(10u64, 100u64)
        .bond_unpause_contract(BONDING_OWNER_ADDRESS_EXPR, None);

    state.world.transfer_step(
        TransferStep::new()
            .from(THIRD_USER_ADDRESS_EXPR)
            .to(MINTER_CONTRACT_ADDRESS_EXPR)
            .esdt_transfer(ITHEUM_TOKEN_IDENTIFIER, 0u64, 100u64),
    );

    state.minter_withdraw(
        MINTER_OWNER_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        100u64,
        Some(TxExpect::user_error("str:Withdrawal address not set")),
    );

    state.set_withdrawal_address(
        MINTER_OWNER_ADDRESS_EXPR,
        AddressValue::from(WITHDRAWAL_ADDRESS_EXPR).to_address(),
        None,
    );

    state.minter_withdraw(
        MINTER_OWNER_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        100u64,
        Some(TxExpect::user_error(
            "str:Only withdrawal address can withdraw tokens",
        )),
    );

    state.minter_withdraw(
        WITHDRAWAL_ADDRESS_EXPR,
        ITHEUM_TOKEN_IDENTIFIER,
        0u64,
        100u64,
        None,
    );

    state
        .world
        .check_state_step(CheckStateStep::new().put_account(
            WITHDRAWAL_ADDRESS_EXPR,
            CheckAccount::new().esdt_balance(ITHEUM_TOKEN_IDENTIFIER_EXPR, "100"),
        ));
}
