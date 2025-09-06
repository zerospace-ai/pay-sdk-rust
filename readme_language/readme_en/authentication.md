
# Request Verification Header

Request Header Definition

| Parameter Name | Constraints | Example                            | Description                                          |
| :------------- | :---------- | :--------------------------------- | :--------------------------------------------------- |
| key            | Length: 64  | ithujj3onrzbgw5t                   | Partner key                                          |
| timestamp      | Length: 32  | 1722586649000                      | Timestamp of request initiation (unit: milliseconds) |
| sign           | Length: 32  | 9e0ccfe3915e94bcc5bf7dd51ad4e8d9   | Partner secret signature                             |
| clientSign     | Length: 512 | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | Partner RSA signature                                |

## `sign` Field Rules

1. Register the partner and obtain the `key` and `secret`.
2. Parse the request. Sort the JSON body by ASCII ascending order of the keys in the JSON, and concatenate the strings dataStr=key1=value1&key2=value2&key3=value3&...
3. Generate a timestamp (unit: milliseconds)
4. Encrypt and generate a sign: Plaintext before encryption: strToHash = Secret+dataStr+timestamp Perform MD5 encryption on the plaintext strToHash to generate the sign.
The specific code is the GenerateMD5Sign function.
5. Place the key, timestamp, and sign in the HTTP header.

## Detailed Explanation of the `clientSign` Signature Algorithm

1. Obtain the request parameters and format them to obtain a new formatted string.

2. Sign the data from step 1 with the RSA private key and save the signature result to a variable.
Generate a signature string for the following parameter array: `user_id = 1 coin = eth address = 0x038B8E7406dED2Be112B6c7E4681Df5316957cad amount = 10.001 trade_id = 20220131012030274786`
Sort each key in the array from a to z. If the first letter is the same, look at the second letter, and so on. After sorting, concatenate all array values ​​with the ampersand (&) character, for example, in $dataString:
`address=0x038B8E7406dED2Be112B6c7E4681Df5316957cad&amount=10.001&coin=eth&trade_id=20220131012030274786&user_id=1`
This string is the concatenated string.

3. Use the private key to sign the data using RSA-MD5. The specific code is in the generate_md5_sign function.

# Public Information

| Name               | Type      | Example                            | Description                                              |
| :----------------- | :-------- | :--------------------------------- | :------------------------------------------------------- |
| Global Status Code | integer   | 1                                  | 1 indicates success. See Global Status Code for details. |
| Message            | string    | ok                                 | Returns text information.                                |
| Data               | json      | {"OpenID":"HEX..."}                | Returns specific data content.                           |
| Time               | timeStamp | 1722587274000                      | UTC time (without time zone, in milliseconds).           |
| Sign               | string    | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | The platform uses RSA to sign all data.                  |

The Sign returned by the platform is the result of encrypting the response data using the RSA algorithm. The frontend should verify the signature against the returned data. For details, please refer to the function func: 

pub fn verify_rsa_signature`
