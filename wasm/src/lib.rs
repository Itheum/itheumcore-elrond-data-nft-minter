// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
<<<<<<< HEAD
// Endpoints:                           48
// Async Callback:                       1
// Total number of exported functions:  50
=======
// Endpoints:                           46
// Async Callback:                       1
// Total number of exported functions:  48
>>>>>>> develop

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    datanftmint
    (
        init => init
        upgrade => upgrade
        initializeContract => initialize_contract
        setLocalRoles => set_local_roles
        mint => mint_token
        burn => burn_token
        setTreasuryAddress => set_treasury_address
        setIsPaused => set_is_paused
        setWhiteListEnabled => set_whitelist_enabled
        setAntiSpamTax => set_anti_spam_tax
        setWhiteListSpots => set_whitelist_spots
        removeWhiteListSpots => remove_whitelist_spots
        setMintTimeLimit => set_mint_time_limit
        setRoyaltiesLimits => set_royalties_limits
        setMaxSupply => set_max_supply
        setAdministrator => set_administrator
        setBondContractAddress => set_bond_contract_address
        setWithdrawalAddress => set_withdrawal_address
        withdraw => withdraw
        getTokenId => token_id
        getTreasuryAddress => treasury_address
        getWithdrawalAddress => withdrawal_address
        getMintedTokens => minted_tokens
        getAntiSpamTax => anti_spam_tax
        getIsPaused => is_paused
        getMaxRoyalties => max_royalties
        getMinRoyalties => min_royalties
        getMaxSupply => max_supply
        getMintedPerAddress => minted_per_address
        mintTimeLimit => mint_time_limit
        lastMintTime => last_mint_time
        getWhiteList => whitelist
        getCollectionFrozenList => frozen_addresses_for_collection
        getSftsFrozenForAddress => frozen_sfts_per_address
        getFrozenCount => frozen_count
        isWhiteListEnabled => whitelist_enabled
        rolesAreSet => roles_are_set
        getAdministrator => administrator
        getBondContractAddress => bond_contract_address
        getUserDataOut => get_user_data_out
        pause => pause_collection
        unpause => unpause_collection
        freeze => freeze_collection_for_address
        unfreeze => unfreeze_collection_for_address
        freezeSingleNFT => freeze_single_token_for_address
        unFreezeSingleNFT => unfreeze_single_token_for_address
        wipeSingleNFT => wipe_single_token_for_address
        get_bond_amount_for_lock_period => get_bond_amount_for_lock_period
        send_bond => send_bond
    )
}

multiversx_sc_wasm_adapter::async_callback! { datanftmint }
