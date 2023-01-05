PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="./wallet.pem"
USER="../wallet2.pem"

ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)

TOKEN="ITHEUM-a61317"
TOKEN_HEX="0x$(echo -n ${TOKEN} | xxd -p -u | tr -d '\n')"

deploy(){
    erdpy --verbose contract deploy \
    --bytecode output/datanftmint.wasm \
    --outfile deployOutput \
    --metadata-not-readable \
    --pem ${WALLET} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --gas-limit 150000000 \
    --send \
    --recall-nonce \
    --outfile="./interaction/deploy-devnet.interaction.json" || return

    TRANSACTION=$(erdpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}
}

initializeContract(){
    # $1 = collection name
    # $2 = collection ticker
    # #3 = anti spam tax
    # $4 = mint time limit

    collection_name="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"
    collection_ticker="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"
    token_identifier=${TOKEN_HEX}
    anti_spam_tax=${3}
    mint_time_limit=${4}
    

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=300000000 \
    --value=50000000000000000 \
    --function "initializeContract" \
    --arguments $collection_name $collection_ticker $token_identifier $anti_spam_tax $mint_time_limit \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}


pause(){
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=90000000 \
    --function "pause" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}


unpause(){
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=90000000 \
    --function "unpause" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

freeze(){
    # $1 = address to freeze

    address="0x$(erdpy wallet bech32 --decode ${1})"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=90000000 \
    --function "freeze" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return 
}



freezeSingleNFT(){
    # $1 = token nonce
    # $2 = address to freeze

    address="0x$(erdpy wallet bech32 --decode ${2})"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=90000000 \
    --function "freezeSingleNFT" \
    --arguments $1 $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return 
}

unfreeze(){
    # $1 = address to freeze

    address="0x$(erdpy wallet bech32 --decode ${1})"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=90000000 \
    --function "unfreeze" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return 
}


unFreezeSingleNFT(){
    # $1 = token nonce
    # $2 = address to unfreeze

    address="0x$(erdpy wallet bech32 --decode ${2})"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=90000000 \
    --function "unFreezeSingleNFT" \
    --arguments $1 $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return 
}


wipeSingleNFT(){
    # $1 = token nonce
    # $2 = address to wipe tokens from

    address="0x$(erdpy wallet bech32 --decode ${2})"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=90000000 \
    --function "wipeSingleNFT" \
    --arguments $1 $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return 
}


burn(){
    #   $1 = NFT/SFT Token Identifier,
    #   $2 = NFT/SFT Token Nonce,
    #   $3 = NFT/SFT Token Amount,

    user_address="$(erdpy wallet pem-address $USER)"
    method="0x$(echo -n 'burn' | xxd -p -u | tr -d '\n')"
    sft_token="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call $user_address \
        --recall-nonce \
        --pem=${USER} \
        --gas-limit=6000000 \
        --function="ESDTNFTTransfer" \
        --arguments $sft_token $2 $3 ${ADDRESS} $method  \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --send || return
}

pauseContract(){
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setIsPaused" \
    --arguments 1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

unPauseContract(){
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setIsPaused" \
    --arguments 0 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setAntiSpamTax(){
    # $1 = token identifier
    # $2 = anti spam tax value

    token_identifier="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setAntiSpamTax" \
    --arguments $token_identifier ${2} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

enableWhiteList(){
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setWhiteListEnabled" \
    --arguments 1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

disableWhiteList(){
    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setWhiteListEnabled" \
    --arguments 0 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setWhiteListSpots(){
    # $1 = address

    address="0x$(erdpy wallet bech32 --decode ${1})"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setWhiteListSpots" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}


removeWhiteListSpots(){
    # $1 = address

    address="0x$(erdpy wallet bech32 --decode ${1})"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "removeWhiteListSpots" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}


setMintTimeLimit(){
    # $1 = mint time limit value u64

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setMintTimeLimit" \
    --arguments ${1} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setRoyaltiesLimits(){
    # $1 = min royalties value
    # $2 = max royalties value

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setRoyaltiesLimits" \
    --arguments ${1} ${2} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setMaxSupply(){
    # $1 = max supply value

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setMaxSupply" \
    --arguments ${1} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setAdministrator(){
    # $1 = address

    address="0x$(erdpy wallet bech32 --decode ${1})"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setAdministrator" \
    --arguments $address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

mintTokenUsingEsdt(){
    # $1 = amount of esdt to send
    # $2 = name
    # $3 = media
    # $4 = data marshal
    # $5 = data stream
    # $6 = data preview
    # $7 = royalties
    # $8 = supply
    # $9 = title
    # $10 = description

    method="0x$(echo -n 'mint' | xxd -p -u | tr -d '\n')"
    name="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"
    media="0x$(echo -n ${3} | xxd -p -u | tr -d '\n')"
    data_marshal="0x$(echo -n ${4} | xxd -p -u | tr -d '\n')"
    data_stream="0x$(echo -n ${5} | xxd -p -u | tr -d '\n')"
    data_preview="0x$(echo -n ${6} | xxd -p -u | tr -d '\n')"
    title="0x$(echo -n ${9} | xxd -p -u | tr -d '\n')"
    description="0x$(echo -n ${10} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call $ADDRESS \
    --recall-nonce \
    --pem=${USER} \
    --gas-limit=100000000 \
    --function "ESDTTransfer" \
    --arguments ${TOKEN_HEX} $1 $method $name $media $data_marshal $data_stream $data_preview $7 $8 $title $description \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

mintTokenUsingEgld(){
    # $1 = amount of egld to send
    # $2 = name
    # $3 = media
    # $4 = data marshal
    # $5 = data stream
    # $6 = data preview
    # $7 = royalties
    # $8 = supply
    # $9 = title
    # $10 = description

    name="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"
    media="0x$(echo -n ${3} | xxd -p -u | tr -d '\n')"
    data_marshal="0x$(echo -n ${4} | xxd -p -u | tr -d '\n')"
    data_stream="0x$(echo -n ${5} | xxd -p -u | tr -d '\n')"
    data_preview="0x$(echo -n ${6} | xxd -p -u | tr -d '\n')"
    title="0x$(echo -n ${9} | xxd -p -u | tr -d '\n')"
    description="0x$(echo -n ${10} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --value=${1} \
    --function "mint" \
    --arguments $name $media $data_marshal $data_stream $data_preview $7 $8 $title $description \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

getUserDataOut(){
    # $1 = address
    # $2 = token identifier

    address="0x$(erdpy wallet bech32 --decode ${1})"
    token_identifier="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract query ${ADDRESS} \
    --proxy ${PROXY} \
    --function 'getUserDataOut' \
    --arguments $address $token_identifier     
}