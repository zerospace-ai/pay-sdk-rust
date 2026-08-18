# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## 欢迎使用 CryptoPay Rust SDK

CryptoPay Rust SDK 是一个专业的加密货币服务 SDK，使用 Rust 实现，提供用户注册、钱包生成、充值回调通知、提现等功能。
它经过长期使用，已被证明安全、稳定且易于扩展。

## 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
pay-sdk-rust = { git = "https://github.com/zerospace-ai/pay-sdk-rust.git" }
```

> **注意：** 编译需要 Rust 1.91+。

## 快速入门

### 1. 准备配置

在使用 SDK 之前，您需要准备 `config.yaml` 配置文件，其中包含商户的认证信息和公私钥：

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

> **💡 提示：** 关于如何生成商户自己的 RSA 密钥对（RsaPrivateKey），以及详细的认证与安全机制，请务必阅读 [认证与安全 (authentication.md)](./authentication.md)。

### 2. 初始化 SDK 并发送请求

以下是一个完整的示例，展示如何初始化 SDK 实例并调用“注册新用户”接口：

```rust
use pay_sdk_rust::sdk::{Sdk, SDKConfig, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载配置
    let cfg = load_config("config.yaml")?;

    // 2. 创建 SDK 实例
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 3. 调用 API：构建注册新用户请求
    let open_id = "PT00001"; // 用户的唯一标识符
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // （此处省略网络请求发起与返回数据解析过程，详细完整代码请参考 examples.md）
    println!("成功构建 CreateUser 请求!");
    println!("Body: {}", req_body);
    println!("Headers 准备完毕: timestamp={}, sign={}, clientSign={}", timestamp, sign, client_sign);

    Ok(())
}
```

## 关键概念与导航

为了更好地使用本 SDK，建议您按照以下顺序阅读其余文档：

1. **[认证与安全 (authentication.md)](./authentication.md)**：学习如何生成 RSA 密钥对，以及 SDK 与平台之间的签名验证机制。
2. **[API 参考 (api-reference.md)](./api-reference.md)**：包含所有受支持的 API 接口的详细说明（如创建钱包、提现等）和回调通知格式。
3. **[示例代码与工具 (examples.md)](./examples.md)**：查看更复杂的场景化示例代码，以及 SDK 内内置命令行工具的使用方法。
4. **[附录 (appendix.md)](./appendix.md)**：支持的 ChainID、代币类型、合约地址等静态字典信息。

## 联系方式

如果您有任何问题，请联系服务提供商：  
Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)