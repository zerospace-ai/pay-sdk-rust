# API Reference 📚

This document details all API interfaces of the CryptoPay Rust SDK, including request parameters, return parameters, and examples.

## Register New User (create_user)🆕🧑‍💻

### Concept
Create a new platform user, requiring the user's unique ID, i.e., UserOpenId.

### HTTP Request
- Interface Name: create_user
- URL: https://sandbox-api.privatex.io/sdk/user/create
- Method: POST

### Request Parameters
| Parameter Name | Required | Type   | Description                                                                 |
| -------------- | -------- | ------ | --------------------------------------------------------------------------- |
| OpenId         | Yes      | string | Recommended to use platform standard prefix + user unique ID to form OpenId |

### Return Parameters
| Parameter Name | Type   | Description        |
| -------------- | ------ | ------------------ |
| code           | int    | Global status code |
| msg            | string | Status description |
| data.OpenId    | string | User unique OpenId |
| sign           | string | Platform signature |

### Example
Request Example:
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
Return Example:
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

For Authentication & Security, please refer to [🧩 authentication.md](./authentication.md)

## Create Wallet (create_wallet) 💰

### Concept
Create a wallet account for the user on the corresponding blockchain network.

### HTTP Request
- Interface Name: create_wallet
- URL: https://sandbox-api.privatex.io/sdk/wallet/create
- Method: POST

### Request Parameters
| Parameter Name | Required | Type   | Description        |
| -------------- | -------- | ------ | ------------------ |
| ChainID        | Yes      | string | Chain ID           |
| OpenId         | Yes      | string | User unique OpenId |

### Return Parameters
| Parameter Name | Type   | Description        |
| -------------- | ------ | ------------------ |
| code           | int    | Global status code |
| msg            | string | Status description |
| data.address   | string | Wallet address     |
| data.OpenId    | string | User unique OpenId |
| sign           | string | Platform signature |

### Example
Request Example:
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
Return Example:
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

## Get Deposit Address (get_wallet_addresses)💳

### Concept
Get the user's blockchain wallet deposit address.

### HTTP Request
- Interface Name: get_wallet_addresses
- URL: https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses
- Method: POST

### Request Parameters
| Parameter Name | Required | Type   | Description                             |
| -------------- | -------- | ------ | --------------------------------------- |
| OpenId         | Yes      | string | User unique OpenId                      |
| ChainIDs       | Yes      | string | Multiple chain IDs, separated by commas |

### Return Parameters
| Parameter Name | Type   | Description        |
| -------------- | ------ | ------------------ |
| code           | int    | Global status code |
| msg            | string | Status description |
| data.Addresses | array  | Address list       |
| sign           | string | Platform signature |

### Example
Request Example:
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
Return Example:
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

## User Withdrawal (user_withdraw_by_open_id)💸

### Concept
User withdrawal operation, transfer from partner account to user-specified address.

* Function: User withdrawal operation interface. Withdrawals must be transferred from the partner's account in the corresponding token withdrawal pool to the user's specified withdrawal wallet address. Partners can set a secure callback address to verify the legitimacy of the withdrawal. If verified as valid, the withdrawal can be completed directly from the merchant's fund pool wallet.

* The withdrawal transaction interface checks whether the default withdrawal hot wallet has sufficient withdrawal assets and handling fees.

* By default, the withdrawal interface uses a security verification code as the unique parameter requirement for withdrawal transactions. It is generally recommended to use the unique withdrawal order number of the business platform as the security code. Submitting a duplicate security verification code will result in an error.

* All withdrawal transaction requests will be matched against the risk control review rules configured on the channel platform. If the parameter request is valid, the transaction request will be accepted. Withdrawal transactions that meet the automatic review rules will be immediately submitted to the network transaction and the hash information of the submitted transaction will be returned (return field data). Withdrawal transaction requests that require secondary review on the channel will return (code=2). The withdrawal request does not need to be submitted again. The administrator must complete the secondary review on the channel platform. After the secondary review is completed, the transaction order will callback to notify the withdrawal transaction status change.

* Prerequisite: The corresponding currency's fund pool must have a sufficient amount of funds to withdraw (especially for ETH network token withdrawals, which require a certain amount of ETH transaction fee balance in the fund pool wallet).

* ⚠️ Note: **For blockchain withdrawals, please ensure that the pre-approval process is complete before calling the interface. Once a blockchain transaction is initiated, it cannot be revoked or returned.**

### HTTP Request
- Interface Name: user_withdraw_by_open_id
- URL: https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID
- Method: POST

### Request Parameters
| Parameter Name | Required | Type   | Description                |
| -------------- | -------- | ------ | -------------------------- |
| OpenId         | Yes      | string | User unique OpenId         |
| TokenId        | Yes      | string | Token ID                   |
| Amount         | Yes      | float  | Withdrawal amount          |
| AddressTo      | Yes      | string | Target address             |
| CallBackUrl    | No       | string | Callback URL               |
| SafeCheckCode  | No       | string | Security verification code |

### Return Parameters

| Parameter Name | Type   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------- | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| code           | int    | Status Code</br>0 Parameter error, duplicate order number, incorrect withdrawal address format, or insufficient withdrawal wallet fees. Detailed information can be found in msg.</br>1 The withdrawal transaction was successfully submitted and has been submitted to the blockchain network. The unique hash of the submitted transaction is contained in data.</br>2 The withdrawal transaction was successfully submitted and requires secondary channel review before the transaction can be completed. After the review is completed, the transaction information will be updated through a callback.</br>-1 The withdrawal transaction failed. You can resubmit the withdrawal request. |
| msg            | string | Status Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| data           | string | Transaction hash. If smart withdrawal is enabled, this field will be returned as an empty string.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| sign           | string | Platform signature                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| timestamp      | string | Current timestamp in milliseconds converted to a string                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |


### Example
Request Example:
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

Return example
```json
{
    "sign": "D+VTPNiwGLzh9eIvkrscwS4UlGKzdnrBgB63RDG4HeobZT6FXqUwYCPgKojynKaxwm5PkmW0xhIASZ4asSCvnYfi0NSFehchZAtUnQIispxKcjsiudWsUznbkEIQ2h2TA/mbUZ1X9+wyh7QhNo6+RkxtgRyRpVb7ARG8pL14cdTAs OTtMLO0W1GO0M83VAv2ybBZNObncX9qy6tdwLQV/KYuNJYyMN0dL0nLKYHnj9Q4d3lEDM45AVJ0153/YIiIgcF BnOWhsQ9rVARcFeXeWd9KJ5OZpmxlxnhcJGcEUY2UDC4zKLZxtUet7CPAyehAMQ5plkpvRrR3Z6lA5zl6GQ==",
    "timestamp": "1725439986754",
    "data": "94f4c29eba73d53dcd3aa1b8cf90a98108d0acf82f38b97a4032dcdf7ff172e7",
    "msg": "ok",
    "code": 1
}
```

## Withdrawal Order Secondary Review 💳

* Function: Merchant withdrawal order risk control secondary review interface
* ⚠️ Note: **The platform assigns merchants a separate risk control RSA public key (different from the deposit/withdrawal callback notification public key)**
* Triggering Time: After the administrator configures the risk control callback URL parameters on the merchant side (system settings), the channel will add an additional risk control callback secondary review for each withdrawal transaction request. Only when the merchant-side risk control URL returns a correct verification pass code will the transaction be validly submitted.
* Technical Requirements: Merchant-side technical implementation and configuration of the secondary review callback interface are required.

#### HTTP Request

The platform sends a withdrawal review request to the merchant

> POST: `/withdrawal/order/check`

#### Request Parameters

| Parameter Name | Required | Type   | Description                                                                                                                                                      |
| :------------- | :------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| safeCode       | No       | string | Unique transaction ID submitted by the merchant, generally corresponding to the merchant's withdrawal order ID (SafeCheckCode for withdrawal transactions)       |
| openId         | Yes      | string | User ID of the merchant submitting the withdrawal transaction                                                                                                    |
| tokenId        | Yes      | string | Currency ID, based on the currency ID provided by the platform                                                                                                   |
| toAddress      | Yes      | string | Withdrawal address                                                                                                                                               |
| amount         | Yes      | string | Withdrawal amount                                                                                                                                                |
| timestamp      | Yes      | int    | Current timestamp                                                                                                                                                |
| sign           | Yes      | string | Signature: Only the parameters in the data field are signed; the correctness of the signature must be verified using the platform's risk control RSA public key. |

#### Return Parameter Description

| Parameter Name | Type   | Description                                                                                       |
| :------------- | :----- | :------------------------------------------------------------------------------------------------ |
| code           | int    | Verification result. 0 indicates a pass; other codes are invalid.                                 |
| timestamp      | int    | Current timestamp, in seconds.                                                                    |
| message        | string | Return message.                                                                                   |
| sign           | string | Signature: The merchant's RSA private key signature for the data field in the response parameter. |

## Deposit and Withdrawal Callback Notifications

1. Deposit and withdrawal transactions will trigger multiple callback notifications. The transaction information and status of the last callback notification will be used.
2. The business side is required to return a valid callback message. The format is described in the return parameter description. A return code of 0 indicates that the callback message has been processed and no further notifications are required. Otherwise, the callback will continue to notify (initially every 2 seconds for 50 times, and then every 10 minutes thereafter) until a confirmation message with a code of 0 is returned.

Contact the service provider to set the callback URL.

> POST

* Function: Defines the callback message format that the platform uses to notify the application side of token transaction information (user withdrawal or deposit). This message is suitable for application-side event notifications regarding token transaction status (withdrawal or deposit). Applications can optionally support the callback notification interface based on their application functionality.

### Request Parameters

| Parameter Name | Required | Type   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :------------- | :------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| openid         | yes      | string | Channel user unique ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| totalvalue     | yes      | string | USDT value corresponding to the deposit or withdrawal transaction (calculated based on the market price at the time of the transaction)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| status         | yes      | int    | Transaction status:</br>1 The transaction is complete and has been successfully submitted to the blockchain network. Transaction details can be queried on-chain using the hash.</br>-1 The transaction has been submitted to the blockchain network, but the on-chain transaction failed. You can re-review it in Merchant Management --> Transaction Management --> [Submit Order Security Code]. The business platform does not need to process the status change and can simply wait for the channel to callback the new status notification.</br>-2 The withdrawal transaction application was rejected by the merchant backend. The withdrawal application has expired. It is recommended that the business platform return the user's withdrawal application after receiving the notification.</br>2 The withdrawal transaction has been submitted to the merchant management. Because it has triggered the configured currency security risk control requirements, the administrator needs to log in to Merchant Management --> Transaction Management --> Withdrawal Review to complete the withdrawal application processing.</br>3 During the withdrawal transaction blockchain processing, the business platform does not need to update the status change and can simply wait for the channel to receive a new status notification. </br>⛑️**Special Reminder: For withdrawal transaction callbacks received by the business platform, if status = -1, the callback will be ignored. After the administrator logs in to the management backend and resubmits the transaction, a new status notification will be pushed to the platform simultaneously.** |         | type | yes | int | 1 for deposit transactions; 2 for withdrawal transactions |
| hash           | yes      | string | Transaction hash value                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| confirm        | yes      | int    | Number of on-chain confirmations for the transaction                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| createdtime    | yes      | string | Creation time                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| from           | yes      | string | Transaction initiator's address                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| to             | yes      | string | Transaction receiving address                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| amount         | yes      | string | Transaction amount                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| chainid        | yes      | string | Transaction chain ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| tokenid        | yes      | string | Transaction tokenid                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| tokenaddress   | yes      | string | Transaction token contract address                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| safecode       | yes      | string | Valid for withdrawal orders, typically a unique withdrawal order ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | orderid |
| timestamp      | yes      | string | Transaction timestamp                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| tag            | no       | string | Optional, for XRP and EOS                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| sign           | yes      | string | Platform signature data **The recipient can use the platform public key to verify the authenticity of the data returned by the platform. It is strongly recommended that the recipient verify the validity of the signature**                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |

Callback Notification Example

```json
{
    "amount": "23.0000000000000000000",
    "chainid": "2",
    "confirm": "1",
    "createdtime": "1732105978000",
    "from": "TPQmWeYVUmW4ZP",
    "hash": "b180f4184be91e12124b01089",
    "safecode": "safecode00001",
    "safecode": "",
    "sign":"",
    "status": "1",
    "timestamp": "1732105988040",
    "to": "TWLd7av6Lumoz9XAUkS8mPG7R51UstVLux",
    "tokenaddress": "TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t",
    "tokenid": "4",
    "totalvalue": "23.00000000",
    "type": "1"
}
```
