# API 參考

本文檔詳細說明了 CryptoPay Rust SDK 的所有 API 接口，包括請求參數、返回參數和示例。

## 目錄

1. [全局公共信息](#1-全局公共信息)
2. [註冊新用戶 (create_user)](#2-註冊新用戶-create_user)
3. [創建錢包 (create_wallet)](#3-創建錢包-create_wallet)
4. [獲取充值地址 (get_wallet_addresses)](#4-獲取充值地址-get_wallet_addresses)
5. [用戶提現 (user_withdraw_by_open_id)](#5-用戶提現-user_withdraw_by_open_id)
6. [提現訂單二次審查 (Webhook)](#6-提現訂單二次審查-webhook)
7. [充值和提現回調通知 (Webhook)](#7-充值和提現回調通知-webhook)
8. [創建收銀台訂單 (new_order)](#8-創建收銀台訂單-new_order)
9. [收銀台訂單支付成功回調 (Webhook)](#9-收銀台訂單支付成功回調-webhook)
10. [查詢錢包餘額 (wallet_balance)](#10-查詢錢包餘額-wallet_balance)

---

## 1. 全局公共信息

所有通過平台返回的 JSON 響應數據均遵循以下公共結構：

| 字段名稱 | 類型 | 示例 | 說明 |
| :--- | :--- | :--- | :--- |
| `code` | integer | `1` | 全局狀態碼，`1` 表示成功，`0` 或 `-1` 等表示失敗 |
| `msg` | string | `ok` | 返回的文本描述信息 |
| `data` | object | `{"OpenId":"PT00001"}` | 返回的具體數據內容，隨接口不同而變化 |
| `timestamp` | string | `"1725439986754"` | UTC 時間戳（毫秒） |
| `sign` | string | `9e0ccfe3915e...` | 平台使用其 RSA 私鑰對響應數據生成的簽名 |

---

## 2. 註冊新用戶 (create_user)

### 接口說明
創建一個新的平台用戶，需要用戶的唯一 ID，即 UserOpenId。

### HTTP 請求
* **URL:** `https://sandbox-api.privatex.io/sdk/user/create`
* **方法:** `POST`

### 請求參數
| 參數名稱 | 必填 | 類型 | 說明 |
| :--- | :--- | :--- | :--- |
| `OpenId` | 是 | string | 推薦使用平台標準前綴 + 用戶唯一 ID 組成 OpenId。 |

### 響應參數
*(包含公共信息)*
| 參數名稱 | 類型 | 說明 |
| :--- | :--- | :--- |
| `data.OpenId` | string | 用戶唯一 OpenId |

### 示例代碼 (cURL)
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

## 3. 創建錢包 (create_wallet)

### 接口說明
為用戶在指定的區塊鏈網絡上創建錢包賬戶。

### HTTP 請求
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/create`
* **方法:** `POST`

### 請求參數
| 參數名稱 | 必填 | 類型 | 說明 |
| :--- | :--- | :--- | :--- |
| `ChainID` | 是 | string | 鏈 ID（見附錄） |
| `OpenId` | 是 | string | 用戶唯一 OpenId |

### 響應參數
*(包含公共信息)*
| 參數名稱 | 類型 | 說明 |
| :--- | :--- | :--- |
| `data.address` | string | 錢包地址 |
| `data.OpenId` | string | 用戶唯一 OpenId |

### 示例代碼 (cURL)
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

## 4. 獲取充值地址 (get_wallet_addresses)

### 接口說明
獲取用戶的區塊鏈錢包充值地址（支持批量查詢多條鏈）。

### HTTP 請求
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses`
* **方法:** `POST`

### 請求參數
| 參數名稱 | 必填 | 類型 | 說明 |
| :--- | :--- | :--- | :--- |
| `OpenId` | 是 | string | 用戶唯一 OpenId |
| `ChainIDs` | 是 | string | 多個鏈 ID，使用逗號分隔 |

### 響應參數
*(包含公共信息)*
| 參數名稱 | 類型 | 說明 |
| :--- | :--- | :--- |
| `data.Addresses` | array | 地址對象列表 |

### 示例代碼 (cURL)
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

## 5. 用戶提現 (user_withdraw_by_open_id)

### 接口說明
發起提現操作，從合作夥伴的資金池中向用戶指定的地址轉賬。

> **⚠️ 注意：**
> * 一旦向區塊鏈網絡發起交易，將**無法撤銷或退回**。請在調用前做好風控檢查。
> * 必須保證提現池中有充足的幣種餘額和作為手續費的 Gas 費（如 ETH/TRX 等）。

### 業務邏輯
1. 通常使用業務平台內唯一的訂單號作為 `SafeCheckCode` 防止重複提現。
2. 交易會匹配平台的風控審查規則：
   * 觸發自動審查通過：即刻上鏈，`data` 會返回交易 hash。
   * 觸發二次審查 (`code=2`)：需要管理員在通道平台上人工審核。審核完成後，狀態將通過 Webhook 異步通知。

### HTTP 請求
* **URL:** `https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID`
* **方法:** `POST`

### 請求參數
| 參數名稱 | 必填 | 類型 | 說明 |
| :--- | :--- | :--- | :--- |
| `OpenId` | 是 | string | 用戶唯一 OpenId |
| `TokenId` | 是 | string | 代幣 ID (見附錄)|
| `Amount` | 是 | float | 提現金額 |
| `AddressTo` | 是 | string | 目標提現地址 |
| `CallBackUrl` | 否 | string | 狀態變更回調 URL|
| `SafeCheckCode` | 否 | string | 安全驗證碼（業務訂單號）|

### 響應狀態碼說明
| `code` 狀態碼 | 說明 |
| :--- | :--- |
| `1` | 交易成功並已提交上鏈，`data` 返回 hash。 |
| `2` | 交易已提交，但觸發了平台風控，需要進行二次審查。 |
| `0` 或 `-1` | 參數錯誤、餘額不足或交易失敗，請查看 `msg` 了解詳細原因。 |

### 示例代碼 (cURL)
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

## 6. 提現訂單二次審查 (Webhook)

### 回調說明
當管理員在商戶後台配置了“風險控制回調 URL”後，平台在處理提現交易時，會主動向該 URL 發起 HTTP 請求，詢問商戶業務端是否放行該筆提現。
**只有當商戶接口返回 `code=0` 時，平台才會執行放行。**

> **⚠️ 注意：** 平台會為商戶分配一個獨立的**風險控制 RSA 公鑰（PlatformRiskPubKey）**，商戶需使用此公鑰驗證本次回調的簽名是否真實。

### 平台發起的 HTTP 請求
* **方法:** `POST`
* **URL:** 商戶配置的 Webhook URL `/withdrawal/order/check`

### 回調參數
| 參數名稱 | 必填 | 類型 | 說明 |
| :--- | :--- | :--- | :--- |
| `safeCode` | 否 | string | 商戶提交的提現訂單號 (SafeCheckCode) |
| `openId` | 是 | string | 提現用戶 ID |
| `tokenId` | 是 | string | 幣種 ID |
| `toAddress` | 是 | string | 提現目標地址 |
| `amount` | 是 | string | 提現金額 |
| `timestamp` | 是 | int | 毫秒時間戳 |
| `sign` | 是 | string | 平台使用風險控制私鑰生成的簽名 |

### 業務端（商戶）需響應的內容
商戶接口在處理完畢後，需返回如下 JSON：

| 參數名稱 | 類型 | 說明 |
| :--- | :--- | :--- |
| `code` | int | 驗證結果。`0` 表示同意放行提現；其他數字表示拒絕。 |
| `timestamp` | int | 當前秒級時間戳 |
| `message` | string | 自定義描述 |
| `sign` | string | 商戶使用自己的 RSA 私鑰對響應數據進行簽名 |

---

## 7. 充值和提現回調通知 (Webhook)

### 回調說明
平台向業務端推送代幣交易狀態（提現成功/失敗 或 用戶充值入賬）的異步通知。

1. **重試機制：** 業務端必須響應返回碼 `0` 才算接收成功。否則平台會按照策略（起初每 2 秒嘗試 50 次，隨後每 10 分鐘一次）持續重試推送。
2. **狀態覆蓋：** 一筆交易可能會觸發多次通知，請以最後一次狀態為準。
3. **失敗處理：** 若提現回調 `status = -1`，管理員在後台重新提交交易後，系統會再次推送新的狀態通知。

### 平台發起的 HTTP 請求
* **方法:** `POST`
* **URL:** 商戶提供給服務商配置的回調 URL

### 回調參數
| 參數名稱 | 必填 | 類型 | 說明 |
| :--- | :--- | :--- | :--- |
| `type` | 是 | int | 業務類型：`1` 為充值；`2` 為提現。 |
| `status` | 是 | int | 交易狀態：<br/> `1`: 上鏈成功，可通過 hash 查詢。<br/> `-1`: 上鏈失敗，可在後台處理。<br/> `2`: 觸發風控，需要管理員後台手工審核。<br/> `-2`: 提現申請被商戶風控接口拒絕。 |
| `openid` | 是 | string | 通道用戶唯一 ID |
| `totalvalue` | 是 | string | 按交易時市場價格折算的 USDT 價值 |
| `hash` | 是 | string | 鏈上交易 Hash |
| `confirm` | 是 | int | 鏈上確認數 |
| `from` | 是 | string | 交易發起地址 |
| `to` | 是 | string | 交易接收地址 |
| `amount` | 是 | string | 交易金額 |
| `chainid` | 是 | string | 鏈 ID |
| `tokenid` | 是 | string | 幣種 ID |
| `tokenaddress` | 是 | string | 智能合約地址 |
| `safecode` | 否 | string | 提現訂單的安全驗證碼 (OrderID) |
| `createdtime` | 是 | string | 交易創建時間 |
| `timestamp` | 是 | string | 推送時間戳 |

### 業務端（商戶）需響應的內容
接收成功請返回包含 `{"code": 0}` 的 JSON 響應體。

---

## 8. 創建收銀台訂單 (new_order)

### 接口說明
該接口用於商戶發起支付或充值請求，系統會返回收款地址和收銀台頁面鏈接（Cashier URL），用戶訪問該地址即可完成後續支付流程。

### HTTP 請求
* **URL:** `https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder`
* **方法:** `POST`

### 請求參數
| 參數名稱 | 必填 | 類型 | 說明 |
| :--- | :--- | :--- | :--- |
| `outOrderNo` | 是 | string | 商戶側唯一訂單號，全平台唯一，用於防止重複提交及對賬（建議帶商戶前綴） |
| `outUserId` | 視模式 | string | 商戶側用戶 ID。**未開啟地址池模式時必填**（否則返回 `10502` 參數錯誤）；已開啟地址池模式（`address_reuse_mode=1`）時可不傳 |
| `tokenId` | 是 | integer | 幣種 ID（同時決定鏈） |
| `quantity` | 是 | decimal | 應付數量（幣的數量，必須 `> 0`；建議用普通小數寫法如 `0.01`，不要用科學計數法） |
| `notifyUrl` | 否 | string | 支付完成後的異步回調通知地址（不傳時回調至商戶後台配置的默認回調地址） |
| `orderDetail` | 否 | string | 訂單描述，原樣保存 |

> **兩種地址模式說明（`outUserId`）：**
> * **固定地址模式：** 傳入 `outUserId`（無論商戶是否開啟地址池），按 `(商戶, outUserId, 鏈)` 分配固定地址。未開啟地址池模式的商戶**必須**傳 `outUserId`，否則返回 `10502`。
> * **地址池模式：** 不傳 `outUserId`（須省略該字段，不能傳空串）且商戶開啟了地址復用 `address_reuse_mode=1`，從空閒地址池分配獨佔地址。注意：此模式下響應和回調中的 `outUserId` 為該池地址原綁定用戶 ID，對賬請使用 `outOrderNo` / `orderNo`（回調為 `outOrderId` / `orderId`）。

### 響應參數
*(包含公共信息)*
| 參數名稱 | 類型 | 說明 |
| :--- | :--- | :--- |
| `data.orderNo` | string | 平台生成的唯一訂單號 |
| `data.outOrderNo` | string | 商戶傳入的原始訂單號 |
| `data.outUserId` | string | 訂單綁定的用戶 ID（地址池模式下為池地址原綁定用戶 ID） |
| `data.tokenId` | integer | 幣種 ID |
| `data.quantity` | string | 應付數量（JSON 中為字符串） |
| `data.addresses` | string | 收款地址（單個地址，區分大小寫，請原樣展示） |
| `data.cashierUrl` | string | **核心字段**：收銀台頁面地址，商戶需引導用戶跳轉至該鏈接進行支付 |

### 示例代碼 (cURL)
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

*(若商戶已開啟地址池模式，可省略 `outUserId` 和 `orderDetail` 字段，且未傳字段不參與簽名。)*

---

## 9. 收銀台訂單支付成功回調 (Webhook)

### 回調說明
當用戶在收銀台完成全額支付（`orderStatus=2`）後，DogPay 會向訂單指定的 `notifyUrl`（未傳時使用商戶後台默認回調地址）發送異步回調通知。部分付款（`1`）和過期（`-1`）不會觸發回調。

### 平台發起的 HTTP 請求
* **方法:** `POST`
* **URL:** 商戶提供的 `notifyUrl`（或默認回調地址）

### 回調參數
| 參數名稱 | 類型 | 說明 |
| :--- | :--- | :--- |
| `orderId` | string | 平台系統訂單號（對應下單返回的 `orderNo`） |
| `outOrderId` | string | 商戶原始訂單號（對應下單時的 `outOrderNo`） |
| `outUserId` | string | 訂單綁定用戶 ID（地址池模式下為池地址原綁定用戶 ID） |
| `orderType` | string | 訂單類型（`1` = 充值代收） |
| `chainId` | string | 鏈 ID |
| `tokenId` | string | 幣種 ID |
| `quantity` | string | 應付數量 |
| `quantityPaid` | string | **用戶實付累計數量** |
| `totalAmount` | string | 應付 USD 估值 |
| `totalAmountPaid` | string | 實付 USD 估值 |
| `exchangeAddress` | string | 收款地址 |
| `orderStatus` | string | **訂單狀態**：回調時固定為 `2`（已完成） |
| `txHash` | string | 最後一筆匹配到該訂單的鏈上交易哈希 |
| `createdAt` | string | 訂單創建時間 |
| `timestamp` | string | 回調發送時間戳（毫秒） |
| `sign` | string | **數據簽名**（用於商戶使用平台公鑰驗簽，防止回調被偽造） |

### 業務端（商戶）需響應內容
接收成功請返回如下 JSON（`code` 為 `"0"` 或數字 `0` 即視為成功，否則平台會繼續重試）：
```json
{
  "code": "0",
  "message": "success"
}
```

---

## 10. 查詢錢包餘額 (wallet_balance)

### 接口說明
查詢指定區塊鏈網絡上某個錢包地址的代幣或主幣餘額。

### HTTP 請求
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/balance`
* **方法:** `POST`

### 請求參數
| 參數名稱 | 必填 | 類型 | 說明 |
| :--- | :--- | :--- | :--- |
| `address` | 是 | string | 錢包地址 |
| `contractAddress` | 是 | string | 合約地址或代幣標識（如 `"XRP"`、`"USDT"`） |
| `chainId` | 是 | integer | 鏈 ID（如 `5` 為 XRP，`1` 為 Ethereum，`56` 為 BNB Chain） |

### 響應參數
*(包含公共信息)*
| 參數名稱 | 類型 | 說明 |
| :--- | :--- | :--- |
| `code` | integer | 全局狀態碼（`1` 表示成功） |
| `msg` | string | 狀態描述信息 |
| `data` | string | 餘額數量（最小單位字符串） |
| `timestamp` | string | 響應時間戳（毫秒） |
| `sign` | string | 平台數據簽名 |

### 示例代碼 (cURL)
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

### 響應示例
```json
{
  "sign" : "",
  "timestamp" : "1725432397796",
  "data" : "1979984",
  "msg" : "ok",
  "code" : 1
}
```