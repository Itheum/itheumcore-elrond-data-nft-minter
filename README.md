# Itheum Core Elrond - SFT Mint & Sale Smart Contract

## Abstract

The Data NFT Minting contract is a tool that can be used in order to tokenize and sell the right to use different forms of data in the form of an SFT with different supplies.

## Introduction

This contract allows the owner of it to create an SFT collection towards which anyone can contribute by creating Data NFTs. The creators can even choose their own royalties or supply.

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
- Setting up the collection
- Setting up the parameters used in creating Data NFT-FT

#### init

```rust
    #[init]
    fn init(&self);
```

The init function is called when deploying or upgrading the smart contract. It receives no arguments and what it does for the data nft minter smart contract is to pause it, enable whitelist and setting the default values for minimum royalties, maximum royalties and maximum supply

### Only owner endpoints

#### initializeContract

```rust
#[payable("EGLD")]
    #[endpoint(initializeContract)]
    fn initialize_contract(
        &self,
        collection_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_identifier: &EgldOrEsdtTokenIdentifier,
        anti_spam_tax: BigUint,
        mint_time_limit: u64,
    );
```

Endpoint that initializes all the data needed for the smart contract to issue the token, anti spam tax and mint time limit used for minting the Data NFT-FT. It can only be used once and it can only be called by the owner of the smart contract. In order for the call to work, the caller must also send 0.05 eGLD when calling the endpoint. This is to cover the cost of creating the Data NFT-FT collection. Call structure: "initializeContract" + "@" + collection_name hex encoded + "@" + token_ticker hex encoded + "@" + token_identifier hex encoded + "@" + anti_spam_tax + "@" + mint_time_limit hex encoded. Example:
"initializeContrct@436f6c6c656374696f6e4e616d65@4e46544654@2049544845554d2d613631333137@015af1d78b58c40000@0384"

#### setWhiteListEnabled

```rust
    #[endpoint(setWhiteListEnabled)]
    fn set_white_list_enabled(
        &self,
        is_enabled: bool
    );
```

Endpoint that sets wether the contract lets only whitelisted addresses to mint or not. If the contract has whitelist disabled, all addresses can mint.
Call structure: "setWhiteListEnabled" + "@" + is_enabled hex encoded
Example: "setWhiteListEnabled@01"

#### setMintTimeLimit

```rust
    #[endpoint(setMintTimeLimit)]
    fn set_mint_time_limit(
        &self,
        mint_time_limit: u64
    );
```

Endpoint that sets a time limit for the mint. An address can mint only once in the time limit.
Call structure: "setMintTimeLimit" +"@" + mint_time_limit hex encoded
Example: "setWhitelistSpots@0384"

#### setRoyaltiesLimits

```rust
    #[endpoint(setRoyaltiesLimits)]
    fn set_royalties_limits(
        &self,
        min_royalties: BigUint,
        max_royalties: BigUint
    );
```

Endpoint that sets the minimum and maximum royalties that can be set by the public when minting a Data NFT-FT.
Call structure: "setRoyaltiesLimits" + "@" + min_royalties hex encoded + "@" + max_royalties hex encoded
Example: "setRoyaltiesLimits@00@01f40"

#### setAdministrator

```rust
   #[endpoint(setAdministrator)]
    fn set_administrator(
    &self,
    administrator: ManagedAddress
    );
```

Endpoint that sets the administrator of the contract. The administrator has different privileges that will be presented in the Owner and administrator endpoints section.
Call structure: "setAdministrator" + "@" + administrator hex encoded
Example: "setAdministrator@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

### Owner and administrator endpoints

#### setIsPaused

```rust
    #[endpoint(setIsPaused)]
    fn set_is_paused(
        &self,
        is_paused: bool
    );
```

Endpoint that sets the value of the pause variable. This variable is used to determined whether minting Data NFT-FT is activated or not.
Call structure: "setIsPaused" + "@" + is_paused hex encoded
Example: "setIsPaused@00"

#### setAntiSpamTax

```rust
    #[endpoint(setAntiSpamTax)]
    fn set_anti_spam_tax(
        &self,
        token_id: &EgldOrEsdtTokenIdentifier,
        tax: BigUint
    );
```

Endpoint that sets the token identifier and the tax value that will be used for minting Data NFT-FT, it will act also as an anti spam tax. The endpoint can take any token identifier, but be aware that diferent tokens have diferent decimals.
Call structure: "setAntiSpamTax" + "@" + token_id hex encoded + "@" + tax hex encoded
Example: "setAntiSpamTax@45474c44@015af1d78b58c40000"

#### setWhiteListSpots

```rust
    #[endpoint(setWhiteListSpots)]
    fn set_whitelist_spots(
        &self,
        whitelist: MultiValueEncoded<ManagedAddress>
    );
```

Endpoint that sets whitelist spots for given addresses. The argument can take a list of addreses.
Call structure: "setWhiteListSpots" + "@" + address1 hex encoded
(can add as many addresses as needed)
Example: "setWhitelistSpots@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101"

#### removeWhiteListSpots

```rust
    #[endpoint(removeWhiteListSpots)]
    fn remove_whitelist_spots(
        &self,
        whitelist: MultiValueEncoded<ManagedAddress>
    );
```

Endpoint that removes whitelist spots for given addresses. The argument can take a list of addreses.
Call structure: "removeWhiteListSpots" + "@" + address1 hex encoded
(can add as many addresses as needed)
Example: "removeWhiteListSpots@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101"

#### setMaxSupply

    ```rust
        #[endpoint(setMaxSupply)]
        fn set_max_supply(
            &self,
            max_supply: BigUint
        );
    ```

Endpoint that sets the value of the max supply variable. This variable is used to determined the maximum supply of the Data NFT-FTs an user can mint.
Call structure: "setMaxSupply" + "@" + max_supply hex encoded
Example: "setMaxSupply@05"

### Public endpoints

#### mint

```rust
 #[payable("*")]
 #[endpoint(mint)]
 fn mint_token(
        &self,
        name: ManagedBuffer,
        media: ManagedBuffer,
        data_marshal: ManagedBuffer,
        data_stream: ManagedBuffer,
        data_preview: ManagedBuffer,
        royalties: BigUint,
        supply: BigUint,
        title: ManagedBuffer,
        description: ManagedBuffer
    );
```

Endpoint that allows anyone to mint Data NFT-FT. The endpoint takes as arguments the name (NFT-FT name), media (NFT-FT media url), data_marshal (marshal service url), data_stream (data stream url), data_preview (data preview url), royalties (royalties value between min_royalties and max_royalties), supply (supply value between 1 and max_supply), title (longer title describing NFT-FT) and description (NFT-FT description).
Call structure for EGLD payment: "mint" + "@" + name hex encoded + "@" + media hex encoded + "@" + data_marshal hex encoded + "@" + data_stream hex encoded + "@" + data_preview hex encoded + "@" + royalties hex encoded + "@" + supply hex encoded + "@" + title hex encoded + "@" + description hex encoded
Example: "mint@4d792044617461204e4654@68747470733a2f2f69746865756d2e696f2f746573742f746573742f746573742f746573742f6d65646961@68747470733a2f2f69746865756d2e696f2f746573742f746573742f746573742f746573742f6d65646961@68747470733a2f2f69746865756d2e696f2f746573742f746573742f746573742f746573742f6d65646961@68747470733a2f2f69746865756d2e696f2f746573742f746573742f746573742f746573742f6d65646961@3c@05@4e46542d4654205469746c65@4e46542d4654204465736372697074696f6e"

Call structure for ESDT payment: "ESDTTransfer" + "@" + token to send hex encoded + "@" + number of tokens to send hex encoded + "@" + "mint" hex encoded + "@" + name hex encoded + "@" + media hex encoded + "@" + data_marshal hex encoded + "@" + data_stream hex encoded + "@" + data_preview hex encoded + "@" + royalties hex encoded + "@" + supply hex encoded + "@" + title hex encoded + "@" + description hex encoded

Example: "ESDTTransfer@49544845554d2d613631333137@015af1d78b58c40000@6d696e74@53616d706c65546f6b656e4e616d65@68747470733a2f2f697066732e696f2f697066732f62616679726569647835367968706f7371626e6b616432767970363734713574776a637666356a67767473626579686f6366346b6a657835776f34@68747470733a2f2f69746865756d6170692e636f6d2f646465782f646174616d61727368616c2f76312f73657276696365732f67656e6572617465@68747470733a2f2f69746865756d2d7265736f75726365732e73332e61702d736f757468656173742d322e616d617a6f6e6177732e636f6d2f6a736f6e2f54484f525f45636f47505f52616365312e637376@68747470733a2f2f69746865756d2d7265736f75726365732e73332e61702d736f757468656173742d322e616d617a6f6e6177732e636f6d2f6a736f6e2f54484f525f45636f47505f52616365312e637376@@01@53616d706c65205469746c65@53616d706c65204465736372697074696f6e"

#### burn

```rust
    #[payable("*")]
    #[endpoint(burn)]
    fn burn_token(&self);
```

Endpoint that allows anyone to burn sent amount of Data NFT-FTs.
Call structure: "ESDTTransfer" + "@" + NFT-FT token identifier hex encoded + "@" + token nonce hex encoded + "@" + number of tokens to burn hex encoded + "@" + contract address hex encoded + "@" +"burn" hex encoded
Example: "ESDTNFTTransfer@4e465446542d373736336637@01@1e@00000000000000000500c72532eb1c8f5e32034b46b5041babade020fdefd5fd@6275726e"

### Views

#### getUSerDataOut

```rust
    #[view(getUserDataOut)]
    fn get_user_data_out(&self,
    address: ManagedAddress,
    tax_token: &EsdtTokenIdentifier
    );
```

Main view of the contract. Receives an address and a token identifier as arguments and returns a structure that contains all the data needed by the frontend in order to limit the user from wrongly intteracting with the contract. The structure contains the following fields:

- anti_spam_tax_value: the value of the anti spam tax for the given token identifier
- is_paused: a boolean that indicates if the contract is paused or not
- max_royalties: the maximum royalties value that can be set by the user
- min_royalties: the minimum royalties value that can be set by the user
- max_supply: the maximum supply value that can be set by the user
- mint_time_limit: the time limit for minting a data NFT-FT.
- last_mint_time: the last time a data NFT-FT was minted by the given address
- whitelist_enabled: a boolean that indicates if the whitelist is enabled or not
  -is_whitelisted: a boolean that indicates if the given address is whitelisted or not

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
