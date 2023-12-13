PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"

WALLET="./wallet.pem"
USER="../wallet2.pem"

ADDRESS=$(mxpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(mxpy data load --key=deployTransaction-devnet)

TOKEN="ITHEUM-fce905"
TOKEN_HEX="0x$(echo -n ${TOKEN} | xxd -p -u | tr -d '\n')"

# to deploy from last reprodubible build, we need to change or vice versa
# --bytecode output/datanftmint.wasm \
# to 
# --bytecode output-docker/datanftmint/datanftmint.wasm \
deploy(){
    mxpy --verbose contract deploy \
    --bytecode output-docker/datanftmint/datanftmint.wasm \
    --outfile deployOutput \
    --metadata-not-readable \
    --metadata-payable-by-sc \
    --pem ${WALLET} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --gas-limit 150000000 \
    --send \
    --recall-nonce \
    --outfile="./interaction/deploy-devnet.interaction.json" || return

    TRANSACTION=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}
    mxpy data store --key=deployTransaction-devnet --value=${TRANSACTION}
}

# any change to code or property requires a full upgrade 
# always check if you are deploy via a reprodubible build and that the code hash is the same before and after upgrade (that is if you are only changing props and not code.. for code, the RB will be different)
# if only changing props, you can't just "append" new props. you have to add the old ones again and then add a new prop you need. i.e. it's not append, it's a whole reset
# for upgrade, --outfile deployOutput is not needed
# in below code example we added --metadata-payable to add PAYABLE to the prop of the SC and removed --metadata-not-readable to make it READABLE
upgrade(){
    mxpy --verbose contract upgrade ${ADDRESS} \
    --bytecode output-docker/datanftmint/datanftmint.wasm \
    --metadata-payable-by-sc \
    --metadata-payable \
    --pem ${WALLET} \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --gas-limit 150000000 \
    --recall-nonce \
    --send || return
}

# if you interact without calling deploy(), then you need to 1st run this to restore the vars from data
restoreDeployData() {
  TRANSACTION=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
  ADDRESS=$(mxpy data parse --file="./interaction/deploy-devnet.interaction.json" --expression="data['contractAddress']")

  # after we upgraded to mxpy 8.1.2, mxpy data parse seems to load the ADDRESS correctly but it breaks when used below with a weird "Bad address" error
  # so, we just hardcode the ADDRESS here. Just make sure you use the "data['contractAddress'] from the latest deploy-devnet.interaction.json file
  ADDRESS="erd1qqqqqqqqqqqqqpgq7thwlde9hvc5ty7lx2j3l9tvy3wgkwu7fsxsvz9rat"
}

initializeContract(){
    # $1 = collection name
    # $2 = collection ticker
    # #3 = anti spam tax
    # $4 = mint time limit
    # $5 = treasury address

    collection_name="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"
    collection_ticker="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"
    token_identifier=${TOKEN_HEX}
    anti_spam_tax=${3}
    mint_time_limit=${4}
    treasury_address="0x$(mxpy wallet bech32 --decode ${5})"
    

    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=300000000 \
    --value=50000000000000000 \
    --function "initializeContract" \
    --arguments $collection_name $collection_ticker $token_identifier $anti_spam_tax $mint_time_limit $treasury_address \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}


setLocalRoles()
{
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=90000000 \
    --function "setLocalRoles" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

pause(){
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=90000000 \
    --function "pause" \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

unpause(){
    mxpy --verbose contract call ${ADDRESS} \
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

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
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

    address="0x$(mxpy wallet bech32 --decode ${2})"

    mxpy --verbose contract call ${ADDRESS} \
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

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
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

    address="0x$(mxpy wallet bech32 --decode ${2})"

    mxpy --verbose contract call ${ADDRESS} \
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

    address="0x$(mxpy wallet bech32 --decode ${2})"

    mxpy --verbose contract call ${ADDRESS} \
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

    user_address="$(mxpy wallet pem-address $USER)"
    method="0x$(echo -n 'burn' | xxd -p -u | tr -d '\n')"
    sft_token="0x$(echo -n ${1} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call $user_address \
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
    mxpy --verbose contract call ${ADDRESS} \
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
    mxpy --verbose contract call ${ADDRESS} \
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

    mxpy --verbose contract call ${ADDRESS} \
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
    mxpy --verbose contract call ${ADDRESS} \
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
    mxpy --verbose contract call ${ADDRESS} \
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

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
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

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
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

    mxpy --verbose contract call ${ADDRESS} \
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

    mxpy --verbose contract call ${ADDRESS} \
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

    mxpy --verbose contract call ${ADDRESS} \
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

    address="0x$(mxpy wallet bech32 --decode ${1})"

    mxpy --verbose contract call ${ADDRESS} \
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
    # $4 = metadata
    # $5 = data marshal
    # $6 = data stream
    # $7 = data preview
    # $8 = royalties
    # $9 = supply
    # $10 = title
    # $11 = description

    method="0x$(echo -n 'mint' | xxd -p -u | tr -d '\n')"
    name="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"
    media="0x$(echo -n ${3} | xxd -p -u | tr -d '\n')"
    metadata="0x$(echo -n ${4} | xxd -p -u | tr -d '\n')"
    data_marshal="0x$(echo -n ${5} | xxd -p -u | tr -d '\n')"
    data_stream="0x$(echo -n ${6} | xxd -p -u | tr -d '\n')"
    data_preview="0x$(echo -n ${7} | xxd -p -u | tr -d '\n')"
    title="0x$(echo -n ${10} | xxd -p -u | tr -d '\n')"
    description="0x$(echo -n ${11} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call $ADDRESS \
    --recall-nonce \
    --pem=${USER} \
    --gas-limit=100000000 \
    --function "ESDTTransfer" \
    --arguments ${TOKEN_HEX} $1 $method $name $media $metadata $data_marshal $data_stream $data_preview $7 $8 $title $description \
    --proxy ${PROXY} \
    --chain ${CHAIN_ID} \
    --send || return
}

mintTokenUsingEgld(){
    # $1 = amount of egld to send
    # $2 = name
    # $3 = media
    # $4 = metadata
    # $5 = data marshal
    # $6 = data stream
    # $7 = data preview
    # $8 = royalties
    # $9 = supply
    # $10 = title
    # $11 = description

    name="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"
    media="0x$(echo -n ${3} | xxd -p -u | tr -d '\n')"
    metadata="0x$(echo -n ${4} | xxd -p -u | tr -d '\n')"
    data_marshal="0x$(echo -n ${5} | xxd -p -u | tr -d '\n')"
    data_stream="0x$(echo -n ${6} | xxd -p -u | tr -d '\n')"
    data_preview="0x$(echo -n ${7} | xxd -p -u | tr -d '\n')"
    title="0x$(echo -n ${10} | xxd -p -u | tr -d '\n')"
    description="0x$(echo -n ${11} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract call ${ADDRESS} \
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

    address="0x$(mxpy wallet bech32 --decode ${1})"
    token_identifier="0x$(echo -n ${2} | xxd -p -u | tr -d '\n')"

    mxpy --verbose contract query ${ADDRESS} \
    --proxy ${PROXY} \
    --function 'getUserDataOut' \
    --arguments $address $token_identifier     
}