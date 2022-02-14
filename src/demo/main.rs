use crate::keys::*;
use crate::types::*;
use ic_cdk::api::call;
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use std::result::Result;

#[query]
async fn ping() -> String {
    return "pong".to_string();
}

static mut LAST_JOB_TIME: u64 = 0;

#[export_name = "canister_heartbeat"]
fn canister_heartbeat() {
    let duration: u64 = 1_000_000_000 * 60 * 1; // 1 minute
    let t = ic_cdk::api::time();
    unsafe {
        if t - duration > LAST_JOB_TIME {
            LAST_JOB_TIME = t;
            ic_cdk::block_on(push_rates_request());
        }
    }
}

async fn push_rates_request() {
    let principal = Principal::from_text(BRIDGE_CANISTER_ID).unwrap();
    let req = BridgeHttpRequest {
        url: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd,aud,eur,brl,cad,dkk,hkd,jpy,nzd,nok,sgd,sek,chf,gbp".to_string(),
        method: "GET".to_string(),
        headers: vec![vec![
            "Content-Type".to_string(),
            "application/json".to_string(),
        ]],
        body: "{}".to_string(),
    };
    let res: Result<(PushedWebRequest,), _> =
        call::call(principal, "push_web_request", (BRIDGE_ACCESS_KEY, req)).await;
    if res.is_ok() {
        let web_req = res.unwrap().clone().0;
        if web_req.id.is_some() {
            pull_rates_response(web_req.id.unwrap().clone()).await;
        } else {
            ic_cdk::println!("{}", web_req.message);
        }
    }
}

async fn pull_rates_response(req_id: String) {
    let principal = Principal::from_text(BRIDGE_CANISTER_ID).unwrap();
    let mut counter: i64 = 0;
    let max_counter: i64 = 5_000_000_000;

    while counter < max_counter {
        if counter % 100_000_000 == 0 {
            ic_cdk::println!("count {}", counter.clone());
            let res: Result<(PulledWebResponse,), _> = call::call(
                principal,
                "pull_web_response",
                (BRIDGE_ACCESS_KEY, req_id.clone()),
            )
            .await;
            if res.is_ok() {
                let web_res = res.unwrap().clone().0;
                if web_res.response.is_some() {
                    counter = max_counter;
                    let json_res: serde_json::Value = serde_json::from_str(&web_res.response.unwrap().body).unwrap();
                    ic_cdk::println!("response success {:?}", json_res);
                }
            }
        }
        counter += 1;
    }
}
