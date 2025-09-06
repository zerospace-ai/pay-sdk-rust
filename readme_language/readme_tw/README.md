# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## 🌟 歡迎使用 CryptoPay Rust SDK

CryptoPay Rust SDK 是一個使用 Rust 實現的專業加密貨幣服務 SDK，提供用戶註冊、錢包生成、充值回調通知、提現等功能。

經過長期使用，已證明其安全、穩定且易於擴展。

## ⚙️ 安裝

```bash
git clone github.com/zerospace-ai/pay-sdk-rust.git
```

注意：編譯需要 rustc 1.91.0 🛠️。
## 🚀 快速入門
### 1. ⚙️ config.yaml

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

欄位說明：

• 🔑 ApiKey / ApiSecret:

註冊商家帳戶時由平台分配，用於 API 請求認證 ✅。

• 🛡️ PlatformPubKey / PlatformRiskPubKey:

平台提供的公鑰，用於驗證平台返回的資料或回調簽名，確保資訊來源可靠。PlatformRiskPubKey 主要用於風險控制相關事件驗證 ⚠️。

• 🗝️ RsaPrivateKey:

商家生成的 RSA 私鑰，用於簽署請求，確保請求內容未被篡改。重要注意：私鑰必須保密 🔒，勿洩露 🚫。

### 2. 生成 RSA 密鑰對 🔐

使用 RSA 密鑰對簽署請求以確保資料安全。以下說明在不同作業系統上如何生成密鑰對並提取密鑰字串。

#### 2.1 使用 OpenSSL 生成密鑰對

```bash
# 生成 2048 位私鑰
openssl genrsa -out rsa_private_key.pem 2048

# 從私鑰生成公鑰
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

> 💡 提示：生成的公鑰需要移除開頭和結尾 -----BEGIN PUBLIC KEY----- / -----END PUBLIC KEY-----，移除換行，轉換為單行字串並提交給平台。
> 
> 提取密鑰字串並將公鑰提交給平台 📤。
>
>在 Mac 和 Windows 上生成 RSA 密鑰對的命令與 Linux 相同。

#### 2.2 提取密鑰字串 🔑

在 Mac/Linux 或 Git Bash/WSL/Cygwin 上：

```bash
# 提取私鑰字串
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# 提取公鑰字串
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

Windows

PowerShell 提取私鑰和公鑰字串：

```powershell
# 私鑰
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# 公鑰
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> ⚠️ 注意：生成的私鑰必須安全保管，勿洩露。


### 🛠️ 3. 建立 SDK 實例

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

## 🔑 關鍵概念

- 🆔 **OpenId**：用戶的唯一識別碼，例如 "HASH1756194148"。
- 🔐 **RSA 密鑰**：用於簽署和驗證請求以確保資料安全。
- ✍️ **API 簽名**：使用 MD5 和 RSA 演算法簽署請求，確保未被篡改。

有關詳細 API 說明，請參閱 [🧩 api-reference.md](./api-reference.md) 和 [🧩 examples.md](./examples.md)。

有關認證與安全，請參閱 [🧩 authentication.md](./authentication.md)

## 📎 附錄

有關更多詳細參考，請查看 [附錄](./appendix.md) 文件，其中包括以下內容：

- [🧩 ChainID 清單](./appendix.md#-chainid-清單)
- [🏷️ Token 類型](./appendix.md#-token-類型)
- [🌐 公共資訊](./appendix.md#-公共信息)
- [🔰 Token 基本資訊](./appendix.md#-token-基本信息)

> 💡 **提示**：附錄提供了支援的鏈資訊、token 類型和公共 token 資料，便於開發人員整合和使用 SDK。

## 📞 聯絡

如果您有任何問題，請聯絡服務提供商。  
💬 Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)