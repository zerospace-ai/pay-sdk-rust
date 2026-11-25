use pay_sdk_rust::{response_define::ResponseNewOrder, rsa_utils, sdk::{load_config, SDKConfig, Sdk}};
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = load_config("./config.yaml")?;
    
    let sdk_config = SDKConfig {
        api_key: cfg.api_key.clone(),
        api_secret: cfg.api_secret.clone(),
        platform_pub_key: cfg.platform_pub_key.clone(),
        platform_risk_pub_key: cfg.platform_risk_pub_key.clone(),
        rsa_private_key: cfg.rsa_private_key.clone(),
    };

    let sdk = Sdk::new(sdk_config);
    let out_order_no = cfg.out_order_no.as_str();
    let out_user_id = if cfg.out_user_id.is_empty() {
        None
    } else {
        Some(cfg.out_user_id.clone())
    };
    let token_id = cfg.token_id.parse::<i32>().unwrap_or(6);
    let quantity = cfg.quantity;
    let notify_url = if cfg.notify_url.is_empty() {
        None
    } else {
        Some(cfg.notify_url.clone())
    };
    let order_detail = if cfg.order_detail.is_empty() {
        None
    } else {
        Some(cfg.order_detail.clone())
    };

    let (req_body, timestamp, sign, client_sign) =
        sdk.new_order(out_order_no, token_id, quantity, notify_url, out_user_id, order_detail)?;

    let client = Client::new();
    let url = "https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder";

    let mut req_builder = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("key", sdk.config.api_key.clone())
        .header("timestamp", timestamp)
        .header("sign", sign);
    if !client_sign.is_empty() {
        req_builder = req_builder.header("clientSign", client_sign);
    }
    let resp = req_builder.body(req_body).send().await?;

    let body = resp.text().await?;
    println!("Response: {}", body);

    let parsed: ResponseNewOrder = serde_json::from_str(&body)?;
    println!("Parsed response: {:?}", parsed);
    if parsed.code != 1 {
        return Err(anyhow::anyhow!("Error response: {}", parsed.msg));
    }
    println!("Deposit Address: {}", parsed.data.addresses);
    println!("Cashier URL: {}", parsed.data.cashier_url);

    match rsa_utils::to_string_map(&body) {
        Ok(map_data) => {
            let sign = parsed.sign.clone();
            match sdk.verify_rsa_signature(&map_data, &sign) {
                Ok(_) => println!("verify signature success!"),
                Err(e) => eprintln!("verify signature failed: {:?}", e),
            }
        }
        Err(e) => eprintln!("convert to map[string]string failed: {:?}", e),
    }

    Ok(())
}
