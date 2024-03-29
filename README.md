# Itheum Core Elrond - NFT-FT Minter Smart Contract

## Abstract

The Data NFT Minting contract is a tool that can be used in order to tokenize and sell the right to use different forms of data in the form of an SFT with different supplies.

## Introduction

This contract allows the owner of it to create an SFT collection towards which anyone can contribute by creating Data NFTs. The creators can even choose their own royalties or supply.

## Prerequisites

This documentation assumes the user has previous programming experience. Moreover, the user should have a basic understanding of the Elrond blockchain. If you are new to the blockchain, please refer to the [Elrond documentation](https://docs.elrond.com/). In order to develop Elrond smart contract related solutions, one needs to have installed [mxpy](https://docs.multiversx.com/sdk-and-tools/sdk-py/installing-mxpy).

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

The init function is called when deploying or upgrading the smart contract. It receives no arguments and does the following: pauses the contract, enable whitelist and sets the default values for minimum royalties, maximum royalties and maximum supply.

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
        treasury_address: ManagedAddress
    );
```

Endpoint that initializes all the data needed for the smart contract to issue the token. The anti spam tax and mint time limit variables are used for regulating the minting of Data NFT-FTs. It can only be used once and it can only be called by the owner of the smart contract. In order for the call to work, the caller must also send 0.05 eGLD when calling the endpoint. This is to cover the cost of creating the Data NFT-FT collection.

Call structure: "initializeContract" + "@" + collection_name hex encoded + "@" + token_ticker hex encoded + "@" + token_identifier hex encoded + "@" + anti_spam_tax + "@" + mint_time_limit hex encoded + "@" + treasury_address hex encoded.

Example: "initializeContract@436f6c6c656374696f6e4e616d65@4e46544654@2049544845554d2d613631333137@015af1d78b58c40000@0384@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

#### setTreasuryAddress

```rust
 fn set_treasury_address(&self,
  address: ManagedAddress
 );
```

Endpoint that sets the treasury address. The treasury address is the address that will receive the anti spam tax when minting a Data NFT-FT.

Call structure: "setTreasuryAddress" + "@" + address hex encoded.

Example: "setTreasuryAddress@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

#### pause

```rust
    #[endpoint(pause)]
    fn pause_collection(&self);
```

Endpoint that pauses the entire collection. No transactions, minting, burning can be made on the collection while it is paused.

Call structure: "pause"

Example: "pause"

#### unpause

```rust
    #[endpoint(unpause)]
    fn unpause_collection(&self);
```

Endpoint that unpauses the entire collection. Normal transactions, minting, burning can be made on the collection while it is unpaused.

Call structure: "unpause"

Example: "unpause"

#### freeze

```rust
    #[endpoint(freeze)]
    fn freeze_collection_for_address(
        &self,
        address: &ManagedAddress
    );
```

Endpoint that freezes the entire collection for a specific address. It will utilize the token issued previously and owned by the smart contract. The freezed address will not be able to interact with the collection.

Call structure: "freeze" + "@" + address hex encoded.

Example: "freeze@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

#### unfreeze

```rust
    #[endpoint(unfreeze)]
    fn unfreeze_collection_for_address(
        &self,
        address: &ManagedAddress
    );
```

Endpoint that unfreezes the entire collection for a specific address. It will utilize the token issued previously and owned by the smart contract. The unfreezed address will be able to interact with the collection.

Call structure: "unfreeze" + "@" + address hex encoded.

Example: "unfreeze@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

#### setMintTimeLimit

```rust
    #[endpoint(setMintTimeLimit)]
    fn set_mint_time_limit(
        &self,
        mint_time_limit: u64
    );
```

Endpoint that sets a time limit for the mint. An address can mint only once in the time limit.

Call structure: "setMintTimeLimit" +"@" + mint_time_limit hex encoded.

Example: "setMintTimeLimit@0384"

#### setAdministrator

```rust
    #[endpoint(setAdministrator)]
    fn set_administrator(
        &self,
        administrator: ManagedAddress
    );
```

Endpoint that sets the administrator of the contract. The administrator has some privileges that will be presented in the [Owner and administrator endpoints](#owner-and-administrator-endpoints) section.

Call structure: "setAdministrator" + "@" + administrator hex encoded.

Example: "setAdministrator@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

#### setWithdrawalAddress

```rust
    #[endpoint(setWithdrawalAddress)]
    fn set_withdrawal_address(
        &self,
        withdrawal_address: ManagedAddress
    );
```

Endpoint that sets the withdrawal address. The withdrawal address is the address that will receive the funds from the smart contract.

Call structure: "setWithdrawalAddress" + "@" + withdrawal_address hex encoded.

Example: "setWithdrawalAddress@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

### Owner and administrator endpoints

#### freezeSingleNFT

```rust
    #[endpoint(freezeSingleNFT)]
    fn freeze_single_token_for_address(
        &self,
        nonce: u64,
        address: &ManagedAddress
    );
```

Endpoint that freezes specific data NFT-FT for a specific address. It will utilize the token issued previously and owned by the smart contract and the nonce of the data NFT-FT. The freezed address will not be able to interact with the specifc data NFT-FT.

Call structure: "freezeSingleNFT" + "@" + nonce hex encoded + "@" + address hex encoded.

Example: "freezeSingleNFT@05@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

#### unFreezeSingleNFT

```rust
    #[endpoint(unFreezeSingleNFT)]
    fn unfreeze_single_token_for_address(
        &self,
        nonce: u64,
        address: &ManagedAddress
    );
```

Endpoint that unfreezes specific data NFT-FT for a specific address. It will utilize the token issued previously and owned by the smart contract and the nonce of the data NFT-FT. The unfreezed address will be able to interact with the specifc data NFT-FT.

Call structure: "unFreezeSingleNFT" + "@" + nonce hex encoded + "@" + address hex encoded.

Example: "unFreezeSingleNFT@05@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

#### wipeSingleNFT

```rust
    #[endpoint(wipeSingleNFT)]
    fn wipe_single_token_for_address(
        &self,
        nonce: u64,
        address: &ManagedAddress
    );
```

Endpoint that wipes specific data NFT-FT for a specific address. It will utilize the token issued previously and owned by the smart contract and the nonce of the data NFT-FT. The token manager may wipe out the tokens held by a frozen account, reducing the supply

Call structure: "wipeSingleNFT" + "@" + nonce hex encoded + "@" + address hex encoded.

Example: "wipeSingleNFT@05@afb9aa109340a83cdb2129635b060a3a2d67ba2659ad86bf6ef49f948c43572c"

#### setIsPaused

```rust
    #[endpoint(setIsPaused)]
    fn set_is_paused(
        &self,
        is_paused: bool
    );
```

Endpoint that sets the value of the pause variable. This variable is used to determined whether minting Data NFT-FT is activated or not.

Call structure: "setIsPaused" + "@" + is_paused hex encoded.

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

Endpoint that sets the token identifier and the tax value that will be used as a price for minting Data NFT-FTs, it will act also as an anti spam tax. The endpoint can take any token identifier, but be aware that diferent tokens have diferent decimals.

Call structure: "setAntiSpamTax" + "@" + token_id hex encoded + "@" + tax hex encoded.

Example: "setAntiSpamTax@45474c44@015af1d78b58c40000"

#### setWhiteListEnabled

```rust
    #[endpoint(setWhiteListEnabled)]
    fn set_whitelist_enabled(
        &self,
        is_enabled: bool
    );
```

Endpoint that sets wether the contract lets only whitelisted addresses to mint or not. If the contract has whitelist disabled, all addresses can mint.

Call structure: "setWhiteListEnabled" + "@" + is_enabled hex encoded.

Example: "setWhiteListEnabled@01"

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
(can add as many addresses as needed).

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
(can add as many addresses as needed).

Example: "removeWhiteListSpots@8bc1730b9afdd4546a039c3baa043f37525822100e04cfc986b6955e05cbf101"

#### setRoyaltiesLimits

```rust
    #[endpoint(setRoyaltiesLimits)]
    fn set_royalties_limits(
        &self,
        min_royalties: BigUint,
        max_royalties: BigUint
    );
```

Endpoint that sets the minimum and maximum royalties that can be set by the users when minting a Data NFT-FT.

Call structure: "setRoyaltiesLimits" + "@" + min_royalties hex encoded + "@" + max_royalties hex encoded.

Example: "setRoyaltiesLimits@00@01f40"

#### setMaxSupply

```rust
    #[endpoint(setMaxSupply)]
    fn set_max_supply(
        &self,
        max_supply: BigUint
    );
```

Endpoint that sets the value of the max supply variable. This variable is used to determined the maximum supply of the Data NFT-FTs an user can mint.

Call structure: "setMaxSupply" + "@" + max_supply hex encoded.

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

Endpoint that allows anyone to mint Data NFT-FTs. The endpoint takes as arguments the name (NFT-FT name), media (NFT-FT media url), data_marshal (marshal service url), data_stream (data stream url), data_preview (data preview url), royalties (royalties value between min_royalties and max_royalties), supply (supply value between 1 and max_supply), title (longer title describing the data NFT-FT dataset) and description (Dataset description).

Call structure for EGLD payment: "mint" + "@" + name hex encoded + "@" + media hex encoded + "@" + data_marshal hex encoded + "@" + data_stream hex encoded + "@" + data_preview hex encoded + "@" + royalties hex encoded + "@" + supply hex encoded + "@" + title hex encoded + "@" + description hex encoded.

Example: "mint@4d792044617461204e4654@68747470733a2f2f69746865756d2e696f2f746573742f746573742f746573742f746573742f6d65646961@68747470733a2f2f69746865756d2e696f2f746573742f746573742f746573742f746573742f6d65646961@68747470733a2f2f69746865756d2e696f2f746573742f746573742f746573742f746573742f6d65646961@68747470733a2f2f69746865756d2e696f2f746573742f746573742f746573742f746573742f6d65646961@3c@05@4e46542d4654205469746c65@4e46542d4654204465736372697074696f6e"

Call structure for ESDT payment: "ESDTTransfer" + "@" + token to send hex encoded + "@" + number of tokens to send hex encoded + "@" + "mint" hex encoded + "@" + name hex encoded + "@" + media hex encoded + "@" + data_marshal hex encoded + "@" + data_stream hex encoded + "@" + data_preview hex encoded + "@" + royalties hex encoded + "@" + supply hex encoded + "@" + title hex encoded + "@" + description hex encoded.

Example: "ESDTTransfer@49544845554d2d613631333137@015af1d78b58c40000@6d696e74@53616d706c65546f6b656e4e616d65@68747470733a2f2f697066732e696f2f697066732f62616679726569647835367968706f7371626e6b616432767970363734713574776a637666356a67767473626579686f6366346b6a657835776f34@68747470733a2f2f69746865756d6170692e636f6d2f646465782f646174616d61727368616c2f76312f73657276696365732f67656e6572617465@68747470733a2f2f69746865756d2d7265736f75726365732e73332e61702d736f757468656173742d322e616d617a6f6e6177732e636f6d2f6a736f6e2f54484f525f45636f47505f52616365312e637376@68747470733a2f2f69746865756d2d7265736f75726365732e73332e61702d736f757468656173742d322e616d617a6f6e6177732e636f6d2f6a736f6e2f54484f525f45636f47505f52616365312e637376@@01@53616d706c65205469746c65@53616d706c65204465736372697074696f6e"

#### burn

```rust
    #[payable("*")]
    #[endpoint(burn)]
    fn burn_token(&self);
```

Endpoint that allows anyone to burn the sent amount of Data NFT-FTs.

Call structure: "ESDTTransfer" + "@" + NFT-FT token identifier hex encoded + "@" + token nonce hex encoded + "@" + number of tokens to burn hex encoded + "@" + contract address hex encoded + "@" +"burn" hex encoded.

Example: "ESDTNFTTransfer@4e465446542d373736336637@01@1e@00000000000000000500c72532eb1c8f5e32034b46b5041babade020fdefd5fd@6275726e"

### withdrawal address endpoints

#### withdraw

```rust
    #[endpoint(withdraw)]
    fn withdraw(&self,
    token_identifier: &EgldOrEsdtTokenIdentifier,
    nonce: u64,
    amount: BigUint
    );
```

Endpoint that allows the withdrawal address to withdraw funds from the smart contract. The endpoint takes as arguments the token identifier, the nonce of the token and the amount to withdraw.

Call structure: "withdraw" + "@" + token_identifier hex encoded + "@" + nonce hex encoded + "@" + amount hex encoded.

Example: "withdraw@4e465446542d373736336637@00@91b77e5e5d9a0000"

### Views

#### getUserDataOut

```rust
    #[view(getUserDataOut)]
    fn get_user_data_out(&self,
    address: ManagedAddress,
    tax_token: &EsdtTokenIdentifier
    );
```

Main view of the contract. Receives an address and a token identifier as arguments and returns a structure that contains all the data needed by the frontend in order to limit the user from wrongly interacting with the smart contract. The structure contains the following fields:

- **anti_spam_tax_value**: the value of the anti spam tax for the given token identifier
- **is_paused**: a boolean that indicates if the contract is paused or not
- **max_royalties**: the maximum royalties value that can be set by the user
- **min_royalties**: the minimum royalties value that can be set by the user
- **max_supply**: the maximum supply value that can be set by the user
- **mint_time_limit**: the time limit for minting a data NFT-FT
- **last_mint_time**: the last time a data NFT-FT was minted by the given address
- **whitelist_enabled**: a boolean that indicates if the whitelist is enabled or not
- **is_whitelisted**: a boolean that indicates if the given address is whitelisted or not
- **minted_per_user**: total number of minted data NFT-FTs for given address
- **total_minted**: the total number of minted data NFT-FTs
- **frozen**: boolean that indicates if the given address is frozen or not for the entire collection
- **frozen_nonces**: a list of frozen nonces (of the smart contract token) for the given address

## Development

This smart contract aims to offer the Elrond community an audited NFT minter smart contract that is easy to use, well documented and secure.

### Setting up dev environment (project development bootstrap) + how to build (and upgrade)

- Uses `multiversx-sc-* 0.39.4` (starting v2.0.0, we used 0.45.1) SDK libs (see Cargo.toml)
- Building requires minimum **mxpy 6.1.1** (starting v2.0.0, we used mxpy 8.1.2). Check version using `mxpy --version`
- To build the project, requires minimum Rust version `1.69.0-nightly` (staring v2.0.0, we used 1.75.0-nightly). Check your Rust version by running `rustc --version`. To update your Rust, run `rustup update`. To set to nightly run `rustup default nightly`
- After you make sure you have the minimum Rust version you can then begin development. After you clone repo and before you run build, deploy or run the tests - follow these steps (most likely only needed the 1st time)
- [Upgrades] Note that when we upgrade smart contract, we should again follow the steps below too as lib version may have changed (but for upgrade I skipped the rustup default nightly cmd and did the others)

```
rustup default nightly
mxpy deps install rust --overwrite
cargo clean
cargo build
```

- The above should all work without any errors, next you can successfully run the following command to build via mxpy: `mxpy contract build`
- mxpy may ask you to install `nodejs` and `wasm-opt` to optimize the build, if so then follow instructions given by mxpy and do this
- You can now run the tests. See "How to test" section below
- You can now update code as needed

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
    cargo test --package datanftmint --test blackbox_tests --  --nocapture // [NOT DONE YET]
    cargo test --package datanftmint --test whitebox_tests --  --nocapture // [NOT DONE YET]

    cargo test --package datanftmint --test rust_tests --  --nocapture  // [OLD WAY, will work!]
```

- Note that in Oct 2023 we moved the Claim contract to blackbox and whitebox testing as recommended by the multiversX dev docs. We have HAVE NOT upgraded this yet for this smart contract so you need to use the OLD WAY

Another way of running the tests is by using the rust-analyzer extension in Visual Studio Code, which is also very helpful for MultiversX Smart Contract development. If one has the extension installed, they can go open and go to the top of the rust_tests file and click the Run Tests button.

Note: In order to run the tests, one has to use the rust nightly version. One can switch to the nightly version by using:

```shell
    rustup default nightly
```

### How to deploy

In order to deploy the smart contract on devnet one can use the interaction snippets present in the devnet. snippets file (which is located in the interactions folder). Before using the snippets, make sure to add your pem file in the root of the project under the name "wallet.pem" (or change the name to whichever one you wish to use in the interaction snippets). If you need info about how to derive a pem file you can find them [here](https://docs.multiversx.com/sdk-and-tools/sdk-py/deriving-the-wallet-pem-file/). To run the functions from the interaction file, one can use:

```shell
    source interaction/devnet.snippets.sh
```

After using that, to deploy one can simply use:

```shell
    deploy
```

### How to interact

After deployment, one can interact with the smart contract and test its functionality. To do so, one can use the interaction snippets already presented above. More explanations can be found about the snippets inside the devnet.snippets file.

### Mainnet Deployment (via Reproducible Builds)

- After the security audit has passed the Mainnet deployment need to be verified to match the version that was audited. This guarantee is given via [Reproducible Builds](https://docs.multiversx.com/developers/reproducible-contract-builds/#how-to-run-a-reproducible-build-using-mxpy)

**Step 1 (Final build + Code Hash):**

- Be in the latest `main` branch. On the commit that was audited. Update the cargo.toml files with the correct version. This should match the version we use in our requirements files (i.e Notion). e.g. 1.0.0. you need to update the `cargo.toml` files in the root folder, wasm folder and meta folder.

- In the `cargo.toml` files make sure you set the correct `edition`. i.e. edition = "2021"

- As the `cargo.toml` files has been updated. Build locally as normal. i.e. see "how to build" above and also run tests as per "how to test". This will reflect the `cargo.toml` update in the linked cargo.lock files and produces the final local meta build files to keep the final github check-in and version tagging perfect.

**Step 2 (Final build + Code Hash):**
Once the main commit is locked in, we can then produce the code hash and build to deploy to devnet 1st (for final testing) and then to mainnet (after sending the code hash to the auditor)

1. Make sure your mxpy version is >= 6 (starting v2.0.0, we used mxpy 8.1.2).
2. If Cargo.lock is in gitignore, remove it, build the contract and make a new commit. Otherwise this step can be skipped. (see Step 1 and repeat if needed)
3. Run the following in the root of the repository (run the latest Docker client in your computer. Used `Docker Desktop 4.18.0 (104112) on MacOX 12.6`):

`mxpy contract reproducible-build --docker-image="multiversx/sdk-rust-contract-builder:v4.1.4"`

Note that if you already have a output-docker from a previous build and deploy then delete this folder.

Also note that if you are upgrading you may need to use a newer docker `sdk-rust-contract-builder` version. You can see the tags here https://hub.docker.com/r/multiversx/sdk-rust-contract-builder/tags. Starting v2.0.0, we used v5.3.0 for the build to upgrade to. We tested this on devnet before doing it on mainnet.

This process may take some time. After it's done you should see "Docker build ran successfully!". An output-docker folder will be created containing the WASM files built in a reproducible way and artifacts.json containing the code hash of the WASM files.

You can then share the auditor the code hash. The auditor will follow the same steps and compare the code hash with yours. If they match, we will be good to go!

Note that "output-docker" folder should not be check-into GIT.

**Step 4 (Send Code Hash to auditor to verify against devnet and give us all final clear):**
We should have got this final clear in Step 2, but we still do a final check here.

**Step 5 (Deploy to Devnet as final build for testing + Move ABI to all apps that need it):**

**Step 6 (Tag the commit in the main branch of Github with the version that was deployed. e.g. 1.0.0):**

**Step 7 (Deploy SC to Mainnet):**

## Contributing

Feel free the contact the development team if you wish to contribute or if you have any questions. If you find any issues, please report them in the Issues sections of the repository. You can also create your own pull requests which will be analyzed by the team.

```

```
