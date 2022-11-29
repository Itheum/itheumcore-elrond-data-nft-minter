use std::u8;

use datanftmint::nft_mint_utils::*;
use datanftmint::requirements::RequirementsModule;
use datanftmint::storage::*;
use datanftmint::*;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm::elrond_codec::multi_encode_iter_or_handle_err;
use elrond_wasm::hex_literal;
use elrond_wasm::types::{ManagedAddress, MultiValueEncoded};
use elrond_wasm::{
    storage::mappers::StorageTokenWrapper,
    types::{Address, EsdtLocalRole},
};

use elrond_wasm_debug::{
    managed_address, managed_biguint, managed_buffer, managed_token_id, managed_token_id_wrapped,
    rust_biguint, testing_framework::*, DebugApi,
};

pub const WASM_PATH: &'static str = "../output/datanftmint.wasm";
pub const OWNER_EGLD_BALANCE: u128 = 100 * 10u128.pow(18u32);
pub const TOKEN_ID: &[u8] = b"ITHEUM-df6f26";
pub const ANOTHER_TOKEN_ID: &[u8] = b"ANOTHER-123456";
pub const COLLECTION_NAME: &[u8] = b"DATANFT-FT";
pub const SFT_TICKER: &[u8] = b"DATANFTFT-1a2b3c";
pub const SFT_NAME: &[u8] = b"DATA NFT-FT";
pub const DATA_MARCHAL: &[u8] = b"DATA-MARCHAL-ENCRYPTED";
pub const DATA_STREAM: &[u8] = b"DATA-STREAM-ECRYPTED";
pub const MEDIA_URI: &[u8] = b"https://ipfs.io/ipfs/123456abcdef/metadata.json";
pub const USER_NFT_NAME: &[u8] = b"USER-NFT-NAME";
pub const MINT_TIME_LIMIT: u64 = 15;
pub const ROLES: &[EsdtLocalRole] = &[EsdtLocalRole::NftCreate, EsdtLocalRole::NftAddQuantity];

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> datanftmint::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub contract_wrapper:
        ContractObjWrapper<datanftmint::ContractObj<DebugApi>, ContractObjBuilder>,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub third_user_address: Address,
}

fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> datanftmint::ContractObj<DebugApi>,
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
    blockchain_wrapper.set_esdt_balance(
        &owner_address,
        ANOTHER_TOKEN_ID,
        &rust_biguint!(1_000_000),
    );
    blockchain_wrapper.set_esdt_balance(&first_user_address, TOKEN_ID, &rust_biguint!(10_000));
    blockchain_wrapper.set_esdt_balance(&owner_address, ANOTHER_TOKEN_ID, &rust_biguint!(10_000));
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

#[test] // Tests whether the contract is deployed and initialized correctly after deployment.
fn deploy_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
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

#[test] //Tests wether pause correct state after deployment
        //Tests wether the owner can unpause the contract
fn pause_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), true)
        })
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(false);
            },
        )
        .assert_ok();
}

#[test] // Tests if the contract has whitelist enabled and is empty
        // Tests if the royalties and supply are set accordingly
fn whitelist_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.white_list_enabled().get(), true)
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.white_list().len(), 0usize)
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.min_royalties().get(), 0u64)
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.max_royalties().get(), 8000u64)
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.max_supply().get(), 20u64)
        })
        .assert_ok();
}

#[test] // Tests whether the owner can initialize the contract correctly.
fn setup_contract_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(SFT_TICKER),
                    &managed_token_id_wrapped!(TOKEN_ID),
                    managed_biguint!(1_000_000),
                    MINT_TIME_LIMIT,
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
                    managed_buffer!(SFT_TICKER),
                    &managed_token_id_wrapped!(TOKEN_ID),
                    managed_biguint!(1_000_000),
                    MINT_TIME_LIMIT,
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(managed_token_id!(SFT_TICKER)),
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
                    managed_buffer!(SFT_TICKER),
                    &managed_token_id_wrapped!(TOKEN_ID),
                    managed_biguint!(1_000_000),
                    MINT_TIME_LIMIT,
                )
            },
        )
        .assert_user_error("Contract was already initialized");
}

#[test] // Tests whether the owner can change the anti spam tax token
        // Tests whether the owner can change the anti spam tax value
fn anti_spam_tax_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_anti_spam_tax(
                    managed_token_id_wrapped!(ANOTHER_TOKEN_ID),
                    managed_biguint!(2_000_000),
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_anti_spam_tax(
                    managed_token_id_wrapped!(TOKEN_ID),
                    managed_biguint!(2_000_000),
                )
            },
        )
        .assert_ok();
}

#[test] // Tests whether minting utilities for string creations works correctly.
        // Tests whether the concatenation and sha256 hash encryption works correctly.
fn nft_mint_utils_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
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
                    managed_buffer!(SFT_TICKER),
                    &managed_token_id_wrapped!(TOKEN_ID),
                    managed_biguint!(1_000_000),
                    MINT_TIME_LIMIT,
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let data_buffer = managed_buffer!(&[DATA_MARCHAL, DATA_STREAM].concat());
            let data_hash = sc.crypto().sha256(data_buffer).as_managed_buffer().clone();
            assert_eq!(
                sc.crate_hash_buffer(
                    &managed_buffer!(DATA_MARCHAL),
                    &managed_buffer!(DATA_STREAM)
                ),
                data_hash
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let uris = sc.create_uris(managed_buffer!(MEDIA_URI));
            let media_uri = uris.find(&managed_buffer!(MEDIA_URI));
            assert_eq!(media_uri, Some(0usize));
        })
        .assert_ok();
}

#[test] // Tests whether the requirements for minting are correctly checked.
        // Tests all possible cases for requirements.
fn requirements_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| {
                sc.initialize_contract(
                    managed_buffer!(COLLECTION_NAME),
                    managed_buffer!(SFT_TICKER),
                    &managed_token_id_wrapped!(TOKEN_ID),
                    managed_biguint!(1_000_000),
                    MINT_TIME_LIMIT,
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(false);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_minting_is_ready();
        })
        .assert_error(4, "Minting is not ready");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(true);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_minting_is_ready();
        })
        .assert_error(4, "Minting is not ready");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(managed_token_id!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_minting_is_ready();
        })
        .assert_error(4, "Minting is not ready");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(false);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_minting_is_ready();
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_is_privileged(&managed_address!(owner_address))
        })
        .assert_ok();

    // // ????
    // b_wrapper
    //     .execute_query(&setup.contract_wrapper, |sc| {
    //         sc.require_is_privileged(&managed_address!(first_user_address))
    //     })
    //     .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_value_is_positive(&managed_biguint!(0u64));
        })
        .assert_error(4, "Value must be positive");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_sft_is_valid(&managed_biguint!(90000u64), &managed_biguint!(2u64));
        })
        .assert_error(4, "Royalties are bigger than max royalties");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_sft_is_valid(&managed_biguint!(u8::MIN), &managed_biguint!(2u64));
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_sft_is_valid(&managed_biguint!(u8::MIN), &managed_biguint!(23u64));
        })
        .assert_error(4, "Max supply exceeded");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_sft_is_valid(&managed_biguint!(u8::MIN), &managed_biguint!(0u64));
        })
        .assert_error(4, "Supply must be positive");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_minting_is_allowed(&managed_address!(first_user_address), 0u64);
        })
        .assert_error(4, "You need to wait more time before minting again");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_minting_is_allowed(&managed_address!(first_user_address), 15u64);
        })
        .assert_error(4, "You are not whitelisted");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(managed_address!(&first_user_address));
                sc.set_whitelist_spots(args);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_minting_is_allowed(&managed_address!(first_user_address), 15u64);
        })
        .assert_ok();
}

#[test] // Tests whether minting works correctly.
fn mint_nft_ft_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(DATA_MARCHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_STREAM),
                    managed_biguint!(2),
                    managed_biguint!(10),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Minting is not ready");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(false);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(managed_token_id!(SFT_TICKER)),
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(DATA_MARCHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_STREAM),
                    managed_biguint!(20),
                    managed_biguint!(0),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Supply must be positive");

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(DATA_MARCHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_STREAM),
                    managed_biguint!(90000),
                    managed_biguint!(1),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Royalties are bigger than max royalties");

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(DATA_MARCHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_STREAM),
                    managed_biguint!(20),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "You are not whitelisted");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_mint_time_limit(15u64);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(DATA_MARCHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_STREAM),
                    managed_biguint!(20),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "You need to wait more time before minting again");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_mint_time_limit(0u64);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(managed_address!(&first_user_address));
                sc.set_whitelist_spots(args);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(DATA_MARCHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_STREAM),
                    managed_biguint!(20),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Value must be positive");

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(2u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(DATA_MARCHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_STREAM),
                    managed_biguint!(20),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Wrong amount of payment sent");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.anti_spam_tax(&managed_token_id_wrapped!(TOKEN_ID))
                .set(managed_biguint!(2));
        })
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            &first_user_address,
            &setup.contract_wrapper,
            &TOKEN_ID,
            0,
            &rust_biguint!(2),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(DATA_MARCHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_STREAM),
                    managed_biguint!(20),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_ok();
}
