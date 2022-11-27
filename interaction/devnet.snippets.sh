PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="./wallet.pem"

ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)

deploy(){
    erdpy --verbose contract deploy \
    --bytecode output/datanftmint.wasm \
    --outfile deployOutput \
    --metadata-not-readable \
    --pem wallet.pem \
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
    # $3 = token royalties
    # $4 = media cid
    # $5 = metadata cid
    # $6 = collection size
    # $7 = max per transaction
    # $8 = max per address

    collection_name="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"
    collection_ticker="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"
    royalties=${3}
    media_cid="0x$(echo -n ${4} | xxd -p -u | tr -d '\n')"
    metadata_cid="0x$(echo -n ${5} | xxd -p -u | tr -d '\n')"
    collection_size=${6}
    max_per_transaction=${7}
    max_per_address=${8}

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=80000000 \
    --value=50000000000000000 \
    --function "initializeContract" \
    --arguments $collection_name $collection_ticker $royalties $media_cid $metadata_cid $collection_size $max_per_transaction $max_per_address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

createToken(){
    # $1 = SFT name

    sft_name="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "createToken" \
    --arguments $sft_name \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

pause(){
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

unpause(){
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

enableWhitelist(){
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

disableWhitelist(){
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

setPrivatePrice(){
    # $1 = token of the price
    # $2 = price

    token_of_price="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setPrivatePrice" \
    --arguments $token_of_price $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setPublicPrice(){
    # $1 = token of the price
    # $2 = price

    token_of_price="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setPublicPrice" \
    --arguments $token_of_price $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setMaxPerAddress(){
    # $1 = amount

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setMaxPerAddress" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setMaxPerTx(){
    # $1 = amount

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setMaxPerTx" \
    --arguments $1 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

setWhiteListSpot(){
    # $1 = address
    # $2 = amount

    address="0x$(erdpy wallet bech32 --decode ${1})"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=6000000 \
    --function "setWhitelistSpots" \
    --arguments $address $2 \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

mintTokenUsingEgld(){
    # $1 = amount of egld to send

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --value=$1 \
    --function "mint" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

mintTokenUsingEsdt(){
    # $1 = esdt to send
    # $2 = amount of esdt to send

    method="0x$(echo -n 'mint' | xxd -p -u | tr -d '\n')"
    token="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=10000000 \
    --function "ESDTTransfer" \
    --arguments $token $2 $method \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}