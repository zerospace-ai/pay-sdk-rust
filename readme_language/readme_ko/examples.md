# 예제 및 도구

이 문서는 두 부분으로 나뉩니다.
1. **시나리오 기반 코드 예제:** 실제 코드에서 API 호출 및 검증을 처리하는 방법을 보여줍니다.
2. **CLI 도구 가이드:** 빠른 테스트를 위해 SDK에 포함된 컴파일된 실행 파일을 사용하는 방법을 설명합니다.

---

## 1. 시나리오 기반 코드 예제

### 1.1 완전한 API 호출 및 응답 검증

다음 코드는 SDK를 사용하여 "사용자 생성" 요청을 빌드하고 HTTP 요청을 보내고 플랫폼에서 반환된 데이터의 서명 보안을 검증하는 방법을 보여줍니다.

```rust
use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. SDK 초기화
    let cfg = load_config("./config.yaml")?;
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 2. 요청 매개변수 및 서명 헤더 생성
    let open_id = "PT00001";
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // 3. HTTP 요청 전송
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
    
    // 4. 응답 서명 자동 검증
    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    if parsed.code != 1 {
        println!("요청 실패: {}", parsed.msg);
        return Ok(());
    }

    let map_data = rsa_utils::to_string_map(&body)?;
    let rsp_data: pay_sdk_rust::response_define::ResponseCreateUser = serde_json::from_str(&body)?;
    
    match sdk.verify_rsa_signature(&map_data, &rsp_data.sign) {
        Ok(_) => println!("✅ 요청 성공 및 검증 완료! 반환된 OpenId: {}", rsp_data.data.open_id),
        Err(e) => println!("❌ 서명 검증 실패: {:?}", e),
    }

    Ok(())
}
```


---

## 2. CLI 도구 사용 가이드

SDK는 각 API 엔드포인트를 빠르게 테스트하기 위한 CLI(명령줄 인터페이스) 바이너리 파일을 제공합니다.

### 2.1 실행 파일 컴파일

SDK 루트 디렉터리에서 `cargo build --examples` 명령을 실행하면 시스템이 `target/debug/examples/` 디렉터리에 각 기능에 대한 바이너리 실행 파일을 생성합니다.
* **Windows:** `.exe`로 끝나는 파일을 생성합니다(예: `create_user.exe`).
* **Mac/Linux:** 확장자 없는 파일을 생성합니다(예: `create_user`).

### 2.2 구성 파일 준비

도구를 실행하기 전에 구성된 `config.yaml` 파일이 실행 디렉터리(일반적으로 프로젝트 루트 또는 `target/debug/examples/`)에 있는지 확인하십시오.

### 2.3 엔드포인트 명령 테스트

#### 새 사용자 등록
1. `config.yaml`의 `UserOpenId` 필드를 수정합니다.
2. `cargo run --example create_user_client`를 실행합니다.
3. OpenId가 이미 등록된 경우 도구에서 오류를 반환합니다.

#### 지갑 등록
1. `config.yaml`에 `UserOpenId` 및 `ChainID`를 지정합니다.
2. `cargo run --example create_wallet`을 실행합니다.

#### 입금 주소 가져오기
1. `config.yaml`에 `UserOpenId`와 조회할 `ChainIDs`(예: "1,56")를 지정합니다.
2. `cargo run --example get_wallet_addresses`를 실행합니다.

#### 출금 신청
1. `config.yaml`에 다음을 지정합니다.
   * `UserOpenId`
   * `TokenId`
   * `Amount`
   * `AddressTo`
   * `SafeCheckCode` (고유한 주문 중복 방지 코드)
   * `CallbackUrl`
2. `cargo run --example user_withdraw_by_open_id`를 실행합니다.

#### 카셔 주문 생성
1. `config.yaml`에서 `OutOrderNo`, `OutUserId`(고정 주소 모드 필수), `TokenId`, `Quantity`, `NotifyUrl`, `OrderDetail`을 지정합니다.
2. `cargo run --example new_order`를 실행합니다.

#### 지갑 잔액 조회
1. `config.yaml`에서 `WalletAddress`, `ContractAddress`, `WalletBalanceChainId`를 지정합니다.
2. `cargo run --example wallet_balance`를 실행합니다.