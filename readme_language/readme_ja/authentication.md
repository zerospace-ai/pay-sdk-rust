# 認証とセキュリティ

このシステムは、厳格なリクエスト署名メカニズムを使用して、加盟店とプラットフォーム間のデータセキュリティを確保します。通信プロセスには、MD5 署名と RSA 署名の検証が含まれます。

## 1. RSA キーペアの生成

加盟店は独自の RSA キーペアを生成し、秘密鍵を使用してリクエストに署名し、公開鍵をプラットフォームに提出する必要があります。

### 1.1 OpenSSL を使用したキーペアの生成

Mac、Linux、または Git Bash/WSL/Cygwin で次のコマンドを実行します。

```bash
# 2048 ビットの秘密鍵を生成する
openssl genrsa -out rsa_private_key.pem 2048

# 秘密鍵から公開鍵を生成する
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

### 1.2 キーストリングの抽出

生成された公開鍵は、`-----BEGIN PUBLIC KEY-----` / `-----END PUBLIC KEY-----` の行と改行文字を削除し、プラットフォームに送信する前に 1 行の文字列に変換する必要があります。

**Mac/Linux/Git Bash での抽出:**

```bash
# 秘密鍵の文字列を抽出
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# 公開鍵の文字列を抽出
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

**Windows PowerShell での抽出:**

```powershell
# 秘密鍵の抽出
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# 公開鍵の抽出
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> **⚠️ 警告:** 抽出された秘密鍵は、ローカルサーバーに安全に保存し (`config.yaml` の `RsaPrivateKey` フィールドに入力)、**絶対に漏洩させてはなりません**。

---

## 2. 検証ヘッダーの定義

HTTP リクエストを送信する際、Header には次の認証パラメーターが含まれている必要があります。

| パラメーター名 | 制約 | 例 | 説明 |
| :--- | :--- | :--- | :--- |
| `key` | 長さ: 64 | `vratson2i5hjxgkd` | 加盟店登録後にプラットフォームによって割り当てられる API Key |
| `timestamp` | 長さ: 32 | `1725076567682` | リクエストのタイムスタンプ (単位: ミリ秒) |
| `sign` | 長さ: 32 | `0592dc64d480fb119d1e07ce0601db` | リクエストコンテンツで MD5 アルゴリズムを使用して生成された署名 |
| `clientSign` | 長さ: 512 | `9e0ccfe3915e94bcc5bfbBsC...` | リクエストコンテンツで加盟店の RSA 秘密鍵を使用して生成された署名 |

---

## 3. 詳細な署名アルゴリズム

SDK は内部的にすべての署名ロジックを自動的に処理します。別の言語のバージョンを独自に開発する必要がある場合、または署名の基礎となる原則を理解する必要がある場合は、以下の手順を参照してください。

### 3.1 `sign` フィールドのルール (MD5 署名)

1. プラットフォームの `Secret` を取得します。
2. リクエストの JSON 本文をキーの ASCII 昇順で並べ替え、`key1=value1&key2=value2...` の形式の文字列に連結し、`dataStr` とします。
3. 現在のタイムスタンプ (ミリ秒) を取得します。
4. 暗号化する平文を結合します: `strToHash = Secret + dataStr + timestamp`。
5. `strToHash` に対して MD5 暗号化を実行すると、生成された結果が `sign` になります。

### 3.2 `clientSign` フィールドのルール (RSA 署名)

1. リクエストパラメーターをキーの ASCII 昇順で並べ替えます。
2. すべての配列値を `&` 文字を使用して接続します。例:
   `address=0x038B8...&amount=10.001&coin=eth&trade_id=2022013101`
3. 加盟店の RSA 秘密鍵を使用して、この連結された文字列に `RSA-MD5` 署名を実行します。生成された結果が `clientSign` になります。

---

## 4. プラットフォーム応答の署名検証

プラットフォームから返されるデータには `sign` フィールドも含まれます。これは、プラットフォームが独自の RSA 秘密鍵を使用して応答データを暗号化した結果です。

加盟店が応答を受け取った後、データが改ざんされていないことを確認するために、`config.yaml` の `PlatformPubKey` (プラットフォームの公開鍵) を使用してこの署名を検証する必要があります。

> **💡 ヒント:** Rust SDK には、開発者がプラットフォームから返されたデータをすばやく検証するための `verify_rsa_signature(&self, map_data: &HashMap<String, String>, sign: &str) -> Result<()>` メソッドが用意されています。