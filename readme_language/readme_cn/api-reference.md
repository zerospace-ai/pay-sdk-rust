# API 参考

本文档详细说明了 CryptoPay Rust SDK 的所有 API 接口，包括请求参数、返回参数和示例。

## 目录

1. [全局公共信息](#1-全局公共信息)
2. [注册新用户 (create_user)](#2-注册新用户-create_user)
3. [创建钱包 (create_wallet)](#3-创建钱包-create_wallet)
4. [获取充值地址 (get_wallet_addresses)](#4-获取充值地址-get_wallet_addresses)
5. [用户提现 (user_withdraw_by_open_id)](#5-用户提现-user_withdraw_by_open_id)
6. [提现订单二次审查 (Webhook)](#6-提现订单二次审查-webhook)
7. [充值和提现回调通知 (Webhook)](#7-充值和提现回调通知-webhook)
8. [创建收银台订单 (new_order)](#8-创建收银台订单-new_order)
9. [收银台订单支付成功回调 (Webhook)](#9-收银台订单支付成功回调-webhook)
10. [查询钱包余额 (wallet_balance)](#10-查询钱包余额-wallet_balance)

---

## 1. 全局公共信息

所有通过平台返回的 JSON 响应数据均遵循以下公共结构：

| 字段名称 | 类型 | 示例 | 说明 |
| :--- | :--- | :--- | :--- |
| `code` | integer | `1` | 全局状态码，`1` 表示成功，`0` 或 `-1` 等表示失败 |
| `msg` | string | `ok` | 返回的文本描述信息 |
| `data` | object | `{"OpenId":"PT00001"}` | 返回的具体数据内容，随接口不同而变化 |
| `timestamp` | string | `"1725439986754"` | UTC 时间戳（毫秒） |
| `sign` | string | `9e0ccfe3915e...` | 平台使用其 RSA 私钥对响应数据生成的签名 |

---

## 2. 注册新用户 (create_user)

### 接口说明
创建一个新的平台用户，需要用户的唯一 ID，即 UserOpenId。

### HTTP 请求
* **URL:** `https://sandbox-api.privatex.io/sdk/user/create`
* **方法:** `POST`

### 请求参数
| 参数名称 | 必填 | 类型 | 说明 |
| :--- | :--- | :--- | :--- |
| `OpenId` | 是 | string | 推荐使用平台标准前缀 + 用户唯一 ID 组成 OpenId。 |

### 响应参数
*(包含公共信息)*
| 参数名称 | 类型 | 说明 |
| :--- | :--- | :--- |
| `data.OpenId` | string | 用户唯一 OpenId |

### 示例代码 (cURL)
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

## 3. 创建钱包 (create_wallet)

### 接口说明
为用户在指定的区块链网络上创建钱包账户。

### HTTP 请求
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/create`
* **方法:** `POST`

### 请求参数
| 参数名称 | 必填 | 类型 | 说明 |
| :--- | :--- | :--- | :--- |
| `ChainID` | 是 | string | 链 ID（见附录） |
| `OpenId` | 是 | string | 用户唯一 OpenId |

### 响应参数
*(包含公共信息)*
| 参数名称 | 类型 | 说明 |
| :--- | :--- | :--- |
| `data.address` | string | 钱包地址 |
| `data.OpenId` | string | 用户唯一 OpenId |

### 示例代码 (cURL)
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

## 4. 获取充值地址 (get_wallet_addresses)

### 接口说明
获取用户的区块链钱包充值地址（支持批量查询多条链）。

### HTTP 请求
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses`
* **方法:** `POST`

### 请求参数
| 参数名称 | 必填 | 类型 | 说明 |
| :--- | :--- | :--- | :--- |
| `OpenId` | 是 | string | 用户唯一 OpenId |
| `ChainIDs` | 是 | string | 多个链 ID，使用逗号分隔 |

### 响应参数
*(包含公共信息)*
| 参数名称 | 类型 | 说明 |
| :--- | :--- | :--- |
| `data.Addresses` | array | 地址对象列表 |

### 示例代码 (cURL)
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

## 5. 用户提现 (user_withdraw_by_open_id)

### 接口说明
发起提现操作，从合作伙伴的资金池中向用户指定的地址转账。

> **⚠️ 注意：**
> * 一旦向区块链网络发起交易，将**无法撤销或退回**。请在调用前做好风控检查。
> * 必须保证提现池中有充足的币种余额和作为手续费的 Gas 费（如 ETH/TRX 等）。

### 业务逻辑
1. 通常使用业务平台内唯一的订单号作为 `SafeCheckCode` 防止重复提现。
2. 交易会匹配平台的风控审查规则：
   * 触发自动审查通过：即刻上链，`data` 会返回交易 hash。
   * 触发二次审查 (`code=2`)：需要管理员在通道平台上人工审核。审核完成后，状态将通过 Webhook 异步通知。

### HTTP 请求
* **URL:** `https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID`
* **方法:** `POST`

### 请求参数
| 参数名称 | 必填 | 类型 | 说明 |
| :--- | :--- | :--- | :--- |
| `OpenId` | 是 | string | 用户唯一 OpenId |
| `TokenId` | 是 | string | 代币 ID (见附录)|
| `Amount` | 是 | float | 提现金额 |
| `AddressTo` | 是 | string | 目标提现地址 |
| `CallBackUrl` | 否 | string | 状态变更回调 URL|
| `SafeCheckCode` | 否 | string | 安全验证码（业务订单号）|

### 响应状态码说明
| `code` 状态码 | 说明 |
| :--- | :--- |
| `1` | 交易成功并已提交上链，`data` 返回 hash。 |
| `2` | 交易已提交，但触发了平台风控，需要进行二次审查。 |
| `0` 或 `-1` | 参数错误、余额不足或交易失败，请查看 `msg` 了解详细原因。 |

### 示例代码 (cURL)
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

## 6. 提现订单二次审查 (Webhook)

### 回调说明
当管理员在商户后台配置了“风险控制回调 URL”后，平台在处理提现交易时，会主动向该 URL 发起 HTTP 请求，询问商户业务端是否放行该笔提现。
**只有当商户接口返回 `code=0` 时，平台才会执行放行。**

> **⚠️ 注意：** 平台会为商户分配一个独立的**风险控制 RSA 公钥（PlatformRiskPubKey）**，商户需使用此公钥验证本次回调的签名是否真实。

### 平台发起的 HTTP 请求
* **方法:** `POST`
* **URL:** 商户配置的 Webhook URL `/withdrawal/order/check`

### 回调参数
| 参数名称 | 必填 | 类型 | 说明 |
| :--- | :--- | :--- | :--- |
| `safeCode` | 否 | string | 商户提交的提现订单号 (SafeCheckCode) |
| `openId` | 是 | string | 提现用户 ID |
| `tokenId` | 是 | string | 币种 ID |
| `toAddress` | 是 | string | 提现目标地址 |
| `amount` | 是 | string | 提现金额 |
| `timestamp` | 是 | int | 毫秒时间戳 |
| `sign` | 是 | string | 平台使用风险控制私钥生成的签名 |

### 业务端（商户）需响应的内容
商户接口在处理完毕后，需返回如下 JSON：

| 参数名称 | 类型 | 说明 |
| :--- | :--- | :--- |
| `code` | int | 验证结果。`0` 表示同意放行提现；其他数字表示拒绝。 |
| `timestamp` | int | 当前秒级时间戳 |
| `message` | string | 自定义描述 |
| `sign` | string | 商户使用自己的 RSA 私钥对响应数据进行签名 |

---

## 7. 充值和提现回调通知 (Webhook)

### 回调说明
平台向业务端推送代币交易状态（提现成功/失败 或 用户充值入账）的异步通知。

1. **重试机制：** 业务端必须响应返回码 `0` 才算接收成功。否则平台会按照策略（起初每 2 秒尝试 50 次，随后每 10 分钟一次）持续重试推送。
2. **状态覆盖：** 一笔交易可能会触发多次通知，请以最后一次状态为准。
3. **失败处理：** 若提现回调 `status = -1`，管理员在后台重新提交交易后，系统会再次推送新的状态通知。

### 平台发起的 HTTP 请求
* **方法:** `POST`
* **URL:** 商户提供给服务商配置的回调 URL

### 回调参数
| 参数名称 | 必填 | 类型 | 说明 |
| :--- | :--- | :--- | :--- |
| `type` | 是 | int | 业务类型：`1` 为充值；`2` 为提现。 |
| `status` | 是 | int | 交易状态：<br/> `1`: 上链成功，可通过 hash 查询。<br/> `-1`: 上链失败，可在后台处理。<br/> `2`: 触发风控，需要管理员后台手工审核。<br/> `-2`: 提现申请被商户风控接口拒绝。 |
| `openid` | 是 | string | 通道用户唯一 ID |
| `totalvalue` | 是 | string | 按交易时市场价格折算的 USDT 价值 |
| `hash` | 是 | string | 链上交易 Hash |
| `confirm` | 是 | int | 链上确认数 |
| `from` | 是 | string | 交易发起地址 |
| `to` | 是 | string | 交易接收地址 |
| `amount` | 是 | string | 交易金额 |
| `chainid` | 是 | string | 链 ID |
| `tokenid` | 是 | string | 币种 ID |
| `tokenaddress` | 是 | string | 智能合约地址 |
| `safecode` | 否 | string | 提现订单的安全验证码 (OrderID) |
| `createdtime` | 是 | string | 交易创建时间 |
| `timestamp` | 是 | string | 推送时间戳 |

### 业务端（商户）需响应的内容
接收成功请返回包含 `{"code": 0}` 的 JSON 响应体。

---

## 8. 创建收银台订单 (new_order)

### 接口说明
该接口用于商户发起支付或充值请求，系统会返回一个支付地址（收银台 URL），用户访问该地址即可完成后续支付流程。

### HTTP 请求
* **URL:** `https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder`
* **方法:** `POST`

### 请求参数
| 参数名称 | 必填 | 类型 | 说明 |
| :--- | :--- | :--- | :--- |
| `outOrderNo` | 是 | string | 商户侧唯一订单号，用于防止重复提交及对账 |
| `tokenId` | 是 | integer | 币种 ID |
| `quantity` | 是 | decimal | 订单金额/数量 |
| `notifyUrl` | 否 | string | 支付成功后的异步回调通知地址 |

### 响应参数
*(包含公共信息)*
| 参数名称 | 类型 | 说明 |
| :--- | :--- | :--- |
| `data.orderNo` | string | 平台生成的唯一订单号 |
| `data.outOrderNo` | string | 商户传入的原始订单号 |
| `data.cashierUrl` | string | **核心字段**：收银台页面地址，商户需引导用户跳转至该链接进行支付 |

### 示例代码 (cURL)
```bash
curl --location 'https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder' \
--header 'key: your_api_key' \
--header 'sign: your_md5_sign' \
--header 'clientSign: your_rsa_sign' \
--header 'Content-Type: application/json' \
--header 'timestamp: 1770606706659' \
--data '{  
  "outOrderNo": "1234623",   
  "tokenId": 6,   
  "quantity": 0.01,   
  "notifyUrl": "https://your-domain.com/callback"   
}'
```

---

## 9. 收银台订单支付成功回调 (Webhook)

### 回调说明
当用户在收银台完成支付后，DogPay 会向商户提供的 `notifyUrl` 发送异步回调通知。

### 平台发起的 HTTP 请求
* **方法:** `POST`
* **URL:** 商户提供的 `notifyUrl`

### 回调参数
| 参数名称 | 类型 | 说明 |
| :--- | :--- | :--- |
| `orderId` | string | 平台系统订单号 |
| `outOrderId` | string | 商户原始订单号 |
| `orderStatus` | string | **订单状态**：2 通常代表支付成功 |
| `tokenId` | string | 币种 ID |
| `quantityPaid` | string | **用户实付数量** |
| `sign` | string | **数据签名**（用于商户验签，防止回调被伪造） |

### 业务端（商户）需响应的内容
接收成功请返回如下 JSON：
```json
{
  "code": "1",
  "message": "success"
}
```

---

## 10. 查询钱包余额 (wallet_balance)

### 接口说明
查询指定区块链网络上某个钱包地址的代币或主币余额。

### HTTP 请求
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/balance`
* **方法:** `POST`

### 请求参数
| 参数名称 | 必填 | 类型 | 说明 |
| :--- | :--- | :--- | :--- |
| `address` | 是 | string | 钱包地址 |
| `contractAddress` | 是 | string | 合约地址或代币标识（如 `"XRP"`、`"USDT"`） |
| `chainId` | 是 | integer | 链 ID（如 `5` 为 XRP，`1` 为 Ethereum，`56` 为 BNB Chain） |

### 响应参数
*(包含公共信息)*
| 参数名称 | 类型 | 说明 |
| :--- | :--- | :--- |
| `code` | integer | 全局状态码（`1` 表示成功） |
| `msg` | string | 状态描述信息 |
| `data` | string | 余额数量（最小单位字符串） |
| `timestamp` | string | 响应时间戳（毫秒） |
| `sign` | string | 平台数据签名 |

### 示例代码 (cURL)
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

### 响应示例
```json
{
  "sign" : "",
  "timestamp" : "1725432397796",
  "data" : "1979984",
  "msg" : "ok",
  "code" : 1
}
```