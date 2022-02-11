use ic_cdk::export::candid::{CandidType, Deserialize};
use std::cell::RefCell;
use std::vec::Vec;
use crate::types::*;
use ic_cdk::storage;
use ic_cdk_macros::*;



pub type WebRequestsList = Vec<StoredHttpRequest>;
pub type WebResponsesList = Vec<StoredHttpResponse>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct GlobalState {
    pub web_requests: RefCell<WebRequestsList>,
    pub web_responses: RefCell<WebResponsesList>,
}

thread_local! {
    pub static STATE: GlobalState = GlobalState {
        web_requests: RefCell::new(WebRequestsList::new()),
        web_responses: RefCell::new(WebResponsesList::new()),
    }
}

#[pre_upgrade]
fn save_data() {
    STATE.with(|s| {
        let web_requests = s.web_requests.borrow();
        let web_responses = s.web_responses.borrow();

        let res = storage::stable_save((web_requests.clone(), web_responses.clone()));
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