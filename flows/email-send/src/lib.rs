use lambda_flows::{request_received, send_response};
use rand::{distributions::Alphanumeric, Rng};
use sendgrid_flows::{send_email, Email};
use store_flows::global_set;

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        if let Some(account) = qry.get("account") {
            if let Some(account) = account.as_str() {
                if let Some(to) = qry.get("to") {
                    if let Some(to) = to.as_str() {
                        let challenge = random_challenge();
                        global_set(
                            "web3bounty_email_challenge",
                            serde_json::json!({
                                "account": account,
                                "email": to,
                                "challenge": challenge
                            }),
                        );

                        let verifier_lambda = std::env::var("VERIFIER_LAMBDA").unwrap();

                        let email = Email {
                            to: vec![String::from(to)],
                            subject: String::from("Verify your email"),
                            content: String::from(format!(
                                r#"
Click the link below to verify your email<br/>
<a href="{verifier_lambda}?challenge={challenge}">{verifier_lambda}?challenge={challenge}</a>"#
                            )),
                        };
                        match send_email("vivian@secondstate.io", &email) {
                            Ok(_) => return send_response(200, vec![], vec![]),
                            Err(e) => return send_response(500, vec![], e.as_bytes().to_vec()),
                        }
                    }
                }
            }
        }
        send_response(400, vec![], vec![]);
    });
}

fn random_challenge() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}
