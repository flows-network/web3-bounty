use lambda_flows::{request_received, send_response};
use sendgrid_flows::{send_email, Email};
use store_flows::global_get;

#[no_mangle]
pub fn run() {
    request_received(|qry, _body| {
        if let Some(to) = qry.get("to") {
            if let Some(to) = to.as_str() {
                let email = Email {
                    to: vec![String::from(to)],
                    subject: String::from("Hi"),
                    content: String::from("New Bee"),
                };
                send_email("wangshishuo@wespoke.com", &email);
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
