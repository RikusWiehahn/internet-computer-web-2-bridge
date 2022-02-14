use crate::state::*;
use std::cell::RefMut;
use ic_cdk::api::time;
use std::vec::Vec;

/*
    Auto clearing expired web requests
*/

static mut LAST_TIME_CLEANED: u64 = 0;

#[export_name = "canister_heartbeat"]
fn canister_heartbeat() {
    let duration: u64 = 1_000_000_000 * 60 * 15; // 15 minutes
    let t = time();
    unsafe {
        if t - duration > LAST_TIME_CLEANED {
            LAST_TIME_CLEANED = t;
            clear_old_data();
        }
    }
}

fn clear_old_data() {
    let cutoff: u64 = 1_000_000_000 * 60 * 15; // 15 minutes
    let t = time();
    STATE.with(|state: &GlobalState| {
        let mut web_responses: RefMut<WebResponsesList> = state.web_responses.borrow_mut();
        let mut web_requests: RefMut<WebRequestsList> = state.web_requests.borrow_mut();
        let mut new_web_responses: WebResponsesList = Vec::new();
        let mut new_web_requests: WebRequestsList = Vec::new();

        for req in web_requests.iter() {
            if req.created_at as u64 > t - cutoff {
                new_web_requests.push(req.clone());
            }
        }
        for res in web_responses.iter() {
            if res.created_at as u64 > t - cutoff {
                new_web_responses.push(res.clone());
            }
        }
        web_requests.clear();
        web_responses.clear();
        web_requests.append(&mut new_web_requests);
        web_responses.append(&mut new_web_responses);
    });
    
}