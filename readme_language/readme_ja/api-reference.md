# API リファレンス

このドキュメントでは、リクエストパラメーター、戻り値パラメーター、例など、CryptoPay Rust SDK のすべての API エンドポイントについて詳しく説明します。

## 目次

1. [グローバル公開情報](#1-グローバル公開情報)
2. [新規ユーザー作成 (create_user)](#2-新規ユーザー作成-create_user)
3. [ウォレット作成 (create_wallet)](#3-ウォレット作成-create_wallet)
4. [入金アドレス取得 (get_wallet_addresses)](#4-入金アドレス取得-get_wallet_addresses)
5. [ユーザー出金 (user_withdraw_by_open_id)](#5-ユーザー出金-user_withdraw_by_open_id)
6. [出金注文の二次審査 (Webhook)](#6-出金注文の二次審査-webhook)
7. [入金および出金コールバック通知 (Webhook)](#7-入金および出金コールバック通知-webhook)
8. [レジ注文の作成 (new_order)](#8-レジ注文の作成-new_order)
9. [レジ注文支払い成功コールバック (Webhook)](#9-レジ注文支払い成功コールバック-webhook)
10. [ウォレット残高照会 (wallet_balance)](#10-ウォレット残高照会-wallet_balance)

---

## 1. グローバル公開情報

プラットフォームによって返されるすべての JSON 応答データは、次のパブリック構造に従います。

| フィールド名 | タイプ | 例 | 説明 |
| :--- | :--- | :--- | :--- |
| `code` | integer | `1` | グローバルステータスコード。`1` は成功を表し、`0` または `-1` は失敗を表します。 |
| `msg` | string | `ok` | 返されるテキストの説明 |
| `data` | object | `{"OpenId":"PT00001"}` | API によって異なる特定のデータコンテンツ |
| `timestamp` | string | `"1725439986754"` | UTC タイムスタンプ (ミリ秒) |
| `sign` | string | `9e0ccfe3915e...` | 応答データに対してプラットフォームが RSA 秘密鍵を使用して生成した署名 |

---

## 2. 新規ユーザー作成 (create_user)

### API の説明
新しいプラットフォームユーザーを作成します。ユーザーの一意の ID である UserOpenId が必要です。

### HTTP リクエスト
* **URL:** `https://sandbox-api.privatex.io/sdk/user/create`
* **メソッド:** `POST`

### リクエストパラメーター
| パラメーター名 | 必須 | タイプ | 説明 |
| :--- | :--- | :--- | :--- |
| `OpenId` | はい | string | プラットフォームの標準プレフィックス + ユーザーの一意の ID を使用して OpenId を形成することをお勧めします。 |

### 応答パラメーター
*(グローバル情報を含む)*
| パラメーター名 | タイプ | 説明 |
| :--- | :--- | :--- |
| `data.OpenId` | string | ユーザーの一意の OpenId |

### コード例 (cURL)
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

## 3. ウォレット作成 (create_wallet)

### API の説明
指定されたブロックチェーンネットワークでユーザーのウォレットアカウントを作成します。

### HTTP リクエスト
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/create`
* **メソッド:** `POST`

### リクエストパラメーター
| パラメーター名 | 必須 | タイプ | 説明 |
| :--- | :--- | :--- | :--- |
| `ChainID` | はい | string | チェーン ID (付録を参照) |
| `OpenId` | はい | string | ユーザーの一意の OpenId |

### 応答パラメーター
*(グローバル情報を含む)*
| パラメーター名 | タイプ | 説明 |
| :--- | :--- | :--- |
| `data.address` | string | ウォレットアドレス |
| `data.OpenId` | string | ユーザーの一意の OpenId |

### コード例 (cURL)
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

## 4. 入金アドレス取得 (get_wallet_addresses)

### API の説明
ユーザーのブロックチェーンウォレットの入金アドレスを取得します (複数のチェーンにわたるバッチクエリをサポートします)。

### HTTP リクエスト
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses`
* **メソッド:** `POST`

### リクエストパラメーター
| パラメーター名 | 必須 | タイプ | 説明 |
| :--- | :--- | :--- | :--- |
| `OpenId` | はい | string | ユーザーの一意の OpenId |
| `ChainIDs` | はい | string | カンマ区切りの複数のチェーン ID |

### 応答パラメーター
*(グローバル情報を含む)*
| パラメーター名 | タイプ | 説明 |
| :--- | :--- | :--- |
| `data.Addresses` | array | アドレスオブジェクトのリスト |

### コード例 (cURL)
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

## 5. ユーザー出金 (user_withdraw_by_open_id)

### API の説明
出金操作を開始し、パートナーの資金プールからユーザーが指定したアドレスに資金を送金します。

> **⚠️ 警告:**
> * トランザクションがブロックチェーンネットワーク上で開始されると、**取り消したり払い戻したりすることはできません**。この API を呼び出す前に、適切なリスク管理チェックを実行してください。
> * 出金プールに十分なトークン残高とガス料金 (ETH/TRX など) があることを確認する必要があります。

### ビジネスロジック
1. 重複出金を防ぐため、ビジネスプラットフォームからの一意の注文番号が通常 `SafeCheckCode` として使用されます。
2. トランザクションはプラットフォームのリスク管理ルールと照合されます。
   * **自動承認:** 直接オンチェーンに送られ、`data` はトランザクションハッシュを返します。
   * **二次審査 (`code=2`):** チャネルプラットフォームの管理者による手動確認が必要です。審査後、ステータスは Webhook を介して非同期で通知されます。

### HTTP リクエスト
* **URL:** `https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID`
* **メソッド:** `POST`

### リクエストパラメーター
| パラメーター名 | 必須 | タイプ | 説明 |
| :--- | :--- | :--- | :--- |
| `OpenId` | はい | string | ユーザーの一意の OpenId |
| `TokenId` | はい | string | トークン ID (付録を参照) |
| `Amount` | はい | float | 出金金額 |
| `AddressTo` | はい | string | ターゲットの出金アドレス |
| `CallBackUrl` | いいえ | string | ステータス変更コールバック URL |
| `SafeCheckCode` | いいえ | string | セキュリティ検証コード (ビジネス注文 ID) |

### 応答ステータスコードの説明
| `code` | 説明 |
| :--- | :--- |
| `1` | トランザクションが成功し、オンチェーンで送信されました。`data` はハッシュを返します。 |
| `2` | トランザクションは送信されましたが、プラットフォームのリスク管理がトリガーされたため、二次審査が必要です。 |
| `0` または `-1` | パラメーターエラー、残高不足、またはトランザクションの失敗。詳細については `msg` を確認してください。 |

### コード例 (cURL)
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

## 6. 出金注文の二次審査 (Webhook)

### コールバックの説明
管理者が加盟店のバックエンドで「リスク管理コールバック URL」を構成すると、プラットフォームは出金トランザクションを処理する際にこの URL に HTTP リクエストを積極的に開始し、加盟店のビジネスエンドに出金を解放するかどうかを尋ねます。
**プラットフォームは、加盟店の API が `code=0` を返した場合にのみ資金を解放します。**

> **⚠️ 注意:** プラットフォームは、独立した **リスク管理用 RSA 公開鍵 (`PlatformRiskPubKey`)** を加盟店に割り当てます。加盟店は、この公開鍵を使用して、このコールバックの署名の信頼性を検証する必要があります。

### プラットフォームによって開始される HTTP リクエスト
* **メソッド:** `POST`
* **URL:** 加盟店が設定した Webhook URL `/withdrawal/order/check`

### コールバックパラメーター
| パラメーター名 | 必須 | タイプ | 説明 |
| :--- | :--- | :--- | :--- |
| `safeCode` | いいえ | string | 加盟店が送信した出金注文 ID (`SafeCheckCode`) |
| `openId` | はい | string | 出金するユーザーの ID |
| `tokenId` | はい | string | トークン ID |
| `toAddress` | はい | string | 出金のターゲットアドレス |
| `amount` | はい | string | 出金金額 |
| `timestamp` | はい | int | ミリ秒のタイムスタンプ |
| `sign` | はい | string | リスク管理の秘密鍵を使用してプラットフォームによって生成された署名 |

### 加盟店からの予想される応答
処理後、加盟店の API は次の JSON を返す必要があります。

| パラメーター名 | タイプ | 説明 |
| :--- | :--- | :--- |
| `code` | int | 検証結果。`0` は解放の合意を意味し、その他の数字は拒否を意味します。 |
| `timestamp` | int | 現在のタイムスタンプ (秒) |
| `message` | string | カスタムの説明 |
| `sign` | string | 加盟店は独自の RSA 秘密鍵を使用して応答データに署名します |

---

## 7. 入金および出金コールバック通知 (Webhook)

### コールバックの説明
プラットフォームは、トークンのトランザクションステータス (出金の成功/失敗、またはユーザーの入金クレジット) の非同期通知をビジネスエンドにプッシュします。

1. **再試行メカニズム:** ビジネスエンドは、正常に受信されたと見なされるには戻りコード `0` で応答する必要があります。そうでない場合、プラットフォームは戦略 (最初は 2 秒ごとに 50 回、その後は 10 分ごとに 1 回) に従ってプッシュを継続的に再試行します。
2. **ステータスの上書き:** 1 つのトランザクションが複数の通知をトリガーする場合があります。最新のステータスに依存してください。
3. **失敗の処理:** 出金コールバックが `status = -1` の場合、管理者がバックエンドでトランザクションを再送信した後、システムは新しいステータス通知を再度プッシュします。

### プラットフォームによって開始される HTTP リクエスト
* **メソッド:** `POST`
* **URL:** 加盟店からサービスプロバイダーに提供されたコールバック URL。

### コールバックパラメーター
| パラメーター名 | 必須 | タイプ | 説明 |
| :--- | :--- | :--- | :--- |
| `type` | はい | int | ビジネスタイプ: `1` は入金、`2` は出金。 |
| `status` | はい | int | トランザクションのステータス:<br/> `1`: オンチェーン成功、ハッシュ経由でクエリ可能。<br/> `-1`: オンチェーン失敗、バックエンドで処理可能。<br/> `2`: リスク管理がトリガーされました。管理バックエンドで手動確認が必要です。<br/> `-2`: 加盟店のリスク管理 API によって出金申請が拒否されました。 |
| `openid` | はい | string | チャネルユーザーの一意の ID |
| `totalvalue` | はい | string | トランザクション時の市場価格に基づいて計算された同等の USDT 額 |
| `hash` | はい | string | オンチェーン トランザクション ハッシュ |
| `confirm` | はい | int | オンチェーン 確認数 |
| `from` | はい | string | トランザクションの開始アドレス |
| `to` | はい | string | トランザクションの受信アドレス |
| `amount` | はい | string | トランザクション金額 |
| `chainid` | はい | string | チェーン ID |
| `tokenid` | はい | string | トークン ID |
| `tokenaddress` | はい | string | スマートコントラクト アドレス |
| `safecode` | いいえ | string | 出金注文のセキュリティ検証コード (OrderID) |
| `createdtime` | はい | string | トランザクション作成時間 |
| `timestamp` | はい | string | プッシュ タイムスタンプ |

### 加盟店からの予想される応答
正常に受信した場合は、`{"code": 0}` を含む JSON 応答本文を返してください。

---

## 8. レジ注文の作成 (new_order)

### API の説明
このインターフェースは、加盟店が支払いまたはチャージリクエストを開始するために使用されます。システムは受取アドレスとレジページリンク（Cashier URL）を返し、ユーザーはこのリンクにアクセスして支払い手続きを完了できます。

### HTTP リクエスト
* **URL:** `https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder`
* **メソッド:** `POST`

### リクエストパラメーター
| パラメーター名 | 必須 | タイプ | 説明 |
| :--- | :--- | :--- | :--- |
| `outOrderNo` | はい | string | 加盟店側の一意の注文番号（プラットフォーム全体で一意。重複防止のため加盟店プレフィックスの付与を推奨） |
| `outUserId` | モードによる | string | 加盟店側のユーザー ID。**アドレスプールモードが無効の場合は必須**（未指定時は `10502` エラー）。アドレスプールモード（`address_reuse_mode=1`）有効時は省略可能 |
| `tokenId` | はい | integer | トークン ID（チェーンも同時に決定） |
| `quantity` | はい | decimal | 支払数量（`> 0` 必須。`0.01` のような通常の小数表記を推奨、指数表記は避けてください） |
| `notifyUrl` | いいえ | string | 支払い完了後の非同期コールバック通知先 URL（省略時は加盟店管理画面のデフォルト通知先を使用） |
| `orderDetail` | いいえ | string | 注文の説明（そのまま保存されます） |

> **2つのアドレスモード（`outUserId` の違い）：**
> * **固定アドレスモード：** `outUserId` を指定すると `(加盟店, outUserId, チェーン)` ごとに固定アドレスが割り当てられます。アドレスプールモードが無効な加盟店は `outUserId` の指定が**必須**です（未指定は `10502`）。
> * **アドレスプールモード：** アドレス再利用（`address_reuse_mode=1`）が有効な状態で `outUserId` を省略（空文字ではなくフィールド自体を省略）すると、空きプールから専用アドレスが割り当てられます。なお、このモードではレスポンスおよびコールバックの `outUserId` はプールアドレスの元の紐付けユーザー ID となるため、照合には必ず `outOrderNo` / `orderNo`（コールバックでは `outOrderId` / `orderId`）を使用してください。

### 応答パラメーター
*(グローバル情報を含む)*
| パラメーター名 | タイプ | 説明 |
| :--- | :--- | :--- |
| `data.orderNo` | string | プラットフォームによって生成された一意の注文 ID |
| `data.outOrderNo` | string | 加盟店から渡された元の注文 ID |
| `data.outUserId` | string | 注文に紐づくユーザー ID（アドレスプールモード時はプールアドレス元の紐付けユーザー ID） |
| `data.tokenId` | integer | トークン ID |
| `data.quantity` | string | 支払数量（JSON 上は文字列） |
| `data.addresses` | string | 受取アドレス（単一アドレス。大文字・小文字を区別するためそのまま表示してください） |
| `data.cashierUrl` | string | **コアフィールド**: レジページの URL。加盟店はユーザーをこのリンクへ誘導して決済を行います |

### コード例 (cURL)
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

*(アドレスプールモード有効時は `outUserId` と `orderDetail` を省略できます。省略されたフィールドは署名計算に含まれません。)*

---

## 9. レジ注文支払い成功コールバック (Webhook)

### コールバックの説明
ユーザーが全額の支払いを完了した時点（`orderStatus=2`）でのみ、DogPay は注文時に指定された `notifyUrl`（未指定時はデフォルト通知先）へ非同期コールバック通知を送信します。一部入金（`1`）や期限切れ（`-1`）ではコールバックは送信されません。

### プラットフォームによって開始される HTTP リクエスト
* **メソッド:** `POST`
* **URL:** 加盟店から提供された `notifyUrl`（またはデフォルト通知先）

### コールバックパラメーター
| パラメーター名 | タイプ | 説明 |
| :--- | :--- | :--- |
| `orderId` | string | プラットフォームのシステム注文 ID（注文作成時の `data.orderNo`） |
| `outOrderId` | string | 加盟店の元の注文 ID（注文作成時の `outOrderNo`） |
| `outUserId` | string | 注文に紐づくユーザー ID（アドレスモードの注記を参照） |
| `orderType` | string | 注文タイプ（`1` = 入金/代収） |
| `chainId` | string | チェーン ID |
| `tokenId` | string | トークン ID |
| `quantity` | string | 支払予定数量 |
| `quantityPaid` | string | **ユーザーが実際に支払った累計数量** |
| `totalAmount` | string | 支払予定の USD 換算額 |
| `totalAmountPaid` | string | 実支払いの USD 換算額 |
| `exchangeAddress` | string | 受取アドレス |
| `orderStatus` | string | **注文ステータス**: コールバック時は `2`（完了）固定 |
| `txHash` | string | 注文に一致した最後のオンチェーン取引ハッシュ |
| `createdAt` | string | 注文作成日時 |
| `timestamp` | string | コールバック送信タイムスタンプ（ミリ秒） |
| `sign` | string | **データ署名**（偽造防止のためプラットフォーム公開鍵で検証してください） |

### 加盟店からの予想される応答
正常に受信した場合は、次の JSON を返してください（`code` が `"0"` または数値 `0` の場合のみ成功とみなされ、それ以外は再送されます）：
```json
{
  "code": "0",
  "message": "success"
}
```

---

## 10. ウォレット残高照会 (wallet_balance)

### API の説明
指定されたブロックチェーンネットワーク上の特定のウォレットアドレスのトークンまたはネイティブコインの残高を照会します。

### HTTP リクエスト
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/balance`
* **メソッド:** `POST`

### リクエストパラメーター
| パラメーター名 | 必須 | タイプ | 説明 |
| :--- | :--- | :--- | :--- |
| `address` | はい | string | ウォレットアドレス |
| `contractAddress` | はい | string | コントラクトアドレスまたはトークンシンボル（例: `"XRP"`、`"USDT"`） |
| `chainId` | はい | integer | チェーン ID（例: XRP の場合は `5`、Ethereum の場合は `1`、BNB Chain の場合は `56`） |

### 応答パラメーター
*(グローバル情報を含む)*
| パラメーター名 | タイプ | 説明 |
| :--- | :--- | :--- |
| `code` | integer | グローバルステータスコード（`1` は成功） |
| `msg` | string | ステータスの説明 |
| `data` | string | トークン/コイン残高（最小単位文字列） |
| `timestamp` | string | 応答タイムスタンプ（ミリ秒） |
| `sign` | string | プラットフォーム署名データ |

### コード例 (cURL)
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

### 応答例
```json
{
  "sign" : "",
  "timestamp" : "1725432397796",
  "data" : "1979984",
  "msg" : "ok",
  "code" : 1
}
```