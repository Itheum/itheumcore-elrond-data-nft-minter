use std::u8;

use datanftmint::collection_management::CollectionManagement;
use datanftmint::nft_mint_utils::*;
use datanftmint::requirements::RequirementsModule;
use datanftmint::storage::*;
use datanftmint::views::{UserDataOut, ViewsModule};
use datanftmint::*;
use multiversx_sc::contract_base::ContractBase;
use multiversx_sc::types::{ManagedBuffer, ManagedVec, MultiValueEncoded};
use multiversx_sc::{
    codec::Empty,
    storage::mappers::StorageTokenWrapper,
    types::{Address, EsdtLocalRole},
};

use multiversx_sc_scenario::multiversx_chain_vm::tx_mock::TxContextRef;
use multiversx_sc_scenario::testing_framework::{BlockchainStateWrapper, ContractObjWrapper};
use multiversx_sc_scenario::*;

pub const WASM_PATH: &'static str = "../output/datanftmint.wasm";
pub const OWNER_EGLD_BALANCE: u128 = 100 * 10u128.pow(18u32);
pub const TOKEN_ID: &[u8] = b"ITHEUM-df6f26";
pub const ANOTHER_TOKEN_ID: &[u8] = b"ANOTHER-123456";
pub const COLLECTION_NAME: &[u8] = b"DATANFT-FT";
pub const SFT_TICKER: &[u8] = b"DATANFTFT-1a2b3c";
pub const SFT_NAME: &[u8] = b"DATA NFT-FT";
pub const DATA_MARSHAL: &[u8] = b"https://DATA-MARSHAL-ENCRYPTED/marshal";
pub const DATA_STREAM: &[u8] = b"https://DATA-STREAM-ECRYPTED/stream";
pub const DATA_PREVIEW: &[u8] = b"https://DATA-STREAM-ECRYPTED/stream-preview";
pub const MEDIA_URI: &[u8] = b"https://ipfs.io/ipfs/123456abcdef/media.json";
pub const METADATA_URI: &[u8] = b"https://ipfs.io/ipfs/123456abcdef/metadata.json";
pub const URL_WITH_SPACES: &[u8] = b"https://DATA-MARSHAL-ENCRYPTED/marshal with spaces";
pub const URL_WITH_RETURN: &[u8] = b"https://DATA-MARSHAL-ENCRYPTED/marshal\r";
pub const URL_WITHOUT_PROTOCOL: &[u8] = b"DATA-MARSHAL-ENCRYPTED/marshal/test/test/test";
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
    pub treasury_address: Address,
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
    let treasury_address =
        blockchain_wrapper.create_user_account(&rust_biguint!(OWNER_EGLD_BALANCE / 10u128));
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
        treasury_address,
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

#[test] //Tests owner setting a new admin
        //Tests whether pause correct state after deployment
        //Tests whether the owner can unpause the contract and pause again
        //Tests whether the admin can unpause the contract
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
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), false)
        })
        .assert_ok();

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
            assert_eq!(sc.is_paused().get(), true)
        })
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

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.is_paused().get(), false)
        })
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
    let treasury_address = &setup.treasury_address;

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
                    managed_address!(treasury_address),
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
                    managed_address!(treasury_address),
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
                    managed_address!(treasury_address),
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
    let treasury_address = &setup.treasury_address;

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
                    managed_address!(treasury_address),
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let data_buffer = managed_buffer!(&[DATA_MARSHAL, DATA_STREAM].concat());
            let data_hash = sc.crypto().sha256(data_buffer).as_managed_buffer().clone();
            assert_eq!(
                sc.crate_hash_buffer(
                    &managed_buffer!(DATA_MARSHAL),
                    &managed_buffer!(DATA_STREAM)
                ),
                data_hash
            );
        })
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let uris = sc.create_uris(managed_buffer!(MEDIA_URI), managed_buffer!(METADATA_URI));
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
    let treasury_address = &setup.treasury_address;

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
                    managed_address!(treasury_address),
                )
            },
        )
        .assert_ok();

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.treasury_address().get(),
                managed_address!(treasury_address)
            );
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
        // Tests if the creator is in the NFT-FT attributes
fn mint_nft_ft_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let treasury_address = &setup.treasury_address;

    // [test] when deployed a smart contract is paused and token_id is empty so require_minting_is_ready asserts
    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2),
                    managed_biguint!(10),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Minting is not ready");

    // [setup] owner unpauses contract
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

    // [setup] owner sets the token ID (in real world, this is done via a callback after actual collection is minted)
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(managed_token_id!(SFT_TICKER)),
        )
        .assert_ok();

    // [test] require_sft_is_valid assert fails as supply is 0
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.treasury_address()
                    .set(&managed_address!(treasury_address));
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(20),
                    managed_biguint!(0),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Supply must be higher than zero");

    // [test] require_sft_is_valid assert fails as royalties exceed the max royalties
    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(90000),
                    managed_biguint!(1),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Royalties are bigger than max royalties");

    // @TODO DAVID: also test following assertions so we have complete test flow: "Royalties are smaller than min royalties",  "Max supply exceeded"

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.min_royalties().set(&managed_biguint!(100u64)),
        )
        .assert_ok();

    // [test] require_sft_is_valid assert fails as royalties are smaller than min royalties
    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(0u64),
                    managed_biguint!(1),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_user_error("Royalties are smaller than min royalties");

    // [test] require_sft_is_valid assert fails as supply exceeds max supply
    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(5000u64),
                    managed_biguint!(1000),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_user_error("Max supply exceeded");

    // [test] require_minting_is_allowed assert fails as caller not whitelisted
    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "You are not whitelisted");

    // [setup] setting mint_time_limit to 15 mins (is it mins or sec or ms?).. i.e. u need to wait 15 mins to try again
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

    // [test] fails as user did not wait 15 mins to try (@TOCONFIRM - if last test reverted then last_mint_time should not be set right? so how does this fail?)
    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "You need to wait more time before minting again");

    // [setup] setting mint_time_limit to 0 mins (is it mins or sec or ms?).. i.e. u need to wait 0 mins to try again
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

    // [setup] setting set_whitelist_spots to caller so he is whitelisted
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

    // [test] require_value_is_positive assert fails as user is setting egld_payment to 0 (should be the ITHEUM tax amount)
    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Value must be higher than zero");

    // [test] fails as tax payment is not sufficient (egld_payment value is less than anti_spam_tax(&payment.token_identifier).get())
    b_wrapper
        .execute_tx(
            &first_user_address,
            &setup.contract_wrapper,
            &rust_biguint!(2u64),
            |sc| {
                sc.mint_token(
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(MEDIA_URI),
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "Wrong amount of payment sent");

    // [setup] setting set_anti_spam_tax to 200 ITHEUM
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_anti_spam_tax(managed_token_id_wrapped!(TOKEN_ID), managed_biguint!(200)),
        )
        .assert_ok();

    // [setup] giving the minter contract special roles needed. (in actual contract this is done on async OK of initialize_contract.issue_semi_fungible)
    b_wrapper.set_esdt_local_roles(setup.contract_wrapper.address_ref(), SFT_TICKER, ROLES);

    // [test] minting will now succeed as it meets all criteria
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_ok();

    // [test] as minting succeeded, minted_tokens should increment by 1
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.minted_tokens().get(), 1u64);
        })
        .assert_ok();
    // check if the payment token was transfered from the contract to the treasury address
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.blockchain()
                    .get_sc_balance(&managed_token_id_wrapped!(TOKEN_ID), 0),
                managed_biguint!(0u64)
            )
        })
        .assert_ok();
    // check if the data NFT-FT is not in the contract balance
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.blockchain()
                    .get_sc_balance(&managed_token_id_wrapped!(SFT_TICKER), 1),
                managed_biguint!(0u64)
            )
        })
        .assert_ok();

    // [test] as minting succeeded, minted_per_address should increment by 1
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.minted_per_address(&managed_address!(first_user_address))
                    .get(),
                1u64
            );
        })
        .assert_ok();

    // [setup] remove the user from whitelist so we can test re-mint prevention
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| {
                let mut args = MultiValueEncoded::new();
                args.push(managed_address!(&first_user_address));
                sc.remove_whitelist_spots(args);
            },
        )
        .assert_ok();

    // [test] mint another SFT but it will fail as user was removed from whitelist
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_error(4, "You are not whitelisted");

    // [setup] whitelisting him again so we can test a remint
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

    // [test] mint another SFT (with new links), it succeeds as the mint_time_limit was set to 0 above
    let data_stream_2: &[u8] = b"https://DATA-STREAM-ECRYPTED/stream-2";
    let data_preview_2: &[u8] = b"https://DATA-STREAM-ECRYPTED/stream-preview";

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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(data_stream_2),
                    managed_buffer!(data_preview_2),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_ok();

    // [test] as minting succeeded, minted_tokens should increment by 1 (1 -> 2)
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(sc.minted_tokens().get(), 2u64);
        })
        .assert_ok();

    // [test] as minting succeeded, minted_per_address should increment by 1 (1 -> 2)
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.minted_per_address(&managed_address!(first_user_address))
                    .get(),
                2u64
            );
        })
        .assert_ok();

    // [test] test if the get_user_data_out view returns the correct final state view based on our tests above
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            let nonces = sc.freezed_sfts_per_address(&managed_address!(first_user_address));
            let mut frozen_nonces = ManagedVec::new();
            for item in nonces.iter() {
                frozen_nonces.push(item);
            }
            let data_out = UserDataOut {
                anti_spam_tax_value: sc.anti_spam_tax(&managed_token_id_wrapped!(TOKEN_ID)).get(),
                is_paused: sc.is_paused().get(),
                max_royalties: sc.max_royalties().get(),
                min_royalties: sc.min_royalties().get(),
                max_supply: sc.max_supply().get(),
                mint_time_limit: sc.mint_time_limit().get(),
                last_mint_time: sc
                    .last_mint_time(&managed_address!(first_user_address))
                    .get(),
                whitelist_enabled: sc.white_list_enabled().get(),
                is_whitelisted: sc
                    .white_list()
                    .contains(&managed_address!(first_user_address)),
                minted_per_user: sc
                    .minted_per_address(&managed_address!(first_user_address))
                    .get(),
                total_minted: sc.minted_tokens().get(),
                frozen: sc
                    .freezed_addresses_for_collection()
                    .contains(&managed_address!(first_user_address)),
                frozen_nonces: frozen_nonces,
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

    // [test] test if DataNftAttributes of 1st (nonce) SFT minted above matches on-chain state (and if creator attr holds user address)
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
                data_marshal_url: managed_buffer!(DATA_MARSHAL),
                data_preview_url: managed_buffer!(DATA_PREVIEW),
                data_stream_url: managed_buffer!(DATA_STREAM),
                title: managed_buffer!(USER_NFT_NAME),
                description: managed_buffer!(USER_NFT_NAME),
            };

            let mut correct_uris: ManagedVec<DebugApi, ManagedBuffer<DebugApi>> = ManagedVec::new();

            correct_uris.push(managed_buffer!(MEDIA_URI));
            correct_uris.push(managed_buffer!(METADATA_URI));

            assert_eq!(correct_uris, token_data.uris);

            assert_eq!(test_attributes, attributes);
        })
        .assert_ok();

    // [test] test if DataNftAttributes of 2nd (nonce) SFT minted above matches on-chain state (and if creator attr holds user address)
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
                data_marshal_url: managed_buffer!(DATA_MARSHAL),
                data_preview_url: managed_buffer!(data_preview_2),
                data_stream_url: managed_buffer!(data_stream_2),
                title: managed_buffer!(USER_NFT_NAME),
                description: managed_buffer!(USER_NFT_NAME),
            };

            assert_eq!(test_attributes, attributes);
        })
        .assert_ok()
}

#[test] //Tests whether the whitelist functionality works as expected
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

#[test] // Tests whether the burn functionality works as expected
fn burn_token_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let treasury_address = &setup.treasury_address;

    // [setup] add caller to whitelist so he can mint
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

    // [setup] add anti-spam tax or minting is prevented
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0),
            |sc| sc.set_anti_spam_tax(managed_token_id_wrapped!(TOKEN_ID), managed_biguint!(200)),
        )
        .assert_ok();

    // [setup] add collection id or minting is prevented
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(5u64 * 10u64.pow(16u32)),
            |sc| sc.token_id().set_token_id(managed_token_id!(SFT_TICKER)),
        )
        .assert_ok();

    // [setup] give contract required sft roles
    b_wrapper.set_esdt_local_roles(setup.contract_wrapper.address_ref(), SFT_TICKER, ROLES);

    // [setup] unpause or minting is prevented
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

    // [test] mint an SFT with 5 supply
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.treasury_address()
                    .set(&managed_address!(treasury_address));
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(20),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_ok();

    // [test] check if blockchain reports a balance of 5 for the newly minted SFT
    b_wrapper.check_nft_balance(
        first_user_address,
        SFT_TICKER,
        1u64,
        &rust_biguint!(5),
        Option::<&Empty>::None,
    );

    // [test] burn just 1 of the supply
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

    // [test] check if blockchain reports a new balance of 4
    b_wrapper.check_nft_balance(
        first_user_address,
        SFT_TICKER,
        1u64,
        &rust_biguint!(4),
        Option::<&Empty>::None,
    );
}

#[test] // Tests whether the url is valid
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
            sc.require_url_is_valid(&managed_buffer!(b""));
        })
        .assert_user_error("URL is empty");

    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            sc.require_url_is_valid(&managed_buffer!(MEDIA_URI))
        })
        .assert_ok();
}

#[test] // Tests whether an user cannot interact with functions that require privileges
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

#[test] // Freeze functions test
fn freeze_function_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let treasury_address = &setup.treasury_address;

    // [setup] owner sets the token ID (in real world, this is done via a callback after actual collection is minted)
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
            &rust_biguint!(0u64),
            |sc| {
                sc.treasury_address()
                    .set(&managed_address!(treasury_address));
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_ok();

    //    [test] owner can freeze collection for address
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freeze_collection_for_address(&managed_address!(first_user_address));
            },
        )
        .assert_ok();

    // [test] check that the address is stored in the frozen storage
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.freezed_addresses_for_collection()
                    .contains(&managed_address!(first_user_address)),
                true
            );
        })
        .assert_ok();
}

#[test] // Unfreeze function test
fn unfreeze_function_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let treasury_address = &setup.treasury_address;

    // [setup] owner sets the token ID (in real world, this is done via a callback after actual collection is minted)
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
            &rust_biguint!(0u64),
            |sc| {
                sc.treasury_address()
                    .set(&managed_address!(treasury_address));
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_ok();
    // [test] freeze collection for address
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freezed_addresses_for_collection()
                    .insert(managed_address!(first_user_address));
            },
        )
        .assert_ok();
    // [test] owner can unfreeze collection for address
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.unfreeze_collection_for_address(&managed_address!(first_user_address));
            },
        )
        .assert_ok();
    // [test] check that the address is removed from the frozen storage
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.freezed_addresses_for_collection()
                    .contains(&managed_address!(first_user_address)),
                false
            );
        })
        .assert_ok();
}

#[test] // Freeze sfts per address function test
fn freeze_sfts_per_address_function_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let treasury_address = &setup.treasury_address;

    // [setup] owner sets the token ID (in real world, this is done via a callback after actual collection is minted)
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
            &rust_biguint!(0u64),
            |sc| {
                sc.treasury_address()
                    .set(&managed_address!(treasury_address));
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_ok();
    // minted 2 tokens
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.minted_per_address(&managed_address!(first_user_address))
                    .get(),
                2u64
            );
        })
        .assert_ok();
    // freeze the second token
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freeze_single_token_for_address(1u64, &managed_address!(first_user_address));
            },
        )
        .assert_ok();
    // check if the storage is updated correctly
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .contains(&1u64),
                true
            );
            assert_eq!(
                sc.freezed_count(&managed_address!(first_user_address))
                    .get(),
                1usize
            );
        })
        .assert_ok();

    // for what reason (found out is not implemented) we get some error if we call two functions that implement esdt_system_sc_proxy (Recipient account is not a smart contract)
    // We imitate the same behaviour as in the contract for freezing the second sft for the same address
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .insert(2u64);
            },
        )
        .assert_ok();
    // setting the freezed count to 2
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freezed_count(&managed_address!(first_user_address))
                    .set(2usize);
            },
        )
        .assert_ok();
    // check if the sfts data is correct
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .len(),
                2usize
            );
            assert_eq!(
                sc.freezed_count(&managed_address!(first_user_address))
                    .get(),
                2usize
            );
        })
        .assert_ok();
}

#[test] // Unfreeze sfts per address function test
fn unfreeze_sfts_per_address_function_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let treasury_address = &setup.treasury_address;

    // [setup] owner sets the token ID (in real world, this is done via a callback after actual collection is minted)
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
            &rust_biguint!(0u64),
            |sc| {
                sc.treasury_address()
                    .set(&managed_address!(treasury_address));
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_ok();
    // minted 2 tokens
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.minted_per_address(&managed_address!(first_user_address))
                    .get(),
                2u64
            );
        })
        .assert_ok();
    // freezing the tokens
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .insert(1u64);
            },
        )
        .assert_ok();
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .insert(2u64);
            },
        )
        .assert_ok();
    // check if the token is added to the freeze_count array
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freezed_count(&managed_address!(first_user_address))
                    .set(2usize);
            },
        )
        .assert_ok();
    // unfreeze the second token
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.unfreeze_single_token_for_address(2u64, &managed_address!(first_user_address)),
        )
        .assert_ok();
    // check if the token is removed from the freezed_sfts_per_address array
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .contains(&1u64),
                true
            );
            assert_eq!(
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .len(),
                1usize
            );
            assert_eq!(
                sc.freezed_count(&managed_address!(first_user_address))
                    .get(),
                1usize
            );
        })
        .assert_ok();
}

#[test] // wipe sfts from address function test
fn wipe_function_test() {
    let mut setup = setup_contract(datanftmint::contract_obj);
    let b_wrapper = &mut setup.blockchain_wrapper;
    let owner_address = &setup.owner_address;
    let first_user_address = &setup.first_user_address;
    let treasury_address = &setup.treasury_address;

    // [setup] owner sets the token ID (in real world, this is done via a callback after actual collection is minted)
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
            &rust_biguint!(0u64),
            |sc| {
                sc.treasury_address()
                    .set(&managed_address!(treasury_address));
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
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
                    managed_buffer!(METADATA_URI),
                    managed_buffer!(DATA_MARSHAL),
                    managed_buffer!(DATA_STREAM),
                    managed_buffer!(DATA_PREVIEW),
                    managed_biguint!(2000u64),
                    managed_biguint!(5),
                    managed_buffer!(USER_NFT_NAME),
                    managed_buffer!(USER_NFT_NAME),
                );
            },
        )
        .assert_ok();
    //   We minted 2 tokens, so we should have 2 in the minted count
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.minted_per_address(&managed_address!(first_user_address))
                    .get(),
                2u64
            );
        })
        .assert_ok();
    // We push the minted tokens to the freezed storage
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .insert(1u64);
            },
        )
        .assert_ok();

    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .insert(2u64);
            },
        )
        .assert_ok();
    // We check if the freezed storage has the correct values
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.freezed_count(&managed_address!(first_user_address))
                    .set(2usize);
            },
        )
        .assert_ok();
    // We check if the freezed storage has the correct values
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .len(),
                2usize
            );
            assert_eq!(
                sc.freezed_count(&managed_address!(first_user_address))
                    .get(),
                2usize
            );
        })
        .assert_ok();
    // We wipe the second token minted
    b_wrapper
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| sc.wipe_single_token_for_address(2u64, &managed_address!(first_user_address)),
        )
        .assert_ok();
    // We check if the freezed storage has the correct values after the wipe
    b_wrapper
        .execute_query(&setup.contract_wrapper, |sc| {
            assert_eq!(
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .contains(&1u64),
                true
            );
            assert_eq!(
                sc.freezed_sfts_per_address(&managed_address!(first_user_address))
                    .len(),
                1usize
            );
            assert_eq!(
                sc.freezed_count(&managed_address!(first_user_address))
                    .get(),
                1usize
            );
        })
        .assert_ok();
}
