# 示例代码与工具

本文档分为两部分：
1. **场景化代码示例：** 演示在实际代码中如何处理 API 的调用与验证。
2. **命令行工具指南：** 介绍如何使用 SDK 附带的示例程序进行快速测试。

---

## 1. 场景化代码示例

### 1.1 完整的 API 调用与响应验证

以下代码展示了如何利用 SDK 构建一个“创建用户”请求，发送 HTTP 请求，并对平台返回的数据签名进行安全验证。

```rust
use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 初始化 SDK (前提：config.yaml 已配置)
    let cfg = load_config("./config.yaml")?;
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 2. 生成请求参数和签名 Header
    let open_id = "PT00001";
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // 3. 发送 HTTP 请求
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
    
    // 4. 自动验证响应签名
    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    if parsed.code != 1 {
        println!("请求失败: {}", parsed.msg);
        return Ok(());
    }

    let map_data = rsa_utils::to_string_map(&body)?;
    let rsp_data: pay_sdk_rust::response_define::ResponseCreateUser = serde_json::from_str(&body)?;
    
    match sdk.verify_rsa_signature(&map_data, &rsp_data.sign) {
        Ok(_) => println!("✅ 请求成功并验证通过！返回的 OpenId: {}", rsp_data.data.open_id),
        Err(e) => println!("❌ 签名验证失败: {:?}", e),
    }

    Ok(())
}
```

---

## 2. 命令行工具使用指南

SDK 提供了快速测试各接口的示例命令行程序。

### 2.1 编译示例文件

在 SDK 根目录下执行以下命令编译示例：

```bash
cargo build --examples
```

编译后的二进制文件将位于 `target/debug/examples/` 目录下。
* **Windows:** 生成 `.exe` 结尾的文件（如 `create_user_client.exe`）。
* **Mac/Linux:** 生成无后缀的文件（如 `create_user_client`）。

### 2.2 准备配置文件

运行工具前，请确保将配置好的 `config.yaml` 文件放置在执行目录下（通常是项目根目录或 `target/debug/examples/`）。

### 2.3 测试各接口命令

#### 注册新用户
1. 在 `config.yaml` 中修改 `UserOpenId` 字段。
2. 运行 `cargo run --example create_user_client`。

#### 钱包注册
1. 在 `config.yaml` 中指定 `UserOpenId` 和 `ChainID`。
2. 运行 `cargo run --example create_wallet`。

#### 获取充值地址
1. 在 `config.yaml` 中指定 `UserOpenId` 和需要查询的 `ChainIDs` (例如 "1,56")。
2. 运行 `cargo run --example get_wallet_addresses`。

#### 申请提现
1. 在 `config.yaml` 中指定：
   * `UserOpenId`
   * `TokenId`
   * `Amount`
   * `AddressTo`
   * `SafeCheckCode` (唯一的订单防重码)
   * `CallbackUrl`
2. 运行 `cargo run --example user_withdraw_by_open_id`。

#### 创建收银台订单
1. 在 `config.yaml` 中指定 `OutOrderNo`, `OutUserId`（固定地址模式必填）, `TokenId`, `Quantity`, `NotifyUrl`, `OrderDetail`。
2. 运行 `cargo run --example new_order`。

#### 查询钱包余额
1. 在 `config.yaml` 中指定 `WalletAddress`, `ContractAddress`, `WalletBalanceChainId`。
2. 运行 `cargo run --example wallet_balance`。