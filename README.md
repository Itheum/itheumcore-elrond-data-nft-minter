# Itheum Core MultiversX - NFT-FT Minter Smart Contract

## Abstract

The Data NFT Minting contract is a tool that can be used in order to tokenize and sell the right to use different forms of data in the form of an SFT with different supplies.

## Introduction

This contract allows the owner of it to create a SFT collection towards which anyone can contribute by creating Data NFTs. The creators can even choose their own royalties or supply.

## Prerequisites

This documentation assumes the user has previous programming experience. Moreover, the user should have a basic understanding of the MultiversX blockchain. If you are new to the blockchain, please refer to the [MultiversX documentation](https://docs.multiversx.com/). In order to develop MultiversX smart contract related solutions, one needs to have installed [mxpy](https://docs.multiversx.com/sdk-and-tools/sdk-py/installing-mxpy).

Understanding this document is also easier if one knows how [ESDT token transactions](https://docs.multiversx.com/developers/esdt-tokens/#transfers-to-a-smart-contract) are structured on the MultiversX blockchain and how [NFT tokens](https://docs.multiversx.com/tokens/nft-tokens/) work on the MultiversX Blockchain.

## Itheum deployed contract addresses

| Devnet                                                         | Mainnet                                                            |
| -------------------------------------------------------------- | ------------------------------------------------------------------ |
| erd1qqqqqqqqqqqqqpgq7thwlde9hvc5ty7lx2j3l9tvy3wgkwu7fsxsvz9rat | erd1qqqqqqqqqqqqqpgqmuzgkurn657afd3r2aldqy2snsknwvrhc77q3lj8l6 yet |

## Endpoints

See `devnet.snippets.sh` for list of available endpoints for user testing.

## Development

This smart contract aims to offer the MultiversX community an audited NFT minter smart contract that is easy to use, well documented and secure.

### Setting up dev environment (project development bootstrap) + how to build (and upgrade)

- Uses `multiversx-sc-* 0.39.4` (In v3.0.0 and v4.0.0, we used 0.47.5) SDK libs (see Cargo.toml)
- Building requires minimum **mxpy 6.1.1** (In >v2.0.0, we used mxpy 9.5.1). Check version using `mxpy --version`
- To build the project, requires minimum Rust version `1.76.0-nightly` (In v3.0.0, we used 1.76.0-nightly). Check your Rust version by running `rustc --version`. To update your Rust, run `rustup update`. To set to nightly run `rustup default nightly`. Note that `mxpy deps install rust --overwrite` also brings in it's own compatible rust version so running `rustup default nightly` might have a higher rust version than what is used via `mxpy deps install rust --overwrite`.
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
    cargo test
```

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

1. Make sure your mxpy version is >= 6 (In v3.0.0, we used mxpy 9.5.1).
2. If Cargo.lock is in gitignore, remove it, build the contract and make a new commit. Otherwise this step can be skipped. (see Step 1 and repeat if needed)
3. Run the following in the root of the repository (run the latest Docker client in your computer. Used `Docker Desktop 4.18.0 (104112) on MacOX 12.6`):

`mxpy contract reproducible-build --docker-image="multiversx/sdk-rust-contract-builder:v4.1.4"`

Note that if you already have a output-docker from a previous build and deploy then delete this folder.

Also note that if you are upgrading you may need to use a newer docker `sdk-rust-contract-builder` version. You can see the tags here https://hub.docker.com/r/multiversx/sdk-rust-contract-builder/tags. In v2.0.0, we used v5.3.0 and for v3.0.0, v4.0.0, we used v6.1.1 for the build to upgrade to. We tested this on devnet before doing it on mainnet.

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
