use datanftmint::storage::ProxyTrait as _;
use multiversx_sc::storage::mappers::SingleValue;
use multiversx_sc_scenario::scenario_model::ScQueryStep;

use crate::minter_state::minter_state::{
    ContractsState, MINTER_ADMIN_ADDRESS_EXPR, MINTER_OWNER_ADDRESS_EXPR,
};

#[test]
fn deploy_and_upgrade_minter_test() {
    let mut state = ContractsState::new();
    let admin = state.admin.clone();

    state
        .deploy_minter()
        .minter_set_administarator(MINTER_OWNER_ADDRESS_EXPR, admin, None);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.minter_contract.is_paused())
            .expect_value(SingleValue::from(true)),
    );

    state.unpause_minter_contract(MINTER_ADMIN_ADDRESS_EXPR, None);

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.minter_contract.is_paused())
            .expect_value(SingleValue::from(false)),
    );

    state.minter_upgrade();

    state.world.sc_query(
        ScQueryStep::new()
            .call(state.minter_contract.is_paused())
            .expect_value(SingleValue::from(true)),
    );
}
