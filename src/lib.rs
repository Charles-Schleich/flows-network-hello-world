use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};

use serde_json::Value;
use std::collections::HashMap;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {

    request_received(|headers, qry: HashMap<String, Value>, body| handler(headers, qry, body, client)).await;
    Ok(())
}

async fn handler(
    headers: Vec<(String, String)>,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
    client: Client,
) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let msg = qry.get("msg").unwrap();

    let resp = format!(
        "Testing Flows Network: This is your message {msg}"
    );

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.as_bytes().to_vec(),
    );
}
