# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## CryptoPay Rust SDK에 오신 것을 환영합니다

CryptoPay Rust SDK는 사용자 등록, 지갑 생성, 입금 콜백 알림 및 출금과 같은 기능을 제공하는 Rust으로 구현된 전문 암호화폐 서비스 SDK입니다.
널리 사용되어 왔으며 안전하고 안정적이며 쉽게 확장할 수 있음이 입증되었습니다.

## 설치

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
pay-sdk-rust = { git = "https://github.com/zerospace-ai/pay-sdk-rust.git" }
```

> **참고:** 컴파일하려면 Rust 1.91+가 필요합니다.

## 빠른 시작

### 1. 구성 준비

SDK를 사용하기 전에 가맹점의 인증 정보와 퍼블릭/프라이빗 키가 포함된 `config.yaml` 구성 파일을 준비해야 합니다.

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

> **💡 팁:** 가맹점 자체 RSA 키 쌍(`RsaPrivateKey`)을 생성하는 방법과 자세한 인증 및 보안 메커니즘에 대한 자세한 내용은 [인증 및 보안(authentication.md)](./authentication.md)을 참조하십시오.

### 2. SDK 초기화 및 요청 전송

다음은 SDK 인스턴스를 초기화하고 "새 사용자 생성" API를 호출하는 방법을 보여주는 전체 예제입니다.

```rust
use pay_sdk_rust::sdk::{Sdk, SDKConfig, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 설정 로드
    let cfg = load_config("config.yaml")?;

    // 2. SDK 인스턴스 생성
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 3. API 호출: 새 사용자 생성 요청 구축
    let open_id = "PT00001"; // 사용자의 고유 식별자
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // (네트워크 요청 실행 및 응답 파싱은 여기에서 생략되었습니다)
    println!("CreateUser 요청 구축 성공!");
    println!("Body: {}", req_body);
    println!("Headers: timestamp={}, sign={}, clientSign={}", timestamp, sign, client_sign);

    Ok(())
}
```

## 핵심 개념 및 탐색

이 SDK를 더 잘 사용하려면 다음 순서로 나머지 문서를 읽는 것이 좋습니다.

1. **[인증 및 보안(authentication.md)](./authentication.md)**: RSA 키 쌍을 생성하는 방법과 SDK와 플랫폼 간의 서명 검증 메커니즘에 대해 알아봅니다.
2. **[API 참조(api-reference.md)](./api-reference.md)**: 지원되는 모든 API 엔드포인트(예: 지갑 생성, 출금) 및 Webhook 형식에 대한 자세한 지침이 포함되어 있습니다.
3. **[예제 및 도구(examples.md)](./examples.md)**: 보다 복잡한 시나리오 기반 코드 예제와 SDK 내장 CLI 도구 사용 지침을 확인합니다.
4. **[부록(appendix.md)](./appendix.md)**: 지원되는 ChainID, 토큰 유형 및 컨트랙트 주소와 같은 정적 사전 정보입니다.

## 연락처

질문이 있으시면 서비스 제공업체에 문의하십시오.  
Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)