# API 參考 📚

本文件詳細說明 CryptoPay Rust SDK 的所有 API 接口，包括請求參數、返回參數和範例。

## 註冊新用戶 (create_user)🆕🧑‍💻

### 概念
創建一個新的平台用戶，需要用戶的唯一 ID，即 UserOpenId。

### HTTP 請求
- 接口名稱：create_user
- URL：https://sandbox-api.privatex.io/sdk/user/create
- 方法：POST

### 請求參數
| 參數名稱 | 必填 | 類型   | 描述                                           |
| -------- | ---- | ------ | ---------------------------------------------- |
| OpenId   | 是   | string | 建議使用平台標準前綴 + 用戶唯一 ID 組成 OpenId |

### 返回參數
| 參數名稱    | 類型   | 描述            |
| ----------- | ------ | --------------- |
| code        | int    | 全局狀態碼      |
| msg         | string | 狀態描述        |
| data.OpenId | string | 用戶唯一 OpenId |
| sign        | string | 平台簽名        |

### 範例
請求範例：
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
返回範例：
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

有關認證與安全，請參考 [🧩 authentication.md](./authentication.md)

## 創建錢包 (create_wallet) 💰

### 概念
為用戶在相應的區塊鏈網絡上創建錢包帳戶。

### HTTP 請求
- 接口名稱：create_wallet
- URL：https://sandbox-api.privatex.io/sdk/wallet/create
- 方法：POST

### 請求參數
| 參數名稱 | 必填 | 類型   | 描述            |
| -------- | ---- | ------ | --------------- |
| ChainID  | 是   | string | 鏈 ID           |
| OpenId   | 是   | string | 用戶唯一 OpenId |

### 返回參數
| 參數名稱     | 類型   | 描述            |
| ------------ | ------ | --------------- |
| code         | int    | 全局狀態碼      |
| msg          | string | 狀態描述        |
| data.address | string | 錢包地址        |
| data.OpenId  | string | 用戶唯一 OpenId |
| sign         | string | 平台簽名        |

### 範例
請求範例：
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
返回範例：
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

## 獲取充值地址 (get_wallet_addresses)💳

### 概念
獲取用戶的區塊鏈錢包充值地址。

### HTTP 請求
- 接口名稱：get_wallet_addresses
- URL：https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses
- 方法：POST

### 請求參數
| 參數名稱 | 必填 | 類型   | 描述                  |
| -------- | ---- | ------ | --------------------- |
| OpenId   | 是   | string | 用戶唯一 OpenId       |
| ChainIDs | 是   | string | 多個鏈 ID，用逗號分隔 |

### 返回參數
| 參數名稱       | 類型   | 描述       |
| -------------- | ------ | ---------- |
| code           | int    | 全局狀態碼 |
| msg            | string | 狀態描述   |
| data.Addresses | array  | 地址列表   |
| sign           | string | 平台簽名   |

### 範例
請求範例：
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
返回範例：
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

## 用戶提現 (user_withdraw_by_open_id)💸

### 概念
用戶提現操作，從合作夥伴帳戶轉移到用戶指定的地址。

* 功能：用戶提現操作接口。提現必須從合作夥伴在相應代幣提現池中的帳戶轉移到用戶指定的提現錢包地址。合作夥伴可以設置安全回調地址來驗證提現的合法性。如果驗證有效，則可以直接從商家的資金池錢包完成提現。

* 提現交易接口檢查默認提現熱錢包是否有足夠的提現資產和手續費。

* 默認情況下，提現接口使用安全驗證碼作為提現交易的唯一參數要求。通常建議使用業務平台的唯一提現訂單號作為安全碼。提交重複的安全驗證碼將導致錯誤。

* 所有提現交易請求將與通道平台上配置的風險控制審核規則匹配。如果參數請求有效，則交易請求將被接受。符合自動審核規則的提現交易將立即提交到網絡交易，並返回提交交易的哈希信息（返回字段 data）。需要通道二次審核的提現交易請求將返回（code=2）。無需再次提交提現請求。管理員必須在通道平台上完成二次審核。二次審核完成後，交易訂單將回調通知提現交易狀態變化。

* 先決條件：相應貨幣的資金池必須有足夠的資金來提現（特別是對於 ETH 網絡代幣提現，需要資金池錢包中有一定數量的 ETH 交易費餘額）。

* ⚠️ 注意：**對於區塊鏈提現，請確保在調用接口之前完成預審批流程。一旦啟動區塊鏈交易，就無法撤銷或退回。**

### HTTP 請求
- 接口名稱：user_withdraw_by_open_id
- URL：https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID
- 方法：POST

### 請求參數
| 參數名稱      | 必填 | 類型   | 描述            |
| ------------- | ---- | ------ | --------------- |
| OpenId        | 是   | string | 用戶唯一 OpenId |
| TokenId       | 是   | string | 代幣 ID         |
| Amount        | 是   | float  | 提現金額        |
| AddressTo     | 是   | string | 目標地址        |
| CallBackUrl   | 否   | string | 回調 URL        |
| SafeCheckCode | 否   | string | 安全驗證碼      |

### 返回參數

| 參數名稱  | 類型   | 描述                                                                                                                                                                                                                                                                                                                         |
| :-------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| code      | int    | 狀態碼</br>0 參數錯誤、重複訂單號、不正確的提現地址格式或提現錢包費用不足。詳細信息可在 msg 中找到。</br>1 提現交易成功提交並已提交到區塊鏈網絡。提交交易的唯一哈希包含在 data 中。</br>2 提現交易成功提交，需要二次通道審核才能完成交易。審核完成後，將通過回調更新交易信息。</br>-1 提現交易失敗。您可以重新提交提現請求。 |
| msg       | string | 狀態描述                                                                                                                                                                                                                                                                                                                     |
| data      | string | 交易哈希。如果啟用智能提現，此字段將返回空字符串。                                                                                                                                                                                                                                                                           |
| sign      | string | 平台簽名                                                                                                                                                                                                                                                                                                                     |
| timestamp | string | 當前時間戳，以毫秒轉換為字符串                                                                                                                                                                                                                                                                                               |


### 範例
請求範例：
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

返回範例
```json
{
    "sign": "D+VTPNiwGLzh9eIvkrscwS4UlGKzdnrBgB63RDG4HeobZT6FXqUwYCPgKojynKaxwm5PkmW0xhIASZ4asSCvnYfi0NSFehchZAtUnQIispxKcjsiudWsUznbkEIQ2h2TA/mbUZ1X9+wyh7QhNo6+RkxtgRyRpVb7ARG8pL14cdTAs OTtMLO0W1GO0M83VAv2ybBZNObncX9qy6tdwLQV/KYuNJYyMN0dL0nLKYHnj9Q4d3lEDM45AVJ0153/YIiIgcF BnOWhsQ9rVARcFeXeWd9KJ5OZpmxlxnhcJGcEUY2UDC4zKLZxtUet7CPAyehAMQ5plkpvRrR3Z6lA5zl6GQ==",
    "timestamp": "1725439986754",
    "data": "94f4c29eba73d53dcd3aa1b8cf90a98108d0acf82f38b97a4032dcdf7ff172e7",
    "msg": "ok",
    "code": 1
}
```

## 提現訂單二次審核 💳

* 功能：商家提現訂單風險控制二次審核接口
* ⚠️ 注意：**平台為商家分配單獨的風險控制 RSA 公鑰（不同於充提回調通知公鑰）**
* 觸發時間：管理員在商家端（系統設置）配置風險控制回調 URL 參數後，通道將為每個提現交易請求添加額外的風險控制回調二次審核。只有當商家端風險控制 URL 返回正確的驗證通過代碼時，交易才有效提交。
* 技術要求：需要商家端技術實現和配置二次審核回調接口。

#### HTTP 請求

平台向商家發送提現審核請求

> POST: `/withdrawal/order/check`

#### 請求參數

| 參數名稱  | 必填 | 類型   | 描述                                                                                |
| :-------- | :--- | :----- | :---------------------------------------------------------------------------------- |
| safeCode  | 否   | string | 商家提交的唯一交易 ID，通常對應商家的提現訂單 ID（提現交易的 SafeCheckCode）        |
| openId    | 是   | string | 提交提現交易的商家用戶 ID                                                           |
| tokenId   | 是   | string | 貨幣 ID，基於平台提供的貨幣 ID                                                      |
| toAddress | 是   | string | 提現地址                                                                            |
| amount    | 是   | string | 提現金額                                                                            |
| timestamp | 是   | int    | 當前時間戳                                                                          |
| sign      | 是   | string | 簽名：僅對 data 字段中的參數簽名；必須使用平台的風險控制 RSA 公鑰驗證簽名的正確性。 |

#### 返回參數描述

| 參數名稱  | 類型   | 描述                                              |
| :-------- | :----- | :------------------------------------------------ |
| code      | int    | 驗證結果。0 表示通過；其他代碼無效。              |
| timestamp | int    | 當前時間戳，以秒為單位。                          |
| message   | string | 返回消息。                                        |
| sign      | string | 簽名：商家 RSA 私鑰對響應參數中 data 字段的簽名。 |

## 充提回調通知

1. 充提交易將觸發多次回調通知。以最後一次回調通知的交易信息和狀態為準。
2. 業務端需要返回有效的回調消息。格式在返回參數描述中說明。返回代碼為 0 表示已處理回調消息，無需進一步通知。否則，回調將繼續通知（最初每 2 秒 50 次，之後每 10 分鐘一次）直到返回代碼為 0 的確認消息。

聯繫服務提供商設置回調 URL。

> POST

* 功能：定義平台用於通知應用端代幣交易信息（用戶提現或充值）的回調消息格式。此消息適用於應用端有關代幣交易狀態（提現或充值）的事件通知。應用可以根據其應用功能選擇支持回調通知接口。

### 請求參數

| 參數名稱     | 必填 | 類型   | 描述                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :----------- | :--- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| openid       | 是   | string | 通道用戶唯一 ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| totalvalue   | 是   | string | 充提交易對應的 USDT 價值（基於交易時的市場價格計算）                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| status       | 是   | int    | 交易狀態：</br>1 交易完成並已成功提交到區塊鏈網絡。可以使用哈希在鏈上查詢交易細節。</br>-1 交易已提交到區塊鏈網絡，但鏈上交易失敗。您可以在商家管理 --> 交易管理 --> [提交訂單安全碼] 中重新審核。業務平台無需處理狀態變化，只需等待通道回調新狀態通知。</br>-2 提現交易申請被商家後端拒絕。提現申請已過期。建議業務平台在收到通知後返回用戶的提現申請。</br>2 提現交易已提交到商家管理。由於觸發了配置的貨幣安全風險控制要求，管理員需要登錄商家管理 --> 交易管理 --> 提現審核來完成提現申請處理。</br>3 在提現交易區塊鏈處理期間，業務平台無需更新狀態變化，只需等待通道接收新狀態通知。 </br>⛑️**特別提醒：對於業務平台收到的提現交易回調，如果 status = -1，將忽略回調。管理員登錄管理後端並重新提交交易後，將同時向平台推送新狀態通知。** |         | type | 是 | int | 1 為充值交易；2 為提現交易 |
| hash         | 是   | string | 交易哈希值                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| confirm      | 是   | int    | 交易的鏈上確認數                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| createdtime  | 是   | string | 創建時間                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| from         | 是   | string | 交易發起方地址                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| to           | 是   | string | 交易接收方地址                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| amount       | 是   | string | 交易金額                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| chainid      | 是   | string | 交易鏈 ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| tokenid      | 是   | string | 交易 tokenid                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| tokenaddress | 是   | string | 交易代幣合約地址                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| safecode     | 是   | string | 適用於提現訂單，通常是唯一提現訂單 ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | orderid |
| timestamp    | 是   | string | 交易時間戳                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |