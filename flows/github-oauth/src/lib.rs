use airtable_flows::create_record;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use lambda_flows::{request_received, send_response};
use serde_json::Value;

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        if let Some(code) = qry.get("code") {
            if let Some(code) = code.as_str() {
                if let Some(token) = get_access(code) {
                    if let Some(user) = get_user(&token) {
                        if let Some(account) = qry.get("state") {
                            if let Some(account) = account.as_str() {
                                let record = serde_json::json!({
                                    "Web3Account": account,
                                    "Login": user["login"],
                                    "Name": user["name"],
                                    "Company": user["company"],
                                    "Blog": user["blog"],
                                    "Email": user["email"],
                                    "Location": user["location"],
                                    "Bio": user["bio"],
                                    "Twitter Username": user["twitter_username"],
                                    "Created At": user["created_at"]
                                });
                                create_record(
                                    &std::env::var("AIRTABLE_ACCOUNT_NAME").unwrap(),
                                    &std::env::var("AIRTABLE_BASE_ID").unwrap(),
                                    "GitHub",
                                    record,
                                );
                            }
                        }
                    }
                }
            }
        }
        send_response(
            302,
            vec![(
                String::from("location"),
                std::env::var("REDIRECT_URI").unwrap(),
            )],
            vec![],
        );
    });
}

fn get_access(code: &str) -> Option<String> {
    let uri = Uri::try_from("https://github.com/login/oauth/access_token").unwrap();
    let params = serde_json::json!({
        "client_id": std::env::var("GITHUB_APP_CLIENT_ID").unwrap(),
        "client_secret": std::env::var("GITHUB_APP_CLIENT_SECRET").unwrap(),
        "code": code,
    });
    let params = serde_json::to_string(&params).unwrap();

    let mut writer = Vec::new();
    if let Ok(res) = Request::new(&uri)
        .method(Method::POST)
        .header("content-type", "application/json")
        .header("accept", "application/json")
        .header("content-length", &params.as_bytes().len())
        .body(&params.as_bytes())
        .send(&mut writer)
    {
        if res.status_code().is_success() {
            if let Ok(res) = serde_json::from_slice::<Value>(&writer) {
                if let Some(at) = res["access_token"].as_str() {
                    return Some(at.to_string());
                }
            }
        }
    }

    None
}

fn get_user(token: &str) -> Option<Value> {
    let uri = Uri::try_from("https://api.github.com/user").unwrap();

    let mut writer = Vec::new();
    if let Ok(res) = Request::new(&uri)
        .method(Method::GET)
        .header("user-agent", "Flows.network function")
        .header("authorization", &format!("Bearer {}", token))
        .header("accept", "application/vnd.github+json")
        .send(&mut writer)
    {
        if res.status_code().is_success() {
            if let Ok(res) = serde_json::from_slice::<Value>(&writer) {
                return Some(res);
            }
        }
    }

    None
}
