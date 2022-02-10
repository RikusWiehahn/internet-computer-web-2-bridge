use crate::types::*;
use ic_cdk_macros::*;
use std::convert::TryFrom;
use std::result::Result;
use std::vec::Vec;
use uriparse::{Path, PathError};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::storage;
use std::cell::RefCell;
use std::collections::HashMap;


#[update]
async fn push_web_request(request: HttpRequest) -> PushedWebRequest {
    let res = WebRequestQueued {
        queued: false,
        web_request_id: request.request_id,
        message: "".to_string(),
    };
    let uuid_res: Result<String, String> = generate_uuid().await;
    if uuid_res.is_err() {
        res.message = uuid_res.err().unwrap().to_string();
        return res;
    }
    let uuid = uuid_res.unwrap();

    STATE.with(|state: &GlobalState| {
        let web_requests: Ref<EnquiryHashMap> = state.web_requests.borrow();
        web_requests.push(uuid.clone(), request.clone());
    });

    res.queued = true;
    res.web_request_id = uuid;
    return res;
}

#[update]
async fn pull_web_request(access_key: String) -> WebRequestPulled {
   
}

#[update]
async fn push_web_response(access_key: String) -> WebResponsePushed {
   
}

#[update]
async fn pull_web_response(access_key: String) -> WebResponsePulled {

   
}

//
//   ####  #####   ##   ##### ######
//  #        #    #  #    #   #
//   ####    #   #    #   #   #####
//       #   #   ######   #   #
//  #    #   #   #    #   #   #
//   ####    #   #    #   #   ######

pub type WebRequestMap = HashMap<String, HttpRequest>;
pub type WebResponseMap = HashMap<String, HttpResponse>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct GlobalState {
    pub web_requests: RefCell<WebRequestMap>,
    pub web_responses: RefCell<WebResponseMap>,
}

thread_local! {
    pub static STATE: GlobalState = GlobalState {
        web_requests: RefCell::new(WebRequestMap::new()),
        web_responses: RefCell::new(WebResponseMap::new()),
    }
}

//
//  #    # #####   ####  #####    ##   #####  ######
//  #    # #    # #    # #    #  #  #  #    # #
//  #    # #    # #      #    # #    # #    # #####
//  #    # #####  #  ### #####  ###### #    # #
//  #    # #      #    # #   #  #    # #    # #
//   ####  #       ####  #    # #    # #####  ######

#[pre_upgrade]
fn save_data() {
    STATE.with(|s| {
        let web_requests = s.web_requests.borrow();
        let web_responses = s.web_responses.borrow();

        let res =
            storage::stable_save((web_requests.clone(), web_responses.clone()));
        if res.is_err() {
            println!("Error saving data: {:?}", res.err().unwrap());
        }
    });
}

#[post_upgrade]
fn retrieve_data() {
    STATE.with(|state| {
        let res = storage::stable_restore();
        if res.is_err() {
            println!("Error retrieving data: {:?}", res.err().unwrap());
            return;
        } else {
            let (web_requests, web_responses) = res.unwrap();
            state.web_requests.replace(web_requests);
            state.web_responses.replace(web_responses);
        }
    });
}

//
//  #    # #    # # #####
//  #    # #    # # #    #
//  #    # #    # # #    #
//  #    # #    # # #    #
//  #    # #    # # #    #
//   ####   ####  # #####

pub async fn generate_uuid() -> Result<String, String> {
    let res = ic_cdk::call(Principal::management_canister(), "raw_rand", ()).await;
    if res.is_err() {
        return Err("Failed to generate UUID".to_string());
    }
    let (bytes,): (Vec<u8>,) = res.unwrap();
    let mut random_bytes: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    for i in 0..16 {
        random_bytes[i] = bytes[i];
    }
    let uuid = Builder::from_bytes(random_bytes)
        .set_variant(Variant::RFC4122)
        .set_version(Version::Random)
        .build();
    Ok(uuid.to_string())
}