# Ejemplos 📝

Este documento proporciona ejemplos de uso para el CryptoPay Rust SDK, incluyendo la ejecución de Demo, generación de claves y manejo de callbacks.

## 1 Objeto de Instancia SDK 🛠️

### 1.1 Configuración Requerida ⚙️

1. Registre su nombre de negocio y obtenga el `ApiKey` y `ApiSecret`;

2. Genere su propio par de claves `RSA`;

3. Prepare la clave pública `RSA` de la plataforma;

### 1.2 Creando un Objeto de Firma 🔏

1. Agregue un archivo de configuración `config.yaml`.

```yaml
# Configurar información de negocio
ApiKey: ""
ApiSecret: ""
# Clave pública de la plataforma
PlatformPubKey: ""
# Clave pública para bloquear la plataforma
PlatformRiskPubKey: ""
# Su propia clave privada
RsaPrivateKey: ""
```

2. Cargue el archivo de configuración y cree el objeto API.

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

### 1.3 Crear y firmar los datos de solicitud. ✍️

Usemos la creación de usuario como ejemplo.

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

### 1.4 Rellenar e Iniciar la Solicitud 🚀

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

### 1.5 Verificar el análisis de los datos de retorno ✅

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

## 2. Generar Comandos de Interface Ejecutables

* 1. Ejecute el comando `cargo build --example create_user_client` en el directorio raíz del SDK para generar archivos binarios ejecutables para cada comando de función en el directorio `pay-sdk-rust/target/debug/examples/`.

* 2. El archivo con el sufijo ".exe" se ejecuta en máquinas Windows de 64 bits; el archivo sin el sufijo ".exe" se ejecuta en Linux/Mac. Por ejemplo, create_user.exe y create_user archivos ejecutables.

* 3. Copie el archivo config.yaml configurado al directorio `pay-sdk-rust/target/debug/examples/`.

## 3. Llamando al Comando 📞

### 3.1. Registrando un Nuevo Usuario 🆕


Vaya al directorio `pay-sdk-rust/target/debug/examples/` del SDK y modifique el campo UserOpenId en el archivo config.yaml allí.

Ejecute el archivo ejecutable create_user o create_user.exe para registrar un nuevo usuario en la plataforma.

Si intenta registrar un nuevo UserOpenId que ya ha sido registrado, se devolverá un error.


### 3.2. Registro de Billetera 💼

Vaya al directorio `pay-sdk-rust/target/debug/examples/` del SDK y especifique los campos `UserOpenId` y `ChainID` en el archivo `config.yaml`.

Ejecute el archivo ejecutable `create_wallet` o `create_wallet.exe` para completar el registro de la billetera del usuario en la plataforma.

### 3.3. Obtener Dirección de Depósito 📍

Vaya al directorio `pay-sdk-rust/target/debug/examples/` del SDK y especifique los campos `UserOpenId` y `ChainIDs` en `config.yaml`.

Ejecute el archivo ejecutable `get_wallet_addresses` o `get_wallet_addresses.exe`.

### 3.4. Retiros 💸

Vaya al directorio `pay-sdk-rust/target/debug/examples/` del SDK y especifique los campos `UserOpenId`, `TokenId`, `Amount`, `AddressTo`, `SafeCheckCode` y `CallbackUrl` en `config.yaml`.

Ejecute el archivo ejecutable `user_withdraw_by_open_id` o `user_withdraw_by_open_id.exe`.