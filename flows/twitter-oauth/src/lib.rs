use airtable_flows::create_record;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use lambda_flows::{request_received, send_response};
use serde_json::Value;
use store_flows::get;
use urlencoding::encode;

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        if let Some(account) = qry.get("state") {
            if let Some(account) = account.as_str() {
                if let Some(code) = qry.get("code") {
                    if let Some(code) = code.as_str() {
                        if let Some(token) = get_access(account, code) {
                            /*
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
                                            "DarumaDockerDev",
                                            "appLjd0KmtnCf3l0r",
                                            "OAuth Users",
                                            record,
                                        );
                                    }
                                }
                            }
                            */
                        }
                    }
                }
            }
        }
        send_response(
            302,
            vec![(
                String::from("Location"),
                String::from("https://www.google.com"),
            )],
            "".as_bytes().to_vec(),
        );
    });
}

fn get_access(account: &str, code: &str) -> Option<String> {
    let challenge = get(&format!("challenge:{}", account));
    if challenge.is_none() {
        return None;
    }

    let params = vec![
        ("grant_type", String::from("authorization_code")),
        (
            "code_verifier",
            challenge.unwrap().as_str().unwrap().to_string(),
        ),
        (
            "client_id",
            std::env::var("TWITTER_OAUTH_CLIENT_ID").unwrap(),
        ),
        ("redirect_uri", std::env::var("REDIRECT_URI").unwrap()),
        ("code", code.to_string()),
    ];
    let params = params
        .iter()
        .map(|p| format!("{}={}", p.0, encode(&p.1)))
        .collect::<Vec<String>>()
        .join("&");

    let mut writer = Vec::new();
    let uri = format!("https://api.twitter.com/2/oauth2/token?{}", params);
    let uri = Uri::try_from(uri.as_str()).unwrap();
    if let Ok(res) = Request::new(&uri)
        .method(Method::POST)
        .header("content-type", "application/x-www-form-urlencoded")
        .send(&mut writer)
    {
        println!("----------");
        if res.status_code().is_success() {
            if let Ok(res) = serde_json::from_slice::<Value>(&writer) {
                println!("{:?}", res);
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
