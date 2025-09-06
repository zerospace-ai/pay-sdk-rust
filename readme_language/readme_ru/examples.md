# Примеры 📝

Этот документ предоставляет примеры использования CryptoPay Rust SDK, включая запуск Demo, генерацию ключей и обработку обратных вызовов.

## 1 Объект экземпляра SDK 🛠️

### 1.1 Необходимая конфигурация ⚙️

1. Зарегистрируйте название вашего бизнеса и получите `ApiKey` и `ApiSecret`;

2. Сгенерируйте свою собственную пару ключей `RSA`;

3. Подготовьте публичный ключ `RSA` платформы;

### 1.2 Создание объекта подписи 🔏

1. Добавьте файл конфигурации `config.yaml`.

```yaml
# Настройка информации о бизнесе
ApiKey: ""
ApiSecret: ""
# Публичный ключ платформы
PlatformPubKey: ""
# Публичный ключ для блокировки платформы
PlatformRiskPubKey: ""
# Ваш собственный приватный ключ
RsaPrivateKey: ""
```

2. Загрузите файл конфигурации и создайте объект API.

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

### 1.3 Создание и подпись данных запроса. ✍️

Возьмем создание пользователя в качестве примера.

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

### 1.4 Заполнение и инициирование запроса 🚀

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

### 1.5 Проверка и разбор возвращаемых данных ✅

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

## 2. Генерация исполняемых команд интерфейса

* 1. Выполните команду `cargo build --example create_user_client` в корневом каталоге SDK, чтобы сгенерировать бинарные исполняемые файлы для каждой функциональной команды в каталоге `pay-sdk-rust/target/debug/examples/`.

* 2. Файл с суффиксом ".exe" запускается на 64-битных машинах Windows; файл без суффикса ".exe" запускается на Linux/Mac. Например, create_user.exe и create_user исполняемые файлы.

* 3. Скопируйте настроенный файл config.yaml в каталог `pay-sdk-rust/target/debug/examples/`.

## 3. Вызов команды 📞

### 3.1. Регистрация нового пользователя 🆕


Перейдите в каталог `pay-sdk-rust/target/debug/examples/` SDK и измените поле UserOpenId в файле config.yaml там.

Запустите исполняемый файл create_user или create_user.exe, чтобы зарегистрировать нового пользователя на платформе.

Если вы попытаетесь зарегистрировать новый UserOpenId, который уже зарегистрирован, будет возвращена ошибка.


### 3.2. Регистрация кошелька 💼

Перейдите в каталог `pay-sdk-rust/target/debug/examples/` SDK и укажите поля `UserOpenId` и `ChainID` в файле `config.yaml`.

Запустите исполняемый файл `create_wallet` или `create_wallet.exe`, чтобы завершить регистрацию кошелька пользователя на платформе.

### 3.3. Получение адреса пополнения 📍

Перейдите в каталог `pay-sdk-rust/target/debug/examples/` SDK и укажите поля `UserOpenId` и `ChainIDs` в `config.yaml`.

Запустите исполняемый файл `get_wallet_addresses` или `get_wallet_addresses.exe`.

### 3.4. Вывод средств 💸

Перейдите в каталог `pay-sdk-rust/target/debug/examples/` SDK и укажите поля `UserOpenId`, `TokenId`, `Amount`, `AddressTo`, `SafeCheckCode` и `CallbackUrl` в `config.yaml`.

Запустите исполняемый файл `user_withdraw_by_open_id` или `user_withdraw_by_open_id.exe`.