# Kimlik Doğrulama ve Güvenlik

Bu sistem, tüccar ile platform arasındaki veri güvenliğini sağlamak için sıkı bir istek imza mekanizması kullanır. İletişim süreci MD5 imzalarını ve RSA imza doğrulamasını içerir.

## 1. RSA Anahtar Çifti Oluşturma

Tüccarların kendi RSA anahtar çiftini oluşturması, istekleri imzalamak için özel anahtarı kullanması ve genel anahtarı platforma göndermesi gerekir.

### 1.1 OpenSSL Kullanarak Anahtar Çifti Oluşturma

Aşağıdaki komutları Mac, Linux veya Git Bash/WSL/Cygwin üzerinde yürütün:

```bash
# 2048 bit özel anahtar oluştur
openssl genrsa -out rsa_private_key.pem 2048

# Özel anahtardan genel anahtar oluştur
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

### 1.2 Anahtar Dizelerini Ayıklama

Oluşturulan genel anahtarın `-----BEGIN PUBLIC KEY-----` / `-----END PUBLIC KEY-----` satırları ile yeni satır karakterleri (newline) kaldırılmalı ve platforma gönderilmeden önce tek satırlık bir dizeye dönüştürülmelidir.

**Mac/Linux/Git Bash üzerinde ayıklama:**

```bash
# Özel anahtar dizesini ayıkla
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# Genel anahtar dizesini ayıkla
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

**Windows PowerShell üzerinde ayıklama:**

```powershell
# Özel anahtarı ayıkla
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# Genel anahtarı ayıkla
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> **⚠️ Uyarı:** Ayıklanan özel anahtar, yerel sunucunuzda güvenli bir şekilde saklanmalı (`config.yaml` içindeki `RsaPrivateKey` alanına girilmeli) ve **kesinlikle sızdırılmamalıdır**.

---

## 2. Doğrulama Header Tanımı

Bir HTTP isteğinde bulunurken, Header'a aşağıdaki kimlik doğrulama parametreleri dahil edilmelidir:

| Parametre Adı | Kısıtlama | Örnek | Açıklama |
| :--- | :--- | :--- | :--- |
| `key` | Uzunluk: 64 | `vratson2i5hjxgkd` | Tüccar kaydından sonra platform tarafından atanan API Key |
| `timestamp` | Uzunluk: 32 | `1725076567682` | İsteğin zaman damgası (Birim: milisaniye) |
| `sign` | Uzunluk: 32 | `0592dc64d480fb119d1e07ce0601db` | İstek içeriğinde MD5 algoritması kullanılarak oluşturulan imza |
| `clientSign` | Uzunluk: 512 | `9e0ccfe3915e94bcc5bfbBsC...` | İstek içeriğinde tüccarın RSA özel anahtarı kullanılarak oluşturulan imza |

---

## 3. Ayrıntılı İmza Algoritması

SDK, tüm imza mantığını dahili olarak otomatik bir şekilde yönetir. Başka bir dildeki sürümü bağımsız olarak geliştirmeniz gerekiyorsa veya imzanın altında yatan prensipleri anlamak istiyorsanız, lütfen aşağıdaki talimatlara başvurun.

### 3.1 `sign` Alanı Kuralları (MD5 İmzası)

1. Platformun `Secret` değerini alın.
2. İsteğin JSON gövdesini anahtara göre artan ASCII sırasına dizin ve `key1=value1&key2=value2...` formatında bir dize halinde birleştirin; buna `dataStr` adını verin.
3. Geçerli zaman damgasını (milisaniye) alın.
4. Şifrelenecek düz metni birleştirin: `strToHash = Secret + dataStr + timestamp`.
5. `strToHash` üzerinde MD5 şifrelemesi gerçekleştirin; oluşturulan sonuç `sign` değeridir.

### 3.2 `clientSign` Alanı Kuralları (RSA İmzası)

1. İstek parametrelerini anahtara göre artan ASCII sırasına dizin.
2. Tüm dizi değerlerini `&` karakterini kullanarak birleştirin, örneğin:
   `address=0x038B8...&amount=10.001&coin=eth&trade_id=2022013101`
3. Birleştirilmiş bu dize üzerinde `RSA-MD5` imzası gerçekleştirmek için tüccarın RSA özel anahtarını kullanın. Oluşturulan sonuç `clientSign` değeridir.

---

## 4. Platform Yanıtı İmza Doğrulaması

Platform tarafından döndürülen veriler ayrıca, platformun yanıt verilerini kendi RSA özel anahtarını kullanarak şifrelemesinin sonucu olan bir `sign` alanı içerecektir.

Tüccar yanıtı aldıktan sonra, verilerin tahrif edilmediğinden emin olmak için bu imzayı doğrulamak amacıyla `config.yaml` içindeki `PlatformPubKey` (platformun genel anahtarı) değerini kullanmalıdır.

> **💡 İpucu:** Rust SDK, geliştiricilerin platform tarafından döndürülen verileri hızlı bir şekilde doğrulaması için `verify_rsa_signature(&self, map_data: &HashMap<String, String>, sign: &str) -> Result<()>` yöntemini sağlar.