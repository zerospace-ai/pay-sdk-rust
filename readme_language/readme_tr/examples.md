# Örnekler 📝

Bu belge, CryptoPay Rust SDK'sı için kullanım örnekleri sağlar, Demo çalıştırma, anahtar üretimi ve geri arama işleme dahil.

## 1 SDK Örnek Nesnesi 🛠️

### 1.1 Gerekli Yapılandırma ⚙️

1. İşletme adınızı kaydedin ve `ApiKey` ve `ApiSecret` elde edin;

2. Kendi `RSA` anahtar çiftinizi üretin;

3. Platformun `RSA` genel anahtarını hazırlayın;

### 1.2 İmza Nesnesi Oluşturma 🔏

1. Bir yapılandırma dosyası `config.yaml` ekleyin.

```yaml
# İşletme bilgilerini yapılandırın
ApiKey: ""
ApiSecret: ""
# Platform genel anahtarı
PlatformPubKey: ""
# Platformu engellemek için genel anahtar
PlatformRiskPubKey: ""
# Kendi özel anahtarınız
RsaPrivateKey: ""
```

2. Yapılandırma dosyasını yükleyin ve API nesnesini oluşturun.

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

### 1.3 İstek Verilerini Oluşturma ve İmzalama ✍️

Kullanıcı oluşturmayı örnek olarak kullanalım.

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

### 1.4 İsteği Doldurma ve Başlatma 🚀

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

### 1.5 Dönüş Verilerini Doğrulama ve Ayrıştırma ✅

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

## 2. Yürütülebilir Arayüz Komutları Oluşturma

* 1. SDK kök dizininde `cargo build --example create_user_client` komutunu çalıştırarak `pay-sdk-rust/target/debug/examples/` dizininde her fonksiyon komutu için ikili yürütülebilir dosyalar üretin.

* 2. ".exe" sonekli dosya 64-bit Windows makinelerinde çalışır; soneksiz dosya Linux/Mac'te çalışır. Örneğin, create_user.exe ve create_user yürütülebilir dosyaları.

* 3. Yapılandırılmış config.yaml dosyasını `pay-sdk-rust/target/debug/examples/` dizinine kopyalayın.

## 3. Komutu Çağırma 📞

### 3.1. Yeni Kullanıcı Kaydetme 🆕


SDK'nın `pay-sdk-rust/target/debug/examples/` dizinine gidin ve oradaki config.yaml dosyasındaki UserOpenId alanını değiştirin.

create_user veya create_user.exe yürütülebilir dosyasını çalıştırarak platformda yeni bir kullanıcı kaydedin.

Zaten kaydedilmiş bir UserOpenId'yi kaydetmeye çalışırsanız, hata döndürülür.


### 3.2. Cüzdan Kaydı 💼

SDK'nın `pay-sdk-rust/target/debug/examples/` dizinine gidin ve `config.yaml` dosyasındaki `UserOpenId` ve `ChainID` alanlarını belirtin.

`create_wallet` veya `create_wallet.exe` yürütülebilir dosyasını çalıştırarak platformda kullanıcının cüzdan kaydını tamamlayın.

### 3.3. Yatırma Adresini Alma 📍

SDK'nın `pay-sdk-rust/target/debug/examples/` dizinine gidin ve `config.yaml`deki `UserOpenId` ve `ChainIDs` alanlarını belirtin.

`get_wallet_addresses` veya `get_wallet_addresses.exe` yürütülebilir dosyasını çalıştırın.

### 3.4. Para Çekme 💸

SDK'nın `pay-sdk-rust/target/debug/examples/` dizinine gidin ve `config.yaml`deki `UserOpenId`, `TokenId`, `Amount`, `AddressTo`, `SafeCheckCode` ve `CallbackUrl` alanlarını belirtin.

`user_withdraw_by_open_id` veya `user_withdraw_by_open_id.exe` yürütülebilir dosyasını çalıştırın.