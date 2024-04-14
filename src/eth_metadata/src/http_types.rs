use candid::{CandidType, Deserialize, Nat};
use serde_bytes::ByteBuf;

// Corresponds to `type HeaderField = record { text; text; };`
#[derive(CandidType, Deserialize)]
pub struct HeaderField(pub String, pub String);

// Corresponds to the `HttpRequest` and `HttpUpdateRequest` records
#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
    pub certificate_version: Option<u16>, 
}

#[derive(CandidType, Deserialize)]
pub struct HttpUpdateRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
}

// For `HttpResponse`
#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
    pub upgrade: Option<bool>,
    pub streaming_strategy: Option<StreamingStrategy>, // Assume definition below
}

// Assuming a definition for `StreamingStrategy` based on your actual Candid definition
#[derive(CandidType, Deserialize)]
pub enum StreamingStrategy {
    Callback { callback: candid::Principal, token: Nat },
    CallbackToken { token: Nat },
}
