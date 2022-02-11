use ic_cdk::export::candid::{CandidType, Deserialize};

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

//
//  #    # ##### ##### #####
//  #    #   #     #   #    #
//  ######   #     #   #    #
//  #    #   #     #   #####
//  #    #   #     #   #
//  #    #   #     #   #

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HeaderField(pub String, pub String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StoredHttpRequest {
    pub id: String,
    pub pulled: bool,
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct StoredHttpResponse {
    pub id: String,
    pub created_at: String,
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: String,
}

