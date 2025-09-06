# 示例 📝

本文档提供了 CryptoPay Rust SDK 的使用示例，包括 Demo 运行、密钥生成和回调处理。

## 1 SDK 实例对象 🛠️

### 1.1 所需配置 ⚙️

1. 注册您的业务名称并获取 `ApiKey` 和 `ApiSecret`；

2. 生成您自己的 `RSA` 密钥对；

3. 准备平台的 `RSA` 公钥；

### 1.2 创建签名对象 🔏

1. 添加配置文件 `config.yaml`。

```yaml
# 配置业务信息
ApiKey: ""
ApiSecret: ""
# 平台公钥
PlatformPubKey: ""
# 用于阻塞平台的公钥
PlatformRiskPubKey: ""
# 您自己的私钥
RsaPrivateKey: ""
```

2. 加载配置文件并创建 API 对象。

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

### 1.3 创建并签名请求数据。 ✍️

以用户创建为例。

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

### 1.4 填充并发起请求 🚀

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

### 1.5 验证解析返回数据 ✅

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

## 2. 生成可执行接口命令

* 1. 在 SDK 根目录执行 `cargo build --example create_user_client` 命令，在 `pay-sdk-rust/target/debug/examples/` 目录生成每个功能命令的二进制可执行文件。

* 2. 带有 ".exe" 后缀的文件在 64 位 Windows 机器上运行；没有 ".exe" 后缀的文件在 Linux/Mac 上运行。例如，create_user.exe 和 create_user 可执行文件。

* 3. 将配置好的 config.yaml 文件复制到 pay-sdk-rust/target/debug/examples/ 目录。

## 3. 调用命令 📞

### 3.1. 注册新用户 🆕


转到 SDK 的 `pay-sdk-rust/target/debug/examples/` 目录，并在其中的 config.yaml 文件中修改 UserOpenId 字段。

运行 create_user 或 create_user.exe 可执行文件，在平台上注册新用户。

如果尝试注册已注册的新 UserOpenId，将返回错误。


### 3.2. 钱包注册 💼

转到 SDK 的 bin 目录，并在 config.yaml 文件中指定 `UserOpenId` 和 `ChainID` 字段。

运行 `create_wallet` 或 `create_wallet.exe` 可执行文件，完成用户在平台上的钱包注册。

### 3.3. 获取充值地址 📍

转到 SDK 的 bin 目录，并在 config.yaml 中指定 `UserOpenId` 和 `ChainIDs` 字段。

运行 `get_wallet_addresses` 或 `get_wallet_addresses.exe` 可执行文件。

### 3.4. 提现 💸

转到 SDK 的 bin 目录，并在 config.yaml 中指定 `UserOpenId`、`TokenId`、`Amount`、`AddressTo`、`SafeCheckCode` 和 `CallbackUrl` 字段。

运行 `user_withdraw_by_open_id` 或 `user_withdraw_by_open_id.exe` 可执行文件。