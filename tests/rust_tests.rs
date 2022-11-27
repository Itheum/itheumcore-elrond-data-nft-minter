use elrond_wasm::{
    elrond_codec::{multi_types::MultiValue2, Empty},
    storage::mappers::StorageTokenWrapper,
    types::{
        Address, EgldOrEsdtTokenIdentifier, EsdtLocalRole, EsdtTokenPayment, ManagedVec,
        MultiValueEncoded,
    },
};
use elrond_wasm_debug::{
    managed_address, managed_biguint, managed_buffer, managed_egld_token_id, managed_token_id,
    managed_token_id_wrapped, rust_biguint, testing_framework::*, DebugApi,
};
use sftmint::storage::*;
use sftmint::*;
use sftmint::{nft_mint_utils::*, views::ViewsModule};

pub const WASM_PATH: &'static str = "../output/sftmint.wasm";
pub const TOKEN_ID: &[u8] = b"ITHEUM-df6f26";
pub const WRONG_TOKEN_ID: &[u8] = b"WRONG-123456";
pub const OWNER_EGLD_BALANCE: u128 = 100 * 10u128.pow(18u32);
pub const COLLECTION_NAME: &[u8] = b"NFMESFT";
pub const SFT_TICKER: &[u8] = b"NFMESFT-1a2b3c";
pub const SFT_NAME: &[u8] = b"NFME SFT";
pub const MEDIA_CID: &[u8] = b"123456abcdef/image.png";
pub const METADATA_CID: &[u8] = b"123456abcdef/metadata.json";
pub const ROLES: &[EsdtLocalRole] = &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftAddQuantity];

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> sftmint::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub contract_wrapper: ContractObjWrapper<sftmint::ContractObj<DebugApi>, ContractObjBuilder>,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub third_user_address: Address,
}

fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> sftmint::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let first_user_address =
        blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE / 10u128));
    let second_user_address =
        blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE / 100u128));
    let third_user_address = blockchain_wrapper.create_user_account(&rust_biguint!(200u64));
    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE));
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );
    blockchain_wrapper.set_esdt_balance(&owner_address, TOKEN_ID, &rust_biguint!(5_000_000));
    blockchain_wrapper.set_esdt_balance(&owner_address, WRONG_TOKEN_ID, &rust_biguint!(1_000_000));
    blockchain_wrapper.set_esdt_balance(&first_user_address, TOKEN_ID, &rust_biguint!(10_000));
    blockchain_wrapper.set_esdt_balance(&owner_address, WRONG_TOKEN_ID, &rust_biguint!(10_000));
    blockchain_wrapper.set_esdt_balance(&second_user_address, TOKEN_ID, &rust_biguint!(0));
    blockchain_wrapper.set_esdt_balance(&third_user_address, TOKEN_ID, &rust_biguint!(1_000));

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        third_user_address,
        contract_wrapper: cf_wrapper,
    }
}

// Tests whether the contract is deployed and initialized correctly after deployment.
#[test]
fn deploy_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.init();
            },
        )
        .assert_ok();
}

// Tests whether the owner can initialize the contract correctly.
#[test]
fn setup_contract_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(2000u64),
                    managed_biguint!(0u64),
                    managed_biguint!(3u64),
                )
            },
        )
        .assert_user_error("Value must be greater than 0");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(2000u64),
                    managed_biguint!(3u64),
                    managed_biguint!(1u64),
                )
            },
        )
        .assert_user_error("Max per tx must be lower or equal to max per address");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(1u64),
                    managed_biguint!(3u64),
                    managed_biguint!(3u64),
                )
            },
        )
        .assert_user_error("Collection size must be greater than or equal to max per address");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(4u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(2000u64),
                    managed_biguint!(1u64),
                    managed_biguint!(3u64),
                )
            },
        )
        .assert_user_error("Issue cost is 0.05 eGLD");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(2000u64),
                    managed_biguint!(1u64),
                    managed_biguint!(3u64),
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(&managed_token_id!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(2000u64),
                    managed_biguint!(1u64),
                    managed_biguint!(3u64),
                )
            },
        )
        .assert_user_error("Contract was already initialized");
}

// Test whether minting utilities for string creations works correctly.
#[test]
fn nft_mint_utils_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(2000u64),
                    managed_biguint!(1u64),
                    managed_biguint!(3u64),
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(&managed_token_id!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.create_attributes(),
                managed_buffer!(&[b"metadata:", METADATA_CID].concat())
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let uris = sc.create_uris();
            let media_uri = uris.find(&managed_buffer!(
                &[b"https://ipfs.io/ipfs/", MEDIA_CID].concat()
            ));
            assert_eq!(media_uri, Some(0usize));

            let metadata_uri =
                uris.find(&managed_buffer!(
                    &[b"https://ipfs.io/ipfs/", METADATA_CID].concat()
                ));
            assert_eq!(metadata_uri, Some(1usize));
        })
        .assert_ok();
}

// Tests whether the pause setting function works as expected.
#[test]
fn set_is_paused_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), true);
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_is_paused(false),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), false);
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_is_paused(true),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), true);
        })
        .assert_ok();
}

// Test whether the private list enabling function works as expected.
#[test]
fn set_whitelist_enabled_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.white_list_enabled().get(), true);
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_white_list_enabled(false),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.white_list_enabled().get(), false);
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_white_list_enabled(true),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.white_list_enabled().get(), true);
        })
        .assert_ok();
}

// Tests whether setting private and public sale prices works correctly.
#[test]
fn set_price_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_public_price(managed_token_id_wrapped!(TOKEN_ID), managed_biguint!(7u64));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_private_price(managed_token_id_wrapped!(TOKEN_ID), managed_biguint!(5u64));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_public_price(managed_egld_token_id!(), managed_biguint!(0u64));
            },
        )
        .assert_user_error("Value must be greater than 0");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_private_price(managed_egld_token_id!(), managed_biguint!(0u64));
            },
        )
        .assert_user_error("Value must be greater than 0");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_private_price(managed_token_id_wrapped!("abc"), managed_biguint!(1u64));
            },
        )
        .assert_user_error("Token id is not valid");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_public_price(managed_token_id_wrapped!("abc"), managed_biguint!(1u64));
            },
        )
        .assert_user_error("Token id is not valid");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.token_private_price()
                    .get(&managed_token_id_wrapped!(TOKEN_ID)),
                Option::Some(managed_biguint!(5u64))
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.token_public_price()
                    .get(&managed_token_id_wrapped!(TOKEN_ID)),
                Option::Some(managed_biguint!(7u64))
            );
        })
        .assert_ok();
}

// Tests whether minting limits setting work as expected.
#[test]
fn set_mint_limits_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(2000u64),
                    managed_biguint!(1u64),
                    managed_biguint!(3u64),
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_max_per_tx(managed_biguint!(2u64));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_max_per_tx(managed_biguint!(5u64));
            },
        )
        .assert_user_error("Value must be lower than or equal to max per address");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_max_per_tx(managed_biguint!(0u64));
            },
        )
        .assert_user_error("Value must be greater than 0");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.max_per_tx().get(), managed_biguint!(2u64));
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_max_per_address(managed_biguint!(10u64));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_max_per_address(managed_biguint!(1u64));
            },
        )
        .assert_user_error("Value must be higher than or equal to max per tx");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_max_per_address(managed_biguint!(0u64));
            },
        )
        .assert_user_error("Value must be greater than 0");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.max_per_address().get(), managed_biguint!(10u64));
        })
        .assert_ok();
}

// Test whitelist spot setting functionality.
#[test]
fn set_whitelist_spots_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let second_user_address = &setup.second_user_address;
    let third_user_address = &setup.third_user_address;
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let args = MultiValueEncoded::new();
                sc.set_whitelist_spots(args);
            },
        )
        .assert_user_error("Given whitelist is empty");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue2(
                    (managed_address!(first_user_address), managed_biguint!(1)).into(),
                ));
                args.push(MultiValue2(
                    (managed_address!(second_user_address), managed_biguint!(2)).into(),
                ));
                sc.set_whitelist_spots(args);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.white_list(&managed_address!(first_user_address)).get(),
                managed_biguint!(1u64)
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.white_list(&managed_address!(second_user_address)).get(),
                managed_biguint!(2u64)
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.white_list(&managed_address!(third_user_address))
                    .is_empty(),
                true
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.white_list(&managed_address!(third_user_address)).get(),
                managed_biguint!(0u64)
            );
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue2(
                    (managed_address!(first_user_address), managed_biguint!(5u64)).into(),
                ));
                args.push(MultiValue2(
                    (
                        managed_address!(second_user_address),
                        managed_biguint!(0u64),
                    )
                        .into(),
                ));
                sc.set_whitelist_spots(args);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.white_list(&managed_address!(first_user_address)).get(),
                managed_biguint!(5u64)
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.white_list(&managed_address!(second_user_address)).get(),
                managed_biguint!(0u64)
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.white_list(&managed_address!(second_user_address))
                    .is_empty(),
                true
            );
        })
        .assert_ok();
}

// Tests whether creating the first token for the SFT sale works correctly.
#[test]
fn create_token_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.create_token(managed_buffer!(SFT_NAME)),
        )
        .assert_user_error("Token id must not be empty");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(2000u64),
                    managed_biguint!(1u64),
                    managed_biguint!(3u64),
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(&managed_token_id!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.create_token(managed_buffer!(SFT_TICKER)),
        )
        .assert_error(10u64, "action is not allowed");

    b_wrapper.set_esdt_local_roles(setup.contract_wrapper.address_ref(), SFT_TICKER, ROLES);

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.create_token(managed_buffer!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.token_created_nonce().get(), 1u64);
        })
        .assert_ok();

    b_wrapper.check_nft_balance(
        setup.contract_wrapper.address_ref(),
        SFT_TICKER,
        1u64,
        &rust_biguint!(1u64),
        Option::<&Empty>::None,
    );
}

// Tests whether the data out view works correctly.
#[test]
fn data_out_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let second_user_address = &setup.second_user_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(10u64),
                    managed_biguint!(2u64),
                    managed_biguint!(5u64),
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(&managed_token_id!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper.set_esdt_local_roles(setup.contract_wrapper.address_ref(), SFT_TICKER, ROLES);

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.create_token(managed_buffer!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_public_price(EgldOrEsdtTokenIdentifier::egld(), managed_biguint!(100)),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_private_price(EgldOrEsdtTokenIdentifier::egld(), managed_biguint!(110)),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let data_out = views::UserDataOut {
                how_many_can_mint: managed_biguint!(0u64),
                public_egld_price: managed_biguint!(100u64),
                private_egld_price: managed_biguint!(110u64),
                public_prices: ManagedVec::new(),
                private_prices: ManagedVec::new(),
                collection_size: managed_biguint!(10u64),
                minted_for_address: managed_biguint!(0u64),
                minted_in_total: managed_biguint!(1u64),
                can_mint: false,
                max_per_tx: managed_biguint!(2u64),
            };
            assert_eq!(
                sc.get_user_data_out_from_contract(&managed_address!(first_user_address)),
                data_out
            );
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue2(
                    (managed_address!(first_user_address), managed_biguint!(1)).into(),
                ));
                args.push(MultiValue2(
                    (managed_address!(second_user_address), managed_biguint!(2)).into(),
                ));
                sc.set_whitelist_spots(args);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let data_out = views::UserDataOut {
                how_many_can_mint: managed_biguint!(1u64),
                public_egld_price: managed_biguint!(100u64),
                private_egld_price: managed_biguint!(110u64),
                public_prices: ManagedVec::new(),
                private_prices: ManagedVec::new(),
                collection_size: managed_biguint!(10u64),
                minted_for_address: managed_biguint!(0u64),
                minted_in_total: managed_biguint!(1u64),
                can_mint: false,
                max_per_tx: managed_biguint!(2u64),
            };
            assert_eq!(
                sc.get_user_data_out_from_contract(&managed_address!(first_user_address)),
                data_out
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let data_out = views::UserDataOut {
                how_many_can_mint: managed_biguint!(2u64),
                public_egld_price: managed_biguint!(100u64),
                private_egld_price: managed_biguint!(110u64),
                public_prices: ManagedVec::new(),
                private_prices: ManagedVec::new(),
                collection_size: managed_biguint!(10u64),
                minted_for_address: managed_biguint!(0u64),
                minted_in_total: managed_biguint!(1u64),
                can_mint: false,
                max_per_tx: managed_biguint!(2u64),
            };
            assert_eq!(
                sc.get_user_data_out_from_contract(&managed_address!(second_user_address)),
                data_out
            );
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_white_list_enabled(false);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_is_paused(false);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let data_out = views::UserDataOut {
                how_many_can_mint: managed_biguint!(5u64),
                public_egld_price: managed_biguint!(100u64),
                private_egld_price: managed_biguint!(110u64),
                public_prices: ManagedVec::new(),
                private_prices: ManagedVec::new(),
                collection_size: managed_biguint!(10u64),
                minted_for_address: managed_biguint!(0u64),
                minted_in_total: managed_biguint!(1u64),
                can_mint: true,
                max_per_tx: managed_biguint!(2u64),
            };
            assert_eq!(
                sc.get_user_data_out_from_contract(&managed_address!(first_user_address)),
                data_out
            );
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let data_out = views::UserDataOut {
                how_many_can_mint: managed_biguint!(4u64),
                public_egld_price: managed_biguint!(100u64),
                private_egld_price: managed_biguint!(110u64),
                public_prices: ManagedVec::new(),
                private_prices: ManagedVec::new(),
                collection_size: managed_biguint!(10u64),
                minted_for_address: managed_biguint!(1u64),
                minted_in_total: managed_biguint!(2u64),
                can_mint: true,
                max_per_tx: managed_biguint!(2u64),
            };
            assert_eq!(
                sc.get_user_data_out_from_contract(&managed_address!(first_user_address)),
                data_out
            );
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_public_price(managed_token_id_wrapped!(TOKEN_ID), managed_biguint!(7u64));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_private_price(
                    managed_token_id_wrapped!(WRONG_TOKEN_ID),
                    managed_biguint!(5u64),
                );
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let mut private_prices = ManagedVec::new();
            private_prices.push(EsdtTokenPayment::new(
                managed_token_id!(WRONG_TOKEN_ID),
                0u64,
                managed_biguint!(5u64),
            ));
            let mut public_prices = ManagedVec::new();
            public_prices.push(EsdtTokenPayment::new(
                managed_token_id!(TOKEN_ID),
                0u64,
                managed_biguint!(7u64),
            ));
            let data_out = views::UserDataOut {
                how_many_can_mint: managed_biguint!(4u64),
                public_egld_price: managed_biguint!(100u64),
                private_egld_price: managed_biguint!(110u64),
                public_prices: public_prices,
                private_prices: private_prices,
                collection_size: managed_biguint!(10u64),
                minted_for_address: managed_biguint!(1u64),
                minted_in_total: managed_biguint!(2u64),
                can_mint: true,
                max_per_tx: managed_biguint!(2u64),
            };
            assert_eq!(
                sc.get_user_data_out_from_contract(&managed_address!(first_user_address)),
                data_out
            );
        })
        .assert_ok();
}

// Tests whether the minting flow for the public works as expected.
#[test]
fn mint_test() {
    let mut setup = setup_contract(sftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let second_user_address = &setup.second_user_address;
    let third_user_address = &setup.third_user_address;

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Minting is not ready");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(COLLECTION_NAME),
                    managed_biguint!(1000u64),
                    managed_buffer!(MEDIA_CID),
                    managed_buffer!(METADATA_CID),
                    managed_biguint!(10u64),
                    managed_biguint!(2u64),
                    managed_biguint!(5u64),
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Minting is not ready");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(&managed_token_id!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Minting is not ready");

    b_wrapper.set_esdt_local_roles(setup.contract_wrapper.address_ref(), SFT_TICKER, ROLES);

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Minting is not ready");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.minted_tokens().get(), managed_biguint!(0u64));
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.create_token(managed_buffer!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.minted_tokens().get(), managed_biguint!(1u64));
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Minting is not ready");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_is_paused(false),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Cannot buy with this token");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_public_price(EgldOrEsdtTokenIdentifier::egld(), managed_biguint!(100)),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Cannot buy with this token");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_private_price(EgldOrEsdtTokenIdentifier::egld(), managed_biguint!(100)),
        )
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            &first_user_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Cannot buy with this token");

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(105),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Wrong amount of payment sent");

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Payment too low");

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(300),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Value must be lower than or equal to max per tx");

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Maximum number of private sale mints for this address exceeded");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(MultiValue2(
                    (managed_address!(first_user_address), managed_biguint!(1)).into(),
                ));
                args.push(MultiValue2(
                    (managed_address!(second_user_address), managed_biguint!(2)).into(),
                ));
                args.push(MultiValue2(
                    (managed_address!(third_user_address), managed_biguint!(3)).into(),
                ));
                sc.set_whitelist_spots(args);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(200),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Maximum number of private sale mints for this address exceeded");

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Maximum number of private sale mints for this address exceeded");

    b_wrapper.check_nft_balance(
        &first_user_address,
        SFT_TICKER,
        1u64,
        &rust_biguint!(1u64),
        Option::<&Empty>::None,
    );

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.minted_tokens().get(), managed_biguint!(2u64));
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Maximum number of private sale mints for this address exceeded");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_white_list_enabled(false),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(200),
            |sc| sc.mint_token(),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(200),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Value must be lower than or equal to max per address");

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(200),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Value must be lower than or equal to max per address");

    b_wrapper
        .execute_tx(
            &second_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(200),
            |sc| sc.mint_token(),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &second_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &third_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(200),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Collection size exceeded");

    b_wrapper
        .execute_tx(
            &third_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &third_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(100),
            |sc| sc.mint_token(),
        )
        .assert_user_error("Collection size exceeded");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.minted_tokens().get(), sc.collection_size().get());
        })
        .assert_ok();
}
