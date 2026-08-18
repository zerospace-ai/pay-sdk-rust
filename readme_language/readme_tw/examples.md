# 示例代碼與工具

本文檔分為兩部分：
1. **場景化代碼示例：** 演示在實際代碼中如何處理 API 的調用與驗證。
2. **命令行工具指南：** 介紹如何使用 SDK 附帶的編譯後執行文件進行快速測試。

---

## 1. 場景化代碼示例

### 1.1 完整的 API 調用與響應驗證

以下代碼展示了如何利用 SDK 構建一個“創建用戶”請求，發送 HTTP 請求，並對平台返回的數據簽名進行安全驗證。

```rust
use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 初始化 SDK
    let cfg = load_config("./config.yaml")?;
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 2. 生成請求參數和簽名 Header
    let open_id = "PT00001";
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // 3. 發送 HTTP 請求
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
    
    // 4. 自動驗證響應簽名
    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    if parsed.code != 1 {
        println!("請求失敗: {}", parsed.msg);
        return Ok(());
    }

    let map_data = rsa_utils::to_string_map(&body)?;
    let rsp_data: pay_sdk_rust::response_define::ResponseCreateUser = serde_json::from_str(&body)?;
    
    match sdk.verify_rsa_signature(&map_data, &rsp_data.sign) {
        Ok(_) => println!("✅ ✅ 請求成功並驗證通過！返回的 OpenId: {}", rsp_data.data.open_id),
        Err(e) => println!("❌ ❌ 簽名驗證失敗: {:?}", e),
    }

    Ok(())
}
```


---

## 2. 命令行工具使用指南

SDK 提供了快速測試各接口的命令行工具二進制文件（CLI）。

### 2.1 編譯執行文件

在 SDK 根目錄下執行 `cargo build --examples` 命令，系統將在 `target/debug/examples/` 目錄下生成各功能的二進制可執行文件。
* **Windows:** 生成 `.exe` 結尾的文件（如 `create_user.exe`）。
* **Mac/Linux:** 生成無後綴的文件（如 `create_user`）。

### 2.2 準備配置文件

運行工具前，請確保將配置好的 `config.yaml` 文件放置在執行目錄下（通常是項目根目錄或 `target/debug/examples/`）。

### 2.3 測試各接口命令

#### 註冊新用戶
1. 在 `config.yaml` 中修改 `UserOpenId` 字段。
2. 運行 `cargo run --example create_user_client`。
3. 若該 OpenId 已註冊，工具將返回錯誤。

#### 錢包註冊
1. 在 `config.yaml` 中指定 `UserOpenId` 和 `ChainID`。
2. 運行 `cargo run --example create_wallet`。

#### 獲取充值地址
1. 在 `config.yaml` 中指定 `UserOpenId` 和需要查詢的 `ChainIDs` (例如 "1,56")。
2. 運行 `cargo run --example get_wallet_addresses`。

#### 申請提現
1. 在 `config.yaml` 中指定：
   * `UserOpenId`
   * `TokenId`
   * `Amount`
   * `AddressTo`
   * `SafeCheckCode` (唯一的訂單防重碼)
   * `CallbackUrl`
2. 運行 `cargo run --example user_withdraw_by_open_id`。

#### 創建收銀台訂單
1. 在 `config.yaml` 中指定 `OutOrderNo`, `TokenId`, `Quantity`, `NotifyUrl`。
2. 運行 `cargo run --example new_order`。

#### 查詢錢包餘額
1. 在 `config.yaml` 中指定 `WalletAddress`, `ContractAddress`, `WalletBalanceChainId`。
2. 運行 `cargo run --example wallet_balance`。