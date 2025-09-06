# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## 🌟 欢迎使用 CryptoPay Rust SDK

CryptoPay Rust SDK 是一个专业的加密货币服务 SDK，使用 Rust 实现，提供用户注册、钱包生成、充值回调通知、提现等功能。

它经过长期使用，已被证明安全、稳定且易于扩展。

## ⚙️ 安装

```bash
git clone github.com/zerospace-ai/pay-sdk-rust.git
```

注意：编译需要 rustc 1.91.0 🛠️。

## 🚀 快速入门
### 1. ⚙️ config.yaml

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

字段描述：

• 🔑 ApiKey / ApiSecret：

在注册商户账户时由平台分配，用于 API 请求认证 ✅。

• 🛡️ PlatformPubKey / PlatformRiskPubKey：

平台提供的公钥，用于验证平台返回的数据或回调签名，确保信息来源可靠。PlatformRiskPubKey 主要用于风险控制相关事件验证 ⚠️。

• 🗝️ RsaPrivateKey：

商户生成的 RSA 私钥，用于对请求签名，确保请求内容未被篡改。重要提示：私钥必须保密 🔒，不要泄露 🚫。

### 2. 生成 RSA 密钥对 🔐

使用 RSA 密钥对对请求进行签名，确保数据安全。以下描述了在不同操作系统上生成密钥对并提取密钥字符串的方法。

#### 2.1 使用 OpenSSL 生成密钥对

```bash
# Generate 2048-bit private key
openssl genrsa -out rsa_private_key.pem 2048

# Generate public key from private key
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

> 💡 Tip: 生成的公钥需要去掉开头和结尾的 -----BEGIN PUBLIC KEY----- / -----END PUBLIC KEY-----，去掉换行符，转换为单行字符串后提交到平台。
> 
> 提取关键字符串，并将公钥提交到平台。 📤。
>
> 在 Mac 和 Windows 上生成 RSA 密钥对的命令与 Linux 相同。

#### 2.2 提取密钥字符串 🔑

On Mac/Linux or Git Bash/WSL/Cygwin:

```bash
# Extract private key string
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# Extract public key string
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

Windows

PowerShell 提取私钥和公钥字符串：

```powershell
# Private key
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# Public key
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> ⚠️ 注意：生成的私钥必须妥善保管，严禁泄露。


### 🛠️ 3. 创建 SDK 实例

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

## 🔑 关键概念

- 🆔 **OpenId**：用户的唯一标识符，例如 "HASH1756194148"。

- 🔐 **RSA Key**：用于签名和验证请求，以确保数据安全。

- ✍️ **API Signature**：使用 MD5 和 RSA 算法对请求进行签名，确保未被篡改。

有关详细的 API 描述，请参阅 [🧩 api-reference.md](./api-reference.md) 和 [🧩 examples.md](./examples.md)。

有关认证与安全，请参阅 [🧩 authentication.md](./authentication.md)

## 📎 附录

有关更多详细参考，请查看 [附录](./appendix.md) 文档，其中包括以下内容：

- [🧩 ChainID 列表](./appendix.md#-chainid-列表)

- [🏷️ Token 类型](./appendix.md#-token-类型)

- [🌐 公共信息](./appendix.md#-公共信息)

- [🔰 Token 基本信息](./appendix.md#-token-基本信息)

> 💡 **Tip**: 附录提供了支持的链信息、代币类型和公开的代币数据，使开发者更容易集成和使用 SDK。

## 📞 联系方式

如果您有任何问题，请联系服务提供商。  
💬 Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)