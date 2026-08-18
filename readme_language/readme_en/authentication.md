# Authentication and Security

This system uses a strict request signature mechanism to ensure data security between the merchant and the platform. The communication process involves MD5 signatures and RSA signature verification.

## 1. Generate RSA Key Pair

Merchants need to generate their own RSA key pair, use the private key to sign requests, and submit the public key to the platform.

### 1.1 Generate Key Pair using OpenSSL

Execute the following commands on Mac, Linux, or Git Bash/WSL/Cygwin:

```bash
# Generate 2048-bit private key
openssl genrsa -out rsa_private_key.pem 2048

# Generate public key from private key
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

### 1.2 Extract Key Strings

The generated public key needs to have the `-----BEGIN PUBLIC KEY-----` / `-----END PUBLIC KEY-----` lines and newline characters removed, and be converted to a single-line string before submitting to the platform.

**Extract on Mac/Linux/Git Bash:**

```bash
# Extract private key string
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# Extract public key string
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

**Extract on Windows PowerShell:**

```powershell
# Extract private key
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# Extract public key
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> **⚠️ Warning:** The extracted private key must be stored securely on your local server (filled into the `RsaPrivateKey` field of `config.yaml`) and **must never be leaked**.

---

## 2. Request Validation Headers Definition

When making an HTTP request, the following authentication parameters must be included in the Header:

| Parameter Name | Constraint | Example | Description |
| :--- | :--- | :--- | :--- |
| `key` | Length: 64 | `vratson2i5hjxgkd` | API Key assigned by the platform after merchant registration |
| `timestamp` | Length: 32 | `1725076567682` | Timestamp of the request (Unit: milliseconds) |
| `sign` | Length: 32 | `0592dc64d480fb119d1e07ce0601db` | Signature generated using the MD5 algorithm on the request content |
| `clientSign` | Length: 512 | `9e0ccfe3915e94bcc5bfbBsC...` | Signature generated using the merchant's RSA private key on the request content |

---

## 3. Detailed Signature Algorithm

The SDK internally handles all signature logic automatically. If you need to develop a version in another language independently, or understand the underlying principles of the signature, please refer to the following instructions.

### 3.1 `sign` Field Rules (MD5 Signature)

1. Obtain the platform's `Secret`.
2. Sort the JSON body of the request in ascending ASCII order by key, and concatenate it into a string formatted as `key1=value1&key2=value2...`, noted as `dataStr`.
3. Obtain the current timestamp (milliseconds).
4. Splice the plaintext to be encrypted: `strToHash = Secret + dataStr + timestamp`.
5. Perform MD5 encryption on `strToHash`, and the generated result is the `sign`.

### 3.2 `clientSign` Field Rules (RSA Signature)

1. Sort the request parameters in ascending ASCII order by key.
2. Connect all array values using the `&` character, for example:
   `address=0x038B8...&amount=10.001&coin=eth&trade_id=2022013101`
3. Use the merchant's RSA private key to perform `RSA-MD5` signature on this concatenated string. The generated result is the `clientSign`.

---

## 4. Platform Response Signature Verification

The data returned by the platform will also contain a `sign` field, which is the result of the platform encrypting the response data using its RSA private key.

After the merchant receives the response, they must use `PlatformPubKey` (the platform's public key) in `config.yaml` to verify this signature to ensure the data has not been tampered with.

> **💡 Tip:** The Rust SDK provides the `verify_rsa_signature(&self, map_data: &HashMap<String, String>, sign: &str) -> Result<()>` method for developers to quickly verify the data returned by the platform.
