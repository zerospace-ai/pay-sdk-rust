# Authentifizierung und Sicherheit

Dieses System verwendet einen strengen Anfrage-Signatur-Mechanismus, um die Datensicherheit zwischen dem Händler und der Plattform zu gewährleisten. Der Kommunikationsprozess umfasst MD5-Signaturen und RSA-Signaturüberprüfungen.

## 1. RSA-Schlüsselpaar generieren

Händler müssen ihr eigenes RSA-Schlüsselpaar generieren, den privaten Schlüssel zum Signieren von Anfragen verwenden und den öffentlichen Schlüssel an die Plattform übermitteln.

### 1.1 Schlüsselpaar mit OpenSSL generieren

Führen Sie die folgenden Befehle unter Mac, Linux oder Git Bash/WSL/Cygwin aus:

```bash
# 2048-Bit privaten Schlüssel generieren
openssl genrsa -out rsa_private_key.pem 2048

# Öffentlichen Schlüssel aus dem privaten Schlüssel generieren
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

### 1.2 Schlüssel-Strings extrahieren

Der generierte öffentliche Schlüssel muss ohne die Zeilen `-----BEGIN PUBLIC KEY-----` / `-----END PUBLIC KEY-----` und ohne Zeilenumbrüche vorliegen, bevor er in einem einzigen String an die Plattform gesendet wird.

**Extrahieren unter Mac/Linux/Git Bash:**

```bash
# Privaten Schlüssel-String extrahieren
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# Öffentlichen Schlüssel-String extrahieren
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

**Extrahieren unter Windows PowerShell:**

```powershell
# Privaten Schlüssel extrahieren
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# Öffentlichen Schlüssel extrahieren
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> **⚠️ Achtung:** Der extrahierte private Schlüssel muss sicher auf Ihrem lokalen Server gespeichert werden (eingetragen im Feld `RsaPrivateKey` der `config.yaml`) und **darf niemals weitergegeben werden**.

---

## 2. Definition der Validierungs-Header

Beim Senden einer HTTP-Anfrage müssen die folgenden Authentifizierungsparameter im Header enthalten sein:

| Parametername | Bedingung | Beispiel | Beschreibung |
| :--- | :--- | :--- | :--- |
| `key` | Länge: 64 | `vratson2i5hjxgkd` | API Key, der von der Plattform nach der Händlerregistrierung vergeben wurde |
| `timestamp` | Länge: 32 | `1725076567682` | Zeitstempel der Anfrage (Einheit: Millisekunden) |
| `sign` | Länge: 32 | `0592dc64d480fb119d1e07ce0601db` | Signatur, die mit dem MD5-Algorithmus aus dem Anfrageinhalt generiert wurde |
| `clientSign` | Länge: 512 | `9e0ccfe3915e94bcc5bfbBsC...` | Signatur, die mit dem privaten RSA-Schlüssel des Händlers aus dem Anfrageinhalt generiert wurde |

---

## 3. Detaillierter Signaturalgorithmus

Das SDK behandelt die gesamte Signaturlogik intern automatisch. Wenn Sie eine Version in einer anderen Sprache unabhängig entwickeln oder die zugrunde liegenden Prinzipien der Signatur verstehen müssen, beachten Sie die folgenden Anweisungen.

### 3.1 Regeln für das `sign`-Feld (MD5-Signatur)

1. Rufen Sie das `Secret` der Plattform ab.
2. Sortieren Sie den JSON-Body der Anfrage in aufsteigender ASCII-Reihenfolge nach Schlüsseln und verketten Sie ihn zu einem String im Format `key1=value1&key2=value2...`, bezeichnet als `dataStr`.
3. Rufen Sie den aktuellen Zeitstempel (Millisekunden) ab.
4. Verbinden Sie den zu verschlüsselnden Klartext: `strToHash = Secret + dataStr + timestamp`.
5. Führen Sie eine MD5-Verschlüsselung für `strToHash` durch. Das generierte Ergebnis ist das `sign`.

### 3.2 Regeln für das `clientSign`-Feld (RSA-Signatur)

1. Sortieren Sie die Anfrageparameter in aufsteigender ASCII-Reihenfolge nach Schlüsseln.
2. Verbinden Sie alle Array-Werte mit dem Zeichen `&`, zum Beispiel:
   `address=0x038B8...&amount=10.001&coin=eth&trade_id=2022013101`
3. Verwenden Sie den privaten RSA-Schlüssel des Händlers, um eine `RSA-MD5`-Signatur für diesen verketteten String durchzuführen. Das generierte Ergebnis ist das `clientSign`.

---

## 4. Überprüfung der Signatur der Plattformantwort

Die von der Plattform zurückgegebenen Daten enthalten ebenfalls ein `sign`-Feld, welches das Ergebnis der Verschlüsselung der Antwortdaten durch die Plattform mit ihrem privaten RSA-Schlüssel ist.

Nachdem der Händler die Antwort erhalten hat, muss er dieses Signatur mit dem `PlatformPubKey` (öffentlicher Schlüssel der Plattform) in `config.yaml` überprüfen, um sicherzustellen, dass die Daten nicht manipuliert wurden.

> **💡 Tipp:** Das Rust SDK bietet die Methode `verify_rsa_signature(&self, map_data: &HashMap<String, String>, sign: &str) -> Result<()>` für Entwickler, um die von der Plattform zurückgegebenen Daten schnell zu überprüfen.