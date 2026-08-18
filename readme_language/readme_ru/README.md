# CryptoPay Rust SDK

![Rust Version](https://img.shields.io/badge/rust-1.91+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Telegram](https://img.shields.io/badge/chat-Telegram-blue?logo=telegram)](https://t.me/ZeroSerivce)

## Добро пожаловать в CryptoPay Rust SDK

CryptoPay Rust SDK — это профессиональный SDK для криптовалютных сервисов, реализованный на Rust. Он предоставляет такие функции, как регистрация пользователей, создание кошельков, уведомления об обратном вызове (callback) при пополнении и вывод средств.
Он широко используется и доказал свою безопасность, стабильность и простоту расширения.

## Установка

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
pay-sdk-rust = { git = "https://github.com/zerospace-ai/pay-sdk-rust.git" }
```

> **Примечание:** Для компиляции требуется Rust 1.91+.

## Быстрый старт

### 1. Подготовка конфигурации

Перед использованием SDK необходимо подготовить файл конфигурации `config.yaml`, который содержит информацию для аутентификации продавца и открытые/закрытые ключи:

```yaml
ApiKey: "your_api_key"
ApiSecret: "your_api_secret"
PlatformPubKey: "platform_public_key"
PlatformRiskPubKey: "platform_risk_public_key"
RsaPrivateKey: "your_rsa_private_key"
```

> **💡 Подсказка:** Подробную информацию о том, как сгенерировать собственную пару ключей RSA продавца (`RsaPrivateKey`), а также о подробных механизмах аутентификации и безопасности, читайте в разделе [Аутентификация и безопасность (authentication.md)](./authentication.md).

### 2. Инициализация SDK и отправка запроса

Ниже приведен полный пример, демонстрирующий инициализацию экземпляра SDK и вызов API "Создать нового пользователя":

```rust
use pay_sdk_rust::sdk::{Sdk, SDKConfig, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Загрузить конфигурацию
    let cfg = load_config("config.yaml")?;

    // 2. Создать экземпляр SDK
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 3. Вызвать API: Создать запрос на создание нового пользователя
    let open_id = "PT00001"; // Уникальный идентификатор пользователя
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // (Выполнение сетевого запроса и анализ ответа здесь опущены)
    println!("Запрос CreateUser успешно создан!");
    println!("Body: {}", req_body);
    println!("Headers: timestamp={}, sign={}, clientSign={}", timestamp, sign, client_sign);

    Ok(())
}
```

## Ключевые концепции и навигация

Для более эффективного использования этого SDK мы рекомендуем прочитать остальные документы в следующем порядке:

1. **[Аутентификация и безопасность (authentication.md)](./authentication.md)**: Узнайте, как генерировать пары ключей RSA и как работает механизм проверки подписи между SDK и платформой.
2. **[Справочник по API (api-reference.md)](./api-reference.md)**: Содержит подробные инструкции для всех поддерживаемых конечных точек API (например, создание кошелька, вывод средств) и форматов вебхуков.
3. **[Примеры и инструменты (examples.md)](./examples.md)**: Посмотрите более сложные примеры кода для различных сценариев и инструкции по использовании встроенных инструментов CLI SDK.
4. **[Приложение (appendix.md)](./appendix.md)**: Статическая словарная информация, такая как поддерживаемые ChainID, типы токенов и адреса контрактов.

## Контакты

Если у вас есть какие-либо вопросы, обратитесь к поставщику услуг:  
Telegram: [@ZeroSerivce](https://t.me/ZeroSerivce)