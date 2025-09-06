# İstek Doğrulama Başlığı

İstek Başlığı Tanımı

| Parametre Adı | Kısıtlamalar | Örnek                              | Açıklama                                         |
| :------------ | :----------- | :--------------------------------- | :----------------------------------------------- |
| key           | Uzunluk: 64  | ithujj3onrzbgw5t                   | Ortak anahtarı                                   |
| timestamp     | Uzunluk: 32  | 1722586649000                      | İstek başlatma zaman damgası (birim: milisaniye) |
| sign          | Uzunluk: 32  | 9e0ccfe3915e94bcc5bf7dd51ad4e8d9   | Ortak gizli imza                                 |
| clientSign    | Uzunluk: 512 | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | Ortak RSA imzası                                 |

## `sign` Alan Kuralları

1. Ortağı kaydedin ve `key` ve `secret` elde edin.
2. İsteği ayrıştırın. JSON gövdesini JSON anahtarlarının ASCII artan sırasına göre sıralayın ve dizeleri dataStr=key1=value1&key2=value2&key3=value3&... olarak birleştirin.
3. Zaman damgası oluşturun (birim: milisaniye)
4. Şifreleyin ve sign oluşturun: Şifreleme öncesi metin: strToHash = Secret+dataStr+timestamp Metni strToHash üzerinde MD5 şifrelemesi yaparak sign oluşturun.
Özel kod GenerateMD5Sign fonksiyonudur.
5. key, timestamp ve sign'ı HTTP başlığına yerleştirin.

## `clientSign` İmza Algoritmasının Ayrıntılı Açıklaması

1. İstek parametrelerini alın ve formatlayarak yeni bir formatlanmış dize elde edin.

2. Adım 1'deki verileri RSA özel anahtarıyla imzalayın ve imza sonucunu bir değişkene kaydedin.
Aşağıdaki parametre dizisi için imza dizesi oluşturun: `user_id = 1 coin = eth address = 0x038B8E7406dED2Be112B6c7E4681Df5316957cad amount = 10.001 trade_id = 20220131012030274786`
Dizideki her anahtarı a'dan z'ye sıralayın. İlk harf aynıysa, ikinci harfe bakın ve böyle devam edin. Sıralandıktan sonra, tüm dizi değerlerini ampersand (&) karakteriyle birleştirin, örneğin $dataString'de:
`address=0x038B8E7406dED2Be112B6c7E4681Df5316957cad&amount=10.001&coin=eth&trade_id=20220131012030274786&user_id=1`
Bu dize birleştirilmiş dizedir.

3. Özel anahtarı kullanarak verileri RSA-MD5 ile imzalayın. Özel kod generate_md5_sign fonksiyonundadır.

# Genel Bilgiler

| Ad               | Tür       | Örnek                              | Açıklama                                                       |
| :--------------- | :-------- | :--------------------------------- | :------------------------------------------------------------- |
| Genel Durum Kodu | integer   | 1                                  | 1 başarıyı belirtir. Ayrıntılar için Genel Durum Koduna bakın. |
| Mesaj            | string    | ok                                 | Metin bilgisi döndürür.                                        |
| Veri             | json      | {"OpenID":"HEX..."}                | Belirli veri içeriğini döndürür.                               |
| Zaman            | timeStamp | 1722587274000                      | UTC zamanı (zaman dilimsiz, milisaniye cinsinden).             |
| İmza             | string    | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | Platform tüm verileri RSA ile imzalar.                         |

Platform tarafından döndürülen Sign, yanıt verilerini RSA algoritması kullanarak şifreleme sonucudur. Ön uç, döndürülen verilere karşı imzayı doğrulamalıdır. Ayrıntılar için func: 

pub fn verify_rsa_signature` fonksiyonuna bakın.