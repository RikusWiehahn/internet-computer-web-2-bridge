use ic_cdk::export::candid::{CandidType, Deserialize};

/*          
    Types used by the bridge
*/

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PushedWebRequest {
    pub id: Option<String>,
    pub message: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PulledWebRequests {
    pub requests:Vec<StoredHttpRequest>,
    pub message: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PushedWebResponse {
    pub id: Option<String>,
    pub message: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct PulledWebResponse {
    pub pending: bool,
    pub message: String,
    pub response: Option<HttpResponse>,
}

// HTTP 

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<Vec<String>>,
    pub body: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StoredHttpRequest {
    pub id: String,
    pub created_at: f64,
    pub pulled: bool,
    pub method: String,
    pub url: String,
    pub headers: Vec<Vec<String>>,
    pub body: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u128,
    pub headers: Vec<Vec<String>>,
    pub body: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StoredHttpResponse {
    pub id: String,
    pub created_at: f64,
    pub status_code: u128,
    pub headers: Vec<Vec<String>>,
    pub body: String,
}

