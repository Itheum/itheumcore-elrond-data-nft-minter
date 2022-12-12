// Code generated by the elrond-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           35
// Async Callback:                       1
// Total number of exported functions:  37

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    datanftmint
    (
        initializeContract
        mint
        burn
        setIsPaused
        setAntiSpamTax
        setWhiteListEnabled
        setWhiteListSpots
        removeWhiteListSpots
        setMintTimeLimit
        setRoyaltiesLimits
        setMaxSupply
        setAdministrator
        getTokenId
        getMintedTokens
        getAntiSpamTax
        getIsPaused
        getMaxRoyalties
        getMinRoyalties
        getMaxSupply
        getMintedPerAddress
        getContractInitialized
        mintTimeLimit
        lastMintTime
        getWhiteList
        getBlackList
        isWhiteListEnabled
        getAdministrator
        getUserDataOut
        pause
        unpause
        freeze
        unfreeze
        freezeSingleNFT
        unFreezeSingleNFT
        wipeSingleNFT
        callBack
    )
}
