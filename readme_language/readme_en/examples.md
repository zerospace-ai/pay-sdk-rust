# Examples 📝

This document provides usage examples for the CryptoPay Rust SDK, including Demo running, key generation, and callback handling.

## 1 SDK Instance Object 🛠️

### 1.1 Required Configuration ⚙️

1. Register your business name and obtain the `ApiKey` and `ApiSecret`;

2. Generate your own `RSA` key pair;

3. Prepare the platform's `RSA` public key;

### 1.2 Creating a Signature Object 🔏

1. Add a configuration file `config.yaml`.

```yaml
# Configure business information
ApiKey: ""
ApiSecret: ""
# Platform public key
PlatformPubKey: ""
# Public key for blocking the platform
PlatformRiskPubKey: ""
# Your own private key
RsaPrivateKey: ""
```

2. Load the configuration file and create the API object.

```rust

	let cfg = load_config("./config.yaml")?;
    
    let sdk_config = SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    };

```

### 1.3 Create and sign the request data. ✍️

Let's use user creation as an example.

```rust

	// ....
	let open_id = "user_002";

    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;


```

```rust
	let map_data: HashMap<String, String> =
            serde_json::from_value(serde_json::to_value(&req)?)?;
	let data_str = rsa_utils::compose_params(&map_data);
	let timestamp = Utc::now().timestamp_millis().to_string();
	let sign = self.generate_md5_sign(&data_str, &timestamp);
	let client_sign = self.generate_rsa_signature(&map_data)?;
	let j_str = serde_json::to_string(&req)?;
```

### 1.4 Filling in and Initiating the Request 🚀

```rust
	// ....
	
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
    println!("Response: {}", body);

```

### 1.5 Verify parsing return data ✅

```rust

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

```

## 2. Generate Executable Interface Commands

* 1. Execute the `cargo build --example create_user_client` command in the SDK root directory to generate binary executable files for each function command in the `pay-sdk-rust/target/debug/examples/` directory.

* 2. The file with the ".exe" suffix runs on 64-bit Windows machines; the file without the ".exe" suffix runs on Linux/Mac. For example, create_user.exe and create_user executable files.

* 3. Copy the configured config.yaml file to the bin directory.

## 3. Calling the Command 📞

### 3.1. Registering a New User 🆕


Rust to the SDK's `pay-sdk-rust/target/debug/examples/` directory and modify the UserOpenId field in the config.yaml file there.

Run the create_user or create_user.exe executable file to register a new user on the platform.

If you attempt to register a new UserOpenId that has already been registered, an error will be returned.


### 3.2. Wallet Registration 💼

Rust to the SDK's `pay-sdk-rust/target/debug/examples/` directory and specify the `UserOpenId` and `ChainID` fields in the `config.yaml` file.

Run the `create_wallet` or `create_wallet.exe` executable file to complete the user's wallet registration on the platform.

### 3.3. Get Deposit Address 📍

Rust to the SDK's `pay-sdk-rust/target/debug/examples/` directory and specify the `UserOpenId` and `ChainIDs` fields in `config.yaml`.

Run the `get_wallet_addresses` or `get_wallet_addresses.exe` executable file.

### 3.4. Withdrawals 💸

Rust to the SDK's `pay-sdk-rust/target/debug/examples/` directory and specify the `UserOpenId`, `TokenId`, `Amount`, `AddressTo`, `SafeCheckCode`, and `CallbackUrl` fields in `config.yaml`.

Run the `user_withdraw_by_open_id` or `user_withdraw_by_open_id.exe` executable file.
