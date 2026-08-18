# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## CryptoPay Rust SDK へようこそ

CryptoPay Rust SDK は、Rust で実装されたプロフェッショナルな暗号通貨サービス SDK であり、ユーザー登録、ウォレット生成、入金コールバック通知、出金などの機能を提供します。
これは広く使用されており、安全で安定しており、簡単に拡張できることが証明されています。

## インストール

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
pay-sdk-rust = { git = "https://github.com/zerospace-ai/pay-sdk-rust.git" }
```

> **注意:** コンパイルには Rust 1.91+ が必要です。

## クイックスタート

### 1. 設定の準備

SDK を使用する前に、加盟店の認証情報と公開鍵/秘密鍵を含む `config.yaml` 設定ファイルを準備する必要があります。

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

> **💡 ヒント:** 加盟店自身の RSA キーペア (`RsaPrivateKey`) の生成方法や、詳細な認証とセキュリティのメカニズムについては、[認証とセキュリティ (authentication.md)](./authentication.md) をお読みください。

### 2. SDK の初期化とリクエストの送信

以下は、SDK インスタンスを初期化し、「新規ユーザー作成」API を呼び出す方法を示す完全な例です。

```rust
use pay_sdk_rust::sdk::{Sdk, SDKConfig, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 設定の読み込み
    let cfg = load_config("config.yaml")?;

    // 2. SDKインスタンスの作成
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 3. API呼び出し：新規ユーザー作成リクエストの構築
    let open_id = "PT00001"; // ユーザーの一意の識別子
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // (ネットワークリクエストの実行とレスポンスの解析はここでは省略されています)
    println!("CreateUserリクエストの構築に成功しました！");
    println!("Body: {}", req_body);
    println!("Headers: timestamp={}, sign={}, clientSign={}", timestamp, sign, client_sign);

    Ok(())
}
```

## 重要な概念とナビゲーション

この SDK をより効果的に使用するために、残りのドキュメントを次の順序で読むことをお勧めします。

1. **[認証とセキュリティ (authentication.md)](./authentication.md)**: RSA キーペアの生成方法と、SDK とプラットフォーム間の署名検証メカニズムについて学習します。
2. **[API リファレンス (api-reference.md)](./api-reference.md)**: サポートされているすべての API エンドポイント (ウォレット作成、出金など) と Webhook 形式の詳細な手順が含まれています。
3. **[例とツール (examples.md)](./examples.md)**: より複雑なシナリオベースのコード例と、SDK 組み込みの CLI ツールの使用手順を確認します。
4. **[付録 (appendix.md)](./appendix.md)**: サポートされている ChainID、トークンタイプ、コントラクトアドレスなどの静的ディクショナリ情報。

## お問い合わせ

ご不明な点がございましたら、サービスプロバイダーにお問い合わせください。  
Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)