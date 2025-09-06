# Beispiele 📝

Dieses Dokument bietet Nutzungsbeispiele für das CryptoPay Rust SDK, einschließlich Demo-Ausführung, Schlüsselerzeugung und Callback-Handhabung.

## 1 SDK-Instanzobjekt 🛠️

### 1.1 Erforderliche Konfiguration ⚙️

1. Registrieren Sie Ihren Geschäftsnamen und erhalten Sie den `ApiKey` und `ApiSecret`;

2. Generieren Sie Ihr eigenes `RSA`-Schlüsselpaar;

3. Bereiten Sie den `RSA`-öffentlichen Schlüssel der Plattform vor;

### 1.2 Erstellen eines Signaturobjekts 🔏

1. Fügen Sie eine Konfigurationsdatei `config.yaml` hinzu.

```yaml
# Konfigurieren Sie Geschäftsinformationen
ApiKey: ""
ApiSecret: ""
# Plattform-öffentlicher Schlüssel
PlatformPubKey: ""
# Öffentlicher Schlüssel zum Blockieren der Plattform
PlatformRiskPubKey: ""
# Ihr eigener privater Schlüssel
RsaPrivateKey: ""
```

2. Laden Sie die Konfigurationsdatei und erstellen Sie das API-Objekt.

```rust

	let cfg = load_config("./config.yaml")?;
    
    let sdk_config = SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    };

```

### 1.3 Erstellen und Signieren der Anfragedaten. ✍️

Nehmen wir die Benutzererstellung als Beispiel.

```rust

	// ....
	let open_id = "user_002";

    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;


```

```rust
	let map_data: HashMap<String, String> =
            serde_json::from_value(serde_json::to_value(&req)?)?;
	let data_str = rsa_utils::compose_params(&map_data);
	let timestamp = Utc::now().timestamp_millis().to_string();
	let sign = self.generate_md5_sign(&data_str, &timestamp);
	let client_sign = self.generate_rsa_signature(&map_data)?;
	let j_str = serde_json::to_string(&req)?;
```

### 1.4 Ausfüllen und Initiieren der Anfrage 🚀

```rust
	// ....
	
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
    println!("Response: {}", body);

```

### 1.5 Überprüfen und Parsen der Rückgabedaten ✅

```rust

    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    println!("Parsed response: {:?}", parsed);
    if parsed.code != 1 {
        return Err(anyhow::anyhow!("Error response: {}", parsed.msg));
    }
    match rsa_utils::to_string_map(&body) {
        Ok(map_data) => {
            match serde_json::from_str::<pay_sdk_rust::response_define::ResponseCreateUser>(&body) {
                Ok(rsp_data) => {
                    let sign = rsp_data.sign.clone();
                    match sdk.verify_rsa_signature(&map_data, &sign) {
                        Ok(_) => println!("verify signature success!"),
                        Err(e) => eprintln!("verify signature failed: {:?}", e),
                    }
                }
                Err(e) => eprintln!("parse JSON ResponseCreateUser failed: {:?}", e),
            }
        }
        Err(e) => eprintln!("convert to map[string]string failed: {:?}", e),
    }

```

## 2. Generieren ausführbarer Schnittstellenbefehle

* 1. Führen Sie den `cargo build --example create_user_client` Befehl im SDK-Stammverzeichnis aus, um `pay-sdk-rust/target/debug/examples/` äre ausführbare Dateien für jeden Funktionsbefehl im bin-Verzeichnis zu generieren.

* 2. Die Datei mit dem ".exe"-Suffix läuft auf 64-Bit-Windows-Maschinen; die Datei ohne das ".exe"-Suffix läuft auf Linux/Mac. Zum Beispiel create_user.exe und create_user ausführbare Dateien.

* 3. Kopieren Sie die konfigurierte config.yaml-Datei in das `pay-sdk-rust/target/debug/examples/`-Verzeichnis.

## 3. Aufrufen des Befehls 📞

### 3.1. Registrieren eines neuen Benutzers 🆕


Gehen Sie zum `pay-sdk-rust/target/debug/examples/`-Verzeichnis des SDK und ändern Sie das UserOpenId-Feld in der config.yaml-Datei dort.

Führen Sie die create_user oder create_user.exe ausführbare Datei aus, um einen neuen Benutzer auf der Plattform zu registrieren.

Wenn Sie versuchen, eine neue UserOpenId zu registrieren, die bereits registriert wurde, wird ein Fehler zurückgegeben.


### 3.2. Wallet-Registrierung 💼

Gehen Sie zum `pay-sdk-rust/target/debug/examples/`-Verzeichnis des SDK und geben Sie die `UserOpenId` und `ChainID`-Felder in der `config.yaml`-Datei an.

Führen Sie die `create_wallet` oder `create_wallet.exe` ausführbare Datei aus, um die Wallet-Registrierung des Benutzers auf der Plattform abzuschließen.

### 3.3. Einzahlungsadresse abrufen 📍

Gehen Sie zum `pay-sdk-rust/target/debug/examples/`-Verzeichnis des SDK und geben Sie die `UserOpenId` und `ChainIDs`-Felder in `config.yaml` an.

Führen Sie die `get_wallet_addresses` oder `get_wallet_addresses.exe` ausführbare Datei aus.

### 3.4. Auszahlungen 💸

Gehen Sie zum `pay-sdk-rust/target/debug/examples/`-Verzeichnis des SDK und geben Sie die `UserOpenId`, `TokenId`, `Amount`, `AddressTo`, `SafeCheckCode` und `CallbackUrl`-Felder in `config.yaml` an.

Führen Sie die `user_withdraw_by_open_id` oder `user_withdraw_by_open_id.exe` ausführbare Datei aus.