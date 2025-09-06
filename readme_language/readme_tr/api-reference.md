# API Referansı 📚

Bu belge, CryptoPay Rust SDK'nın tüm API arayüzlerini detaylandırır, istek parametreleri, dönüş parametreleri ve örnekler dahil.

## Yeni Kullanıcı Kaydet (create_user)🆕🧑‍💻

### Kavram
Yeni bir platform kullanıcısı oluşturun, kullanıcının benzersiz ID'si yani UserOpenId gereklidir.

### HTTP İsteği
- Arayüz Adı: create_user
- URL: https://sandbox-api.privatex.io/sdk/user/create
- Yöntem: POST

### İstek Parametreleri
| Parametre Adı | Zorunlu | Tür    | Açıklama                                                                               |
| ------------- | ------- | ------ | -------------------------------------------------------------------------------------- |
| OpenId        | Evet    | string | Platform standart ön eki + kullanıcı benzersiz ID'si ile OpenId oluşturulması önerilir |

### Dönüş Parametreleri
| Parametre Adı | Tür    | Açıklama                   |
| ------------- | ------ | -------------------------- |
| code          | int    | Genel durum kodu           |
| msg           | string | Durum açıklaması           |
| data.OpenId   | string | Kullanıcı benzersiz OpenId |
| sign          | string | Platform imzası            |

### Örnek
İstek Örneği:
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
Dönüş Örneği:
```json
{
    "code": 1,
    "msg": "ok",
    "data": {
        "OpenId": "PT00001"
    },
    "sign": "..."
}
```

Kimlik Doğrulama ve Güvenlik için lütfen [🧩 authentication.md](./authentication.md) dosyasına bakın

## Cüzdan Oluştur (create_wallet) 💰

### Kavram
Kullanıcı için ilgili blockchain ağında bir cüzdan hesabı oluşturun.

### HTTP İsteği
- Arayüz Adı: create_wallet
- URL: https://sandbox-api.privatex.io/sdk/wallet/create
- Yöntem: POST

### İstek Parametreleri
| Parametre Adı | Zorunlu | Tür    | Açıklama                   |
| ------------- | ------- | ------ | -------------------------- |
| ChainID       | Evet    | string | Zincir ID                  |
| OpenId        | Evet    | string | Kullanıcı benzersiz OpenId |

### Dönüş Parametreleri
| Parametre Adı | Tür    | Açıklama                   |
| ------------- | ------ | -------------------------- |
| code          | int    | Genel durum kodu           |
| msg           | string | Durum açıklaması           |
| data.address  | string | Cüzdan adresi              |
| data.OpenId   | string | Kullanıcı benzersiz OpenId |
| sign          | string | Platform imzası            |

### Örnek
İstek Örneği:
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
Dönüş Örneği:
```json
{
    "code": 1,
    "msg": "ok",
    "data": {
        "address": "...",
        "OpenId": "PT00001"
    },
    "sign": "..."
}
```

## Para Yatırma Adresi Al (get_wallet_addresses)💳

### Kavram
Kullanıcının blockchain cüzdan para yatırma adresini alın.

### HTTP İsteği
- Arayüz Adı: get_wallet_addresses
- URL: https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses
- Yöntem: POST

### İstek Parametreleri
| Parametre Adı | Zorunlu | Tür    | Açıklama                                     |
| ------------- | ------- | ------ | -------------------------------------------- |
| OpenId        | Evet    | string | Kullanıcı benzersiz OpenId                   |
| ChainIDs      | Evet    | string | Birden fazla zincir ID'si, virgülle ayrılmış |

### Dönüş Parametreleri
| Parametre Adı  | Tür    | Açıklama         |
| -------------- | ------ | ---------------- |
| code           | int    | Genel durum kodu |
| msg            | string | Durum açıklaması |
| data.Addresses | array  | Adres listesi    |
| sign           | string | Platform imzası  |

### Örnek
İstek Örneği:
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
Dönüş Örneği:
```json
{
    "code": 1,
    "msg": "ok",
    "data": {
        "Addresses": [
            {
                "chainID": 56,
                "address": "..."
            }
        ]
    },
    "sign": "..."
}
```

## Kullanıcı Para Çekme (user_withdraw_by_open_id)💸

### Kavram
Kullanıcı para çekme işlemi, ortak hesaptan kullanıcı belirtilen adrese transfer.

* İşlev: Kullanıcı para çekme işlemi arayüzü. Para çekmeler, ilgili token para çekme havuzundaki ortak hesabından kullanıcının belirtilen para çekme cüzdan adresine transfer edilmelidir. Ortaklar, para çekmenin meşruiyetini doğrulamak için güvenli bir geri çağrı adresi ayarlayabilir. Eğer doğrulanmışsa, para çekme doğrudan tüccarın fon havuzu cüzdanından tamamlanabilir.

* Para çekme işlem arayüzü, varsayılan para çekme sıcak cüzdanının yeterli para çekme varlıkları ve işlem ücretlerine sahip olup olmadığını kontrol eder.

* Varsayılan olarak, para çekme arayüzü güvenlik doğrulama kodunu para çekme işlemleri için benzersiz parametre gereksinimi olarak kullanır. Genellikle iş platformunun benzersiz para çekme sipariş numarasını güvenlik kodu olarak kullanmak önerilir. Yinelenen güvenlik doğrulama kodu göndermek hataya neden olur.

* Tüm para çekme işlem istekleri, kanal platformunda yapılandırılan risk kontrol inceleme kurallarıyla eşleştirilir. Eğer parametre isteği geçerliyse, işlem isteği kabul edilir. Otomatik inceleme kurallarını karşılayan para çekme işlemleri hemen ağ işlemine gönderilir ve gönderilen işlemin hash bilgisi döndürülür (dönüş alanı data). Kanalda ikincil inceleme gerektiren para çekme işlem istekleri (code=2) döndürür. Para çekme isteği tekrar gönderilmez. Yönetici, kanal platformunda ikincil incelemeyi tamamlamalıdır. İkincil inceleme tamamlandıktan sonra, işlem siparişi para çekme işlem durumu değişikliğini bildirmek için geri çağrı yapar.

* Önkoşul: İlgili para biriminin fon havuzunda yeterli miktarda fon bulunmalıdır (özellikle ETH ağı token para çekmeleri için, fon havuzu cüzdanında belirli miktarda ETH işlem ücreti bakiyesi gereklidir).

* ⚠️ Not: **Blockchain para çekmeleri için, arayüzü çağırmadan önce ön onay sürecinin tamamlandığından emin olun. Bir blockchain işlemi başlatıldıktan sonra, iptal edilemez veya iade edilemez.**

### HTTP İsteği
- Arayüz Adı: user_withdraw_by_open_id
- URL: https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID
- Yöntem: POST

### İstek Parametreleri
| Parametre Adı | Zorunlu | Tür    | Açıklama                   |
| ------------- | ------- | ------ | -------------------------- |
| OpenId        | Evet    | string | Kullanıcı benzersiz OpenId |
| TokenId       | Evet    | string | Token ID                   |
| Amount        | Evet    | float  | Para çekme miktarı         |
| AddressTo     | Evet    | string | Hedef adres                |
| CallBackUrl   | Hayır   | string | Geri çağrı URL'si          |
| SafeCheckCode | Hayır   | string | Güvenlik doğrulama kodu    |

### Dönüş Parametreleri

| Parametre Adı | Tür    | Açıklama                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------ | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| code          | int    | Durum Kodu</br>0 Parametre hatası, yinelenen sipariş numarası, yanlış para çekme adresi formatı veya yetersiz para çekme cüzdan ücretleri. Ayrıntılı bilgi msg'de bulunabilir.</br>1 Para çekme işlemi başarıyla gönderildi ve blockchain ağına gönderildi. Gönderilen işlemin benzersiz hash'i data'da bulunur.</br>2 Para çekme işlemi başarıyla gönderildi ve işlem tamamlanmadan önce ikincil kanal incelemesi gerektirir. İnceleme tamamlandıktan sonra, işlem bilgileri geri çağrı yoluyla güncellenir.</br>-1 Para çekme işlemi başarısız oldu. Para çekme isteğini tekrar gönderebilirsiniz. |
| msg           | string | Durum Açıklama                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| data          | string | İşlem hash'i. Akıllı para çekme etkinleştirilmişse, bu alan boş dize olarak döndürülür.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| sign          | string | Platform imzası                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| timestamp     | string | Mevcut zaman damgası milisaniye olarak dize dönüştürülmüş                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |


### Örnek
İstek Örneği:
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

Dönüş örneği
```json
{
    "sign": "D+VTPNiwGLzh9eIvkrscwS4UlGKzdnrBgB63RDG4HeobZT6FXqUwYCPgKojynKaxwm5PkmW0xhIASZ4asSCvnYfi0NSFehchZAtUnQIispxKcjsiudWsUznbkEIQ2h2TA/mbUZ1X9+wyh7QhNo6+RkxtgRyRpVb7ARG8pL14cdTAs OTtMLO0W1GO0M83VAv2ybBZNObncX9qy6tdwLQV/KYuNJYyMN0dL0nLKYHnj9Q4d3lEDM45AVJ0153/YIiIgcF BnOWhsQ9rVARcFeXeWd9KJ5OZpmxlxnhcJGcEUY2UDC4zKLZxtUet7CPAyehAMQ5plkpvRrR3Z6lA5zl6GQ==",
    "timestamp": "1725439986754",
    "data": "94f4c29eba73d53dcd3aa1b8cf90a98108d0acf82f38b97a4032dcdf7ff172e7",
    "msg": "ok",
    "code": 1
}
```

## Para Çekme Siparişi İkincil İnceleme 💳

* İşlev: Tüccar para çekme siparişi risk kontrolü ikincil inceleme arayüzü
* ⚠️ Not: **Platform, tüccarlara ayrı bir risk kontrol RSA genel anahtarı atar (para yatırma/çekme geri çağrı bildirim genel anahtarından farklı)**
* Tetikleme Zamanı: Yönetici, tüccar tarafında risk kontrol geri çağrı URL parametrelerini yapılandırdıktan sonra (sistem ayarları), kanal her para çekme işlem isteği için ek bir risk kontrol geri çağrı ikincil incelemesi ekler. Yalnızca tüccar tarafı risk kontrol URL'si doğru bir doğrulama geçme kodu döndürdüğünde işlem geçerli bir şekilde gönderilir.
* Teknik Gereksinimler: Tüccar tarafı teknik uygulama ve ikincil inceleme geri çağrı arayüzünün yapılandırılması gereklidir.

#### HTTP İsteği

Platform, tüccara para çekme incelemesi isteği gönderir

> POST: `/withdrawal/order/check`

#### İstek Parametreleri

| Parametre Adı | Zorunlu | Tür    | Açıklama                                                                                                                                                    |
| :------------ | :------ | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| safeCode      | Hayır   | string | Tüccar tarafından gönderilen benzersiz işlem ID'si, genellikle tüccarın para çekme sipariş ID'sine karşılık gelir (para çekme işlemleri için SafeCheckCode) |
| openId        | Evet    | string | Para çekme işlemını gönderen tüccar kullanıcı ID'si                                                                                                         |
| tokenId       | Evet    | string | Para birimi ID'si, platform tarafından sağlanan para birimi ID'sine dayanır                                                                                 |
| toAddress     | Evet    | string | Para çekme adresi                                                                                                                                           |
| amount        | Evet    | string | Para çekme miktarı                                                                                                                                          |
| timestamp     | Evet    | int    | Mevcut zaman damgası                                                                                                                                        |
| sign          | Evet    | string | İmza: Yalnızca data alanındaki parametreler imzalanır; imzanın doğruluğu platformun risk kontrol RSA genel anahtarı kullanılarak doğrulanmalıdır.           |

#### Dönüş Parametre Açıklama

| Parametre Adı | Tür    | Açıklama                                                                       |
| :------------ | :----- | :----------------------------------------------------------------------------- |
| code          | int    | Doğrulama sonucu. 0 geçerli olduğunu belirtir; diğer kodlar geçersizdir.       |
| timestamp     | int    | Mevcut zaman damgası, saniye cinsinden.                                        |
| message       | string | Dönüş mesajı.                                                                  |
| sign          | string | İmza: Yanıt parametresindeki data alanı için tüccarın RSA özel anahtar imzası. |

## Para Yatırma ve Çekme Geri Çağrı Bildirimleri

1. Para yatırma ve çekme işlemleri birden fazla geri çağrı bildirimini tetikler. Son geri çağrı bildiriminin işlem bilgileri ve durumu kullanılır.
2. İş tarafı geçerli bir geri çağrı mesajı döndürmelidir. Biçim dönüş parametre açıklamasında belirtilmiştir. 0 dönüş kodu, geri çağrı mesajının işlendiğini ve daha fazla bildirim gerekmediğini belirtir. Aksi takdirde, geri çağrı devam eder (başlangıçta her 2 saniyede 50 kez, ardından her 10 dakikada bir) ta ki code 0 ile doğrulama mesajı döndürülene kadar.

Geri çağrı URL'sini ayarlamak için hizmet sağlayıcısıyla iletişime geçin.

> POST

* İşlev: Platformun, uygulama tarafına token işlem bilgilerini (kullanıcı para çekme veya yatırma) bildirmek için kullandığı geri çağrı mesaj formatını tanımlar. Bu mesaj, uygulama tarafı token işlem durumu (çekme veya yatırma) olay bildirimleri için uygundur. Uygulamalar, uygulama işlevlerine göre geri çağrı bildirim arayüzünü isteğe bağlı olarak destekleyebilir.

### İstek Parametreleri

| Parametre Adı | Zorunlu | Tür    | Açıklama                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :------------ | :------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| openid        | evet    | string | Kanal kullanıcı benzersiz ID'si                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| totalvalue    | evet    | string | Para yatırma veya çekme işlemine karşılık gelen USDT değeri (işlem zamanındaki piyasa fiyatına göre hesaplanır)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| status        | evet    | int    | İşlem durumu:</br>1 İşlem tamamlandı ve başarıyla blockchain ağına gönderildi. İşlem detayları hash kullanılarak zincirde sorgulanabilir.</br>-1 İşlem blockchain ağına gönderildi, ancak zincirdeki işlem başarısız oldu. Tüccar Yönetimi --> İşlem Yönetimi --> [Sipariş Güvenlik Kodunu Gönder] altında tekrar inceleyebilirsiniz. İş platformu durum değişikliğini işlemeye gerek yoktur ve kanalın yeni durum bildirimini geri çağrı yapmasını bekleyebilir.</br>-2 Para çekme işlem başvurusu tüccar arka ucu tarafından reddedildi. Para çekme başvurusu süresi doldu. İş platformunun bildirim aldıktan sonra kullanıcının para çekme başvurusunu iade etmesi önerilir.</br>2 Para çekme işlemi tüccar yönetimine gönderildi. Yapılandırılan para birimi güvenlik risk kontrol gereksinimlerini tetiklediği için, yönetici Tüccar Yönetimi --> İşlem Yönetimi --> Para Çekme İncelemesi'nde para çekme başvuru işlemini tamamlamalıdır.</br>3 Para çekme işlemi blockchain işleme sırasında, iş platformu durum değişikliğini güncellemeye gerek yoktur ve kanalın yeni durum bildirimini almasını bekleyebilir. </br>⛑️**Özel Hatırlatma: İş platformu tarafından alınan para çekme işlem geri çağrıları için, eğer status = -1 ise, geri çağrı yoksayılır. Yönetici yönetim arka ucuna giriş yaptıktan ve işlemi tekrar gönderdikten sonra, yeni durum bildirimi platforma aynı anda itilir.** |  | type | evet | int | 1 para yatırma işlemleri için; 2 para çekme işlemleri için |
| hash          | evet    | string | İşlem hash değeri                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| confirm       | evet    | int    | İşlemin zincirdeki onay sayısı                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| createdtime   | evet    | string | Oluşturma zamanı                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| from          | evet    | string | İşlem başlatıcısının adresi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| to            | evet    | string | İşlem alıcı adresi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |