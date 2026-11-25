# API-Referenz

Dieses Dokument enthält detaillierte Informationen zu allen API-Endpunkten des CryptoPay Rust SDK, einschließlich Anforderungsparametern, Rückgabeparametern und Beispielen.

## Inhaltsverzeichnis

1. [Globale öffentliche Informationen](#1-globale-öffentliche-informationen)
2. [Neuen Benutzer erstellen (create_user)](#2-neuen-benutzer-erstellen-create_user)
3. [Wallet erstellen (create_wallet)](#3-wallet-erstellen-create_wallet)
4. [Einzahlungsadressen abrufen (get_wallet_addresses)](#4-einzahlungsadressen-abrufen-get_wallet_addresses)
5. [Benutzerauszahlung (user_withdraw_by_open_id)](#5-benutzerauszahlung-user_withdraw_by_open_id)
6. [Zweitprüfung für Auszahlungsaufträge (Webhook)](#6-zweitprüfung-für-auszahlungsaufträge-webhook)
7. [Ein- und Auszahlungs-Rückrufbenachrichtigung (Webhook)](#7-ein--und-auszahlungs-rückrufbenachrichtigung-webhook)
8. [Kassenbestellung erstellen (new_order)](#8-kassenbestellung-erstellen-new_order)
9. [Kassenbestellung Zahlungsbestätigung Callback (Webhook)](#9-kassenbestellung-zahlungsbestätigung-callback-webhook)
10. [Wallet-Guthaben abfragen (wallet_balance)](#10-wallet-guthaben-abfragen-wallet_balance)

---

## 1. Globale öffentliche Informationen

Alle von der Plattform zurückgegebenen JSON-Antwortdaten folgen dieser öffentlichen Struktur:

| Feldname | Typ | Beispiel | Beschreibung |
| :--- | :--- | :--- | :--- |
| `code` | integer | `1` | Globaler Statuscode. `1` steht für Erfolg, `0` oder `-1` steht für Fehler. |
| `msg` | string | `ok` | Zurückgegebene Textbeschreibung |
| `data` | object | `{"OpenId":"PT00001"}` | Spezifischer zurückgegebener Dateninhalt, der je nach API variiert |
| `timestamp` | string | `"1725439986754"` | UTC-Zeitstempel (Millisekunden) |
| `sign` | string | `9e0ccfe3915e...` | Signatur, die von der Plattform mit ihrem privaten RSA-Schlüssel für die Antwortdaten generiert wurde |

---

## 2. Neuen Benutzer erstellen (create_user)

### API-Beschreibung
Erstellt einen neuen Plattform-Benutzer. Erfordert die eindeutige Benutzer-ID (UserOpenId).

### HTTP-Anfrage
* **URL:** `https://sandbox-api.privatex.io/sdk/user/create`
* **Methode:** `POST`

### Anfrageparameter
| Parametername | Erforderlich | Typ | Beschreibung |
| :--- | :--- | :--- | :--- |
| `OpenId` | Ja | string | Es wird empfohlen, das Standardpräfix der Plattform + die eindeutige ID des Benutzers zu verwenden, um die OpenId zu bilden. |

### Antwortparameter
*(Enthält globale Informationen)*
| Parametername | Typ | Beschreibung |
| :--- | :--- | :--- |
| `data.OpenId` | string | Eindeutige OpenId des Benutzers |

### Codebeispiel (cURL)
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

## 3. Wallet erstellen (create_wallet)

### API-Beschreibung
Erstellt ein Wallet-Konto für den Benutzer im angegebenen Blockchain-Netzwerk.

### HTTP-Anfrage
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/create`
* **Methode:** `POST`

### Anfrageparameter
| Parametername | Erforderlich | Typ | Beschreibung |
| :--- | :--- | :--- | :--- |
| `ChainID` | Ja | string | Chain ID (Siehe Anhang) |
| `OpenId` | Ja | string | Eindeutige OpenId des Benutzers |

### Antwortparameter
*(Enthält globale Informationen)*
| Parametername | Typ | Beschreibung |
| :--- | :--- | :--- |
| `data.address` | string | Wallet-Adresse |
| `data.OpenId` | string | Eindeutige OpenId des Benutzers |

### Codebeispiel (cURL)
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

## 4. Einzahlungsadressen abrufen (get_wallet_addresses)

### API-Beschreibung
Ruft die Blockchain-Wallet-Einzahlungsadressen für einen Benutzer ab (unterstützt Batch-Abfragen über mehrere Chains).

### HTTP-Anfrage
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses`
* **Methode:** `POST`

### Anfrageparameter
| Parametername | Erforderlich | Typ | Beschreibung |
| :--- | :--- | :--- | :--- |
| `OpenId` | Ja | string | Eindeutige OpenId des Benutzers |
| `ChainIDs` | Ja | string | Mehrere Chain-IDs, getrennt durch Kommas |

### Antwortparameter
*(Enthält globale Informationen)*
| Parametername | Typ | Beschreibung |
| :--- | :--- | :--- |
| `data.Addresses` | array | Liste von Adressobjekten |

### Codebeispiel (cURL)
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

## 5. Benutzerauszahlung (user_withdraw_by_open_id)

### API-Beschreibung
Initiiert einen Auszahlungsvorgang, bei dem Gelder aus dem Fonds des Partners auf die vom Benutzer angegebene Adresse übertragen werden.

> **⚠️ Warnung:**
> * Sobald eine Transaktion im Blockchain-Netzwerk initiiert wurde, **kann sie nicht mehr rückgängig gemacht oder erstattet werden**. Bitte stellen Sie sicher, dass vor dem Aufruf dieser API ordnungsgemäße Risikokontrollen durchgeführt werden.
> * Sie müssen sicherstellen, dass der Auszahlungspool über ein ausreichendes Guthaben des Tokens und für die Gas-Gebühren (wie ETH/TRX) verfügt.

### Geschäftslogik
1. Eine eindeutige Bestellnummer von der Geschäftsplattform wird normalerweise als `SafeCheckCode` verwendet, um doppelte Auszahlungen zu verhindern.
2. Die Transaktion wird mit den Risikokontrollregeln der Plattform abgeglichen:
   * **Automatisch genehmigt:** Sie wird direkt in die Chain aufgenommen und `data` gibt den Transaktions-Hash zurück.
   * **Zweitprüfung (`code=2`):** Erfordert eine manuelle Überprüfung durch einen Administrator auf der Channel-Plattform. Nach der Überprüfung wird der Status asynchron über einen Webhook mitgeteilt.

### HTTP-Anfrage
* **URL:** `https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID`
* **Methode:** `POST`

### Anfrageparameter
| Parametername | Erforderlich | Typ | Beschreibung |
| :--- | :--- | :--- | :--- |
| `OpenId` | Ja | string | Eindeutige OpenId des Benutzers |
| `TokenId` | Ja | string | Token-ID (Siehe Anhang) |
| `Amount` | Ja | float | Auszahlungsbetrag |
| `AddressTo` | Ja | string | Zieladresse für die Auszahlung |
| `CallBackUrl` | Nein | string | URL für Statusänderungs-Callback |
| `SafeCheckCode` | Nein | string | Sicherheitsverifizierungscode (Bestell-ID des Geschäfts) |

### Beschreibung der Antwort-Statuscodes
| `code` | Beschreibung |
| :--- | :--- |
| `1` | Transaktion erfolgreich und on-chain übermittelt, `data` gibt den Hash zurück. |
| `2` | Transaktion übermittelt, hat jedoch die Risikokontrolle der Plattform ausgelöst, erfordert eine Zweitprüfung. |
| `0` oder `-1` | Parameterfehler, unzureichendes Guthaben oder fehlgeschlagene Transaktion. Überprüfen Sie `msg` auf Details. |

### Codebeispiel (cURL)
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

## 6. Zweitprüfung für Auszahlungsaufträge (Webhook)

### Callback-Beschreibung
Wenn ein Administrator im Händler-Backend eine "Risikokontroll-Callback-URL" konfiguriert, initiiert die Plattform bei der Verarbeitung einer Auszahlungstransaktion aktiv eine HTTP-Anfrage an diese URL, um die Geschäftsseite des Händlers zu fragen, ob die Auszahlung freigegeben werden soll.
**Die Plattform gibt die Gelder nur frei, wenn die Händler-API `code=0` zurückgibt.**

> **⚠️ Hinweis:** Die Plattform weist dem Händler einen unabhängigen **öffentlichen RSA-Schlüssel für die Risikokontrolle (`PlatformRiskPubKey`)** zu. Der Händler muss diesen öffentlichen Schlüssel verwenden, um die Authentizität der Signatur für diesen Callback zu überprüfen.

### Von der Plattform initiierte HTTP-Anfrage
* **Methode:** `POST`
* **URL:** Konfigurierte Webhook-URL des Händlers `/withdrawal/order/check`

### Callback-Parameter
| Parametername | Erforderlich | Typ | Beschreibung |
| :--- | :--- | :--- | :--- |
| `safeCode` | Nein | string | Die vom Händler übermittelte Bestell-ID für die Auszahlung (`SafeCheckCode`) |
| `openId` | Ja | string | ID des auszahlenden Benutzers |
| `tokenId` | Ja | string | Token-ID |
| `toAddress` | Ja | string | Zieladresse der Auszahlung |
| `amount` | Ja | string | Auszahlungsbetrag |
| `timestamp` | Ja | int | Millisekunden-Zeitstempel |
| `sign` | Ja | string | Signatur, die von der Plattform mit ihrem privaten Risikokontrollschlüssel generiert wurde |

### Erwartete Antwort vom Händler
Nach der Verarbeitung muss die Händler-API folgendes JSON zurückgeben:

| Parametername | Typ | Beschreibung |
| :--- | :--- | :--- |
| `code` | int | Überprüfungsergebnis. `0` bedeutet Zustimmung zur Freigabe; andere Zahlen bedeuten Ablehnung. |
| `timestamp` | int | Aktueller Zeitstempel in Sekunden |
| `message` | string | Benutzerdefinierte Beschreibung |
| `sign` | string | Der Händler signiert die Antwortdaten mit seinem eigenen privaten RSA-Schlüssel |

---

## 7. Ein- und Auszahlungs-Rückrufbenachrichtigung (Webhook)

### Callback-Beschreibung
Die Plattform pusht asynchrone Benachrichtigungen über den Token-Transaktionsstatus (Erfolg/Fehlschlag der Auszahlung oder Benutzereinzahlung) an die Geschäftsseite.

1. **Wiederholungsmechanismus:** Die Geschäftsseite muss mit dem Rückgabecode `0` antworten, damit die Benachrichtigung als erfolgreich empfangen gilt. Andernfalls wiederholt die Plattform den Push-Vorgang kontinuierlich gemäß der Strategie (anfangs 50 Mal alle 2 Sekunden, dann einmal alle 10 Minuten).
2. **Statusüberschreibung:** Eine Transaktion kann mehrere Benachrichtigungen auslösen. Bitte verlassen Sie sich auf den neuesten Status.
3. **Fehlerbehandlung:** Wenn der Auszahlungs-Callback `status = -1` lautet, pusht das System erneut eine neue Statusbenachrichtigung, nachdem der Administrator die Transaktion im Backend erneut übermittelt hat.

### Von der Plattform initiierte HTTP-Anfrage
* **Methode:** `POST`
* **URL:** Die vom Händler dem Dienstanbieter zur Verfügung gestellte Callback-URL.

### Callback-Parameter
| Parametername | Erforderlich | Typ | Beschreibung |
| :--- | :--- | :--- | :--- |
| `type` | Ja | int | Geschäftstyp: `1` für Einzahlung; `2` für Auszahlung. |
| `status` | Ja | int | Transaktionsstatus:<br/> `1`: On-chain erfolgreich, kann über den Hash abgefragt werden.<br/> `-1`: On-chain fehlgeschlagen, kann im Backend verarbeitet werden.<br/> `2`: Risikokontrolle ausgelöst, erfordert manuelle Überprüfung im Admin-Backend.<br/> `-2`: Auszahlungsantrag durch die Risikokontroll-API des Händlers abgelehnt. |
| `openid` | Ja | string | Eindeutige ID des Channel-Benutzers |
| `totalvalue` | Ja | string | Äquivalenter USDT-Wert, berechnet auf Basis des Marktpreises zum Zeitpunkt der Transaktion |
| `hash` | Ja | string | On-chain Transaktions-Hash |
| `confirm` | Ja | int | On-chain Bestätigungsanzahl |
| `from` | Ja | string | Adresse des Transaktionsinitiators |
| `to` | Ja | string | Empfangsadresse der Transaktion |
| `amount` | Ja | string | Transaktionsbetrag |
| `chainid` | Ja | string | Chain-ID |
| `tokenid` | Ja | string | Token-ID |
| `tokenaddress` | Ja | string | Smart-Contract-Adresse |
| `safecode` | Nein | string | Sicherheitsverifizierungscode für Auszahlungsauftrag (OrderID) |
| `createdtime` | Ja | string | Erstellungszeit der Transaktion |
| `timestamp` | Ja | string | Push-Zeitstempel |

### Erwartete Antwort vom Händler
Wenn erfolgreich empfangen, geben Sie bitte einen JSON-Antwortkörper zurück, der `{"code": 0}` enthält.

---

## 8. Kassenbestellung erstellen (new_order)

### API-Beschreibung
Diese Schnittstelle wird von Händlern verwendet, um Zahlungs- oder Aufladeanfragen zu initiieren. Das System gibt eine Empfangsadresse sowie einen Zahlungsseiten-Link (Cashier-URL) zurück, über den der Benutzer die Zahlung abschließen kann.

### HTTP-Anfrage
* **URL:** `https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder`
* **Methode:** `POST`

### Anfrageparameter
| Parametername | Erforderlich | Typ | Beschreibung |
| :--- | :--- | :--- | :--- |
| `outOrderNo` | Ja | string | Eindeutige Bestellnummer auf der Händlerseite (plattformweit eindeutig; Händlerpräfix empfohlen) |
| `outUserId` | Modusabhängig | string | Benutzer-ID auf Händlerseite. **Erforderlich, wenn der Adresspool-Modus nicht aktiviert ist** (sonst Fehler `10502`); im Adresspool-Modus (`address_reuse_mode=1`) weglassen |
| `tokenId` | Ja | integer | Token-ID (bestimmt zugleich das Blockchain-Netzwerk) |
| `quantity` | Ja | decimal | Zu zahlende Token-Menge (muss `> 0` sein; normale Dezimalschreibweise wie `0.01` verwenden, keine wissenschaftliche Notation) |
| `notifyUrl` | Nein | string | Asynchrone Callback-Benachrichtigungsadresse nach vollständiger Zahlung (bei Weglassen wird die Standard-Callback-URL im Händler-Backend verwendet) |
| `orderDetail` | Nein | string | Bestellbeschreibung (wird unverändert gespeichert) |

> **Adressmodi (`outUserId`):**
> * **Fester Adressmodus:** Wird `outUserId` übergeben, wird pro `(Händler, outUserId, Chain)` eine feste Adresse zugewiesen. Händler ohne aktivierten Adresspool **müssen** `outUserId` angeben, sonst wird Fehler `10502` zurückgegeben.
> * **Adresspool-Modus:** Wird `outUserId` weggelassen (Feld auslassen/`null`, kein leerer String) und ist `address_reuse_mode=1` aktiviert, wird eine exklusive freie Adresse aus dem Pool zugewiesen. Hinweis: In diesem Modus enthalten `data.outUserId` und das Callback-Feld `outUserId` die ursprünglich an diese Pool-Adresse gebundene Benutzer-ID — verwenden Sie zum Abgleich stets `outOrderNo` / `orderNo` (`outOrderId` / `orderId`).

### Antwortparameter
*(Enthält globale Informationen)*
| Parametername | Typ | Beschreibung |
| :--- | :--- | :--- |
| `data.orderNo` | string | Vom System generierte eindeutige Bestell-ID |
| `data.outOrderNo` | string | Vom Händler übermittelte ursprüngliche Bestell-ID |
| `data.outUserId` | string | Gebundene Benutzer-ID (im Adresspool-Modus die ursprüngliche Benutzer-ID der Pool-Adresse) |
| `data.tokenId` | integer | Token-ID |
| `data.quantity` | string | Zu zahlende Menge (als String im JSON) |
| `data.addresses` | string | Empfangsadresse (einzelne Adresse, Groß-/Kleinschreibung beachten) |
| `data.cashierUrl` | string | **Kernfeld**: Adresse der Kassenseite; Händler leiten Benutzer zur Zahlung dorthin weiter |

### Codebeispiel (cURL)
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

*(Bei aktiviertem Adresspool-Modus können `outUserId` und `orderDetail` weggelassen werden; weggelassene Felder fließen nicht in die Signatur ein.)*

---

## 9. Kassenbestellung Zahlungsbestätigung Callback (Webhook)

### Callback-Beschreibung
Erst nach vollständiger Bezahlung (`orderStatus=2`) sendet DogPay eine asynchrone Callback-Benachrichtigung an die angegebene `notifyUrl` (oder die Standard-Callback-URL des Händlers). Teilzahlungen (`1`) und abgelaufene Bestellungen (`-1`) lösen keinen Callback aus.

### Von der Plattform initiierte HTTP-Anfrage
* **Methode:** `POST`
* **URL:** Vom Händler bereitgestellte `notifyUrl` (oder Standard-Callback-URL)

### Callback-Parameter
| Parametername | Typ | Beschreibung |
| :--- | :--- | :--- |
| `orderId` | string | Plattform-Systembestellnummer (entspricht `data.orderNo`) |
| `outOrderId` | string | Ursprüngliche Händlerbestellnummer (entspricht `outOrderNo`) |
| `outUserId` | string | Gebundene Benutzer-ID (siehe Hinweis zu Adressmodi) |
| `orderType` | string | Auftragstyp (`1` = Einzahlung/Inkasso) |
| `chainId` | string | Chain-ID |
| `tokenId` | string | Token-ID |
| `quantity` | string | Zu zahlende Menge |
| `quantityPaid` | string | **Tatsächlich vom Benutzer kumulativ bezahlte Menge** |
| `totalAmount` | string | Geschätzter Sollbetrag in USD |
| `totalAmountPaid` | string | Geschätzter bezahlter Betrag in USD |
| `exchangeAddress` | string | Empfangsadresse |
| `orderStatus` | string | **Bestellstatus**: beim Callback fest auf `2` (abgeschlossen) |
| `txHash` | string | On-Chain-Transaktions-Hash der zuletzt zugeordneten Einzahlung |
| `createdAt` | string | Erstellungszeitpunkt der Bestellung |
| `timestamp` | string | Callback-Zeitstempel (Millisekunden) |
| `sign` | string | **Datensignatur** (mit dem öffentlichen Schlüssel der Plattform verifizieren) |

### Erwartete Antwort vom Händler
Geben Sie bei erfolgreichem Empfang folgendes JSON zurück (`code` muss `"0"` oder `0` sein, sonst wiederholt die Plattform den Versuch):
```json
{
  "code": "0",
  "message": "success"
}
```

---

## 10. Wallet-Guthaben abfragen (wallet_balance)

### API-Beschreibung
Fragt das Token- oder Coin-Guthaben für eine angegebene Wallet-Adresse in einem bestimmten Blockchain-Netzwerk ab.

### HTTP-Anfrage
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/balance`
* **Methode:** `POST`

### Anfrageparameter
| Parametername | Erforderlich | Typ | Beschreibung |
| :--- | :--- | :--- | :--- |
| `address` | Ja | string | Wallet-Adresse |
| `contractAddress` | Ja | string | Token-Vertragsadresse oder Token-Symbol (z. B. `"XRP"`, `"USDT"`) |
| `chainId` | Ja | integer | Chain-ID (z. B. `5` für XRP, `1` für Ethereum, `56` für BNB Chain) |

### Antwortparameter
*(Enthält globale Informationen)*
| Parametername | Typ | Beschreibung |
| :--- | :--- | :--- |
| `code` | integer | Globaler Statuscode (`1` für Erfolg) |
| `msg` | string | Statusbeschreibung |
| `data` | string | Token- / Coin-Guthabenbetrag (in kleinsten Einheiten) |
| `timestamp` | string | Antwortzeitstempel (Millisekunden) |
| `sign` | string | Plattform-Datensignatur |

### Codebeispiel (cURL)
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

### Antwortbeispiel
```json
{
  "sign" : "",
  "timestamp" : "1725432397796",
  "data" : "1979984",
  "msg" : "ok",
  "code" : 1
}
```