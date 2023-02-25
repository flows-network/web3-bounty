use airtable_flows::{create_record, search_records};
use lambda_flows::{request_received, send_response};
use rand::{distributions::Alphanumeric, Rng};
use sendgrid_flows::{send_email, Email};
use serde_json::Value;
use std::collections::HashMap;
use store_flows::global_set;

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        create_email_record(&qry);
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
                            Ok(_) => {
                                return send_response(
                                    200,
                                    vec![(
                                        String::from("Access-Control-Allow-Origin"),
                                        String::from("*"),
                                    )],
                                    vec![],
                                )
                            }
                            Err(e) => {
                                return send_response(
                                    500,
                                    vec![(
                                        String::from("Access-Control-Allow-Origin"),
                                        String::from("*"),
                                    )],
                                    e.as_bytes().to_vec(),
                                )
                            }
                        }
                    }
                }
            }
        }
        send_response(
            400,
            vec![(
                String::from("Access-Control-Allow-Origin"),
                String::from("*"),
            )],
            vec![],
        );
    });
}

fn random_challenge() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}

fn create_email_record(qry: &HashMap<String, Value>) {
    if let Some(account) = qry.get("account") {
        if let Some(account) = account.as_str() {
            if let Some(to) = qry.get("to") {
                if let Some(to) = to.as_str() {
                    if let Some(records) = search_records(
                        &std::env::var("AIRTABLE_ACCOUNT_NAME").unwrap(),
                        &std::env::var("AIRTABLE_BASE_ID").unwrap(),
                        "Email",
                        format!("AND(LOWER({{Web3Account}})=LOWER('{account}'), LOWER({{Email}})=LOWER('{to}'))").as_str(),
                    ) {
                        if records.is_object() {
                            if let Some(records) = records.get("records") {
                                if records.is_array() {
                                    if !records.as_array().unwrap().is_empty() {
                                        return;
                                    }
                                }
                            }
                        }

                        let record = serde_json::json!({
                            "Web3Account": account,
                            "Email": to,
                        });
                        create_record(
                            &std::env::var("AIRTABLE_ACCOUNT_NAME").unwrap(),
                            &std::env::var("AIRTABLE_BASE_ID").unwrap(),
                            "Email",
                            record,
                        );
                    }
                }
            }
        }
    }
}
