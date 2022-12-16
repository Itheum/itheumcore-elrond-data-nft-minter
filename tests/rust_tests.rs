use std::u8;

use datanftmint::collection_management::CollectionManagement;
use datanftmint::nft_mint_utils::*;
use datanftmint::requirements::RequirementsModule;
use datanftmint::storage::*;
use datanftmint::views::{UserDataOut, ViewsModule};
use datanftmint::*;
use elrond_wasm::contract_base::ContractBase;
use elrond_wasm::types::MultiValueEncoded;
use elrond_wasm::{
    elrond_codec::Empty,
    storage::mappers::StorageTokenWrapper,
    types::{Address, EsdtLocalRole},
};

use elrond_wasm_debug::tx_mock::TxContextRef;
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
pub const DATA_MARCHAL: &[u8] = b"https://DATA-MARCHAL-ENCRYPTED/marshal";
pub const DATA_STREAM: &[u8] = b"https://DATA-STREAM-ECRYPTED/stream";
pub const MEDIA_URI: &[u8] = b"https://ipfs.io/ipfs/123456abcdef/metadata.json";
pub const URL_WITH_SPACES: &[u8] = b"https://DATA-MARCHAL-ENCRYPTED/marshal with spaces";
pub const URL_WITH_RETURN: &[u8] = b"https://DATA-MARCHAL-ENCRYPTED/marshal\r";
pub const URL_WITHOUT_PROTOCOL: &[u8] = b"DATA-MARCHAL-ENCRYPTED/marshal/test/test/test";
pub const USER_NFT_NAME: &[u8] = b"USER-NFT-NAME";
pub const MINT_TIME_LIMIT: u64 = 15;
pub const ROLES: &[EsdtLocalRole] = &[
    EsdtLocalRole::NftCreate,
    EsdtLocalRole::NftAddQuantity,
    EsdtLocalRole::NftBurn,
];

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
    let first_user_address = &setup.first_user_address;

    b_wrapper
        .execute_tx(
            owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_administrator(managed_address!(first_user_address));
            },
        )
        .assert_ok();

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

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(false);
            },
        )
        .assert_ok();
}

#[test] // Tests if the contract has whitelist enabled and is empty by default after deployment
        // Tests if other values are set correctly after deployment
        // Tests if the owner and administrator can change the max supply and royalties
fn value_setters_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let administrator_address = &setup.first_user_address;

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

    b_wrapper
        .execute_tx(
            owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_administrator(managed_address!(administrator_address));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_max_supply(managed_biguint!(100u64));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_royalties_limits(managed_biguint!(0u64), managed_biguint!(100u64));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            administrator_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_royalties_limits(managed_biguint!(0u64), managed_biguint!(100u64));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            administrator_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_max_supply(managed_biguint!(100u64));
            },
        )
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

#[test] // Tests whether the owner and administrator can change the anti spam tax token
        // Tests whether the owner and administrator can change the anti spam tax value
fn anti_spam_tax_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let administrator_address = &setup.first_user_address;

    b_wrapper
        .execute_tx(
            owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_administrator(managed_address!(administrator_address));
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

    b_wrapper
        .execute_tx(
            &administrator_address,
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
            &administrator_address,
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
    let second_user_address = &setup.second_user_address;

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

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_administrator(managed_address!(first_user_address));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_max_supply(managed_biguint!(20));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_is_paused(true);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &second_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_is_paused(true);
            },
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_administrator(managed_address!(second_user_address));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &second_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_is_paused(true);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_is_paused(true);
            },
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_is_privileged(&managed_address!(first_user_address))
        })
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_value_is_positive(&managed_biguint!(0u64));
        })
        .assert_error(4, "Value must be higher than zero");

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
        .assert_error(4, "Supply must be higher than zero");

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
        // Tests wheter the creator is in the NFT-FT attributes
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
        .assert_error(4, "Supply must be higher than zero");

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
        .assert_error(4, "Value must be higher than zero");

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
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_anti_spam_tax(managed_token_id_wrapped!(TOKEN_ID), managed_biguint!(200)),
        )
        .assert_ok();

    b_wrapper.set_esdt_local_roles(setup.contract_wrapper.address_ref(), SFT_TICKER, ROLES);

    b_wrapper
        .execute_esdt_transfer(
            first_user_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(200),
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

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.minted_tokens().get(), 1u64);
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.minted_per_address(&managed_address!(first_user_address))
                    .get(),
                1u64
            );
        })
        .assert_ok();

    b_wrapper
        .execute_esdt_transfer(
            first_user_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(200),
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

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let data_out = UserDataOut {
                anti_spam_tax_value: sc.anti_spam_tax(&managed_token_id_wrapped!(TOKEN_ID)).get(),
                is_paused: sc.is_paused().get(),
                max_royalities: sc.max_royalties().get(),
                min_royalities: sc.min_royalties().get(),
                max_supply: sc.max_supply().get(),
                mint_time_limit: sc.mint_time_limit().get(),
                last_mint_time: sc
                    .last_mint_time(&managed_address!(first_user_address))
                    .get(),
                whitelist_enabled: sc.white_list_enabled().get(),
                is_whitelisted: sc
                    .white_list()
                    .contains(&managed_address!(first_user_address)),
            };
            assert_eq!(
                sc.get_user_data_out(
                    &managed_address!(first_user_address),
                    &managed_token_id_wrapped!(TOKEN_ID)
                ),
                data_out
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let token_data = sc.blockchain().get_esdt_token_data(
                &managed_address!(first_user_address),
                &managed_token_id!(SFT_TICKER),
                1u64,
            );
            let attributes = token_data.decode_attributes::<DataNftAttributes<TxContextRef>>();

            let test_attributes: DataNftAttributes<TxContextRef> = DataNftAttributes {
                creation_time: attributes.creation_time,
                creator: managed_address!(first_user_address),
                data_marshal_url: managed_buffer!(DATA_MARCHAL),
                data_preview_url: managed_buffer!(DATA_STREAM),
                data_stream_url: managed_buffer!(DATA_STREAM),
                title: managed_buffer!(USER_NFT_NAME),
                description: managed_buffer!(USER_NFT_NAME),
            };

            assert_eq!(test_attributes, attributes);
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let token_data = sc.blockchain().get_esdt_token_data(
                &managed_address!(first_user_address),
                &managed_token_id!(SFT_TICKER),
                2u64,
            );
            let attributes = token_data.decode_attributes::<DataNftAttributes<TxContextRef>>();

            let test_attributes: DataNftAttributes<TxContextRef> = DataNftAttributes {
                creation_time: attributes.creation_time,
                creator: managed_address!(first_user_address),
                data_marshal_url: managed_buffer!(DATA_MARCHAL),
                data_preview_url: managed_buffer!(DATA_STREAM),
                data_stream_url: managed_buffer!(DATA_STREAM),
                title: managed_buffer!(USER_NFT_NAME),
                description: managed_buffer!(USER_NFT_NAME),
            };

            assert_eq!(test_attributes, attributes);
        })
        .assert_ok()
}

#[test] //Tests wheter the whitelist functionality works as expected
fn white_list_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let second_user_address = &setup.second_user_address;

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_administrator(managed_address!(second_user_address));
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_white_list_enabled(true);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &second_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_white_list_enabled(true);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let whitelist = MultiValueEncoded::new();
            sc.set_whitelist_spots(whitelist)
        })
        .assert_user_error("Given whitelist is empty");

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(managed_address!(first_user_address));
                sc.set_whitelist_spots(args);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &second_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(managed_address!(first_user_address));
                sc.set_whitelist_spots(args);
            },
        )
        .assert_user_error("Address already in whitelist");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let whitelist = MultiValueEncoded::new();
            sc.remove_whitelist_spots(whitelist)
        })
        .assert_user_error("Given whitelist is empty");

    b_wrapper
        .execute_tx(
            &second_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(managed_address!(first_user_address));
                sc.remove_whitelist_spots(args);
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
                args.push(managed_address!(first_user_address));
                sc.remove_whitelist_spots(args);
            },
        )
        .assert_user_error("Address not in whitelist");
}

#[test] // Tests wheter the burn functionality works as expected
fn burn_token_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;

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
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_anti_spam_tax(managed_token_id_wrapped!(TOKEN_ID), managed_biguint!(200)),
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

    b_wrapper.set_esdt_local_roles(setup.contract_wrapper.address_ref(), SFT_TICKER, ROLES);

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
        .execute_esdt_transfer(
            first_user_address,
            &setup.contract_wrapper,
            TOKEN_ID,
            0,
            &rust_biguint!(200),
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

    b_wrapper.check_nft_balance(
        first_user_address,
        SFT_TICKER,
        1u64,
        &rust_biguint!(5),
        Option::<&Empty>::None,
    );

    b_wrapper
        .execute_esdt_transfer(
            &first_user_address,
            &setup.contract_wrapper,
            SFT_TICKER,
            1u64,
            &rust_biguint!(1),
            |sc| {
                sc.burn_token();
            },
        )
        .assert_ok();

    b_wrapper.check_nft_balance(
        first_user_address,
        SFT_TICKER,
        1u64,
        &rust_biguint!(4),
        Option::<&Empty>::None,
    );
}

#[test] // Tests wheter the url is valid
fn url_validation_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_url_is_valid(&managed_buffer!(URL_WITHOUT_PROTOCOL))
        })
        .assert_user_error("URL must start with https://");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_url_is_valid(&managed_buffer!(URL_WITH_SPACES))
        })
        .assert_user_error("URL contains invalid characters");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_url_is_valid(&managed_buffer!(URL_WITH_RETURN))
        })
        .assert_user_error("URL contains invalid characters");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_url_is_valid(&managed_buffer!(SFT_TICKER))
        })
        .assert_user_error("URL length is too small");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_url_is_adequate_length(&managed_buffer!(SFT_TICKER))
        })
        .assert_user_error("URL length is too small");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_url_is_valid(&managed_buffer!(&[
                SFT_TICKER,
                DATA_MARCHAL,
                DATA_STREAM,
                MEDIA_URI,
                DATA_STREAM,
                MEDIA_URI,
                DATA_STREAM,
                MEDIA_URI,
                SFT_TICKER,
                DATA_MARCHAL,
                DATA_STREAM,
                MEDIA_URI,
                DATA_STREAM,
                MEDIA_URI,
                DATA_STREAM,
                MEDIA_URI
            ]
            .concat()))
        })
        .assert_user_error("URL length is too big");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_url_is_adequate_length(&managed_buffer!(&[
                SFT_TICKER,
                DATA_MARCHAL,
                DATA_STREAM,
                MEDIA_URI,
                DATA_STREAM,
                MEDIA_URI,
                DATA_STREAM,
                MEDIA_URI,
                SFT_TICKER,
                DATA_MARCHAL,
                DATA_STREAM,
                MEDIA_URI,
                DATA_STREAM,
                MEDIA_URI,
                DATA_STREAM,
                MEDIA_URI
            ]
            .concat()))
        })
        .assert_user_error("URL length is too big");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_url_is_valid(&managed_buffer!(MEDIA_URI))
        })
        .assert_ok();
}

#[test] // Tests wheter an user cannont interact with functions that require privileges
fn privileges_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let user_address = &setup.first_user_address;
    let second_user_address = &setup.second_user_address;
    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_is_paused(false);
            },
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_anti_spam_tax(managed_token_id_wrapped!(TOKEN_ID), managed_biguint!(200)),
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.set_white_list_enabled(false),
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(managed_address!(user_address));
                sc.set_whitelist_spots(args);
            },
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(managed_address!(user_address));
                sc.remove_whitelist_spots(args);
            },
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.set_royalties_limits(managed_biguint!(200), managed_biguint!(200)),
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.set_max_supply(managed_biguint!(200)),
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.freeze_single_token_for_address(1u64, &managed_address!(second_user_address)),
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.unfreeze_single_token_for_address(1u64, &managed_address!(second_user_address)),
        )
        .assert_user_error("Address is not privileged");

    b_wrapper
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.wipe_single_token_for_address(1u64, &managed_address!(second_user_address)),
        )
        .assert_user_error("Address is not privileged");
}
