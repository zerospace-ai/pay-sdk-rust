# 請求驗證頭

請求頭定義

| 參數名稱   | 約束      | 範例                               | 描述                         |
| :--------- | :-------- | :--------------------------------- | :--------------------------- |
| key        | 長度：64  | ithujj3onrzbgw5t                   | 合作夥伴 key                 |
| timestamp  | 長度：32  | 1722586649000                      | 請求發起時間戳（單位：毫秒） |
| sign       | 長度：32  | 9e0ccfe3915e94bcc5bf7dd51ad4e8d9   | 合作夥伴 secret 簽名         |
| clientSign | 長度：512 | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | 合作夥伴 RSA 簽名            |

## `sign` 字段規則

1. 註冊合作夥伴並獲取 `key` 和 `secret`。
2. 解析請求。將 JSON body 按 JSON 中的鍵的 ASCII 升序排序，並連接字符串 dataStr=key1=value1&key2=value2&key3=value3&...
3. 生成時間戳（單位：毫秒）
4. 加密生成 sign：加密前明文：strToHash = Secret+dataStr+timestamp 對明文 strToHash 進行 MD5 加密生成 sign。
具體代碼為 GenerateMD5Sign 函數。
5. 將 key、timestamp 和 sign 放置在 HTTP 頭中。

## `clientSign` 簽名算法詳細解釋

1. 獲取請求參數並格式化以獲取新的格式化字符串。

2. 使用 RSA 私鑰對步驟 1 中的數據簽名，並將簽名結果保存到變量。
生成以下參數數組的簽名字符串：`user_id = 1 coin = eth address = 0x038B8E7406dED2Be112B6c7E4681Df5316957cad amount = 10.001 trade_id = 20220131012030274786`
將數組中的每個鍵從 a 到 z 排序。如果首字母相同，則查看第二個字母，依此類推。排序後，使用 & 字符連接所有數組值，例如在 $dataString 中：
`address=0x038B8E7406dED2Be112B6c7E4681Df5316957cad&amount=10.001&coin=eth&trade_id=20220131012030274786&user_id=1`
此字符串即為連接字符串。

3. 使用私鑰對數據進行 RSA-MD5 簽名。具體代碼在 generate_md5_sign 函數中。

# 公共信息

| 名稱       | 類型      | 範例                               | 描述                                   |
| :--------- | :-------- | :--------------------------------- | :------------------------------------- |
| 全局狀態碼 | integer   | 1                                  | 1 表示成功。詳細信息請參見全局狀態碼。 |
| 消息       | string    | ok                                 | 返回文字信息。                         |
| 數據       | json      | {"OpenID":"HEX..."}                | 返回特定數據內容。                     |
| 時間       | timeStamp | 1722587274000                      | UTC 時間（無時區，以毫秒為單位）。     |
| 簽名       | string    | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | 平台使用 RSA 對所有數據簽名。          |

平台返回的 Sign 是使用 RSA 算法對響應數據加密的結果。前端應對返回數據驗證簽名。詳細信息請參考函數 func: 

pub fn verify_rsa_signature`