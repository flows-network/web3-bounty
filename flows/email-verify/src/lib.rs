use airtable_flows::{search_records, update_record};
use lambda_flows::{request_received, send_response};
use store_flows::global_get;

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        if let Some(challenge) = qry.get("challenge") {
            if let Some(challenge) = challenge.as_str() {
                if let Some(v) = global_get("web3bounty_email_challenge") {
                    if let Some(ch) = v.get("challenge") {
                        if challenge == ch.as_str().unwrap_or_default() {
                            let account = v.get("account").unwrap();
                            let email = v.get("email").unwrap();

                            if let Some(record_id) = find_record(
                                account.as_str().unwrap_or_default(),
                                email.as_str().unwrap_or_default(),
                            ) {
                                let record = serde_json::json!({
                                    "Verified": true
                                });
                                update_record(
                                    &std::env::var("AIRTABLE_ACCOUNT_NAME").unwrap(),
                                    &std::env::var("AIRTABLE_BASE_ID").unwrap(),
                                    "Email",
                                    &record_id,
                                    record,
                                );

                                return send_response(
                                    200,
                                    vec![(String::from("Content-Type"), String::from("text/html"))],
                                    r#"Email has been verified.
                                    <script>
                                        setTimeout(function() {
                                            window.close();
                                        }, 1500);
                                    </script>
                                    "#
                                    .as_bytes()
                                    .to_vec(),
                                );
                            }
                        }
                    }
                }
            }
        }
        send_response(400, vec![], "Invalid challenge code".as_bytes().to_vec());
    });
}

fn find_record(account: &str, email: &str) -> Option<String> {
    if let Some(records) = search_records(
        &std::env::var("AIRTABLE_ACCOUNT_NAME").unwrap(),
        &std::env::var("AIRTABLE_BASE_ID").unwrap(),
        "Email",
        format!(
            "AND(LOWER({{Web3Account}})=LOWER('{account}'), LOWER({{Email}})=LOWER('{email}'))"
        )
        .as_str(),
    ) {
        if records.is_object() {
            if let Some(records) = records.get("records") {
                if records.is_array() {
                    return Some(records[0]["id"].as_str().unwrap_or_default().to_string());
                }
            }
        }
    }

    None
}
