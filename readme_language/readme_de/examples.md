# Beispiele und Tools

Dieses Dokument ist in zwei Teile gegliedert:
1. **Szenariobasierte Codebeispiele:** Demonstriert, wie API-Aufrufe und Verifizierungen im praktischen Code gehandhabt werden.
2. **CLI-Tools-Handbuch:** Erklärt, wie die mit dem SDK kompilierten ausführbaren Dateien für schnelle Tests verwendet werden.

---

## 1. Szenariobasierte Codebeispiele

### 1.1 Vollständiger API-Aufruf und Antwortverifizierung

Der folgende Code demonstriert, wie das SDK verwendet wird, um eine "Benutzer erstellen"-Anfrage (Create User) zu erstellen, die HTTP-Anfrage zu senden und die Sicherheit der Signatur der von der Plattform zurückgegebenen Daten zu überprüfen.

```rust
use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. SDK initialisieren
    let cfg = load_config("./config.yaml")?;
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 2. Anfrageparameter und Signatur-Header generieren
    let open_id = "PT00001";
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // 3. HTTP-Anfrage senden
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
    
    // 4. Antwortsignatur automatisch verifizieren
    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    if parsed.code != 1 {
        println!("Anfrage fehlgeschlagen: {}", parsed.msg);
        return Ok(());
    }

    let map_data = rsa_utils::to_string_map(&body)?;
    let rsp_data: pay_sdk_rust::response_define::ResponseCreateUser = serde_json::from_str(&body)?;
    
    match sdk.verify_rsa_signature(&map_data, &rsp_data.sign) {
        Ok(_) => println!("✅ Anfrage erfolgreich und verifiziert! Zurückgegebene OpenId: {}", rsp_data.data.open_id),
        Err(e) => println!("❌ Signaturverifizierung fehlgeschlagen: {:?}", e),
    }

    Ok(())
}
```


---

## 2. Handbuch zur Verwendung von CLI-Tools

Das SDK bietet ausführbare Binärdateien (Command Line Interface, CLI) zum schnellen Testen jedes API-Endpunkts.

### 2.1 Ausführbare Dateien kompilieren

Führen Sie den Befehl `cargo build --examples` im Stammverzeichnis des SDK aus. Das System generiert dann ausführbare Binärdateien für jede Funktion im Verzeichnis `target/debug/examples/`.
* **Windows:** Generiert Dateien mit der Endung `.exe` (z. B. `create_user.exe`).
* **Mac/Linux:** Generiert Dateien ohne Dateiendung (z. B. `create_user`).

### 2.2 Konfigurationsdatei vorbereiten

Stellen Sie vor dem Ausführen der Tools sicher, dass sich die konfigurierte Datei `config.yaml` im Ausführungsverzeichnis (normalerweise dem Projektstammverzeichnis oder `target/debug/examples/`) befindet.

### 2.3 Endpunkt-Befehle testen

#### Neuen Benutzer registrieren
1. Ändern Sie das Feld `UserOpenId` in `config.yaml`.
2. Führen Sie `cargo run --example create_user_client` aus.
3. Wenn the OpenId bereits registriert ist, gibt das Tool einen Fehler zurück.

#### Wallet-Registrierung
1. Geben Sie `UserOpenId` und `ChainID` in `config.yaml` an.
2. Führen Sie `cargo run --example create_wallet` aus.

#### Einzahlungsadressen abrufen
1. Geben Sie `UserOpenId` und die abgefragten `ChainIDs` (z. B. "1,56") in `config.yaml` an.
2. Führen Sie `cargo run --example get_wallet_addresses` aus.

#### Auszahlung beantragen
1. Geben Sie Folgendes in `config.yaml` an:
   * `UserOpenId`
   * `TokenId`
   * `Amount`
   * `AddressTo`
   * `SafeCheckCode` (Eindeutiger Code zur Verhinderung von doppelten Bestellungen)
   * `CallbackUrl`
2. Führen Sie `cargo run --example user_withdraw_by_open_id` aus.

#### Kassenbestellung erstellen
1. Geben Sie `OutOrderNo`, `OutUserId` (im festen Adressmodus erforderlich), `TokenId`, `Quantity`, `NotifyUrl` und `OrderDetail` in `config.yaml` an.
2. Führen Sie `cargo run --example new_order` aus.

#### Wallet-Guthaben abfragen
1. Geben Sie `WalletAddress`, `ContractAddress` und `WalletBalanceChainId` in `config.yaml` an.
2. Führen Sie `cargo run --example wallet_balance` aus.