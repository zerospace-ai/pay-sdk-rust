# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## Welcome to CryptoPay Rust SDK

CryptoPay Rust SDK is a professional cryptocurrency service SDK implemented in Rust, providing features such as user registration, wallet generation, deposit callback notification, and withdrawal.
It has been widely used and proven to be secure, stable, and easily extensible.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
pay-sdk-rust = { git = "https://github.com/zerospace-ai/pay-sdk-rust.git" }
```

> **Note:** Compilation requires Rust 1.91+.

## Quick Start

### 1. Prepare Configuration

Before using the SDK, you need to prepare the `config.yaml` configuration file, which contains the merchant's authentication information and public/private keys:

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

> **💡 Tip:** For details on how to generate the merchant's own RSA key pair (`RsaPrivateKey`) and the detailed authentication and security mechanisms, please read [Authentication and Security (authentication.md)](./authentication.md).

### 2. Initialize SDK and Send Request

Here is a complete example demonstrating how to initialize the SDK instance and call the "Create New User" API:

```rust
use pay_sdk_rust::sdk::{Sdk, SDKConfig, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load configuration
    let cfg = load_config("config.yaml")?;

    // 2. Create SDK instance
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 3. Call API: Build the create new user request
    let open_id = "PT00001"; // User's unique identifier
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // (Network request execution and response parsing are omitted here, please refer to examples.md for full code)
    println!("Successfully built CreateUser request!");
    println!("Body: {}", req_body);
    println!("Headers prepared: timestamp={}, sign={}, clientSign={}", timestamp, sign, client_sign);

    Ok(())
}
```

## Key Concepts and Navigation

To better use this SDK, we recommend reading the remaining documents in the following order:

1. **[Authentication and Security (authentication.md)](./authentication.md)**: Learn how to generate RSA key pairs and the signature verification mechanism between the SDK and the platform.
2. **[API Reference (api-reference.md)](./api-reference.md)**: Contains detailed instructions for all supported API endpoints (e.g., wallet creation, withdrawal) and callback notification formats.
3. **[Examples and Tools (examples.md)](./examples.md)**: View more complex scenario-based code examples and instructions on using the SDK's built-in CLI tools.
4. **[Appendix (appendix.md)](./appendix.md)**: Static dictionary information such as supported ChainIDs, token types, and contract addresses.

## Contact

If you have any questions, please contact the service provider:  
Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)
