# API 参考 📚

本文档详细说明了 CryptoPay Rust SDK 的所有 API 接口，包括请求参数、返回参数和示例。

## 注册新用户 (create_user)🆕🧑‍💻

### 概念
创建一个新的平台用户，需要用户的唯一 ID，即 UserOpenId。

### HTTP 请求
- 接口名称：create_user
- URL: https://sandbox-api.privatex.io/sdk/user/create
- 方法：POST

### 请求参数
| 参数名称 | 必填 | 类型   | 描述                                           |
| -------- | ---- | ------ | ---------------------------------------------- |
| OpenId   | 是   | string | 推荐使用平台标准前缀 + 用户唯一 ID 组成 OpenId |

### 返回参数
| 参数名称    | 类型   | 描述            |
| ----------- | ------ | --------------- |
| code        | int    | 全局状态码      |
| msg         | string | 状态描述        |
| data.OpenId | string | 用户唯一 OpenId |
| sign        | string | 平台签名        |

### 示例
请求示例：
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
返回示例：
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

有关认证与安全，请参阅 [🧩 authentication.md](./authentication.md)

## 创建钱包 (create_wallet) 💰

### 概念
为用户在相应区块链网络上创建钱包账户。

### HTTP 请求
- 接口名称：create_wallet
- URL: https://sandbox-api.privatex.io/sdk/wallet/create
- 方法：POST

### 请求参数
| 参数名称 | 必填 | 类型   | 描述            |
| -------- | ---- | ------ | --------------- |
| ChainID  | 是   | string | 链 ID           |
| OpenId   | 是   | string | 用户唯一 OpenId |

### 返回参数
| 参数名称     | 类型   | 描述            |
| ------------ | ------ | --------------- |
| code         | int    | 全局状态码      |
| msg          | string | 状态描述        |
| data.address | string | 钱包地址        |
| data.OpenId  | string | 用户唯一 OpenId |
| sign         | string | 平台签名        |

### 示例
请求示例：
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
返回示例：
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

## 获取充值地址 (get_wallet_addresses)💳

### 概念
获取用户的区块链钱包充值地址。

### HTTP 请求
- 接口名称：get_wallet_addresses
- URL: https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses
- 方法：POST

### 请求参数
| 参数名称 | 必填 | 类型   | 描述                  |
| -------- | ---- | ------ | --------------------- |
| OpenId   | 是   | string | 用户唯一 OpenId       |
| ChainIDs | 是   | string | 多个链 ID，用逗号分隔 |

### 返回参数
| 参数名称       | 类型   | 描述       |
| -------------- | ------ | ---------- |
| code           | int    | 全局状态码 |
| msg            | string | 状态描述   |
| data.Addresses | array  | 地址列表   |
| sign           | string | 平台签名   |

### 示例
请求示例：
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
返回示例：
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

## 用户提现 (user_withdraw_by_open_id)💸

### 概念
用户提现操作，从合作伙伴账户转移到用户指定地址。

* 功能：用户提现操作接口。提现必须从合作伙伴账户在相应代币提现池中转移到用户指定的提现钱包地址。合作伙伴可以设置安全回调地址来验证提现的合法性。如果验证有效，则可以直接从商户资金池钱包完成提现。

* 提现交易接口检查默认提现热钱包是否有足够的提现资产和手续费。

* 默认情况下，提现接口使用安全验证码作为提现交易的唯一参数要求。通常建议使用业务平台的唯一提现订单号作为安全码。提交重复的安全验证码将导致错误。

* 所有提现交易请求将与通道平台上配置的风险控制审查规则匹配。如果参数请求有效，则交易请求将被接受。符合自动审查规则的提现交易将立即提交到网络交易，并返回提交交易的hash信息（返回字段data）。需要通道二次审查的提现交易请求将返回（code=2）。无需再次提交提现请求。管理员必须在通道平台上完成二次审查。二次审查完成后，交易订单将回调通知提现交易状态变更。

* 前提：相应货币的资金池必须有足够的资金进行提现（特别是对于ETH网络代币提现，需要资金池钱包中有一定量的ETH交易费余额）。

* ⚠️ 注意：**对于区块链提现，请确保在调用接口前完成预批准过程。一旦启动区块链交易，就无法撤销或退回。**

### HTTP 请求
- 接口名称：user_withdraw_by_open_id
- URL: https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID
- 方法：POST

### 请求参数
| 参数名称      | 必填 | 类型   | 描述            |
| ------------- | ---- | ------ | --------------- |
| OpenId        | 是   | string | 用户唯一 OpenId |
| TokenId       | 是   | string | 代币 ID         |
| Amount        | 是   | float  | 提现金额        |
| AddressTo     | 是   | string | 目标地址        |
| CallBackUrl   | 否   | string | 回调 URL        |
| SafeCheckCode | 否   | string | 安全验证码      |

### 返回参数

| 参数名称  | 类型   | 描述                                                                                                                                                                                                                                                                                                                   |
| :-------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| code      | int    | 状态码</br>0 参数错误、重复订单号、提现地址格式不正确或提现钱包费用不足。详细信息可在msg中找到。</br>1 提现交易提交成功，已提交到区块链网络。提交交易的唯一hash包含在data中。</br>2 提现交易提交成功，需要二次通道审查才能完成交易。审查完成后，将通过回调更新交易信息。</br>-1 提现交易失败。您可以重新提交提现请求。 |
| msg       | string | 状态描述                                                                                                                                                                                                                                                                                                               |
| data      | string | 交易hash。如果启用智能提现，此字段将返回为空字符串。                                                                                                                                                                                                                                                                   |
| sign      | string | 平台签名                                                                                                                                                                                                                                                                                                               |
| timestamp | string | 当前时间戳，以毫秒转换为字符串                                                                                                                                                                                                                                                                                         |


### 示例
请求示例：
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

返回示例
```json
{
    "sign": "D+VTPNiwGLzh9eIvkrscwS4UlGKzdnrBgB63RDG4HeobZT6FXqUwYCPgKojynKaxwm5PkmW0xhIASZ4asSCvnYfi0NSFehchZAtUnQIispxKcjsiudWsUznbkEIQ2h2TA/mbUZ1X9+wyh7QhNo6+RkxtgRyRpVb7ARG8pL14cdTAs OTtMLO0W1GO0M83VAv2ybBZNObncX9qy6tdwLQV/KYuNJYyMN0dL0nLKYHnj9Q4d3lEDM45AVJ0153/YIiIgcFBnOWhsQ9rVARcFeXeWd9KJ5OZpmxlxnhcJGcEUY2UDC4zKLZxtUet7CPAyehAMQ5plkpvRrR3Z6lA5zl6GQ==",
    "timestamp": "1725439986754",
    "data": "94f4c29eba73d53dcd3aa1b8cf90a98108d0acf82f38b97a4032dcdf7ff172e7",
    "msg": "ok",
    "code": 1
}
```

## 提现订单二次审查 💳

* 功能：商户提现订单风险控制二次审查接口
* ⚠️ 注意：**平台为商户分配单独的风险控制 RSA 公钥（不同于充值/提现回调通知公钥）**
* 触发时间：管理员在商户端（系统设置）配置风险控制回调 URL 参数后，通道将为每个提现交易请求添加额外的风险控制回调二次审查。只有当商户端风险控制 URL 返回正确的验证通过代码时，交易才会有效提交。
* 技术要求：需要商户端技术实现和配置二次审查回调接口。

#### HTTP 请求

平台向商户发送提现审查请求

> POST: `/withdrawal/order/check`

#### 请求参数

| 参数名称  | 必填 | 类型   | 描述                                                                                |
| :-------- | :--- | :----- | :---------------------------------------------------------------------------------- |
| safeCode  | 否   | string | 商户提交的唯一交易 ID，通常对应商户的提现订单 ID（提现交易的 SafeCheckCode）        |
| openId    | 是   | string | 提交提现交易的商户用户 ID                                                           |
| tokenId   | 是   | string | 货币 ID，基于平台提供的货币 ID                                                      |
| toAddress | 是   | string | 提现地址                                                                            |
| amount    | 是   | string | 提现金额                                                                            |
| timestamp | 是   | int    | 当前时间戳                                                                          |
| sign      | 是   | string | 签名：仅对 data 字段中的参数签名；必须使用平台的风险控制 RSA 公钥验证签名的正确性。 |

#### 返回参数描述

| 参数名称  | 类型   | 描述                                                |
| :-------- | :----- | :-------------------------------------------------- |
| code      | int    | 验证结果。0 表示通过；其他代码无效。                |
| timestamp | int    | 当前时间戳，以秒为单位。                            |
| message   | string | 返回消息。                                          |
| sign      | string | 签名：商户的 RSA 私钥对响应参数中的 data 字段签名。 |

## 充值和提现回调通知

1. 充值和提现交易将触发多次回调通知。以最后一次回调通知的交易信息和状态为准。
2. 业务端需要返回有效的回调消息。格式在返回参数描述中描述。返回代码为 0 表示已处理回调消息，无需进一步通知。否则，回调将继续通知（最初每 2 秒 50 次，然后每 10 分钟一次）直到返回代码为 0 的确认消息。

联系服务提供商设置回调 URL。

> POST

* 功能：定义平台用于通知应用端代币交易信息（用户提现或充值）的回调消息格式。此消息适用于应用端关于代币交易状态（提现或充值）的事件通知。应用可以根据其应用功能选择支持回调通知接口。

### 请求参数

| 参数名称     | 必填 | 类型   | 描述                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :----------- | :--- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| openid       | 是   | string | 通道用户唯一 ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| totalvalue   | 是   | string | 充值或提现交易对应的 USDT 值（基于交易时的市场价格计算）                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| status       | 是   | int    | 交易状态：</br>1 交易完成，已成功提交到区块链网络。可以使用 hash 在链上查询交易细节。</br>-1 交易已提交到区块链网络，但链上交易失败。您可以在商户管理 --> 交易管理 --> [提交订单安全码] 中重新审查。业务平台无需处理状态变更，只需等待通道回调新状态通知。</br>-2 提现交易申请被商户后端拒绝。提现申请已过期。建议业务平台在收到通知后返回用户的提现申请。</br>2 提现交易已提交到商户管理。由于触发了配置的货币安全风险控制要求，管理员需要登录商户管理 --> 交易管理 --> 提现审查 来完成提现申请处理。</br>3 在提现交易区块链处理期间，业务平台无需更新状态变更，只需等待通道接收新状态通知。 </br>⛑️**特别提醒：对于业务平台收到的提现交易回调，如果 status = -1，将忽略回调。管理员登录管理后端并重新提交交易后，将同时向平台推送新状态通知。** |         | type | 是 | int | 1 为充值交易；2 为提现交易 |
| hash         | 是   | string | 交易 hash 值                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| confirm      | 是   | int    | 交易的链上确认数                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| createdtime  | 是   | string | 创建时间                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| from         | 是   | string | 交易发起者地址                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| to           | 是   | string | 交易接收地址                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| amount       | 是   | string | 交易金额                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| chainid      | 是   | string | 交易链 ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| tokenid      | 是   | string | 交易 tokenid                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| tokenaddress | 是   | string | 交易代币合约地址                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| safecode     | 是   | string | 适用于提现订单，通常是唯一提现订单 ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | orderid |
| timestamp    | 是   | string | 交易时间戳                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |