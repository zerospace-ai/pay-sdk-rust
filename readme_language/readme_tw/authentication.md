# 認證與安全

本系統使用嚴格的請求簽名機制來確保商戶與平台之間的數據安全。通信過程涉及 MD5 簽名和 RSA 簽名驗證。

## 1. 生成 RSA 密鑰對

商戶需要自己生成 RSA 密鑰對，使用私鑰對請求進行簽名，並將公鑰提交給平台。

### 1.1 使用 OpenSSL 生成密鑰對

在 Mac、Linux 或 Git Bash/WSL/Cygwin 上執行以下命令：

```bash
# 生成 2048 位私鑰
openssl genrsa -out rsa_private_key.pem 2048

# 根據私鑰生成公鑰
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

### 1.2 提取密鑰字符串

生成的公鑰需要去掉開頭和結尾的 `-----BEGIN PUBLIC KEY-----` / `-----END PUBLIC KEY-----`，並去掉換行符，轉換為單行字符串後提交到平台。

**在 Mac/Linux/Git Bash 上提取：**

```bash
# 提取私鑰字符串
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# 提取公鑰字符串
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

**在 Windows PowerShell 上提取：**

```powershell
# 提取私鑰
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# 提取公钥
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> **⚠️ 注意：** 提取出的私鑰必須保存在本地服務器（填入 `config.yaml` 的 `RsaPrivateKey` 字段），**嚴禁洩露**。

---

## 2. 請求驗證頭定義

在發起 HTTP 請求時，需要在 Header 中包含以下認證參數：

| 參數名稱 | 約束 | 示例 | 說明 |
| :--- | :--- | :--- | :--- |
| `key` | 長度: 64 | `vratson2i5hjxgkd` | 商戶註冊後由平台分配的 API Key |
| `timestamp` | 長度: 32 | `1725076567682` | 請求發起的時間戳 (單位: 毫秒) |
| `sign` | 長度: 32 | `0592dc64d480fb119d1e07ce0601db` | 使用 MD5 算法對請求內容生成的簽名 |
| `clientSign` | 長度: 512 | `9e0ccfe3915e94bcc5bfbBsC...` | 使用商戶的 RSA 私鑰對請求內容生成的簽名 |

---

## 3. 簽名算法詳細說明

SDK 內部已經自動處理了所有簽名邏輯。如果你需要獨立開發其他語言版本，或者了解簽名的底層原理，請參考以下說明。

### 3.1 `sign` 字段規則 (MD5 簽名)

1. 獲取平台的 `Secret`。
2. 將請求的 JSON body 按鍵的 ASCII 升序排序，並拼接成格式為 `key1=value1&key2=value2...` 的字符串，記為 `dataStr`。
3. 獲取當前時間戳（毫秒）。
4. 拼接待加密明文：`strToHash = Secret + dataStr + timestamp`。
5. 對 `strToHash` 執行 MD5 加密，生成的結果即為 `sign`。

### 3.2 `clientSign` 字段規則 (RSA 簽名)

1. 將請求參數按照鍵的 ASCII 升序排序。
2. 使用 `&` 字符連接所有數組值，例如：
   `address=0x038B8...&amount=10.001&coin=eth&trade_id=2022013101`
3. 使用商戶的 RSA 私鑰對此拼接字符串進行 `RSA-MD5` 簽名，生成的結果即為 `clientSign`。

---

## 4. 平台響應簽名驗證

平台返回的數據中也會包含 `sign` 字段，這是平台使用其 RSA 私鑰對響應數據加密的結果。

商戶端收到響應後，必須使用 `config.yaml` 中的 `PlatformPubKey`（平台公鑰）來驗證此簽名，以確保數據未被篡改。

> **💡 提示：** Rust SDK 已提供 `verify_rsa_signature(&self, map_data: &HashMap<String, String>, sign: &str) -> Result<()>` 方法供開發者快捷驗證平台返回的數據。