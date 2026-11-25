# Örnekler ve Araçlar

Bu belge iki bölüme ayrılmıştır:
1. **Senaryo Tabanlı Kod Örnekleri:** Pratik kodda API çağrılarının ve doğrulamalarının nasıl işleneceğini gösterir.
2. **CLI Araçları Kılavuzu:** Hızlı test için SDK'da bulunan derlenmiş yürütülebilir dosyaların nasıl kullanılacağını açıklar.

---

## 1. Senaryo Tabanlı Kod Örnekleri

### 1.1 Tam API Çağrısı ve Yanıt Doğrulaması

Aşağıdaki kod, bir "Kullanıcı Oluştur" isteği oluşturmak, HTTP isteğini göndermek ve platform tarafından döndürülen verilerin imzasının güvenliğini doğrulamak için SDK'nın nasıl kullanılacağını gösterir.

```rust
use pay_sdk_rust::{
    response_define::ResponseCommon,
    rsa_utils,
    sdk::{load_config, SDKConfig, Sdk},
};
use reqwest::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. SDK'yı başlat
    let cfg = load_config("./config.yaml")?;
    let sdk = Sdk::new(SDKConfig {
        api_key: cfg.api_key,
        api_secret: cfg.api_secret,
        platform_pub_key: cfg.platform_pub_key,
        platform_risk_pub_key: cfg.platform_risk_pub_key,
        rsa_private_key: cfg.rsa_private_key,
    });

    // 2. İstek parametrelerini ve imza başlığını oluştur
    let open_id = "PT00001";
    let (req_body, timestamp, sign, client_sign) = sdk.create_user(open_id)?;

    // 3. HTTP isteği gönder
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
    
    // 4. Yanıt imzasını otomatik olarak doğrula
    let parsed: ResponseCommon = serde_json::from_str(&body)?;
    if parsed.code != 1 {
        println!("İstek başarısız oldu: {}", parsed.msg);
        return Ok(());
    }

    let map_data = rsa_utils::to_string_map(&body)?;
    let rsp_data: pay_sdk_rust::response_define::ResponseCreateUser = serde_json::from_str(&body)?;
    
    match sdk.verify_rsa_signature(&map_data, &rsp_data.sign) {
        Ok(_) => println!("✅ İstek başarılı ve doğrulandı! Döndürülen OpenId: {}", rsp_data.data.open_id),
        Err(e) => println!("❌ İmza doğrulaması başarısız oldu: {:?}", e),
    }

    Ok(())
}
```


---

## 2. CLI Araçları Kullanım Kılavuzu

SDK, her API uç noktasını hızlı bir şekilde test etmek için Komut Satırı Arayüzü (CLI) ikili dosyaları sağlar.

### 2.1 Yürütülebilir Dosyaları Derleme

SDK kök dizininde `cargo build --examples` komutunu çalıştırın ve sistem `target/debug/examples/` dizinindeki her işlev için ikili yürütülebilir dosyalar üretecektir.
* **Windows:** `.exe` ile biten dosyalar oluşturur (örneğin, `create_user.exe`).
* **Mac/Linux:** Uzantısız dosyalar oluşturur (örneğin, `create_user`).

### 2.2 Yapılandırma Dosyasını Hazırlama

Araçları çalıştırmadan önce, yapılandırılmış `config.yaml` dosyasının yürütme dizinine (genellikle proje kök dizini veya `target/debug/examples/`) yerleştirildiğinden emin olun.

### 2.3 Uç Nokta Komutlarını Test Etme

#### Yeni Kullanıcı Kaydet
1. `config.yaml` içindeki `UserOpenId` alanını değiştirin.
2. `cargo run --example create_user_client` komutunu çalıştırın.
3. OpenId zaten kayıtlıysa, araç bir hata döndürür.

#### Cüzdan Kaydı
1. `config.yaml` içinde `UserOpenId` ve `ChainID` değerlerini belirtin.
2. `cargo run --example create_wallet` komutunu çalıştırın.

#### Para Yatırma Adreslerini Al
1. `config.yaml` içinde `UserOpenId` ve sorgulanan `ChainIDs` değerlerini (örneğin, "1,56") belirtin.
2. `cargo run --example get_wallet_addresses` komutunu çalıştırın.

#### Para Çekme Talebi
1. `config.yaml` içinde aşağıdakileri belirtin:
   * `UserOpenId`
   * `TokenId`
   * `Amount`
   * `AddressTo`
   * `SafeCheckCode` (Benzersiz sipariş tekrarlama önleme kodu)
   * `CallbackUrl`
2. `cargo run --example user_withdraw_by_open_id` komutunu çalıştırın.

#### Kasa Siparişi Oluştur
1. `config.yaml` dosyasında `OutOrderNo`, `OutUserId` (sabit adres modunda zorunlu), `TokenId`, `Quantity`, `NotifyUrl` ve `OrderDetail` bilgilerini belirtin.
2. `cargo run --example new_order` komutunu çalıştırın.

#### Cüzdan Bakiyesini Sorgula
1. `config.yaml` içinde `WalletAddress`, `ContractAddress` ve `WalletBalanceChainId` değerlerini belirtin.
2. `cargo run --example wallet_balance` komutunu çalıştırın.