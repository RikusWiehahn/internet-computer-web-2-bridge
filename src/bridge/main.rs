use crate::state::*;
use crate::types::*;
use crate::utils::*;
use ic_cdk_macros::*;
use std::cell::RefMut;
use ic_cdk::api::time;
use std::cell::Ref;
use std::result::Result;
use std::vec::Vec;


#[query]
async fn ping() -> String {
    return "pong".to_string();
}

#[update]
async fn push_web_request(access_key: String, request: HttpRequest) -> PushedWebRequest {
    let mut res = PushedWebRequest {
        id: None,
        message: "".to_string(),
    };
    
    let auth = verify_access_key(&access_key);
    if auth.is_err() {
        res.message = auth.err().unwrap();
        return res;
    }

    let uuid_res: Result<String, String> = generate_uuid().await;
    if uuid_res.is_err() {
        res.message = uuid_res.err().unwrap().to_string();
        return res;
    }
    let uuid = uuid_res.unwrap();

    let http_request_to_save = StoredHttpRequest {
        id: uuid.clone(),
        created_at: time() as f64,
        pulled: false,
        method: request.method.to_string(),
        url: request.url.to_string(),
        headers: request.headers,
        body: request.body,
    };

    STATE.with(|state: &GlobalState| {
        let mut web_requests: RefMut<WebRequestsList> = state.web_requests.borrow_mut();
        web_requests.push(http_request_to_save);
    });

    res.id = Some(uuid);
    return res;
}

#[update]
async fn pull_web_requests(access_key: String) -> PulledWebRequests {
    
    let mut res: PulledWebRequests = PulledWebRequests {
        requests: Vec::new(),
        message: "".to_string(),
    };
    let auth = verify_access_key(&access_key);
    if auth.is_err() {
        res.message = auth.err().unwrap();
        return res;
    }

    let mut requests: Vec<StoredHttpRequest> = Vec::new();
    STATE.with(|state: &GlobalState| {
        let mut web_requests: RefMut<WebRequestsList> = state.web_requests.borrow_mut();
        for request in web_requests.iter_mut() {
            if !request.pulled {
                request.pulled = true;
                requests.push(request.clone());
            }
        }
    });

    res.requests = requests;
    return res;
}

#[update]
async fn push_web_response(access_key: String, response: StoredHttpResponse) -> PushedWebResponse {
    let mut res = PushedWebResponse {
        id: None,
        message: "".to_string(),
    };
    if response.id == "" {
        res.message = "Response id is invalid".to_string();
        return res;
    }
    let auth = verify_access_key(&access_key);
    if auth.is_err() {
        res.message = auth.err().unwrap();
        return res;
    }
    STATE.with(|state: &GlobalState| {
        let mut web_responses: RefMut<WebResponsesList> = state.web_responses.borrow_mut();
        let mut web_requests: RefMut<WebRequestsList> = state.web_requests.borrow_mut();

        web_responses.push(response.clone());
        let mut req_index: Option<usize> = None;
        for (i, request) in web_requests.iter_mut().enumerate() {
            if request.id == response.id {
                req_index = Some(i);
                break;
            }
        }
        if req_index.is_some() {
            web_requests.remove(req_index.unwrap());
        }
    });

    res.id = Some(response.id.clone());
    return res;
}

#[update]
async fn pull_web_response(access_key: String, id: String) -> PulledWebResponse {
    let mut res: PulledWebResponse = PulledWebResponse {
        pending: false,
        response: None,
        message: "".to_string(),
    };
    let auth = verify_access_key(&access_key);
    if auth.is_err() {
        res.message = auth.err().unwrap();
        return res;
    };

    STATE.with(|state: &GlobalState| {
        let mut web_responses: RefMut<WebResponsesList> = state.web_responses.borrow_mut();
        ic_cdk::println!("responses count {:?}", web_responses.len());

        let mut res_index: Option<usize> = None;
        let mut response: Option<StoredHttpResponse> = None;
        for (i, stored_response) in web_responses.iter_mut().enumerate() {
            if stored_response.id == id {
                res_index = Some(i);
                response = Some(stored_response.clone());
            }
        }
        if res_index.is_some() {
            web_responses.remove(res_index.unwrap());
            let response_to_return = response.unwrap();
            res.response = Some(HttpResponse {
                headers: response_to_return.headers,
                body: response_to_return.body,
                status_code: response_to_return.status_code,
            });
        }
    });

    if res.response.is_some() {
        res.pending = false;
        return res;
    }

    STATE.with(|state: &GlobalState| {
        let web_requests: Ref<WebRequestsList> = state.web_requests.borrow();
        for stored_request in web_requests.iter() {
            if stored_request.id == id {
                res.pending = true;
            }
        }
    });
    if res.pending {
        res.message = "Web request is still pending".to_string();
        return res;
    }

    res.message = "Web request is not found".to_string();
    return res;
}
