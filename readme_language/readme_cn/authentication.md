# 请求验证头

请求头定义

| 参数名称 | 约束 | 示例 | 描述 |
| :--------- | :-------- | :--------------------------------- | :----------------------------- |
| key | 长度: 64 | ithujj3onrzbgw5t | 合作伙伴 key |
| timestamp | 长度: 32 | 1722586649000 | 请求发起时间戳 (单位: 毫秒) |
| sign | 长度: 32 | 9e0ccfe3915e94bcc5bf7dd51ad4e8d9 | 合作伙伴 secret 签名 |
| clientSign | 长度: 512 | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | 合作伙伴 RSA 签名 |

## `sign` 字段规则

1. 注册合作伙伴并获取 `key` 和 `secret`。
2. 解析请求。将 JSON body 按键的 ASCII 升序排序，并连接字符串 dataStr=key1=value1&key2=value2&key3=value3&...
3. 生成时间戳 (单位: 毫秒)
4. 加密生成 sign: 加密前的明文: strToHash = Secret+dataStr+timestamp 对明文 strToHash 执行 MD5 加密生成 sign。
具体代码为 GenerateMD5Sign 函数。
5. 将 key、timestamp 和 sign 放置在 HTTP 头中。

## `clientSign` 签名算法详细说明

1. 获取请求参数并格式化以获得新的格式化字符串。

2. 使用 RSA 私钥对步骤 1 中的数据签名，并将签名结果保存到变量。
为以下参数数组生成签名字符串: `user_id = 1 coin = eth address = 0x038B8E7406dED2Be112B6c7E4681Df5316957cad amount = 10.001 trade_id = 20220131012030274786`
将数组中的每个键从 a 到 z 排序。如果第一个字母相同，则查看第二个字母，依此类推。排序后，使用 & 字符连接所有数组值，例如在 $dataString 中:
`address=0x038B8E7406dED2Be112B6c7E4681Df5316957cad&amount=10.001&coin=eth&trade_id=20220131012030274786&user_id=1`
此字符串即为连接字符串。

3. 使用私钥对数据进行 RSA-MD5 签名。具体代码在 generate_md5_sign 函数中。

# 公共信息

| 名称 | 类型 | 示例 | 描述 |
| :--------- | :-------- | :--------------------------------- | :--------------------------------- |
| 全局状态码 | integer | 1 | 1 表示成功。详情见全局状态码。 |
| 消息 | string | ok | 返回文本信息。 |
| 数据 | json | {"OpenID":"HEX..."} | 返回具体数据内容。 |
| 时间 | timeStamp | 1722587274000 | UTC 时间（无时区，以毫秒为单位）。 |
| 签名 | string | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | 平台使用 RSA 对所有数据签名。 |

平台返回的 Sign 是使用 RSA 算法对响应数据加密的结果。前端应针对返回的数据验证签名。详情请参阅函数 func: 

`pub fn verify_rsa_signature`