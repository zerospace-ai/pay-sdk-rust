# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## Willkommen beim CryptoPay Rust SDK

Das CryptoPay Rust SDK ist ein professionelles Kryptowährungs-Service-SDK, das in Rust implementiert ist. Es bietet Funktionen wie Benutzerregistrierung, Wallet-Erstellung, Einzahlungs-Rückrufbenachrichtigungen und Auszahlungen.
Es ist weit verbreitet und hat sich als sicher, stabil und leicht erweiterbar erwiesen.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
pay-sdk-rust = { git = "https://github.com/zerospace-ai/pay-sdk-rust.git" }
```

> **Hinweis:** Die Kompilierung erfordert Rust 1.91+.

## Schnellstart

### 1. Konfiguration vorbereiten

Bevor Sie das SDK verwenden, müssen Sie die Konfigurationsdatei `config.yaml` vorbereiten, die die Authentifizierungsinformationen des Händlers und die öffentlichen/privaten Schlüssel enthält:

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

> **💡 Tipp:** Für Details zur Generierung des eigenen RSA-Schlüsselpaars (`RsaPrivateKey`) und zu den detaillierten Authentifizierungs- und Sicherheitsmechanismen lesen Sie bitte [Authentifizierung und Sicherheit (authentication.md)](./authentication.md).

### 2. SDK initialisieren und Anfrage senden

Hier ist ein vollständiges Beispiel, das zeigt, wie die SDK-Instanz initialisiert und die API "Neuen Benutzer erstellen" aufgerufen wird:

```rust
use pay_sdk_rust::sdk::{Sdk, SDKConfig, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Konfiguration laden
    let cfg = load_config("config.yaml")?;

    // 2. SDK-Instanz erstellen
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 3. API aufrufen: Anfrage für neuen Benutzer erstellen
    let open_id = "PT00001"; // Eindeutige Kennung des Benutzers
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // (Netzwerkanfrage und Antwort-Parsing werden hier weggelassen)
    println!("CreateUser-Anfrage erfolgreich erstellt!");
    println!("Body: {}", req_body);
    println!("Headers: timestamp={}, sign={}, clientSign={}", timestamp, sign, client_sign);

    Ok(())
}
```

## Wichtige Konzepte und Navigation

Um dieses SDK optimal nutzen zu können, empfehlen wir, die restlichen Dokumente in dieser Reihenfolge zu lesen:

1. **[Authentifizierung und Sicherheit (authentication.md)](./authentication.md)**: Erfahren Sie, wie Sie RSA-Schlüsselpaare generieren und den Signatur-Verifizierungsmechanismus zwischen SDK und Plattform verstehen.
2. **[API-Referenz (api-reference.md)](./api-reference.md)**: Enthält detaillierte Anweisungen für alle unterstützten API-Endpunkte (z. B. Wallet-Erstellung, Auszahlung) und Webhook-Formate.
3. **[Beispiele und Tools (examples.md)](./examples.md)**: Zeigt komplexere szenariobasierte Codebeispiele und Anweisungen zur Verwendung der integrierten CLI-Tools.
4. **[Anhang (appendix.md)](./appendix.md)**: Statische Wörterbuchinformationen wie unterstützte ChainIDs, Token-Typen und Vertragsadressen.

## Kontakt

Wenn Sie Fragen haben, kontaktieren Sie bitte den Dienstanbieter:  
Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)