use flowsnet_platform_sdk::logger;
use lambda_flows::{request_received, send_response};
use reqwest::{self, Client};
use serde_json::Value;
use std::collections::HashMap;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    request_received(|headers, qry, body| handler(headers, qry, body, client)).await;
    Ok(())
}
const IP_API: &str = "api.ipify.org";

async fn handler(
    headers: Vec<(String, String)>,
    qry: HashMap<String, Value>,
    _body: Vec<u8>,
    client: Client,
) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    let msg = qry.get("msg").unwrap();
    let ip_resp = match client.get(IP_API).send().await {
        Ok(resp) => match resp.text().await {
            Ok(ok) => ok,
            Err(err) => format!("Could not decode response from {IP_API} to text {err}"),
        },
        Err(err) => format!("Request to {IP_API} Failed {}", err),
    };

    let resp = format!(
        "Welcome to flows.network.\n Echo Message: {msg}\nResult from internal IP Fetch :{ip_resp}"
    );

    send_response(
        200,
        vec![(String::from("content-type"), String::from("text/html"))],
        resp.as_bytes().to_vec(),
    );
}
