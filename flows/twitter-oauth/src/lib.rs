use airtable_flows::create_record;
use base64::{engine::general_purpose, Engine as _};
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use lambda_flows::{request_received, send_response};
use serde_json::Value;
use store_flows::global_get;
use urlencoding::encode;

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        if let Some(account) = qry.get("state") {
            if let Some(account) = account.as_str() {
                if let Some(code) = qry.get("code") {
                    if let Some(code) = code.as_str() {
                        if let Some(token) = get_access(account, code) {
                            if let Some(user) = get_user(&token) {
                                let data = &user["data"];
                                let record = serde_json::json!({
                                    "Web3Account": account,
                                    "Username": data["username"],
                                    "Name": data["name"],
                                    "Verified": format!("{}", data["verified"]),
                                    "Verified Type": data["verified_type"],
                                    "Protected": format!("{}", data["protected"]),
                                    "Id": data["id"],
                                    "Created At": data["created_at"],
                                    "Profile Image Url": data["profile_image_url"],
                                    "Description": user["description"],
                                    "Followers Count": data["public_metrics"]["followers_count"],
                                    "Following Count": data["public_metrics"]["following_count"],
                                    "Tweet Count": data["public_metrics"]["tweet_count"],
                                    "Listed Count": data["public_metrics"]["listed_count"]
                                });
                                create_record(
                                    &std::env::var("AIRTABLE_ACCOUNT_NAME").unwrap(),
                                    &std::env::var("AIRTABLE_BASE_ID").unwrap(),
                                    "Twitter",
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

fn get_access(account: &str, code: &str) -> Option<String> {
    let challenge = global_get(&format!("twitter:challenge:{}", account));
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
        (
            "redirect_uri",
            std::env::var("TWITTER_OAUTH_REDIRECT_URI").unwrap(),
        ),
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
    let authorization = general_purpose::STANDARD.encode(format!(
        "{}:{}",
        std::env::var("TWITTER_OAUTH_CLIENT_ID").unwrap(),
        std::env::var("TWITTER_OAUTH_CLIENT_SECRET").unwrap(),
    ));
    if let Ok(res) = Request::new(&uri)
        .method(Method::POST)
        .header("content-type", "application/x-www-form-urlencoded")
        .header("authorization", &format!("Basic {}", authorization))
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
    let uri = Uri::try_from("https://api.twitter.com/2/users/me?user.fields=created_at,description,entities,id,location,name,pinned_tweet_id,profile_image_url,protected,public_metrics,url,username,verified,verified_type,withheld").unwrap();

    let mut writer = Vec::new();
    if let Ok(res) = Request::new(&uri)
        .method(Method::GET)
        .header("authorization", &format!("Bearer {}", token))
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
