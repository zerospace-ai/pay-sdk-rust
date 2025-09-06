# 例 📝

このドキュメントは、CryptoPay Rust SDKの使用例を提供します。Demoの実行、キーの生成、およびコールバックの処理を含みます。

## 1 SDKインスタンスオブジェクト 🛠️

### 1.1 必要な設定 ⚙️

1. ビジネス名を登録し、`ApiKey` と `ApiSecret` を取得します；

2. 独自の `RSA` キーペアを生成します；

3. プラットフォームの `RSA` 公開鍵を準備します；

### 1.2 署名オブジェクトの作成 🔏

1. 設定ファイル `config.yaml` を追加します。

```yaml
# ビジネス情報を設定
ApiKey: ""
ApiSecret: ""
# プラットフォーム公開鍵
PlatformPubKey: ""
# プラットフォームのブロック公開鍵
PlatformRiskPubKey: ""
# 独自の秘密鍵
RsaPrivateKey: ""
```

2. 設定ファイルをロードし、APIオブジェクトを作成します。

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

### 1.3 リクエストデータの作成と署名 ✍️

ユーザー作成を例にします。

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

### 1.4 リクエストの入力と開始 🚀

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

### 1.5 返却データの検証と解析 ✅

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

## 2. 実行可能なインターフェースコマンドの生成

* 1. SDKルートディレクトリで`cargo build --example create_user_client`コマンドを実行し、`pay-sdk-rust/target/debug/examples/`ディレクトリに各機能コマンドのバイナリ実行可能ファイルを生成します。

* 2. ".exe" サフィックスのファイルは64ビットWindowsマシンで実行されます；サフィックスなしのファイルはLinux/Macで実行されます。例えば、create_user.exe と create_user 実行可能ファイル。

* 3. 設定されたconfig.yamlファイルを`pay-sdk-rust/target/debug/examples/`ディレクトリにコピーします。

## 3. コマンドの呼び出し 📞

### 3.1. 新規ユーザーの登録 🆕


SDKの`pay-sdk-rust/target/debug/examples/`ディレクトリに移動し、そこにあるconfig.yamlファイルのUserOpenIdフィールドを変更します。

create_user または create_user.exe 実行可能ファイルを実行して、プラットフォームに新規ユーザーを登録します。

すでに登録されている新規UserOpenIdを登録しようとすると、エラーが返されます。


### 3.2. ウォレット登録 💼

SDKの`pay-sdk-rust/target/debug/examples/`ディレクトリに移動し、`config.yaml` ファイルで `UserOpenId` と `ChainID` フィールドを指定します。

`create_wallet` または `create_wallet.exe` 実行可能ファイルを実行して、プラットフォームでのユーザーのウォレット登録を完了します。

### 3.3. 入金アドレスの取得 📍

SDKの`pay-sdk-rust/target/debug/examples/`ディレクトリに移動し、`config.yaml` で `UserOpenId` と `ChainIDs` フィールドを指定します。

`get_wallet_addresses` または `get_wallet_addresses.exe` 実行可能ファイルを実行します。

### 3.4. 出金 💸

SDKの`pay-sdk-rust/target/debug/examples/`ディレクトリに移動し、`config.yaml` で `UserOpenId`、`TokenId`、`Amount`、`AddressTo`、`SafeCheckCode`、および `CallbackUrl` フィールドを指定します。

`user_withdraw_by_open_id` または `user_withdraw_by_open_id.exe` 実行可能ファイルを実行します。