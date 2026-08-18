use pay_sdk_rust::{
    response_define::ResponseWalletBalance,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
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
    let address = if !cfg.wallet_address.is_empty() {
        cfg.wallet_address.as_str()
    } else {
        "rXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
    };
    let contract_address = if !cfg.contract_address.is_empty() {
        cfg.contract_address.as_str()
    } else {
        "XRP"
    };
    let chain_id = if cfg.wallet_balance_chain_id != 0 {
        cfg.wallet_balance_chain_id
    } else {
        5
    };

    let (req_body, timestamp, sign, client_sign) =
        sdk.get_wallet_balance(address, contract_address, chain_id)?;

    let client = Client::new();
    let url = "https://sandbox-api.privatex.io/sdk/wallet/balance";

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

    let parsed: ResponseWalletBalance = serde_json::from_str(&body)?;
    println!("Parsed response: {:?}", parsed);
    if parsed.code != 1 {
        return Err(anyhow::anyhow!("Error response: {}", parsed.msg));
    }

    if let Some(sign) = &parsed.sign {
        if !sign.is_empty() {
            match rsa_utils::to_string_map(&body) {
                Ok(map_data) => match sdk.verify_rsa_signature(&map_data, sign) {
                    Ok(_) => println!("verify signature success!"),
                    Err(e) => eprintln!("verify signature failed: {:?}", e),
                },
                Err(e) => eprintln!("convert to map[string]string failed: {:?}", e),
            }
        }
    }

    println!("Wallet Address: {}", address);
    println!("Contract / Token: {}", contract_address);
    println!("Chain ID: {}", chain_id);
    println!("Balance: {}", parsed.data.as_deref().unwrap_or("0"));

    Ok(())
}
