use std::{rc::Rc, str::FromStr};
use serde::{Serialize, Deserialize};
use serde_json::Result;
use eth_rpc::call_contract;
use ethers_core::{
    abi::{Contract, Token}, 
    types::U256,
};

use crate::eth_rpc;

// Define a struct that represents your data structure
#[derive(Serialize, Deserialize)]
struct TokenMetadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<Attributes>,
    // Add other fields as needed, such as "media", etc.
}

// Define a struct for the nested "attributes" object
#[derive(Serialize, Deserialize)]
struct Attributes {
    trait_type: String,
    value: String,
}

thread_local! {
    static ABI: Rc<Contract> = Rc::new(include_abi!("../abi/nft_contract.json"));
}

pub async fn gen_token_metadata_json(token_id: u64) -> String {
    let network = "base"; // replace with your network
    let contract_address = "0x6b91B2ab683Ca5953fA2A1Df5D599842B69c2cDB"; // replace with your contract address

    // This calls a method from the specified contract
    let abi = ABI.with(Rc::clone);
    let trait1_result = call_contract(
        &network,
        contract_address.to_string(),
        &abi,
        "_colourOf", // replace with your contract function to retrieve your trait
        &[Token::Uint(token_id.into())], // replace with your contract function arguments
    )
    .await;

    let trait1 = trait1_result.get(0).unwrap();

    let trait2_result = call_contract(
        &network,
        contract_address.to_string(),
        &abi,
        "_logoOf", // replace with your contract function to retrieve your trait
        &[Token::Uint(token_id.into())], // replace with your contract function arguments
    ).await;

    let trait2 = trait2_result.get(0).unwrap();

    // Retrieve more traits as needed

    let trait1_u256 = U256::from_str(&trait1.to_string()).unwrap(); // assumes values are stored as U256 on-chain - adjust to a different type as needed
    let trait2_u256 = U256::from_str(&trait2.to_string()).unwrap();
    // Convert more traits as needed

    // Construct the token metadata JSON
    let token_metadata = create_token_json(&trait1_u256, &trait2_u256).unwrap();
    
    return token_metadata;
}

// Function that constructs the TokenMetadata struct and returns a JSON string
fn create_token_json(trait1: &U256, trait2: &U256 /* add more traits as needed */) -> Result<String> {

    let trait1_str = trait1_from_idx(&trait1.to_string());
    let trait2_str = trait2_from_idx(&trait2.to_string());

    let trait1_str_lower = trait1_str.to_lowercase();
    let trait2_str_lower = trait2_str.to_lowercase();
    // Add more traits as needed

    let trait1 = Attributes {
        trait_type: "Colour".to_string(), // replace with your trait name
        value: trait1_str,
    };

    let trait2 = Attributes {
        trait_type: "Icon".to_string(), // replace with your trait name or remove if not needed
        value: trait2_str,
    };

    // add more traits as needed

    let attributes = vec![trait1, trait2];
    
    // Replace the name and description fields with your NFT's metadata, and compute the relevant image_uri based on the token's traits
    let token = TokenMetadata {
        name: "100% On-Chain".to_string(),
        description: "This isn't just any dynamic NFT - this is 100% On-Chain dynamic NFT!".to_string(),
        image: format!("https://ipfs.io/ipfs/QmdvzrqnxsWjVHHKDRZ8scYtSTy8HAcfoqmeb7oAbSSXXA/{}-{}.png", trait1_str_lower, trait2_str_lower),
        attributes,
        // Add other fields as needed, such as "media", etc.
    };

    // Serialize the TokenMetadata instance to a JSON string
    serde_json::to_string(&token)
}

// Helper functions to convert trait indices to trait values. You may not need these if your on-chain traits stored as the values directly
fn trait1_from_idx(colour: &str) -> String {
    // Replace with your actual trait values
    match colour {
        "0" => "None".to_string(),
        "1" => "Red".to_string(),
        "2" => "Green".to_string(),
        "3" => "Blue".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn trait2_from_idx(icon: &str) -> String {
    // Replace with your actual trait values
    match icon {
        "0" => "ETH".to_string(),
        "1" => "ICP".to_string(),
        "2" => "Base".to_string(),
        "3" => "FC".to_string(),
        _ => "Unknown".to_string(),
    }
}