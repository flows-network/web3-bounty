use airtable_flows::search_records;
use lambda_flows::{request_received, send_response};

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        if let Some(account) = qry.get("account") {
            if let Some(account) = account.as_str() {
                if let Some(records) = search_records(
                    "DarumaDocker",
                    "appkkFvmnoRcpDRXY",
                    "GitHub",
                    format!("{{Web3Account}}='{account}'").as_str(),
                ) {
                    if records.is_object() {
                        if let Some(records) = records.get("records") {
                            if records.is_array() {
                                if let Ok(records) = serde_json::to_vec(records) {
                                    send_response(
                                        200,
                                        vec![
                                            (
                                                String::from("Content-Type"),
                                                String::from("application/json"),
                                            ),
                                            (
                                                String::from("Access-Control-Allow-Origin"),
                                                String::from("*"),
                                            ),
                                        ],
                                        records,
                                    );
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        send_response(
            400,
            vec![
                (String::from("Content-Type"), String::from("text/html")),
                (
                    String::from("Access-Control-Allow-Origin"),
                    String::from("*"),
                ),
            ],
            "".as_bytes().to_vec(),
        );
    });
}
