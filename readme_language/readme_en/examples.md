# Examples and Tools

This document is divided into two parts:
1. **Scenario-Based Code Examples:** Demonstrates how to handle API calls and verification in practical code.
2. **CLI Tools Guide:** Explains how to use the SDK's example programs for quick testing.

---

## 1. Scenario-Based Code Examples

### 1.1 Complete API Call and Response Verification

The following code demonstrates how to use the SDK to build a "Create User" request, send the HTTP request, and verify the security of the signature of the data returned by the platform.

```rust
use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize SDK (Prerequisite: config.yaml is configured)
    let cfg = load_config("./config.yaml")?;
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 2. Generate request parameters and signature Header
    let open_id = "PT00001";
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // 3. Send HTTP request
    let client = Client::new();
    let url = "https://sandbox-api.privatex.io/sdk/user/create";
    let resp = client.post(url)
        .header("Content-Type", "application/json")
        .header("key", sdk.config.api_key.clone())
        .header("timestamp", timestamp)
        .header("sign", sign)
        .header("clientSign", client_sign)
        .body(req_body)
        .send()
        .await?;

    let body = resp.text().await?;
    
    // 4. Automatically verify response signature
    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    if parsed.code != 1 {
        println!("Request failed: {}", parsed.msg);
        return Ok(());
    }

    let map_data = rsa_utils::to_string_map(&body)?;
    let rsp_data: pay_sdk_rust::response_define::ResponseCreateUser = serde_json::from_str(&body)?;
    
    match sdk.verify_rsa_signature(&map_data, &rsp_data.sign) {
        Ok(_) => println!("✅ Request successful and verified! Returned OpenId: {}", rsp_data.data.open_id),
        Err(e) => println!("❌ Signature verification failed: {:?}", e),
    }

    Ok(())
}
```

---

## 2. CLI Tools Usage Guide

The SDK provides example command-line programs for quickly testing each API endpoint.

### 2.1 Compile Example Files

Run the following command in the SDK root directory to compile the examples:

```bash
cargo build --examples
```

The compiled binary files will be located in the `target/debug/examples/` directory.
* **Windows:** Generates files ending in `.exe` (e.g., `create_user_client.exe`).
* **Mac/Linux:** Generates files without suffixes (e.g., `create_user_client`).

### 2.2 Prepare Configuration File

Before running the tools, ensure that the configured `config.yaml` file is placed in the execution directory (usually the project root or `target/debug/examples/`).

### 2.3 Testing Endpoint Commands

#### Register New User
1. Modify the `UserOpenId` field in `config.yaml`.
2. Run `cargo run --example create_user_client`.

#### Wallet Registration
1. Specify `UserOpenId` and `ChainID` in `config.yaml`.
2. Run `cargo run --example create_wallet`.

#### Get Deposit Addresses
1. Specify `UserOpenId` and the queried `ChainIDs` (e.g., "1,56") in `config.yaml`.
2. Run `cargo run --example get_wallet_addresses`.

#### Apply for Withdrawal
1. Specify the following in `config.yaml`:
   * `UserOpenId`
   * `TokenId`
   * `Amount`
   * `AddressTo`
   * `SafeCheckCode` (Unique order anti-duplication code)
   * `CallbackUrl`
2. Run `cargo run --example user_withdraw_by_open_id`.

#### Create Cashier Order
1. Specify `OutOrderNo`, `TokenId`, `Quantity`, and `NotifyUrl` in `config.yaml`.
2. Run `cargo run --example new_order`.

#### Query Wallet Balance
1. Specify `WalletAddress`, `ContractAddress`, and `WalletBalanceChainId` in `config.yaml`.
2. Run `cargo run --example wallet_balance`.