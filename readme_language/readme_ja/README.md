# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## 🌟 CryptoPay Rust SDKへようこそ

CryptoPay Rust SDKは、Golangで実装されたプロフェッショナルな暗号通貨サービスSDKで、ユーザー登録、ウォレット生成、入金コールバック通知、出金などの機能を提供します。

長期使用を通じて、安全で安定し、拡張しやすいことが証明されています。

## ⚙️ インストール

```bash
git clone github.com/zerospace-ai/pay-sdk-rust.git
```

注: コンパイルにはGo 1.18+が必要です 🛠️。
## 🚀 クイックスタート
### 1. ⚙️ config.yaml

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

フィールドの説明:

• 🔑 ApiKey / ApiSecret:

マーチャントアカウント登録時にプラットフォームから割り当てられ、APIリクエスト認証に使用されます ✅。

• 🛡️ PlatformPubKey / PlatformRiskPubKey:

プラットフォームから提供される公開鍵で、プラットフォームから返されるデータやコールバックの署名を検証し、情報の信頼性を確保します。PlatformRiskPubKeyは主にリスクコントロール関連のイベント検証に使用されます ⚠️。

• 🗝️ RsaPrivateKey:

マーチャントが生成したRSA秘密鍵で、リクエストに署名し、リクエスト内容が改ざんされていないことを保証します。重要な注意: 秘密鍵は機密に保つ必要があります 🔒、公開しないでください 🚫。

### 2. RSAキーペアの生成 🔐

リクエストに署名するためにRSAキーペアを使用してデータセキュリティを確保します。以下では、異なるオペレーティングシステムでキーペアを生成し、キー文字列を抽出する方法を説明します。

#### 2.1 OpenSSLを使用してキーペアを生成

```bash
# 2048ビットの秘密鍵を生成
openssl genrsa -out rsa_private_key.pem 2048

# 秘密鍵から公開鍵を生成
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

> 💡 Tip: 生成された公開鍵は、始めと終わりの -----BEGIN PUBLIC KEY----- / -----END PUBLIC KEY----- を削除し、改行を削除して1行の文字列に変換し、プラットフォームに提出します。
> 
> キー文字列を抽出し、公開鍵をプラットフォームに提出します 📤。
>
>MacとWindowsでのRSAキーペア生成コマンドはLinuxと同じです。

#### 2.2 キー文字列の抽出 🔑

Mac/LinuxまたはGit Bash/WSL/Cygwinで:

```bash
# 秘密鍵文字列を抽出
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# 公開鍵文字列を抽出
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

Windows

PowerShellで秘密鍵と公開鍵文字列を抽出:

```powershell
# 秘密鍵
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# 公開鍵
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> ⚠️ Note: 生成された秘密鍵は安全に保管し、漏洩しないようにしてください。


### 🛠️ 3. SDKインスタンスの作成

```rust
    let cfg = load_config("./config.yaml")?;

    let sdk_config = SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    };

    let sdk = Sdk::new(sdk_config);
    
```

## 🔑 キーコンセプト

- 🆔 **OpenId**: ユーザーの一意の識別子、例えば "HASH1756194148"。
- 🔐 **RSA Key**: リクエストの署名と検証に使用され、データセキュリティを確保します。
- ✍️ **API Signature**: MD5とRSAアルゴリズムを使用してリクエストに署名し、改ざんされていないことを保証します。

詳細なAPI説明については、[🧩 api-reference.md](./api-reference.md) と [🧩 examples.md](./examples.md) を参照してください。

認証とセキュリティについては、[🧩 authentication.md](./authentication.md) を参照してください

## 📎 付録

より詳細な参照については、[付録](./appendix.md) ドキュメントを確認してください。以下の内容を含みます:

- [🧩 ChainID List](./appendix.md#-chainid-リスト)
- [🏷️ Token Types](./appendix.md#-トークンタイプ)
- [🌐 Public Information](./appendix.md#-公開情報)
- [🔰 Token Basic Information](./appendix.md#-トークン基本情報)

> 💡 **Tip**: 付録では、サポートされているチェーン情報、トークンタイプ、および公開トークンデータを提供し、開発者がSDKを統合して使用しやすくします。

## 📞 連絡先

質問がある場合は、サービスプロバイダーに連絡してください。  
💬 Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)