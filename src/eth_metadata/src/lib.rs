use ic_cdk::{update, query};
use serde_bytes::ByteBuf;
use token_metadata::gen_token_metadata_json;

#[macro_use]
mod eth_rpc;
mod util;
mod token_metadata;
mod http_types;

#[query]
async fn http_request(req: http_types::HttpRequest) -> http_types::HttpResponse {
    let path = req.url;
    let params = util::parse_query_params(&path);
    if path.starts_with("/metadata") && params.get("token-id").is_some() && params.get("token-id").unwrap().parse::<u64>().is_ok() {
        http_types::HttpResponse {
            status_code: 200,
            headers: Vec::new(),
            body: ByteBuf::from("".as_bytes().to_vec()),
            streaming_strategy: None,
            upgrade: Some(true),
        }
    } else {
        http_types::HttpResponse {
            status_code: 404,
            headers: Vec::new(),
            body: ByteBuf::from(b"404 Not found :".to_vec()),
            streaming_strategy: None,
            upgrade: None,
        }
    }
}

#[update]
async fn http_request_update(req: http_types::HttpUpdateRequest) -> http_types::HttpResponse {
    let path = req.url;
    let params = util::parse_query_params(&path);
    if path.starts_with("/metadata") && params.get("token-id").is_some() && params.get("token-id").unwrap().parse::<u64>().is_ok() {
        let token_id = params.get("token-id").unwrap().parse::<u64>().unwrap();
        let token_metadata = gen_token_metadata_json(token_id).await;
        http_types::HttpResponse {
            status_code: 200,
            headers: Vec::new(),
            body: ByteBuf::from(token_metadata.as_bytes().to_vec()),
            streaming_strategy: None,
            upgrade: None,
        }
    } else {
        http_types::HttpResponse {
            status_code: 404,
            headers: Vec::new(),
            body: ByteBuf::from(b"404 Not found :".to_vec()),
            streaming_strategy: None,
            upgrade: None,
        }
    }
}