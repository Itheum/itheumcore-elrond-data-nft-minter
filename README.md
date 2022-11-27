# Itheum Core Elrond - SFT Mint & Sale Smart Contract

## Abstract

The SFT Mint & Sale smart contract is a tool that can be used in order to pre-order or pre-sell a future collection of NFTs that is yet to be created. Moreover, it can simply be used to mint and then sell a single SFT to multiple wallets.

## Introduction

This contract allows the owner of it to create an SFT collection that contains only one type of SFT in it, available to mint in a pre-set quantity which can be changed at any time. Moreove, it allows the owner to sell that collection of SFTs to multiple wallets under certain conditions (private sale, public sale, given prices, etc).

## Prerequisites

This documentation assumes the user has previous programming experience. Moreover, the user should have a basic understanding of the Elrond blockchain. If you are new to the blockchain, please refer to the [Elrond documentation](https://docs.elrond.com/). In order to develop Elrond smart contract related solutions, one needs to have installed [erdpy](https://docs.elrond.com/sdk-and-tools/erdpy/installing-erdpy/).

Understanding this document is also easier if one knows how [ESDT token transactions](https://docs.elrond.com/developers/esdt-tokens/#transfers-to-a-smart-contract) are structured on the Elrond blockchain and how [NFT tokens](https://docs.elrond.com/tokens/nft-tokens/) work on the Elrond Blockchain.

## Itheum deployed SFT mint & sale contract addresses

| Devnet           | Mainnet          |
| ---------------- | ---------------- |
| Not deployed yet | Not deployed yet |

## Endpoints

### Setup endpoints

The setup workflow for the smart contract is as follows:

- The SC deployment
- Setting up the collection and sale parameters
- Minting the SFT to be sold

#### init

```rust
    #[init]
    fn init(&self);
```

The init function is called when deploying or upgrading the smart contract. It receives no arguments and it the only thing it does for the smart contract is to pause it and set the private sale as being enabled.

#### initializeContract

```rust
    #[payable("EGLD")]
    #[endpoint(initializeContract)]
    fn initialize_contract(
        &self,
        collection_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_royalties: BigUint,
        token_media_cid: ManagedBuffer,
        token_metadata_cid: ManagedBuffer,
        collection_size: BigUint,
        max_per_tx: BigUint,
        max_per_address: BigUint,
    );
```

Endpoint that sets the initializes all the data the contract needs to mint the SFT and sell it. It can only be used once and it can only be called by the owner of the contract. In order for the call to work, the caller must also send 0.05 eGLD when calling the endpoint. This is to cover the cost of creating the SFT collection.
Call structure: "initializeContract" + "@" + collection_name hex encoded + "@" + token_ticker hex encoded + "@" + token_royalties hex encoded + "@" + token_media_cid hex encoded + "@" + token_metadata_cid hex encoded + "@" + collection_size hex encoded + "@" + max_per_tx hex encoded + "@" + max_per_address hex encoded
Example: "initializeContract@544553544E414D45@545449434B@03E8@746573742F766964656F2E6D7034@746573742F646174612E6A736F6E@0A@02@05"

#### createToken

```rust
    #[only_owner]
    #[endpoint(createToken)]
    fn create_token(
        &self,
        token_name: ManagedBuffer
    );
```

Endpoint that creates and mints first edition of the SFT to be sold. Take a token name as argument, which will be the name of the SFT minted.
Call structure: "createToken" + "@" + token_name hex encoded
Example: "createToken@54657374204D65"

### Only owner endpoints

#### setIsPaused

```rust
    #[endpoint(setIsPaused)]
    fn set_is_paused(
        &self,
        is_paused: bool
    );
```

Endpoint that sets the value of the pause variable. This variable is used to determined whether buying SFTs is activated or not.
Call structure: "setIsPaused" + "@" + is_paused hex encoded
Example: "setIsPaused@00"

#### setWhiteListEnabled

```rust
    #[endpoint(setWhiteListEnabled)]
    fn set_white_list_enabled(
        &self,
        is_enabled: bool
    );
```

Endpoint that sets wether the contract is in private sale mode or not. If the contract is in private sale mode, only whitelist addresses can mint.
Call structure: "setWhiteListEnabled" + "@" + is_enabled hex encoded
Example: "setWhiteListEnabled@01"

#### setPrivatePrice

```rust
    #[endpoint(setPrivatePrice)]
    fn set_private_price(
        &self,
        token_id: EgldOrEsdtTokenIdentifier,
        price: BigUint
    );
```

Endpoint that sets the price at which the SFT will be sold to whitelisted users. Gets as argument token for which to set the price and the price to be set for it. Extra care should be taken to enter denominated values. For example, if the price is 0.1 EGLD, the value should be 100000000000000000.
Example: "setPrivatePrice@45474C44@016345785d8a0000"

#### setPublicPrice

```rust
    #[endpoint(setPublicPrice)]
    fn set_public_price(
        &self,
        token_id: EgldOrEsdtTokenIdentifier,
        price: BigUint
    );
```

Endpoint that sets the price at which the SFT will be sold in the public sale. Gets as argument token for which to set the price and the price to be set for it. Extra care should be taken to enter denominated values.
Example: "setPublicPrice@49544845554D2D613631333137@8AC7230489E80000"

#### setMaxPerAddress

```rust
    #[endpoint(setMaxPerAddress)]
    fn set_max_per_address(
        &self,
        max_per_address: BigUint
    );
```

Endpoint that sets the maximum number of SFTs that can be bought by a single address. Gets as argument the number to set. This number represents the number of SFTs that can be bought in private sale and public sale cumulated.
Example: "setMaxPerAddress@05"

#### setMaxPerTx

```rust
    #[endpoint(setMaxPerTx)]
    fn set_max_per_tx(
        &self,
        max_per_tx: BigUint
    );
```

Endpoint that sets the maximum number of SFTs that can be bought in a single transaction. Gets as argument the number to set. This number represents the number of SFTs that can be bought in a single transaction in both private sale and public sale.
Example: "setMaxPerTx@02"

#### setWhiteListSpots

```rust
    #[endpoint(setWhiteListSpots)]
    fn set_whitelist_spots(
        &self,
        whitelist: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>,
    );
```

Endpoint that sets whitelist spots for given addresses. The arguments are given in pairs of addresses and numbers. The address is the address to be whitelisted and the number is the number of SFTs that address can buy in private sale.
Call structure: "setWhitelistSpots" + "@" + Address hex encoded + "@" + number_to_buy hex encoded (can add as many pairs as needed)
Example: "setWhitelistSpots@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101@02"

### Public endpoints

#### mint

```rust
    #[payable("*")]
    #[endpoint(mint)]
    fn mint_token(&self);
```

Endpoint that allows anyone to try minting SFTs.

Call structure for EGLD payment: "mintToken"
Example: "mintToken"

Call structure for ESDT payment: "ESDTTransfer" + "@" + token to send hex encoded + "@" + number of tokens to send hex encoded + "@" + "mintToken" hex encoded
Example: "ESDTTransfer@49544845554D2D613631333137@8AC7230489E80000@bWludFRva2Vu"

### Views

#### getSftsLeftToMint

```rust
    #[view(getSftsLeftToMint)]
    fn sfts_left_to_mint(&self) -> BigUint;
```

View that returns the number of SFTs left to mint.

Call structure: "getSftsLeftToMint"
Example: "getSftsLeftToMint"

#### getUserDataOutFromContract

```rust
    #[view(getUserDataOutFromContract)]
    fn get_user_data_out_from_contract(&self, address: &ManagedAddress) -> UserDataOut<Self::Api>;
```

Main view of the contract. Receives an address as an argument and returns a structure that contains all the data needed by the frontend in order to limit the user from wrongly intteracting with the contract. The structure contains the following fields:

- how_many_can_mint: how many SFTs the address given as an argument can mint
- public_egld_price: the price of the SFT in EGLD in public sale
- private_egld_price: the price of the SFT in EGLD in private sale
- private_prices: a vector of ESDT tokens and prices for each of the tokens in private sale
- public_prices: a vector of ESDT tokens and prices for each of the tokens in public sale
- collection_size: the total number of SFTs to be sold
- minted_for_address: the total number of SFTs minted for the address given as an argument
- minted_in_total: the total number of SFTs minted
- can_mint: a boolean that determines whether anyone can mint at the moment of call or not
- max_per_tx: how many per tx can be minted

## Development

This smart contract aims to offer the Elrond community an audited SFT sale smart contract that is easy to use, well documented and secure.

### Architecture

The Smart Contract is structured in 6 files:

- events: This file has all the defined events of the smart contract. They are emitted whenever something relevant happens in the smart contract. Their role is to make debugging and logging easier and to allow data collecting based on the smart contract.
- storage: This file has all the storage/memory declaration of the smart contract. This is the main file that allows the smart contract to save data in the blockchain.
- views: This file contains all the read-only endpoints of the smart contract. These endpoints are used to retrieve relevant data from the smart contract.
- requirements: This file contains requirements for the endpoints of the smart contract. In order to avoid code duplication, encourage a healthy project structure and increase code readability we have decided to separate most of the requirements that would otherwise have been duplicated from the endpoints and put them here.
- nft_mint_utils: This file contains helper functions for minting SFTs.
- lib: This is the main file of the smart contract, where all the logic of the smart contract is implemented. This connects all the other files (modules) and uses them to implement what is the contract itself.

### How to test

The tests are located in the tests folder, in the rust_tests file. In order to run the tests one can use the command:

```shell
    cargo test --package datanftmint --test rust_tests --  --nocapture
```

Another way of running the tests is by using the rust-analyzer extension in Visual Studio Code, which is also very helpful for Elrond Smart Contract development. If one has the extension installed, they can go open and go to the top of the rust_tests file and click the Run Tests button.

Note: In order to run the tests, one has to use the rust nightly version. One can switch to the nightly version by using:

```shell
    rustup default nightly
```

### How to deploy

In order to deploy the smart contract on devnet one can use the interaction snippets present in the devnet.snippets file (which is located in the interactions folder). Before using the snippets, make sure to add your pem file in the root of the project under the name "wallet.pem" (or change the name to whichever one you wish to use in the interaction snippets). If you need info about how to derive a pem file you can find them [here](https://docs.elrond.com/sdk-and-tools/erdpy/deriving-the-wallet-pem-file/). To run the functions from the interaction file, one can use:

```shell
    source interaction/devnet.snippets.sh
```

After using that, to deploy one can simply use:

```shell
    deploy
```

### How to interact

After deployment, one can interact with the smart contract and test its functionality. To do so, one can use the interaction snippets already presented above. More explanations can be found about the snippets inside the devnet.snippets file.

## Contributing

Feel free the contact the development team if you wish to contribute or if you have any questions. If you find any issues, please report them in the Issues sections of the repository. You can also create your own pull requests which will be analyzed by the team.
