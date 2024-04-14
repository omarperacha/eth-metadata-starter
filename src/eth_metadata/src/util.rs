use hex::FromHexError;
use std::collections::HashMap;

pub fn to_hex(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}

pub fn from_hex(data: &str) -> Result<Vec<u8>, FromHexError> {
    hex::decode(&data[2..])
}

pub fn parse_query_params(url: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();

    // Check if the URL contains query parameters
    if let Some(query_start) = url.find('?') {
        let queries = &url[query_start+1..]; // Get the substring after '?'
        for param in queries.split('&') {
            let pair: Vec<&str> = param.split('=').collect();
            if pair.len() == 2 {
                params.insert(pair[0].to_string(), pair[1].to_string());
            }
        }
    }

    params
}
