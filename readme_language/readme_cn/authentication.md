# 认证与安全

本系统使用严格的请求签名机制来确保商户与平台之间的数据安全。通信过程涉及 MD5 签名和 RSA 签名验证。

## 1. 生成 RSA 密钥对

商户需要自己生成 RSA 密钥对，使用私钥对请求进行签名，并将公钥提交给平台。

### 1.1 使用 OpenSSL 生成密钥对

在 Mac、Linux 或 Git Bash/WSL/Cygwin 上执行以下命令：

```bash
# 生成 2048 位私钥
openssl genrsa -out rsa_private_key.pem 2048

# 根据私钥生成公钥
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

### 1.2 提取密钥字符串

生成的公钥需要去掉开头和结尾的 `-----BEGIN PUBLIC KEY-----` / `-----END PUBLIC KEY-----`，并去掉换行符，转换为单行字符串后提交到平台。

**在 Mac/Linux/Git Bash 上提取：**

```bash
# 提取私钥字符串
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# 提取公钥字符串
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

**在 Windows PowerShell 上提取：**

```powershell
# 提取私钥
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# 提取公钥
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> **⚠️ 注意：** 提取出的私钥必须保存在本地服务器（填入 `config.yaml` 的 `RsaPrivateKey` 字段），**严禁泄露**。

---

## 2. 请求验证头定义

在发起 HTTP 请求时，需要在 Header 中包含以下认证参数：

| 参数名称 | 约束 | 示例 | 说明 |
| :--- | :--- | :--- | :--- |
| `key` | 长度: 64 | `vratson2i5hjxgkd` | 商户注册后由平台分配的 API Key |
| `timestamp` | 长度: 32 | `1725076567682` | 请求发起的时间戳 (单位: 毫秒) |
| `sign` | 长度: 32 | `0592dc64d480fb119d1e07ce0601db` | 使用 MD5 算法对请求内容生成的签名 |
| `clientSign` | 长度: 512 | `9e0ccfe3915e94bcc5bfbBsC...` | 使用商户的 RSA 私钥对请求内容生成的签名 |

---

## 3. 签名算法详细说明

SDK 内部已经自动处理了所有签名逻辑。如果你需要独立开发其他语言版本，或者了解签名的底层原理，请参考以下说明。

### 3.1 `sign` 字段规则 (MD5 签名)

1. 获取平台的 `Secret`。
2. 将请求的 JSON body 按键的 ASCII 升序排序，并拼接成格式为 `key1=value1&key2=value2...` 的字符串，记为 `dataStr`。
3. 获取当前时间戳（毫秒）。
4. 拼接待加密明文：`strToHash = Secret + dataStr + timestamp`。
5. 对 `strToHash` 执行 MD5 加密，生成的结果即为 `sign`。

### 3.2 `clientSign` 字段规则 (RSA 签名)

1. 将请求参数按照键的 ASCII 升序排序。
2. 使用 `&` 字符连接所有数组值，例如：
   `address=0x038B8...&amount=10.001&coin=eth&trade_id=2022013101`
3. 使用商户的 RSA 私钥对此拼接字符串进行 `RSA-MD5` 签名，生成的结果即为 `clientSign`。

---

## 4. 平台响应签名验证

平台返回的数据中也会包含 `sign` 字段，这是平台使用其 RSA 私钥对响应数据加密的结果。

商户端收到响应后，必须使用 `config.yaml` 中的 `PlatformPubKey`（平台公钥）来验证此签名，以确保数据未被篡改。

> **💡 提示：** Rust SDK 已提供 `verify_rsa_signature(&self, map_data: &HashMap<String, String>, sign: &str) -> Result<()>` 方法供开发者快捷验证平台返回的数据。