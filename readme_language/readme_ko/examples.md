# 예시 📝

이 문서는 CryptoPay Rust SDK의 사용 예시를 제공합니다. 데모 실행, 키 생성 및 콜백 처리 등을 포함합니다.

## 1 SDK 인스턴스 객체 🛠️

### 1.1 필수 구성 ⚙️

1. 비즈니스 이름을 등록하고 `ApiKey`와 `ApiSecret`을 얻습니다;

2. 자신의 `RSA` 키 쌍을 생성합니다;

3. 플랫폼의 `RSA` 공개 키를 준비합니다;

### 1.2 서명 객체 생성 🔏

1. 구성 파일 `config.yaml`을 추가합니다.

```yaml
# 비즈니스 정보 구성
ApiKey: ""
ApiSecret: ""
# 플랫폼 공개 키
PlatformPubKey: ""
# 플랫폼 차단용 공개 키
PlatformRiskPubKey: ""
# 자신의 개인 키
RsaPrivateKey: ""
```

2. 구성 파일을 로드하고 API 객체를 생성합니다.

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

### 1.3 요청 데이터 생성 및 서명 ✍️

사용자 생성을 예로 들어 보겠습니다.

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

### 1.4 요청 채우기 및 시작 🚀

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

### 1.5 반환 데이터 검증 및 파싱 ✅

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

## 2. 실행 가능한 인터페이스 명령 생성

* 1. SDK 루트 디렉토리에서 `cargo build --example create_user_client` 명령을 실행하여 `pay-sdk-rust/target/debug/examples/` 디렉토리에 각 기능 명령의 바이너리 실행 파일을 생성합니다.

* 2. ".exe" 접미사가 있는 파일은 64비트 Windows 머신에서 실행됩니다; 접미사가 없는 파일은 Linux/Mac에서 실행됩니다. 예: create_user.exe 및 create_user 실행 파일.

* 3. 구성된 config.yaml 파일을 `pay-sdk-rust/target/debug/examples/` 디렉토리로 복사합니다.

## 3. 명령 호출 📞

### 3.1. 새 사용자 등록 🆕


SDK의 `pay-sdk-rust/target/debug/examples/` 디렉토리로 이동하여 그곳의 config.yaml 파일에서 UserOpenId 필드를 수정합니다.

create_user 또는 create_user.exe 실행 파일을 실행하여 플랫폼에 새 사용자를 등록합니다.

이미 등록된 새 UserOpenId를 등록하려고 하면 오류가 반환됩니다.


### 3.2. 지갑 등록 💼

SDK의 `pay-sdk-rust/target/debug/examples/` 디렉토리로 이동하여 `config.yaml` 파일에서 `UserOpenId` 및 `ChainID` 필드를 지정합니다.

`create_wallet` 또는 `create_wallet.exe` 실행 파일을 실행하여 플랫폼에서 사용자 지갑 등록을 완료합니다.

### 3.3. 입금 주소 가져오기 📍

SDK의 `pay-sdk-rust/target/debug/examples/` 디렉토리로 이동하여 `config.yaml`에서 `UserOpenId` 및 `ChainIDs` 필드를 지정합니다.

`get_wallet_addresses` 또는 `get_wallet_addresses.exe` 실행 파일을 실행합니다.

### 3.4. 출금 💸

SDK의 `pay-sdk-rust/target/debug/examples/` 디렉토리로 이동하여 `config.yaml`에서 `UserOpenId`, `TokenId`, `Amount`, `AddressTo`, `SafeCheckCode` 및 `CallbackUrl` 필드를 지정합니다.

`user_withdraw_by_open_id` 또는 `user_withdraw_by_open_id.exe` 실행 파일을 실행합니다.