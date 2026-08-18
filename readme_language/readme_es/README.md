# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## Bienvenido a CryptoPay Rust SDK

CryptoPay Rust SDK es un SDK de servicio de criptomonedas profesional implementado en Rust, que proporciona funciones como registro de usuarios, generación de billeteras, notificaciones de devolución de llamadas de depósitos y retiros.
Ha sido ampliamente utilizado y ha demostrado ser seguro, estable y fácilmente ampliable.

## Instalación

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
pay-sdk-rust = { git = "https://github.com/zerospace-ai/pay-sdk-rust.git" }
```

> **Nota:** La compilación requiere Rust 1.91+.

## Inicio Rápido

### 1. Preparar la Configuración

Antes de utilizar el SDK, debe preparar el archivo de configuración `config.yaml`, que contiene la información de autenticación del comerciante y las claves públicas/privadas:

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

> **💡 Consejo:** Para obtener detalles sobre cómo generar el propio par de claves RSA del comerciante (`RsaPrivateKey`) y los mecanismos detallados de autenticación y seguridad, lea [Autenticación y Seguridad (authentication.md)](./authentication.md).

### 2. Inicializar el SDK y Enviar Solicitud

Aquí hay un ejemplo completo que demuestra cómo inicializar la instancia del SDK y llamar a la API "Crear Nuevo Usuario":

```rust
use pay_sdk_rust::sdk::{Sdk, SDKConfig, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Cargar configuración
    let cfg = load_config("config.yaml")?;

    // 2. Crear instancia de SDK
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 3. Llamar API: Crear solicitud de nuevo usuario
    let open_id = "PT00001"; // Identificador único del usuario
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // (La ejecución de la solicitud de red y el análisis de la respuesta se omiten aquí)
    println!("¡Solicitud CreateUser creada con éxito!");
    println!("Body: {}", req_body);
    println!("Headers: timestamp={}, sign={}, clientSign={}", timestamp, sign, client_sign);

    Ok(())
}
```

## Conceptos Clave y Navegación

Para utilizar mejor este SDK, le recomendamos que lea los documentos restantes en el siguiente orden:

1. **[Autenticación y Seguridad (authentication.md)](./authentication.md)**: Aprenda cómo generar pares de claves RSA y el mecanismo de verificación de firmas entre el SDK y la plataforma.
2. **[Referencia de API (api-reference.md)](./api-reference.md)**: Contiene instrucciones detalladas para todos los puntos finales de API compatibles (por ejemplo, creación de billetera, retiro) y formatos de webhook.
3. **[Ejemplos y Herramientas (examples.md)](./examples.md)**: Vea ejemplos de código basados en escenarios más complejos e instrucciones sobre el uso de las herramientas CLI integradas del SDK.
4. **[Apéndice (appendix.md)](./appendix.md)**: Información de diccionario estático, como ChainIDs admitidos, tipos de tokens y direcciones de contratos.

## Contacto

Si tiene alguna pregunta, comuníquese con el proveedor de servicios:  
Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)