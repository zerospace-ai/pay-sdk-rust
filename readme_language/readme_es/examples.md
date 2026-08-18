# Ejemplos y Herramientas

Este documento se divide en dos partes:
1. **Ejemplos de Código Basados en Escenarios:** Demuestra cómo manejar llamadas API y verificaciones en código práctico.
2. **Guía de Herramientas CLI:** Explica cómo utilizar los archivos ejecutables compilados incluidos con el SDK para pruebas rápidas.

---

## 1. Ejemplos de Código Basados en Escenarios

### 1.1 Llamada Completa a la API y Verificación de Respuesta

El siguiente código demuestra cómo utilizar el SDK para construir una solicitud "Crear Usuario", enviar la solicitud HTTP y verificar la seguridad de la firma de los datos devueltos por la plataforma.

```rust
use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Inicializar SDK
    let cfg = load_config("./config.yaml")?;
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 2. Generar parámetros de solicitud y encabezado de firma
    let open_id = "PT00001";
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // 3. Enviar solicitud HTTP
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
    
    // 4. Verificar automáticamente la firma de la respuesta
    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    if parsed.code != 1 {
        println!("Solicitud fallida: {}", parsed.msg);
        return Ok(());
    }

    let map_data = rsa_utils::to_string_map(&body)?;
    let rsp_data: pay_sdk_rust::response_define::ResponseCreateUser = serde_json::from_str(&body)?;
    
    match sdk.verify_rsa_signature(&map_data, &rsp_data.sign) {
        Ok(_) => println!("✅ ¡Solicitud exitosa y verificada! OpenId devuelto: {}", rsp_data.data.open_id),
        Err(e) => println!("❌ Fallo en la verificación de la firma: {:?}", e),
    }

    Ok(())
}
```


---

## 2. Guía de Uso de Herramientas CLI

El SDK proporciona archivos binarios de la Interfaz de Línea de Comandos (CLI) para probar rápidamente cada punto final de la API.

### 2.1 Compilar Archivos Ejecutables

Ejecute el comando `cargo build --examples` en el directorio raíz del SDK, y el sistema generará archivos ejecutables binarios para cada función en el directorio `target/debug/examples/`.
* **Windows:** Genera archivos que terminan en `.exe` (por ejemplo, `create_user.exe`).
* **Mac/Linux:** Genera archivos sin sufijos (por ejemplo, `create_user`).

### 2.2 Preparar Archivo de Configuración

Antes de ejecutar las herramientas, asegúrese de que el archivo configurado `config.yaml` se encuentre en el directorio de ejecución (generalmente el directorio raíz del proyecto o `target/debug/examples/`).

### 2.3 Prueba de Comandos de Puntos Finales

#### Registrar Nuevo Usuario
1. Modifique el campo `UserOpenId` en `config.yaml`.
2. Ejecute `cargo run --example create_user_client`.
3. Si el OpenId ya está registrado, la herramienta devolverá un error.

#### Registro de Billetera
1. Especifique `UserOpenId` y `ChainID` en `config.yaml`.
2. Ejecute `cargo run --example create_wallet`.

#### Obtener Direcciones de Depósito
1. Especifique `UserOpenId` y los `ChainIDs` a consultar (por ejemplo, "1,56") en `config.yaml`.
2. Ejecute `cargo run --example get_wallet_addresses`.

#### Solicitar Retiro
1. Especifique lo siguiente en `config.yaml`:
   * `UserOpenId`
   * `TokenId`
   * `Amount`
   * `AddressTo`
   * `SafeCheckCode` (Código único para prevenir duplicidad de órdenes)
   * `CallbackUrl`
2. Ejecute `cargo run --example user_withdraw_by_open_id`.

#### Crear orden de caja
1. Especifique `OutOrderNo`, `TokenId`, `Quantity` y `NotifyUrl` en `config.yaml`.
2. Ejecute `cargo run --example new_order`.

#### Consultar Saldo de Billetera
1. Especifique `WalletAddress`, `ContractAddress` y `WalletBalanceChainId` en `config.yaml`.
2. Ejecute `cargo run --example wallet_balance`.