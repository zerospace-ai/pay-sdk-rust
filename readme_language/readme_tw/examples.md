# 範例 📝

本文件提供 CryptoPay Rust SDK 的使用範例，包括 Demo 運行、金鑰生成和回調處理。

## 1 SDK 實例物件 🛠️

### 1.1 所需配置 ⚙️

1. 註冊您的業務名稱並獲取 `ApiKey` 和 `ApiSecret`；

2. 生成您自己的 `RSA` 金鑰對；

3. 準備平台的 `RSA` 公鑰；

### 1.2 創建簽名物件 🔏

1. 添加配置文件 `config.yaml`。

```yaml
# 配置業務信息
ApiKey: ""
ApiSecret: ""
# 平台公鑰
PlatformPubKey: ""
# 用於封鎖平台的公鑰
PlatformRiskPubKey: ""
# 您自己的私鑰
RsaPrivateKey: ""
```

2. 加載配置文件並創建 API 物件。

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

### 1.3 創建並簽名請求數據。 ✍️

讓我們以用戶創建為例。

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

### 1.4 填充並發起請求 🚀

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

### 1.5 驗證解析返回數據 ✅

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

## 2. 生成可執行接口命令

* 1. 在 SDK 根目錄中執行 `cargo build --example create_user_client` 命令，在 `pay-sdk-rust/target/debug/examples/` 目錄中生成每個功能命令的二進制可執行文件。

* 2. 帶有 ".exe" 後綴的文件在 64 位 Windows 機器上運行；不帶 ".exe" 後綴的文件在 Linux/Mac 上運行。例如，create_user.exe 和 create_user 可執行文件。

* 3. 將配置好的 config.yaml 文件複製到 `pay-sdk-rust/target/debug/examples/` 目錄。

## 3. 調用命令 📞

### 3.1. 註冊新用戶 🆕


前往 SDK 的 `pay-sdk-rust/target/debug/examples/` 目錄，並在其中的 config.yaml 文件中修改 UserOpenId 字段。

運行 create_user 或 create_user.exe 可執行文件，在平台上註冊新用戶。

如果您嘗試註冊已註冊的 UserOpenId，將返回錯誤。


### 3.2. 錢包註冊 💼

前往 SDK 的 `pay-sdk-rust/target/debug/examples/` 目錄，並在 config.yaml 文件中指定 `UserOpenId` 和 `ChainID` 字段。

運行 `create_wallet` 或 `create_wallet.exe` 可執行文件，在平台上完成用戶的錢包註冊。

### 3.3. 獲取充值地址 📍

前往 SDK 的 `pay-sdk-rust/target/debug/examples/` 目錄，並在 config.yaml 中指定 `UserOpenId` 和 `ChainIDs` 字段。

運行 `get_wallet_addresses` 或 `get_wallet_addresses.exe` 可執行文件。

### 3.4. 提現 💸

前往 SDK 的 `pay-sdk-rust/target/debug/examples/` 目錄，並在 config.yaml 中指定 `UserOpenId`、`TokenId`、`Amount`、`AddressTo`、`SafeCheckCode` 和 `CallbackUrl` 字段。

運行 `user_withdraw_by_open_id` 或 `user_withdraw_by_open_id.exe` 可執行文件。