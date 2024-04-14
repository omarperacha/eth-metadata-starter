# ETH Metadata Starter

This repository provides an example and quickstart template for hybrid-chain dynamic NFTs on any EVM chain. It is adapted from the [icp-eth-starter](https://github.com/dfinity/icp-eth-starter) repository to accept incoming HTTP requests from a smart contract and return token metadata.

The eth-metadata-starter project contains canister code which provides metadata for the [💯 On-Chain NFT](https://onchain100.omarperacha.com/) collection on Base Mainnet (contract address: [0x6b91B2ab683Ca5953fA2A1Df5D599842B69c2cDB](https://basescan.org/address/0x6b91b2ab683ca5953fa2a1df5d599842b69c2cdb)), along with instructions to easily adapt it for any EVM smart contract.

## Hybrid-chain dNFTs

Dynamic NFTs on Ethereum tend to suffer from a dilemma:

- Either metadata is centralised, allowing for easy modification of the NFT traits by a trusted authority

OR

- Traits are stored in the contract directly, allowing for decentralised metadata - **but** dynamic traits can't be returned in the metadata JSON.

Hybrid-chain NFTs solve this issue by querying EVM token metadata from an IC canister which accepts HTTP requests and which can read the dynamic traits directly from the EVM smart contract where they are stored.

A request to the IC canister for a token's metadata will return a JSON that conforms to the NFT Metadata Standard **and** that has up-to-date dynamic trait values, enabling fully decentralised and trustless dynamic NFTs **without** compromising on user experience.

## Customising for your own NFT contract

To create your own hybrid-chain dNFT, make the following adjustments to these files in the `src/eth_metadata` folder:

- in `/abi/nft_contract.json`, replace the example ABI with your actual contract's ABI
- in `/src/eth_rpc.rs`, add the rpc_url for the relevant network to the `get_rpc_endpoint` function, if not already there
- in `/src/token_metadata.rs`, set the network and contract address. Adjust the code to call the correct methods from your contract to retrieve traits for the given tokenID and parse the return values into a token metadata JSON.

Much of where you'll need to adjust has been marked with comments, for convenience.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, you can query metadata by making a GET request to: `http://{canisterID}.localhost:4943/metadata?token-id={tokenID}`.

For example, if your canister ID is bkyz2-fmaaa-aaaaa-qaaaq-cai and you want to query metadata for a token with ID 1:
`http://bkyz2-fmaaa-aaaaa-qaaaq-cai.localhost:4943/metadata?token-id=1`

## Running the project on IC

When you're ready to deploy to production, run:

```bash
dfx deploy --network ic
```

Note: deploying to IC requires [Cycles](https://internetcomputer.org/docs/current/concepts/tokens-cycles#cycles).

To call the canister on IC, make a GET request to: `https://{canisterID}.icp0.io/metadata?token-id={tokenID}` or `https://{canisterID}.raw.icp0.io/metadata?token-id={tokenID}`. Use of `.raw.icp0.io` domains is generally not recommended for production.

For example, if your canister ID is bkyz2-fmaaa-aaaaa-qaaaq-cai and you want to query metadata for a token with ID 1:
`http://bkyz2-fmaaa-aaaaa-qaaaq-cai.icp0.io/metadata?token-id=1`

## Calling the canister from an EVM smart contract

In general, you will want to override the `_baseURI()` method of your NFT smart contract to return: `https://{canisterID}.icp0.io/metadata?token-id=` 

For example, if your canister ID is bkyz2-fmaaa-aaaaa-qaaaq-cai:

```solidity
function _baseURI() internal view override returns (string memory) {
    return "http://bkyz2-fmaaa-aaaaa-qaaaq-cai.icp0.io/metadata?token-id=";
}
```

The smart contract will then automatically append the tokenID passed to the `tokenURI()` method to this baseURI for every metadata query.
