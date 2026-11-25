# API Referansı

Bu belge, istek parametreleri, dönüş parametreleri ve örnekler dahil olmak üzere CryptoPay Rust SDK'nın tüm API uç noktalarını ayrıntılı olarak açıklar.

## İçindekiler

1. [Genel Herkese Açık Bilgiler](#1-genel-herkese-açık-bilgiler)
2. [Yeni Kullanıcı Oluştur (create_user)](#2-yeni-kullanıcı-oluştur-create_user)
3. [Cüzdan Oluştur (create_wallet)](#3-cüzdan-oluştur-create_wallet)
4. [Para Yatırma Adreslerini Al (get_wallet_addresses)](#4-para-yatırma-adreslerini-al-get_wallet_addresses)
5. [Kullanıcı Para Çekme (user_withdraw_by_open_id)](#5-kullanıcı-para-çekme-user_withdraw_by_open_id)
6. [Para Çekme Siparişi İkincil İnceleme (Webhook)](#6-para-çekme-siparişi-ikincil-inceleme-webhook)
7. [Para Yatırma ve Para Çekme Geri Çağırma Bildirimi (Webhook)](#7-para-yatırma-ve-para-çekme-geri-çağırma-bildirimi-webhook)
8. [Kasa Siparişi Oluştur (new_order)](#8-kasa-siparişi-oluştur-new_order)
9. [Kasa Siparişi Ödeme Başarısı Geri Çağırma (Webhook)](#9-kasa-siparişi-ödeme-başarısı-geri-çağırma-webhook)
10. [Cüzdan Bakiyesini Sorgula (wallet_balance)](#10-cüzdan-bakiyesini-sorgula-wallet_balance)

---

## 1. Genel Herkese Açık Bilgiler

Platform tarafından döndürülen tüm JSON yanıt verileri bu genel yapıyı izler:

| Alan Adı | Tür | Örnek | Açıklama |
| :--- | :--- | :--- | :--- |
| `code` | integer | `1` | Genel durum kodu. `1` başarıyı temsil eder, `0` veya `-1` başarısızlığı temsil eder |
| `msg` | string | `ok` | Döndürülen metin açıklaması |
| `data` | object | `{"OpenId":"PT00001"}` | API'ye göre değişen, döndürülen belirli veri içeriği |
| `timestamp` | string | `"1725439986754"` | UTC Zaman Damgası (milisaniye) |
| `sign` | string | `9e0ccfe3915e...` | Yanıt verilerinde kendi RSA özel anahtarını kullanarak platform tarafından oluşturulan imza |

---

## 2. Yeni Kullanıcı Oluştur (create_user)

### API Açıklaması
Kullanıcının UserOpenId olan benzersiz kimliğini gerektiren yeni bir platform kullanıcısı oluşturur.

### HTTP İsteği
* **URL:** `https://sandbox-api.privatex.io/sdk/user/create`
* **Yöntem:** `POST`

### İstek Parametreleri
| Parametre Adı | Zorunlu | Tür | Açıklama |
| :--- | :--- | :--- | :--- |
| `OpenId` | Evet | string | OpenId'yi oluşturmak için platform standart öneki + kullanıcının benzersiz kimliğinin kullanılması önerilir. |

### Yanıt Parametreleri
*(Genel Bilgileri İçerir)*
| Parametre Adı | Tür | Açıklama |
| :--- | :--- | :--- |
| `data.OpenId` | string | Kullanıcının benzersiz OpenId'si |

### Örnek Kod (cURL)
```bash
curl --location 'https://sandbox-api.privatex.io/sdk/user/create' \
--header 'key: vratson2i5hjxgkd' \
--header 'sign: 0592dc64d480fb119d1e07ce06011db8' \
--header 'clientSign: xxxxxxxxxxxxxxxxx' \
--header 'Content-Type: application/json' \
--header 'timestamp: 1725076567682' \
--data '{ 
  "OpenId":"PT00001"
}'
```

---

## 3. Cüzdan Oluştur (create_wallet)

### API Açıklaması
Belirtilen blockchain ağında kullanıcı için bir cüzdan hesabı oluşturur.

### HTTP İsteği
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/create`
* **Yöntem:** `POST`

### İstek Parametreleri
| Parametre Adı | Zorunlu | Tür | Açıklama |
| :--- | :--- | :--- | :--- |
| `ChainID` | Evet | string | Zincir Kimliği (Eke Bakın) |
| `OpenId` | Evet | string | Kullanıcının benzersiz OpenId'si |

### Yanıt Parametreleri
*(Genel Bilgileri İçerir)*
| Parametre Adı | Tür | Açıklama |
| :--- | :--- | :--- |
| `data.address` | string | Cüzdan Adresi |
| `data.OpenId` | string | Kullanıcının benzersiz OpenId'si |

### Örnek Kod (cURL)
```bash
curl --location 'https://sandbox-api.privatex.io/sdk/wallet/create' \
--header 'key: vratson2i5hjxgkd' \
--header 'sign: 0592dc64d480fb119d1e07ce06011db8' \
--header 'clientSign: xxxxxxxxxxxxxxxxx' \
--header 'Content-Type: application/json' \
--header 'timestamp: 1725076567682' \
--data '{
  "OpenId":"PT00001",
  "ChainID":"1"
}'
```

---

## 4. Para Yatırma Adreslerini Al (get_wallet_addresses)

### API Açıklaması
Bir kullanıcı için blockchain cüzdanı para yatırma adreslerini alır (birden fazla zincirde toplu sorgulamayı destekler).

### HTTP İsteği
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses`
* **Yöntem:** `POST`

### İstek Parametreleri
| Parametre Adı | Zorunlu | Tür | Açıklama |
| :--- | :--- | :--- | :--- |
| `OpenId` | Evet | string | Kullanıcının benzersiz OpenId'si |
| `ChainIDs` | Evet | string | Virgülle ayrılmış birden fazla Zincir Kimliği |

### Yanıt Parametreleri
*(Genel Bilgileri İçerir)*
| Parametre Adı | Tür | Açıklama |
| :--- | :--- | :--- |
| `data.Addresses` | array | Adres nesnelerinin listesi |

### Örnek Kod (cURL)
```bash
curl --location 'https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses' \
--header 'key: vratson2i5hjxgkd' \
--header 'sign: 0592dc64d480fb119d1e07ce06011db8' \
--header 'clientSign: xxxxxxxxxxxxxxxxx' \
--header 'Content-Type: application/json' \
--header 'timestamp: 1725076567682' \
--data '{
  "OpenId":"PT00001",
  "ChainIDs":"56,2"
}'
```

---

## 5. Kullanıcı Para Çekme (user_withdraw_by_open_id)

### API Açıklaması
Ortağın fon havuzundan kullanıcı tarafından belirtilen adrese fon aktararak bir para çekme işlemi başlatır.

> **⚠️ Uyarı:**
> * Blockchain ağında bir işlem başlatıldığında, **geri alınamaz veya iade edilemez**. Lütfen bu API'yi çağırmadan önce uygun risk kontrol kontrollerinin yapıldığından emin olun.
> * Para çekme havuzunun yeterli token bakiyesi ve Gaz ücretlerine (ETH/TRX gibi) sahip olduğundan emin olmalısınız.

### İş Mantığı
1. Mükerrer para çekme işlemlerini önlemek için genellikle iş platformundan benzersiz bir sipariş numarası `SafeCheckCode` olarak kullanılır.
2. İşlem, platformun risk kontrol kurallarıyla eşleştirilir:
   * **Otomatik Onaylı:** Doğrudan zincir üzerine (on-chain) gider ve `data` işlem karma (hash) değerini döndürür.
   * **İkincil İnceleme (`code=2`):** Kanal platformunda bir yönetici tarafından manuel inceleme gerektirir. İncelendikten sonra, durum Webhook aracılığıyla eşzamansız olarak bildirilir.

### HTTP İsteği
* **URL:** `https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID`
* **Yöntem:** `POST`

### İstek Parametreleri
| Parametre Adı | Zorunlu | Tür | Açıklama |
| :--- | :--- | :--- | :--- |
| `OpenId` | Evet | string | Kullanıcının benzersiz OpenId'si |
| `TokenId` | Evet | string | Token Kimliği (Eke Bakın)|
| `Amount` | Evet | float | Para çekme miktarı |
| `AddressTo` | Evet | string | Hedef para çekme adresi |
| `CallBackUrl` | Hayır | string | Durum değişikliği geri çağırma URL'si |
| `SafeCheckCode` | Hayır | string | Güvenlik doğrulama kodu (İşletme Sipariş Kimliği) |

### Yanıt Durum Kodu Açıklaması
| `code` | Açıklama |
| :--- | :--- |
| `1` | İşlem başarılı ve zincir üzerine gönderildi, `data` karma değerini (hash) döndürür. |
| `2` | İşlem gönderildi, ancak platform risk kontrolünü tetikledi ve ikincil inceleme gerektiriyor. |
| `0` veya `-1` | Parametre hatası, yetersiz bakiye veya işlem başarısız. Ayrıntılar için `msg` değerini kontrol edin. |

### Örnek Kod (cURL)
```bash
curl --location 'https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID' \
--header 'key: vratson2i5hjxgkd' \
--header 'sign: 0592dc64d480fb119d1e07ce06011db8' \
--header 'clientSign: xxxxxxxxxxxxxxxxx' \
--header 'Content-Type: application/json' \
--header 'timestamp: 1725076567682' \
--data '{ 
  "OpenId": "PT00001", 
  "TokenId": "4", 
  "Amount": "0.02", 
  "AddressTo": "TQdL5yttJPTx7hJmBhGfo2LcE7AXLPtHSg", 
  "CallBackUrl": "http://xxxxxx/withdraw_callback", 
  "SafeCheckCode": "1000000000000000"
}'
```

---

## 6. Para Çekme Siparişi İkincil İnceleme (Webhook)

### Geri Çağırma (Callback) Açıklaması
Bir yönetici, tüccar arka ucunda (backend) bir "Risk Kontrolü Geri Çağırma URL'si" yapılandırdığında platform, para çekmenin serbest bırakılıp bırakılmayacağını tüccarın iş tarafına sormak için bir para çekme işlemini işlerken aktif olarak bu URL'ye bir HTTP isteği başlatır.
**Platform, yalnızca tüccar API'si `code=0` değerini döndürürse fonları serbest bırakacaktır.**

> **⚠️ Not:** Platform, tüccara bağımsız bir **Risk Kontrolü RSA Genel Anahtarı (`PlatformRiskPubKey`)** atayacaktır. Tüccar, bu geri çağırmanın imzasının orijinalliğini doğrulamak için bu genel anahtarı kullanmalıdır.

### Platform Tarafından Başlatılan HTTP İsteği
* **Yöntem:** `POST`
* **URL:** Tüccar tarafından yapılandırılmış Webhook URL'si `/withdrawal/order/check`

### Geri Çağırma Parametreleri
| Parametre Adı | Zorunlu | Tür | Açıklama |
| :--- | :--- | :--- | :--- |
| `safeCode` | Hayır | string | Tüccar tarafından gönderilen para çekme siparişi kimliği (`SafeCheckCode`) |
| `openId` | Evet | string | Para Çeken Kullanıcının Kimliği |
| `tokenId` | Evet | string | Token Kimliği |
| `toAddress` | Evet | string | Para Çekme Hedef Adresi |
| `amount` | Evet | string | Para Çekme Miktarı |
| `timestamp` | Evet | int | Milisaniye zaman damgası |
| `sign` | Evet | string | Risk kontrolü özel anahtarı kullanılarak platform tarafından oluşturulan imza |

### Tüccardan Beklenen Yanıt
İşlemden sonra tüccar API'si aşağıdaki JSON'ı döndürmelidir:

| Parametre Adı | Tür | Açıklama |
| :--- | :--- | :--- |
| `code` | int | Doğrulama sonucu. `0`, serbest bırakmayı kabul etmek anlamına gelir; diğer sayılar reddetmek anlamına gelir. |
| `timestamp` | int | Saniye cinsinden geçerli zaman damgası |
| `message` | string | Özel açıklama |
| `sign` | string | Tüccar, kendi RSA özel anahtarını kullanarak yanıt verilerini imzalar |

---

## 7. Para Yatırma ve Para Çekme Geri Çağırma Bildirimi (Webhook)

### Geri Çağırma Açıklaması
Platform, iş tarafına token işlem durumu (para çekme başarısı/başarısızlığı veya kullanıcı para yatırma kredisi) hakkında eşzamansız bildirimler iletir.

1. **Yeniden Deneme Mekanizması:** İş tarafı, başarılı bir şekilde alınmış sayılması için `0` dönüş koduyla yanıt vermelidir. Aksi takdirde, platform stratejiye göre iletmeyi sürekli olarak yeniden deneyecektir (başlangıçta her 2 saniyede bir 50 kez, ardından her 10 dakikada bir).
2. **Durum Geçersiz Kılma:** Bir işlem birden fazla bildirimi tetikleyebilir. Lütfen en son duruma güvenin.
3. **Başarısızlık İşleme:** Para çekme geri çağırması `status = -1` ise, yönetici işlemi arka uçta (backend) yeniden gönderdikten sonra sistem yeni bir durum bildirimini tekrar iletecektir.

### Platform Tarafından Başlatılan HTTP İsteği
* **Yöntem:** `POST`
* **URL:** Tüccar tarafından servis sağlayıcıya sağlanan geri çağırma URL'si.

### Geri Çağırma Parametreleri
| Parametre Adı | Zorunlu | Tür | Açıklama |
| :--- | :--- | :--- | :--- |
| `type` | Evet | int | İşletme Türü: Para yatırma için `1`; para çekme için `2`. |
| `status` | Evet | int | İşlem Durumu:<br/> `1`: Zincir üstü (On-chain) başarılı, karma (hash) üzerinden sorgulanabilir.<br/> `-1`: Zincir üstü başarısız, arka uçta işlenebilir.<br/> `2`: Risk kontrolü tetiklendi, yönetici arka ucunda manuel inceleme gerektiriyor.<br/> `-2`: Para çekme başvurusu tüccar risk kontrol API'si tarafından reddedildi. |
| `openid` | Evet | string | Kanal Kullanıcısının Benzersiz Kimliği |
| `totalvalue` | Evet | string | İşlem anındaki piyasa fiyatına göre hesaplanan eşdeğer USDT değeri |
| `hash` | Evet | string | Zincir Üstü İşlem Karması (Hash) |
| `confirm` | Evet | int | Zincir Üstü Onay Sayısı |
| `from` | Evet | string | İşlemi Başlatan Adres |
| `to` | Evet | string | İşlem Alma Adresi |
| `amount` | Evet | string | İşlem Tutarı |
| `chainid` | Evet | string | Zincir Kimliği |
| `tokenid` | Evet | string | Token Kimliği |
| `tokenaddress` | Evet | string | Akıllı Sözleşme Adresi |
| `safecode` | Hayır | string | Para Çekme Siparişi Güvenlik Doğrulama Kodu (OrderID) |
| `createdtime` | Evet | string | İşlem Oluşturulma Zamanı |
| `timestamp` | Evet | string | İletme Zaman Damgası |

### Tüccardan Beklenen Yanıt
Başarıyla alınırsa, lütfen `{"code": 0}` içeren bir JSON yanıt gövdesi döndürün.

---

## 8. Kasa Siparişi Oluştur (new_order)

### API Açıklaması
Bu arayüz, tüccarların ödeme veya yükleme talepleri başlatması için kullanılır. Sistem bir ödeme adresi ve kasa sayfası bağlantısı (Cashier URL) döndürür; kullanıcı bu bağlantı üzerinden ödemeyi tamamlayabilir.

### HTTP İsteği
* **URL:** `https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder`
* **Yöntem:** `POST`

### İstek Parametreleri
| Parametre Adı | Zorunlu mu? | Tür | Açıklama |
| :--- | :--- | :--- | :--- |
| `outOrderNo` | Evet | string | Tüccar tarafındaki benzersiz sipariş kimliği (platform genelinde benzersizdir; tüccar öneki eklenmesi önerilir) |
| `outUserId` | Moda bağlı | string | Tüccar tarafındaki kullanıcı kimliği. **Adres havuzu modu etkin değilse zorunludur** (aksi halde `10502` hatası döner); adres havuzu modunda (`address_reuse_mode=1`) gönderilmeyebilir |
| `tokenId` | Evet | integer | Token Kimliği (blockchain ağını da belirler) |
| `quantity` | Evet | decimal | Ödenecek token miktarı (`> 0` olmalıdır; `0.01` gibi düz ondalık gösterim kullanın, bilimsel gösterimden kaçının) |
| `notifyUrl` | Hayır | string | Ödeme tamamlandıktan sonraki asenkron geri çağırma URL'si (belirtilmezse tüccar panelindeki varsayılan adres kullanılır) |
| `orderDetail` | Hayır | string | Sipariş açıklaması (olduğu gibi kaydedilir) |

> **İki Adres Modu (`outUserId`):**
> * **Sabit adres modu:** `outUserId` gönderildiğinde `(tüccar, outUserId, ağ)` başına sabit bir adres atanır. Adres havuzu modu etkin olmayan tüccarlar `outUserId` değerini **mutlaka** göndermelidir; aksi halde `10502` hatası döner.
> * **Adres havuzu modu:** `address_reuse_mode=1` etkinken `outUserId` gönderilmezse (boş string değil, alan tamamen hariç tutulmalı/`null`) boşta olan havuzdan özel bir adres atanır. Not: Bu modda yanıttaki `data.outUserId` ve geri çağırmadaki `outUserId` havuz adresinin ilk bağlı kullanıcısını gösterir — mutabakat için daima `outOrderNo` / `orderNo` (`outOrderId` / `orderId`) kullanın.

### Yanıt Parametreleri
*(Global Bilgileri İçerir)*
| Parametre Adı | Tür | Açıklama |
| :--- | :--- | :--- |
| `data.orderNo` | string | Platform tarafından oluşturulan benzersiz sipariş kimliği |
| `data.outOrderNo` | string | Tüccar tarafından iletilen orijinal sipariş kimliği |
| `data.outUserId` | string | Bağlı kullanıcı kimliği (adres havuzu modunda havuz adresinin orijinal kullanıcı kimliği) |
| `data.tokenId` | integer | Token Kimliği |
| `data.quantity` | string | Ödenecek miktar (JSON içinde string) |
| `data.addresses` | string | Tahsilat adresi (tek adres, büyük/küçük harfe duyarlıdır) |
| `data.cashierUrl` | string | **Temel Alan**: Kasa sayfası adresi; tüccarların ödeme için kullanıcıları bu bağlantıya yönlendirmesi gerekir |

### Kod Örneği (cURL)
```bash
curl --location 'https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder' \
--header 'key: your_api_key' \
--header 'sign: 1b5b885633560f447f38073f964ad51b' \
--header 'clientSign: your_rsa_sign_base64' \
--header 'Content-Type: application/json' \
--header 'timestamp: 1770606706659' \
--data '{
  "outOrderNo": "1234623",
  "outUserId": "user_10086",
  "tokenId": 6,
  "quantity": 0.01,
  "notifyUrl": "https://your-domain.com/callback",
  "orderDetail": "VIP会员充值"
}'
```

*(Adres havuzu modu etkinse `outUserId` ve `orderDetail` alanları hariç tutulabilir; gönderilmeyen alanlar imzalamaya dahil edilmez.)*

---

## 9. Kasa Siparişi Ödeme Başarısı Geri Çağırma (Webhook)

### Geri Çağırma Açıklaması
Kullanıcı tam ödemeyi tamamladığında (`orderStatus=2`), DogPay siparişte belirtilen `notifyUrl` adresine (belirtilmemişse varsayılan geri çağırma adresine) asenkron bildirim gönderir. Kısmi ödeme (`1`) ve süresi dolmuş siparişler (`-1`) geri çağırma tetiklemez.

### Platform Tarafından Başlatılan HTTP İsteği
* **Yöntem:** `POST`
* **URL:** Tüccar tarafından sağlanan `notifyUrl` (veya varsayılan geri çağırma URL'si)

### Geri Çağırma Parametreleri
| Parametre Adı | Tür | Açıklama |
| :--- | :--- | :--- |
| `orderId` | string | Platform sistem sipariş kimliği (`data.orderNo` ile eşleşir) |
| `outOrderId` | string | Tüccar orijinal sipariş kimliği (`outOrderNo` ile eşleşir) |
| `outUserId` | string | Bağlı kullanıcı kimliği (yukarıdaki adres modları notuna bakın) |
| `orderType` | string | Sipariş türü (`1` = yatırım/tahsilat) |
| `chainId` | string | Ağ (Chain) Kimliği |
| `tokenId` | string | Token Kimliği |
| `quantity` | string | Ödenecek miktar |
| `quantityPaid` | string | **Kullanıcı tarafından ödenen gerçek toplam miktar** |
| `totalAmount` | string | Ödenecek tahmini USD değeri |
| `totalAmountPaid` | string | Ödenen tahmini USD değeri |
| `exchangeAddress` | string | Tahsilat adresi |
| `orderStatus` | string | **Sipariş Durumu**: geri çağırmada sabit `2` (tamamlandı) |
| `txHash` | string | Siparişle eşleşen son zincir üstü işlem hash'i |
| `createdAt` | string | Sipariş oluşturulma zamanı |
| `timestamp` | string | Geri çağırma zaman damgası (milisaniye) |
| `sign` | string | **Veri İmzası** (sahte bildirimleri önlemek için platform açık anahtarıyla doğrulayın) |

### Tüccardan Beklenen Yanıt
Başarıyla alındıysa aşağıdaki JSON'u döndürün (`code` değeri `"0"` veya `0` olmalıdır; aksi halde platform yeniden dener):
```json
{
  "code": "0",
  "message": "success"
}
```

---

## 10. Cüzdan Bakiyesini Sorgula (wallet_balance)

### API Açıklaması
Belirtilen blokzincir ağındaki belirli bir cüzdan adresinin belirteç veya yerel coin bakiyesini sorgular.

### HTTP İsteği
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/balance`
* **Yöntem:** `POST`

### İstek Parametreleri
| Parametre Adı | Zorunlu | Tür | Açıklama |
| :--- | :--- | :--- | :--- |
| `address` | Evet | string | Cüzdan adresi |
| `contractAddress` | Evet | string | Sözleşme adresi veya belirteç sembolü (örn. `"XRP"`, `"USDT"`) |
| `chainId` | Evet | integer | Zincir Kimliği (örn. XRP için `5`, Ethereum için `1`, BNB Chain için `56`) |

### Yanıt Parametreleri
*(Genel Bilgileri İçerir)*
| Parametre Adı | Tür | Açıklama |
| :--- | :--- | :--- |
| `code` | integer | Genel durum kodu (`1` başarıyı temsil eder) |
| `msg` | string | Durum açıklaması |
| `data` | string | Belirteç / coin bakiye miktarı (en küçük birimlerde dize) |
| `timestamp` | string | Yanıt zaman damgası (milisaniye) |
| `sign` | string | Platform veri imzası |

### Örnek Kod (cURL)
```bash
curl --location --request POST 'https://sandbox-api.privatex.io/sdk/wallet/balance' \
--header 'key: your_api_key' \
--header 'sign: your_md5_sign' \
--header 'Content-Type: application/json' \
--header 'timestamp: 1725076567682' \
--data-raw '{
  "address":"rXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "contractAddress":"XRP",
  "chainId":5
}'
```

### Yanıt Örneği
```json
{
  "sign" : "",
  "timestamp" : "1725432397796",
  "data" : "1979984",
  "msg" : "ok",
  "code" : 1
}
```