use lambda_flows::{request_received, send_response};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;
use store_flows::global_set;
use urlencoding::encode;

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        if let Some(account) = qry.get("account") {
            if let Some(account) = account.as_str() {
                let challenge = random_chanllenge();
                global_set(
                    &format!("twitter:challenge:{}", account),
                    Value::String(challenge.clone()),
                );
                let twitter_client_id = std::env::var("TWITTER_OAUTH_CLIENT_ID").unwrap();
                let redirect_uri = std::env::var("REDIRECT_URI").unwrap();
                let queries = vec![
                    ("state", account),
                    ("code_challenge_method", "plain"),
                    ("code_challenge", &challenge),
                    ("client_id", &twitter_client_id),
                    ("scope", "tweet.read users.read offline.access"),
                    ("response_type", "code"),
                    ("redirect_uri", &redirect_uri),
                ];

                let query = queries
                    .iter()
                    .map(|q| format!("{}={}", q.0, encode(q.1)))
                    .collect::<Vec<String>>()
                    .join("&");

                send_response(
                    302,
                    vec![(
                        String::from("Location"),
                        format!("https://twitter.com/i/oauth2/authorize?{}", query),
                    )],
                    "".as_bytes().to_vec(),
                );
                return;
            }
        }
        send_response(
            400,
            vec![(String::from("Content-Type"), String::from("text/plain"))],
            "No account".as_bytes().to_vec(),
        );
    });
}

fn random_chanllenge() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}
