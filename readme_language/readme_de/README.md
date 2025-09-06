# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## 🌟 Willkommen zum CryptoPay Rust SDK

CryptoPay Rust SDK ist ein professionelles Kryptowährungs-Service-SDK, das in Rust implementiert ist und Funktionen wie Benutzerregistrierung, Wallet-Generierung, Einzahlungs-Callback-Benachrichtigungen, Auszahlungen und mehr bietet.

Es hat sich durch langfristige Nutzung als sicher, stabil und leicht erweiterbar erwiesen.

## ⚙️ Installation

```bash
git clone github.com/zerospace-ai/pay-sdk-rust.git
```

Hinweis: Die Kompilierung erfordert rustc 1.91.0 🛠️.
## 🚀 Schnellstart
### 1. ⚙️ config.yaml

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

Feld-Beschreibungen:

• 🔑 ApiKey / ApiSecret:

Wird von der Plattform bei der Registrierung eines Händlerkontos zugewiesen, verwendet für die API-Anfrage-Authentifizierung ✅.

• 🛡️ PlatformPubKey / PlatformRiskPubKey:

Öffentliche Schlüssel, die von der Plattform bereitgestellt werden, verwendet zur Überprüfung von Daten oder Callback-Signaturen, die von der Plattform zurückgegeben werden, um zuverlässige Informationsquellen zu gewährleisten. PlatformRiskPubKey wird hauptsächlich für die Überprüfung risikobezogener Ereignisse verwendet ⚠️.

• 🗝️ RsaPrivateKey:

RSA-Privatschlüssel, der vom Händler generiert wird, verwendet zum Signieren von Anfragen, um sicherzustellen, dass der Anfrageinhalt nicht manipuliert wird. Wichtiger Hinweis: Der Privatschlüssel muss vertraulich gehalten werden 🔒, nicht preisgeben 🚫.

### 2. RSA-Schlüsselpaar generieren 🔐

Die Verwendung eines RSA-Schlüsselpaars zum Signieren von Anfragen gewährleistet die Datensicherheit. Im Folgenden wird beschrieben, wie ein Schlüsselpaar generiert und Schlüssel-Strings auf verschiedenen Betriebssystemen extrahiert werden.

#### 2.1 Schlüsselpaar mit OpenSSL generieren

```bash
# 2048-Bit-Privatschlüssel generieren
openssl genrsa -out rsa_private_key.pem 2048

# Öffentlichen Schlüssel aus Privatschlüssel generieren
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

> 💡 Tipp: Der generierte öffentliche Schlüssel muss den Anfang und das Ende -----BEGIN PUBLIC KEY----- / -----END PUBLIC KEY----- entfernen, Zeilenumbrüche entfernen, in eine einzeilige Zeichenkette umwandeln und an die Plattform übermitteln.
> 
> Schlüssel-Strings extrahieren und den öffentlichen Schlüssel an die Plattform übermitteln 📤.
>
>Die Befehle zum Generieren von RSA-Schlüsselpaaren auf Mac und Windows sind die gleichen wie auf Linux.

#### 2.2 Schlüssel-Strings extrahieren 🔑

Auf Mac/Linux oder Git Bash/WSL/Cygwin:

```bash
# Privatschlüssel-String extrahieren
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# Öffentlichen Schlüssel-String extrahieren
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

Windows

PowerShell extrahiert private und public key strings:

```powershell
# Privatschlüssel
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# Öffentlicher Schlüssel
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> ⚠️ Hinweis: Der generierte Privatschlüssel muss sicher aufbewahrt werden und darf nicht durchsickern.


### 🛠️ 3. SDK-Instanz erstellen

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

## 🔑 Wichtige Konzepte

- 🆔 **OpenId**: Einzigartige Benutzerkennung, z.B. "HASH1756194148".
- 🔐 **RSA-Schlüssel**: Wird zum Signieren und Überprüfen von Anfragen verwendet, um die Datensicherheit zu gewährleisten.
- ✍️ **API-Signatur**: Verwenden Sie MD5- und RSA-Algorithmen zum Signieren von Anfragen, um sicherzustellen, dass sie nicht manipuliert werden.

Für detaillierte API-Beschreibungen siehe [🧩 api-reference.md](./api-reference.md) und [🧩 examples.md](./examples.md).

Für Authentifizierung & Sicherheit siehe [🧩 authentication.md](./authentication.md)

## 📎 Anhang

Für detailliertere Referenzen schauen Sie bitte in das [Anhang](./appendix.md)-Dokument, das folgende Inhalte enthält:

- [🧩 ChainID-Liste](./appendix.md#-chainid-liste)
- [🏷️ Token-Typen](./appendix.md#-token-typ)
- [🌐 Öffentliche Informationen](./appendix.md#-öffentliche-informationen)
- [🔰 Token-Grundinformationen](./appendix.md#-token-grundinformationen)

> 💡 **Tipp**: Der Anhang bietet unterstützte Ketteninformationen, Token-Typen und öffentliche Token-Daten, was es Entwicklern erleichtert, das SDK zu integrieren und zu verwenden.

## 📞 Kontakt

Falls Sie Fragen haben, kontaktieren Sie bitte den Dienstleister.  
💬 Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)