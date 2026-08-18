# 인증 및 보안

이 시스템은 가맹점과 플랫폼 간의 데이터 보안을 보장하기 위해 엄격한 요청 서명 메커니즘을 사용합니다. 통신 프로세스에는 MD5 서명과 RSA 서명 검증이 포함됩니다.

## 1. RSA 키 쌍 생성

가맹점은 자체 RSA 키 쌍을 생성하고 프라이빗 키를 사용하여 요청에 서명하고 퍼블릭 키를 플랫폼에 제출해야 합니다.

### 1.1 OpenSSL을 사용하여 키 쌍 생성

Mac, Linux 또는 Git Bash/WSL/Cygwin에서 다음 명령을 실행합니다.

```bash
# 2048비트 프라이빗 키 생성
openssl genrsa -out rsa_private_key.pem 2048

# 프라이빗 키에서 퍼블릭 키 생성
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

### 1.2 키 문자열 추출

생성된 퍼블릭 키는 플랫폼에 제출하기 전에 `-----BEGIN PUBLIC KEY-----` / `-----END PUBLIC KEY-----` 줄과 줄 바꿈 문자를 제거하고 한 줄 문자열로 변환해야 합니다.

**Mac/Linux/Git Bash에서 추출:**

```bash
# 프라이빗 키 문자열 추출
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# 퍼블릭 키 문자열 추출
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

**Windows PowerShell에서 추출:**

```powershell
# 프라이빗 키 추출
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# 퍼블릭 키 추출
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> **⚠️ 경고:** 추출된 프라이빗 키는 로컬 서버에 안전하게 저장해야 하며(`config.yaml`의 `RsaPrivateKey` 필드에 입력) **절대 유출되어서는 안 됩니다**.

---

## 2. 검증 헤더 정의

HTTP 요청을 수행할 때 헤더에 다음 인증 매개변수가 포함되어야 합니다.

| 매개변수 이름 | 제약 조건 | 예 | 설명 |
| :--- | :--- | :--- | :--- |
| `key` | 길이: 64 | `vratson2i5hjxgkd` | 가맹점 등록 후 플랫폼에서 할당한 API Key |
| `timestamp` | 길이: 32 | `1725076567682` | 요청의 타임스탬프 (단위: 밀리초) |
| `sign` | 길이: 32 | `0592dc64d480fb119d1e07ce0601db` | 요청 내용에 MD5 알고리즘을 사용하여 생성된 서명 |
| `clientSign` | 길이: 512 | `9e0ccfe3915e94bcc5bfbBsC...` | 요청 내용에 가맹점의 RSA 프라이빗 키를 사용하여 생성된 서명 |

---

## 3. 세부 서명 알고리즘

SDK는 내부적으로 모든 서명 로직을 자동으로 처리합니다. 다른 언어의 버전을 독립적으로 개발하거나 서명의 기본 원리를 이해해야 하는 경우 다음 지침을 참조하십시오.

### 3.1 `sign` 필드 규칙 (MD5 서명)

1. 플랫폼의 `Secret`을 얻습니다.
2. 요청의 JSON 본문을 키별로 오름차순 ASCII 순서로 정렬하고 `key1=value1&key2=value2...` 형식의 문자열로 연결하여 `dataStr`로 기록합니다.
3. 현재 타임스탬프(밀리초)를 얻습니다.
4. 암호화할 일반 텍스트를 결합합니다: `strToHash = Secret + dataStr + timestamp`.
5. `strToHash`에 대해 MD5 암호화를 수행하고 생성된 결과는 `sign`입니다.

### 3.2 `clientSign` 필드 규칙 (RSA 서명)

1. 요청 매개변수를 키별로 오름차순 ASCII 순서로 정렬합니다.
2. `&` 문자를 사용하여 모든 배열 값을 연결합니다. 예:
   `address=0x038B8...&amount=10.001&coin=eth&trade_id=2022013101`
3. 가맹점의 RSA 프라이빗 키를 사용하여 이 연결된 문자열에 `RSA-MD5` 서명을 수행합니다. 생성된 결과는 `clientSign`입니다.

---

## 4. 플랫폼 응답 서명 검증

플랫폼에서 반환된 데이터에는 플랫폼이 RSA 프라이빗 키를 사용하여 응답 데이터를 암호화한 결과인 `sign` 필드도 포함됩니다.

가맹점은 응답을 받은 후 데이터가 변조되지 않았는지 확인하기 위해 `config.yaml`의 `PlatformPubKey`(플랫폼 퍼블릭 키)를 사용하여 이 서명을 검증해야 합니다.

> **💡 팁:** Rust SDK는 개발자가 플랫폼에서 반환된 데이터를 빠르게 검증할 수 있도록 `verify_rsa_signature(&self, map_data: &HashMap<String, String>, sign: &str) -> Result<()>` 메서드를 제공합니다.