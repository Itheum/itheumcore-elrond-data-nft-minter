use datanftmint::{requirements::RequirementsModule, storage::StorageModule as _, DataNftMint};
use multiversx_sc::{storage::mappers::StorageTokenWrapper as _, types::BigUint};
use multiversx_sc_scenario::{
    api::SingleTxApi, managed_address, managed_buffer, managed_token_id,
    scenario_model::AddressValue,
};

use crate::minter_state::minter_state::ITHEUM_TOKEN_IDENTIFIER;

mod endpoints;
mod minter_state;

#[test]
fn minter_contract_ready_test() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract.init();

        minter_contract.require_ready_for_minting_and_burning();
    });
    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();

        minter_contract.administrator().set(managed_address!(
            &AddressValue::from("address:admin").to_address()
        ));

        minter_contract.require_ready_for_minting_and_burning();
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();

        minter_contract.is_paused().set(false);

        minter_contract.require_ready_for_minting_and_burning();
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();

        minter_contract.is_paused().set(false);
        minter_contract.administrator().set(managed_address!(
            &AddressValue::from("address:admin").to_address()
        ));

        minter_contract
            .treasury_address()
            .set(managed_address!(
                &AddressValue::from("address:treasury").to_address()
            ));

        minter_contract.require_ready_for_minting_and_burning();
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();

        minter_contract.is_paused().set(false);
        minter_contract.administrator().set(managed_address!(
            &AddressValue::from("address:admin").to_address()
        ));

        minter_contract
            .treasury_address()
            .set(managed_address!(
                &AddressValue::from("address:treasury").to_address()
            ));

        minter_contract
            .token_id()
            .set_token_id(managed_token_id!(b"TOKEN-fb133"));

        minter_contract.require_ready_for_minting_and_burning();
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();

        minter_contract.is_paused().set(false);
        minter_contract.administrator().set(managed_address!(
            &AddressValue::from("address:admin").to_address()
        ));

        minter_contract
            .treasury_address()
            .set(managed_address!(
                &AddressValue::from("address:treasury").to_address()
            ));

        minter_contract
            .token_id()
            .set_token_id(managed_token_id!(b"TOKEN-fb133"));

        minter_contract
            .bond_contract_address()
            .set(managed_address!(
                &AddressValue::from("address:bond").to_address()
            ));

        minter_contract.require_ready_for_minting_and_burning();
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();

        minter_contract.is_paused().set(false);
        minter_contract.administrator().set(managed_address!(
            &AddressValue::from("address:admin").to_address()
        ));

        minter_contract
            .treasury_address()
            .set(managed_address!(
                &AddressValue::from("address:treasury").to_address()
            ));

        minter_contract
            .token_id()
            .set_token_id(managed_token_id!(ITHEUM_TOKEN_IDENTIFIER));

        minter_contract
            .bond_contract_address()
            .set(managed_address!(
                &AddressValue::from("address:bond").to_address()
            ));

        minter_contract.roles_are_set().set(true);

        minter_contract
            .donation_treasury_address()
            .set(managed_address!(
                &AddressValue::from("address:donation").to_address()
            ));

        minter_contract.require_ready_for_minting_and_burning();
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_withdrawal_address_is_set() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract.require_withdrawal_address_is_set();
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract
            .withdrawal_address()
            .set(managed_address!(
                &AddressValue::from("address:withdraw").to_address()
            ));

        minter_contract.require_withdrawal_address_is_set();
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_minting_is_allowed_test() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract.init();
        minter_contract.require_minting_is_allowed(
            &managed_address!(&AddressValue::from("address:test").to_address()),
            0,
        );
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();
        minter_contract
            .last_mint_time(&managed_address!(
                &AddressValue::from("address:test").to_address()
            ))
            .set(12);

        minter_contract.mint_time_limit().set(10);

        minter_contract.require_minting_is_allowed(
            &managed_address!(&AddressValue::from("address:test").to_address()),
            11,
        );
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();
        minter_contract
            .last_mint_time(&managed_address!(
                &AddressValue::from("address:test").to_address()
            ))
            .set(12);

        minter_contract.mint_time_limit().set(10);

        minter_contract.require_minting_is_allowed(
            &managed_address!(&AddressValue::from("address:test").to_address()),
            23,
        );
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();
        minter_contract
            .last_mint_time(&managed_address!(
                &AddressValue::from("address:test").to_address()
            ))
            .set(12);

        minter_contract.mint_time_limit().set(10);

        minter_contract.whitelist_enabled().set(false);

        minter_contract.require_minting_is_allowed(
            &managed_address!(&AddressValue::from("address:test").to_address()),
            23,
        );
    });

    assert_eq!(result.is_ok(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.init();
        minter_contract
            .last_mint_time(&managed_address!(
                &AddressValue::from("address:test").to_address()
            ))
            .set(12);

        minter_contract.mint_time_limit().set(10);

        minter_contract.whitelist_enabled().set(true);

        minter_contract.whitelist().insert(managed_address!(
            &AddressValue::from("address:test").to_address()
        ));

        minter_contract.require_minting_is_allowed(
            &managed_address!(&AddressValue::from("address:test").to_address()),
            23,
        );
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_value_is_positive_test() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract.require_value_is_positive(&BigUint::zero());
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_value_is_positive(&BigUint::from(1u64));
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_sft_is_valid_test() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract.require_sft_is_valid(&BigUint::zero(), &BigUint::zero());
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.max_royalties().set(BigUint::from(100u64));
        minter_contract.min_royalties().set(BigUint::from(10u64));
        minter_contract.max_supply().set(BigUint::from(1000u64));

        minter_contract.require_sft_is_valid(&BigUint::from(100u64), &BigUint::from(1000u64));
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_title_description_are_valid_test() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract
            .require_title_description_are_valid(&managed_buffer!(b""), &managed_buffer!(b""));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract
            .require_title_description_are_valid(&managed_buffer!(b"Title"), &managed_buffer!(b""));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_title_description_are_valid(
            &managed_buffer!(b""),
            &managed_buffer!(b"Description"),
        );
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_title_description_are_valid(
            &managed_buffer!(b"Title"),
            &managed_buffer!(b"Description"),
        );
    });

    assert_eq!(result.is_ok(), true);

    let result = std::panic::catch_unwind(|| {
        minter_contract.require_title_description_are_valid(
            &managed_buffer!(&[1u8; 101]),
            &managed_buffer!(&[1u8; 400]),
        );
    });

    assert_eq!(result.is_err(), true);

    let result = std::panic::catch_unwind(|| {
        minter_contract.require_title_description_are_valid(
            &managed_buffer!(&[1u8; 100]),
            &managed_buffer!(&[1u8; 401]),
        );
    });

    assert_eq!(result.is_err(), true);

    let result = std::panic::catch_unwind(|| {
        minter_contract.require_title_description_are_valid(
            &managed_buffer!(&[1u8; 100]),
            &managed_buffer!(&[1u8; 400]),
        );
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_url_is_valid_test() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_valid(&managed_buffer!(b""));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_valid(&managed_buffer!(b"http://"));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_valid(&managed_buffer!(b"https://"));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_valid(&managed_buffer!(b"https://test.com/test/test  "));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_valid(&managed_buffer!(b"https://test.com/test/test\r\n"));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_valid(&managed_buffer!(b"https://test.com/test/test"));
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_url_is_adequate_length_test() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_adequate_length(&managed_buffer!(b""));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_adequate_length(&managed_buffer!(&[1u8; 401]));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_adequate_length(&managed_buffer!(&[1u8; 400]));
    });

    assert_eq!(result.is_ok(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_url_is_adequate_length(&managed_buffer!(&[1u8; 15]));
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_royalties_are_valid_test() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract.require_royalties_are_valid(&BigUint::from(100u64), &BigUint::from(10u64));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract.require_royalties_are_valid(&BigUint::from(10u64), &BigUint::from(100u64));
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_token_issued_test() {
    let minter_conttract = datanftmint::contract_obj::<SingleTxApi>();

    let result = std::panic::catch_unwind(|| {
        minter_conttract.require_token_issued();
    });

    assert_eq!(result.is_err(), true);

    let result = std::panic::catch_unwind(|| {
        minter_conttract
            .token_id()
            .set_token_id(managed_token_id!(ITHEUM_TOKEN_IDENTIFIER));

        minter_conttract.require_token_issued();
    });

    assert_eq!(result.is_ok(), true);
}

#[test]
fn require_is_withdrawal_address_test() {
    let minter_contract = datanftmint::contract_obj::<SingleTxApi>();

    let mut result = std::panic::catch_unwind(|| {
        minter_contract
            .withdrawal_address()
            .set(managed_address!(
                &AddressValue::from("address:withdraw").to_address()
            ));

        minter_contract
            .require_is_withdrawal_address(&managed_address!(
                &AddressValue::from("address:test").to_address()
            ));
    });

    assert_eq!(result.is_err(), true);

    result = std::panic::catch_unwind(|| {
        minter_contract
            .withdrawal_address()
            .set(managed_address!(
                &AddressValue::from("address:withdraw").to_address()
            ));

        minter_contract.require_is_withdrawal_address(&managed_address!(&AddressValue::from(
            "address:withdraw"
        )
        .to_address()));
    });

    assert_eq!(result.is_ok(), true);
}
