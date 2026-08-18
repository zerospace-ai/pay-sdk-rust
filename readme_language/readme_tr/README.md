# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## CryptoPay Rust SDK'ya Hoş Geldiniz

CryptoPay Rust SDK, Rust ile uygulanan ve kullanıcı kaydı, cüzdan oluşturma, para yatırma geri çağırma (callback) bildirimleri ve para çekme gibi işlevler sağlayan profesyonel bir kripto para hizmeti SDK'sıdır.
Yaygın olarak kullanılmış olup güvenli, kararlı ve kolayca genişletilebilir olduğu kanıtlanmıştır.

## Kurulum

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
pay-sdk-rust = { git = "https://github.com/zerospace-ai/pay-sdk-rust.git" }
```

> **Not:** Derleme için Rust 1.91+ gerekir.

## Hızlı Başlangıç

### 1. Yapılandırmayı Hazırlama

SDK'yı kullanmadan önce, tüccarın kimlik doğrulama bilgilerini ve genel/özel anahtarlarını içeren `config.yaml` yapılandırma dosyasını hazırlamanız gerekir:

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

> **💡 İpucu:** Tüccarın kendi RSA anahtar çiftini (`RsaPrivateKey`) nasıl oluşturacağı ve ayrıntılı kimlik doğrulama ile güvenlik mekanizmaları hakkında ayrıntılar için lütfen [Kimlik Doğrulama ve Güvenlik (authentication.md)](./authentication.md) bölümünü okuyun.

### 2. SDK'yı Başlatma ve İstek Gönderme

Aşağıda, SDK örneğini nasıl başlatacağınızı ve "Yeni Kullanıcı Oluştur" API'sini nasıl çağıracağınızı gösteren eksiksiz bir örnek bulunmaktadır:

```rust
use pay_sdk_rust::sdk::{Sdk, SDKConfig, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Yapılandırmayı yükle
    let cfg = load_config("config.yaml")?;

    // 2. SDK örneği oluştur
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 3. API Çağır: Yeni kullanıcı oluşturma isteği oluştur
    let open_id = "PT00001"; // Kullanıcının benzersiz kimliği
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // (Ağ isteği yürütme ve yanıt ayrıştırma burada atlanmıştır)
    println!("CreateUser isteği başarıyla oluşturuldu!");
    println!("Body: {}", req_body);
    println!("Headers: timestamp={}, sign={}, clientSign={}", timestamp, sign, client_sign);

    Ok(())
}
```

## Temel Kavramlar ve Gezinme

Bu SDK'yı daha iyi kullanabilmek için geri kalan belgeleri aşağıdaki sırayla okumanızı öneririz:

1. **[Kimlik Doğrulama ve Güvenlik (authentication.md)](./authentication.md)**: RSA anahtar çiftlerinin nasıl oluşturulacağını ve SDK ile platform arasındaki imza doğrulama mekanizmasını öğrenin.
2. **[API Referansı (api-reference.md)](./api-reference.md)**: Desteklenen tüm API uç noktaları (örneğin, cüzdan oluşturma, para çekme) ve Webhook formatları için ayrıntılı talimatlar içerir.
3. **[Örnekler ve Araçlar (examples.md)](./examples.md)**: Daha karmaşık senaryo tabanlı kod örneklerini ve SDK'nın yerleşik CLI araçlarının kullanım talimatlarını görün.
4. **[Ek (appendix.md)](./appendix.md)**: Desteklenen ChainID'ler, token türleri ve sözleşme adresleri gibi statik sözlük bilgileri.

## İletişim

Herhangi bir sorunuz varsa, lütfen servis sağlayıcısıyla iletişime geçin:  
Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)