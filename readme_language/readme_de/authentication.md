# Anfrage-Verifizierungs-Header

Definition des Anfrage-Headers

| Parametername | Einschränkungen | Beispiel                           | Beschreibung                                                |
| :------------ | :-------------- | :--------------------------------- | :---------------------------------------------------------- |
| key           | Länge: 64       | ithujj3onrzbgw5t                   | Partner-Schlüssel                                           |
| timestamp     | Länge: 32       | 1722586649000                      | Zeitstempel der Anfrageinitiierung (Einheit: Millisekunden) |
| sign          | Länge: 32       | 9e0ccfe3915e94bcc5bf7dd51ad4e8d9   | Partner-Secret-Signatur                                     |
| clientSign    | Länge: 512      | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | Partner-RSA-Signatur                                        |

## Regeln für das `sign`-Feld

1. Registrieren Sie den Partner und erhalten Sie den `key` und `secret`.
2. Parsen Sie die Anfrage. Sortieren Sie den JSON-Body nach aufsteigender ASCII-Reihenfolge der Schlüssel im JSON und verbinden Sie die Strings dataStr=key1=value1&key2=value2&key3=value3&...
3. Generieren Sie einen Zeitstempel (Einheit: Millisekunden)
4. Verschlüsseln und generieren Sie ein sign: Klartext vor der Verschlüsselung: strToHash = Secret+dataStr+timestamp Führen Sie eine MD5-Verschlüsselung des Klartexts strToHash durch, um das sign zu generieren.
Der spezifische Code ist die GenerateMD5Sign-Funktion.
5. Platzieren Sie den key, timestamp und sign im HTTP-Header.

## Detaillierte Erklärung des `clientSign`-Signaturalgorithmus

1. Erhalten Sie die Anfrageparameter und formatieren Sie sie, um eine neue formatierte Zeichenkette zu erhalten.

2. Signieren Sie die Daten aus Schritt 1 mit dem RSA-Privatschlüssel und speichern Sie das Signaturergebnis in einer Variable.
Generieren Sie eine Signaturzeichenkette für das folgende Parameterarray: `user_id = 1 coin = eth address = 0x038B8E7406dED2Be112B6c7E4681Df5316957cad amount = 10.001 trade_id = 20220131012030274786`
Sortieren Sie jeden Schlüssel im Array von a bis z. Wenn der erste Buchstabe gleich ist, schauen Sie auf den zweiten Buchstaben usw. Nach der Sortierung verbinden Sie alle Array-Werte mit dem Ampersand (&)-Zeichen, z.B. in $dataString:
`address=0x038B8E7406dED2Be112B6c7E4681Df5316957cad&amount=10.001&coin=eth&trade_id=20220131012030274786&user_id=1`
Diese Zeichenkette ist die verbundene Zeichenkette.

3. Verwenden Sie den Privatschlüssel, um die Daten mit RSA-MD5 zu signieren. Der spezifische Code ist in der generate_md5_sign-Funktion.

# Öffentliche Informationen

| Name                | Typ       | Beispiel                           | Beschreibung                                          |
| :------------------ | :-------- | :--------------------------------- | :---------------------------------------------------- |
| Globaler Statuscode | integer   | 1                                  | 1 bedeutet Erfolg. Details siehe Globaler Statuscode. |
| Nachricht           | string    | ok                                 | Gibt Textinformationen zurück.                        |
| Daten               | json      | {"OpenID":"HEX..."}                | Gibt spezifischen Dateninhalt zurück.                 |
| Zeit                | timeStamp | 1722587274000                      | UTC-Zeit (ohne Zeitzone, in Millisekunden).           |
| Sign                | string    | 9e0ccfe3915e94bcc5bfbBsC5EUxV6 ... | Die Plattform signiert alle Daten mit RSA.            |

Das von der Plattform zurückgegebene Sign ist das Ergebnis der Verschlüsselung der Antwortdaten mit dem RSA-Algorithmus. Das Frontend sollte die Signatur gegen die zurückgegebenen Daten überprüfen. Für Details siehe die Funktion func: 

pub fn verify_rsa_signature`