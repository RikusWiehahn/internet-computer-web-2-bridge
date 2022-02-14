use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct BridgeHttpRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<Vec<String>>,
    pub body: String,
}
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PushedWebRequest {
    pub id: Option<String>,
    pub message: String,
}
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PulledWebResponse {
    pub pending: bool,
    pub message: String,
    pub response: Option<BridgeHttpResponse>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct BridgeHttpResponse {
    pub status_code: u128,
    pub headers: Vec<Vec<String>>,
    pub body: String,
}