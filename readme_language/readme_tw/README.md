# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## 歡迎使用 CryptoPay Rust SDK

CryptoPay Rust SDK 是一個專業的加密貨幣服務 SDK，使用 Rust 實現，提供用戶註冊、錢包生成、充值回調通知、提現等功能。
它經過長期使用，已被證明安全、穩定且易於擴展。

## 安裝

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
pay-sdk-rust = { git = "https://github.com/zerospace-ai/pay-sdk-rust.git" }
```

> **注意：** 編譯需要 Rust 1.91+。

## 快速入門

### 1. 準備配置

在使用 SDK 之前，您需要準備 `config.yaml` 配置文件，其中包含商戶的認證信息和公私鑰：

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

> **💡 提示：** 關於如何生成商戶自己的 RSA 密鑰對（RsaPrivateKey），以及詳細的認證與安全機制，請務必閱讀 [認證與安全 (authentication.md)](./authentication.md)。

### 2. 初始化 SDK 並發送請求

以下是一個完整的示例，展示如何初始化 SDK 實例並調用“註冊新用戶”接口：

```rust
use pay_sdk_rust::sdk::{Sdk, SDKConfig, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 載入配置
    let cfg = load_config("config.yaml")?;

    // 2. 建立 SDK 實例
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 3. 呼叫 API：構建註冊新用戶請求
    let open_id = "PT00001"; // 用戶的唯一識別碼
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // (（此處省略網絡請求發起與返回數據解析過程）)
    println!("成功構建 CreateUser 請求!");
    println!("Body: {}", req_body);
    println!("Headers: timestamp={}, sign={}, clientSign={}", timestamp, sign, client_sign);

    Ok(())
}
```

## 關鍵概念與導航

為了更好地使用本 SDK，建議您按照以下順序閱讀其餘文檔：

1. **[認證與安全 (authentication.md)](./authentication.md)**：學習如何生成 RSA 密鑰對，以及 SDK 與平台之間的簽名驗證機制。
2. **[API 參考 (api-reference.md)](./api-reference.md)**：包含所有受支持的 API 接口的詳細說明（如創建錢包、提現等）和回調通知格式。
3. **[示例代碼與工具 (examples.md)](./examples.md)**：查看更複雜的場景化示例代碼，以及 SDK 內置命令行工具的使用方法。
4. **[附錄 (appendix.md)](./appendix.md)**：支持的 ChainID、代幣類型、合約地址等靜態字典信息。

## 聯繫方式

如果您有任何問題，請聯繫服務提供商：  
Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)