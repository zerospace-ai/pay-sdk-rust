use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = load_config("./config.yaml")?;

    let sdk_config = SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    };

    let sdk = Sdk::new(sdk_config);
    let open_id = "user_010";

    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    let client = Client::new();
    let url = "https://sandbox-api.privatex.io/sdk/user/create";

    let resp = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("key", sdk.config.api_key.clone())
        .header("timestamp", timestamp)
        .header("sign", sign)
        .header("clientSign", client_sign)
        .body(req_body)
        .send()
        .await?;

    let body = resp.text().await?;
    println!("Response: {}", body);

    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    println!("Parsed response: {:?}", parsed);
    if parsed.code != 1 {
        return Err(anyhow::anyhow!("Error response: {}", parsed.msg));
    }
    match rsa_utils::to_string_map(&body) {
        Ok(map_data) => {
            match serde_json::from_str::<pay_sdk_rust::response_define::ResponseCreateUser>(&body) {
                Ok(rsp_data) => {
                    let sign = rsp_data.sign.clone();
                    match sdk.verify_rsa_signature(&map_data, &sign) {
                        Ok(_) => println!("verify signature success!"),
                        Err(e) => eprintln!("verify signature failed: {:?}", e),
                    }
                }
                Err(e) => eprintln!("parse JSON ResponseCreateUser failed: {:?}", e),
            }
        }
        Err(e) => eprintln!("convert to map[string]string failed: {:?}", e),
    }

    Ok(())
}
