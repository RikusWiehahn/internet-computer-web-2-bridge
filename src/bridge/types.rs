
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct WebRequestPushed {
    pub queued: bool,
    pub id: String,
    pub message: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct WebRequestPulled {
    pub id: String,
    pub output: HttpRequest,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct WebResponsePushed {
    pub queued: bool,
    pub id: String,
    pub message: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct WebResponsePulled {
    pub completed: bool,
    pub success: bool,
    pub message: String,
    pub output: HttpResponse,
}

//
//  #    # ##### ##### #####
//  #    #   #     #   #    #
//  ######   #     #   #    #
//  #    #   #     #   #####
//  #    #   #     #   #
//  #    #   #     #   #

#[derive(CandidType, Deserialize)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub request_id: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub response_id: String,
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}
