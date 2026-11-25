# Примеры и инструменты

Этот документ разделен на две части:
1. **Примеры кода на основе сценариев:** Демонстрирует, как обрабатывать вызовы API и проверки в практическом коде.
2. **Руководство по инструментам CLI:** Объясняет, как использовать скомпилированные исполняемые файлы, поставляемые с SDK, для быстрого тестирования.

---

## 1. Примеры кода на основе сценариев

### 1.1 Полный вызов API и проверка ответа

В следующем коде показано, как использовать SDK для создания запроса "Создать пользователя", отправки HTTP-запроса и проверки безопасности подписи данных, возвращаемых платформой.

```rust
use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Инициализировать SDK
    let cfg = load_config("./config.yaml")?;
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 2. Сгенерировать параметры запроса и заголовок подписи
    let open_id = "PT00001";
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // 3. Отправить HTTP-запрос
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
    
    // 4. Автоматически проверить подпись ответа
    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    if parsed.code != 1 {
        println!("Запрос не удался: {}", parsed.msg);
        return Ok(());
    }

    let map_data = rsa_utils::to_string_map(&body)?;
    let rsp_data: pay_sdk_rust::response_define::ResponseCreateUser = serde_json::from_str(&body)?;
    
    match sdk.verify_rsa_signature(&map_data, &rsp_data.sign) {
        Ok(_) => println!("✅ Запрос успешен и проверен! Возвращенный OpenId: {}", rsp_data.data.open_id),
        Err(e) => println!("❌ Ошибка проверки подписи: {:?}", e),
    }

    Ok(())
}
```


---

## 2. Руководство по использованию инструментов CLI

SDK предоставляет бинарные файлы интерфейса командной строки (CLI) для быстрого тестирования каждой конечной точки API.

### 2.1 Компиляция исполняемых файлов

Выполните команду `cargo build --examples` в корневом каталоге SDK, и система сгенерирует бинарные исполняемые файлы для каждой функции в каталоге `target/debug/examples/`.
* **Windows:** Генерирует файлы, оканчивающиеся на `.exe` (например, `create_user.exe`).
* **Mac/Linux:** Генерирует файлы без суффиксов (например, `create_user`).

### 2.2 Подготовка файла конфигурации

Перед запуском инструментов убедитесь, что настроенный файл `config.yaml` помещен в каталог выполнения (обычно это корневой каталог проекта или `target/debug/examples/`).

### 2.3 Тестирование команд конечной точки

#### Регистрация нового пользователя
1. Измените поле `UserOpenId` в `config.yaml`.
2. Запустите `cargo run --example create_user_client`.
3. Если OpenId уже зарегистрирован, инструмент вернет ошибку.

#### Регистрация кошелька
1. Укажите `UserOpenId` и `ChainID` в `config.yaml`.
2. Запустите `cargo run --example create_wallet`.

#### Получение адресов для депозита
1. Укажите `UserOpenId` и запрашиваемые `ChainIDs` (например, "1,56") в `config.yaml`.
2. Запустите `cargo run --example get_wallet_addresses`.

#### Запрос на вывод средств
1. Укажите следующее в `config.yaml`:
   * `UserOpenId`
   * `TokenId`
   * `Amount`
   * `AddressTo`
   * `SafeCheckCode` (Уникальный код для предотвращения дублирования заказов)
   * `CallbackUrl`
2. Запустите `cargo run --example user_withdraw_by_open_id`.

#### Создание кассового ордера
1. Укажите `OutOrderNo`, `OutUserId` (обязательно в режиме фиксированного адреса), `TokenId`, `Quantity`, `NotifyUrl` и `OrderDetail` в `config.yaml`.
2. Запустите `cargo run --example new_order`.

#### Запрос баланса кошелька
1. Укажите `WalletAddress`, `ContractAddress` и `WalletBalanceChainId` в `config.yaml`.
2. Запустите `cargo run --example wallet_balance`.