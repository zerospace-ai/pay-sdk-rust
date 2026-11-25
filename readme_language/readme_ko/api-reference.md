# API 참조

이 문서는 요청 매개변수, 반환 매개변수 및 예제를 포함하여 CryptoPay Rust SDK의 모든 API 엔드포인트에 대해 자세히 설명합니다.

## 목차

1. [글로벌 공용 정보](#1-글로벌-공용-정보)
2. [새 사용자 생성 (create_user)](#2-새-사용자-생성-create_user)
3. [지갑 생성 (create_wallet)](#3-지갑-생성-create_wallet)
4. [입금 주소 얻기 (get_wallet_addresses)](#4-입금-주소-얻기-get_wallet_addresses)
5. [사용자 출금 (user_withdraw_by_open_id)](#5-사용자-출금-user_withdraw_by_open_id)
6. [출금 주문 2차 검토 (Webhook)](#6-출금-주문-2차-검토-webhook)
7. [입금 및 출금 콜백 알림 (Webhook)](#7-입금-및-출금-콜백-알림-webhook)
8. [카셔 주문 생성 (new_order)](#8-카셔-주문-생성-new_order)
9. [카셔 주문 결제 성공 콜백 (Webhook)](#9-카셔-주문-결제-성공-콜백-webhook)
10. [지갑 잔액 조회 (wallet_balance)](#10-지갑-잔액-조회-wallet_balance)

---

## 1. 글로벌 공용 정보

플랫폼에서 반환된 모든 JSON 응답 데이터는 다음과 같은 공용 구조를 따릅니다.

| 필드 이름 | 유형 | 예 | 설명 |
| :--- | :--- | :--- | :--- |
| `code` | integer | `1` | 글로벌 상태 코드. `1`은 성공을 나타내고 `0` 또는 `-1`은 실패를 나타냅니다. |
| `msg` | string | `ok` | 반환된 텍스트 설명 |
| `data` | object | `{"OpenId":"PT00001"}` | API에 따라 달라지는 특정 데이터 내용이 반환됨 |
| `timestamp` | string | `"1725439986754"` | UTC 타임스탬프 (밀리초) |
| `sign` | string | `9e0ccfe3915e...` | 플랫폼이 응답 데이터에 대해 RSA 프라이빗 키를 사용하여 생성한 서명 |

---

## 2. 새 사용자 생성 (create_user)

### API 설명
새 플랫폼 사용자를 생성하며 사용자의 고유 ID인 UserOpenId가 필요합니다.

### HTTP 요청
* **URL:** `https://sandbox-api.privatex.io/sdk/user/create`
* **메서드:** `POST`

### 요청 매개변수
| 매개변수 이름 | 필수 여부 | 유형 | 설명 |
| :--- | :--- | :--- | :--- |
| `OpenId` | 예 | string | 플랫폼 표준 접두사 + 사용자의 고유 ID를 사용하여 OpenId를 구성하는 것이 좋습니다. |

### 응답 매개변수
*(글로벌 정보 포함)*
| 매개변수 이름 | 유형 | 설명 |
| :--- | :--- | :--- |
| `data.OpenId` | string | 사용자의 고유 OpenId |

### 예제 코드 (cURL)
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

## 3. 지갑 생성 (create_wallet)

### API 설명
지정된 블록체인 네트워크에서 사용자의 지갑 계정을 생성합니다.

### HTTP 요청
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/create`
* **메서드:** `POST`

### 요청 매개변수
| 매개변수 이름 | 필수 여부 | 유형 | 설명 |
| :--- | :--- | :--- | :--- |
| `ChainID` | 예 | string | 체인 ID (부록 참조) |
| `OpenId` | 예 | string | 사용자의 고유 OpenId |

### 응답 매개변수
*(글로벌 정보 포함)*
| 매개변수 이름 | 유형 | 설명 |
| :--- | :--- | :--- |
| `data.address` | string | 지갑 주소 |
| `data.OpenId` | string | 사용자의 고유 OpenId |

### 예제 코드 (cURL)
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

## 4. 입금 주소 얻기 (get_wallet_addresses)

### API 설명
사용자의 블록체인 지갑 입금 주소를 얻습니다 (여러 체인에 걸친 일괄 조회 지원).

### HTTP 요청
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses`
* **메서드:** `POST`

### 요청 매개변수
| 매개변수 이름 | 필수 여부 | 유형 | 설명 |
| :--- | :--- | :--- | :--- |
| `OpenId` | 예 | string | 사용자의 고유 OpenId |
| `ChainIDs` | 예 | string | 쉼표로 구분된 여러 체인 ID |

### 응답 매개변수
*(글로벌 정보 포함)*
| 매개변수 이름 | 유형 | 설명 |
| :--- | :--- | :--- |
| `data.Addresses` | array | 주소 객체 목록 |

### 예제 코드 (cURL)
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

## 5. 사용자 출금 (user_withdraw_by_open_id)

### API 설명
출금 작업을 시작하여 파트너의 펀드 풀에서 사용자가 지정한 주소로 자금을 이체합니다.

> **⚠️ 경고:**
> * 블록체인 네트워크에서 트랜잭션이 시작되면 **취소하거나 환불할 수 없습니다**. 이 API를 호출하기 전에 적절한 위험 제어 검사를 수행하십시오.
> * 출금 풀에 토큰과 가스비(예: ETH/TRX) 잔액이 충분한지 확인해야 합니다.

### 비즈니스 로직
1. 중복 출금을 방지하기 위해 비즈니스 플랫폼의 고유 주문 번호가 일반적으로 `SafeCheckCode`로 사용됩니다.
2. 트랜잭션은 플랫폼의 위험 제어 규칙과 대조됩니다.
   * **자동 승인:** 직접 온체인으로 이동하며 `data`는 트랜잭션 해시를 반환합니다.
   * **2차 검토 (`code=2`):** 채널 플랫폼의 관리자가 수동으로 검토해야 합니다. 검토가 완료되면 상태가 Webhook을 통해 비동기식으로 알림됩니다.

### HTTP 요청
* **URL:** `https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID`
* **메서드:** `POST`

### 요청 매개변수
| 매개변수 이름 | 필수 여부 | 유형 | 설명 |
| :--- | :--- | :--- | :--- |
| `OpenId` | 예 | string | 사용자의 고유 OpenId |
| `TokenId` | 예 | string | 토큰 ID (부록 참조)|
| `Amount` | 예 | float | 출금 금액 |
| `AddressTo` | 예 | string | 출금 대상 주소 |
| `CallBackUrl` | 아니요 | string | 상태 변경 콜백 URL |
| `SafeCheckCode` | 아니요 | string | 보안 인증 코드 (비즈니스 주문 ID) |

### 응답 상태 코드 설명
| `code` | 설명 |
| :--- | :--- |
| `1` | 트랜잭션이 성공하여 온체인으로 제출되었으며 `data`는 해시를 반환합니다. |
| `2` | 트랜잭션이 제출되었으나 플랫폼의 위험 제어를 트리거하여 2차 검토가 필요합니다. |
| `0` 또는 `-1` | 매개변수 오류, 잔액 부족 또는 트랜잭션 실패. 자세한 내용은 `msg`를 확인하십시오. |

### 예제 코드 (cURL)
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

## 6. 출금 주문 2차 검토 (Webhook)

### 콜백 설명
관리자가 가맹점 백엔드에서 "위험 제어 콜백 URL"을 구성하면 플랫폼은 출금 트랜잭션을 처리할 때 이 URL에 대한 HTTP 요청을 적극적으로 시작하여 가맹점의 비즈니스 측에 출금을 해제할지 여부를 묻습니다.
**플랫폼은 가맹점 API가 `code=0`을 반환하는 경우에만 자금을 해제합니다.**

> **⚠️ 참고:** 플랫폼은 가맹점에 독립적인 **위험 제어 RSA 퍼블릭 키(`PlatformRiskPubKey`)**를 할당합니다. 가맹점은 이 콜백 서명의 진위 여부를 확인하기 위해 이 퍼블릭 키를 사용해야 합니다.

### 플랫폼에서 시작하는 HTTP 요청
* **메서드:** `POST`
* **URL:** 가맹점이 구성한 Webhook URL `/withdrawal/order/check`

### 콜백 매개변수
| 매개변수 이름 | 필수 여부 | 유형 | 설명 |
| :--- | :--- | :--- | :--- |
| `safeCode` | 아니요 | string | 가맹점이 제출한 출금 주문 ID (`SafeCheckCode`) |
| `openId` | 예 | string | 출금하는 사용자 ID |
| `tokenId` | 예 | string | 토큰 ID |
| `toAddress` | 예 | string | 출금 대상 주소 |
| `amount` | 예 | string | 출금 금액 |
| `timestamp` | 예 | int | 밀리초 타임스탬프 |
| `sign` | 예 | string | 위험 제어 프라이빗 키를 사용하여 플랫폼에서 생성한 서명 |

### 가맹점의 예상 응답
처리 후 가맹점 API는 다음 JSON을 반환해야 합니다.

| 매개변수 이름 | 유형 | 설명 |
| :--- | :--- | :--- |
| `code` | int | 인증 결과. `0`은 해제에 동의함을 의미하며 다른 숫자는 거부를 의미합니다. |
| `timestamp` | int | 초 단위의 현재 타임스탬프 |
| `message` | string | 사용자 지정 설명 |
| `sign` | string | 가맹점은 자체 RSA 프라이빗 키를 사용하여 응답 데이터에 서명합니다. |

---

## 7. 입금 및 출금 콜백 알림 (Webhook)

### 콜백 설명
플랫폼은 토큰 트랜잭션 상태(출금 성공/실패 또는 사용자 입금 크레딧)에 대한 비동기 알림을 비즈니스 측에 푸시합니다.

1. **재시도 메커니즘:** 비즈니스 측에서 반환 코드 `0`으로 응답해야 성공적으로 수신된 것으로 간주됩니다. 그렇지 않으면 플랫폼은 전략(초기에는 2초마다 50회, 그 후에는 10분마다 한 번)에 따라 푸시를 지속적으로 재시도합니다.
2. **상태 무시:** 하나의 트랜잭션이 여러 알림을 트리거할 수 있습니다. 최신 상태에 의존하십시오.
3. **실패 처리:** 출금 콜백 `status = -1`인 경우 관리자가 백엔드에서 트랜잭션을 다시 제출하면 시스템에서 새 상태 알림을 다시 푸시합니다.

### 플랫폼에서 시작하는 HTTP 요청
* **메서드:** `POST`
* **URL:** 가맹점이 서비스 제공업체에 제공한 콜백 URL.

### 콜백 매개변수
| 매개변수 이름 | 필수 여부 | 유형 | 설명 |
| :--- | :--- | :--- | :--- |
| `type` | 예 | int | 비즈니스 유형: 입금의 경우 `1`; 출금의 경우 `2`. |
| `status` | 예 | int | 트랜잭션 상태:<br/> `1`: 온체인 성공, 해시를 통해 쿼리 가능.<br/> `-1`: 온체인 실패, 백엔드에서 처리 가능.<br/> `2`: 위험 제어 트리거, 관리자 백엔드에서 수동 검토 필요.<br/> `-2`: 가맹점 위험 제어 API에 의해 출금 신청 거부됨. |
| `openid` | 예 | string | 채널 사용자의 고유 ID |
| `totalvalue` | 예 | string | 트랜잭션 시 시장 가격을 기준으로 계산된 상응하는 USDT 가치 |
| `hash` | 예 | string | 온체인 트랜잭션 해시 |
| `confirm` | 예 | int | 온체인 확인 횟수 |
| `from` | 예 | string | 트랜잭션 시작자 주소 |
| `to` | 예 | string | 트랜잭션 수신 주소 |
| `amount` | 예 | string | 트랜잭션 금액 |
| `chainid` | 예 | string | 체인 ID |
| `tokenid` | 예 | string | 토큰 ID |
| `tokenaddress` | 예 | string | 스마트 컨트랙트 주소 |
| `safecode` | 아니요 | string | 출금 주문 보안 인증 코드 (OrderID) |
| `createdtime` | 예 | string | 트랜잭션 생성 시간 |
| `timestamp` | 예 | string | 푸시 타임스탬프 |

### 가맹점의 예상 응답
성공적으로 수신되면 `{"code": 0}`이 포함된 JSON 응답 본문을 반환하십시오.

---

## 8. 카셔 주문 생성 (new_order)

### API 설명
이 인터페이스는 가맹점이 결제 또는 충전 요청을 시작하는 데 사용됩니다. 시스템은 입금 주소와 결제 페이지 링크(Cashier URL)를 반환하며, 사용자는 이 링크로 이동하여 결제를 완료할 수 있습니다.

### HTTP 요청
* **URL:** `https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder`
* **메서드:** `POST`

### 요청 매개변수
| 매개변수 이름 | 필수 여부 | 유형 | 설명 |
| :--- | :--- | :--- | :--- |
| `outOrderNo` | 예 | string | 가맹점 측 고유 주문 번호 (전체 플랫폼 고유, 중복 방지를 위해 가맹점 접두사 권장) |
| `outUserId` | 모드에 따름 | string | 가맹점 측 사용자 ID. **주소 풀 모드가 비활성화된 경우 필수**(미전달 시 `10502` 오류); 주소 풀 모드(`address_reuse_mode=1`)에서는 생략 가능 |
| `tokenId` | 예 | integer | 토큰 ID (체인도 함께 결정됨) |
| `quantity` | 예 | decimal | 결제 수량 (`> 0` 필수; `0.01`과 같은 일반 소수 표기 권장, 과학적 표기법 금지) |
| `notifyUrl` | 아니요 | string | 결제 완료 후 비동기 콜백 알림 URL (생략 시 가맹점 관리자 기본 콜백 URL 사용) |
| `orderDetail` | 아니요 | string | 주문 설명 (원본 그대로 저장) |

> **두 가지 주소 모드 안내 (`outUserId`):**
> * **고정 주소 모드:** `outUserId`를 전달하면 `(가맹점, outUserId, 체인)`별로 고정 주소가 할당됩니다. 주소 풀 모드가 비활성화된 가맹점은 **반드시** `outUserId`를 전달해야 하며, 미전달 시 `10502`가 반환됩니다.
> * **주소 풀 모드:** 주소 재사용(`address_reuse_mode=1`)이 활성화된 상태에서 `outUserId`를 생략(빈 문자열이 아닌 필드 자체 생략)하면 유휴 풀에서 전용 주소가 할당됩니다. 이 모드에서 응답 및 콜백의 `outUserId`는 해당 풀 주소의 원래 바인딩 사용자 ID이므로, 정산/대조는 반드시 `outOrderNo` / `orderNo` (콜백에서는 `outOrderId` / `orderId`)를 사용하세요.

### 응답 매개변수
*(전역 정보 포함)*
| 매개변수 이름 | 유형 | 설명 |
| :--- | :--- | :--- |
| `data.orderNo` | string | 플랫폼에서 생성한 고유 주문 ID |
| `data.outOrderNo` | string | 가맹점에서 전달한 원래 주문 ID |
| `data.outUserId` | string | 주문에 바인딩된 사용자 ID (주소 풀 모드에서는 풀 주소의 원래 바인딩 사용자 ID) |
| `data.tokenId` | integer | 토큰 ID |
| `data.quantity` | string | 결제 수량 (JSON 문자열) |
| `data.addresses` | string | 입금 수취 주소 (단일 주소, 대소문자 구분하므로 원본 그대로 표시) |
| `data.cashierUrl` | string | **핵심 필드**: 카셔 페이지 주소. 가맹점은 사용자를 이 링크로 안내하여 결제해야 합니다 |

### 코드 예시 (cURL)
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

*(주소 풀 모드가 활성화된 경우 `outUserId`와 `orderDetail`을 생략할 수 있으며, 생략된 필드는 서명에 포함되지 않습니다.)*

---

## 9. 카셔 주문 결제 성공 콜백 (Webhook)

### 콜백 설명
사용자가 전액 결제를 완료한 경우(`orderStatus=2`)에만 DogPay가 주문의 `notifyUrl`(미전달 시 기본 콜백 URL)로 비동기 콜백 알림을 전송합니다. 부분 결제(`1`) 및 만료(`-1`) 시에는 콜백이 전송되지 않습니다.

### 플랫폼에서 시작하는 HTTP 요청
* **메서드:** `POST`
* **URL:** 가맹점에서 제공한 `notifyUrl` (또는 기본 콜백 URL)

### 콜백 매개변수
| 매개변수 이름 | 유형 | 설명 |
| :--- | :--- | :--- |
| `orderId` | string | 플랫폼 시스템 주문 ID (`data.orderNo`와 동일) |
| `outOrderId` | string | 가맹점 원래 주문 ID (`outOrderNo`와 동일) |
| `outUserId` | string | 주문 바인딩 사용자 ID (위 주소 모드 안내 참고) |
| `orderType` | string | 주문 유형 (`1` = 충전/수납) |
| `chainId` | string | 체인 ID |
| `tokenId` | string | 토큰 ID |
| `quantity` | string | 결제 예정 수량 |
| `quantityPaid` | string | **사용자가 실제로 결제한 누적 수량** |
| `totalAmount` | string | 결제 예정 USD 환산 금액 |
| `totalAmountPaid` | string | 실결제 USD 환산 금액 |
| `exchangeAddress` | string | 입금 수취 주소 |
| `orderStatus` | string | **주문 상태**: 콜백 시 `2`(완료)로 고정 |
| `txHash` | string | 해당 주문과 매칭된 마지막 온체인 트랜잭션 해시 |
| `createdAt` | string | 주문 생성 시간 |
| `timestamp` | string | 콜백 전송 타임스탬프 (밀리초) |
| `sign` | string | **데이터 서명** (위조 방지를 위해 플랫폼 공개 키로 검증) |

### 가맹점의 예상 응답
성공적으로 수신한 경우 다음 JSON을 반환하십시오 (`code`가 `"0"` 또는 숫자 `0`이어야 성공으로 처리되며, 그 외 응답은 재시도됩니다):
```json
{
  "code": "0",
  "message": "success"
}
```

---

## 10. 지갑 잔액 조회 (wallet_balance)

### API 설명
지정된 블록체인 네트워크에서 특정 지갑 주소의 토큰 또는 네이티브 코인 잔액을 조회합니다.

### HTTP 요청
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/balance`
* **메서드:** `POST`

### 요청 매개변수
| 매개변수 이름 | 필수 여부 | 유형 | 설명 |
| :--- | :--- | :--- | :--- |
| `address` | 예 | string | 지갑 주소 |
| `contractAddress` | 예 | string | 토큰 계약 주소 또는 토큰 심볼 (예: `"XRP"`, `"USDT"`) |
| `chainId` | 예 | integer | 체인 ID (예: XRP의 경우 `5`, Ethereum의 경우 `1`, BNB Chain의 경우 `56`) |

### 응답 매개변수
*(글로벌 정보 포함)*
| 매개변수 이름 | 유형 | 설명 |
| :--- | :--- | :--- |
| `code` | integer | 글로벌 상태 코드 (`1`은 성공) |
| `msg` | string | 상태 설명 |
| `data` | string | 토큰/코인 잔액 수량 (최소 단위 문자열) |
| `timestamp` | string | 응답 타임스탬프 (밀리초) |
| `sign` | string | 플랫폼 데이터 서명 |

### 예제 코드 (cURL)
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

### 응답 예제
```json
{
  "sign" : "",
  "timestamp" : "1725432397796",
  "data" : "1979984",
  "msg" : "ok",
  "code" : 1
}
```