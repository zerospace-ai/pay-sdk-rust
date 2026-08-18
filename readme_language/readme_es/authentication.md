# Autenticación y Seguridad

Este sistema utiliza un estricto mecanismo de firma de solicitudes para garantizar la seguridad de los datos entre el comerciante y la plataforma. El proceso de comunicación implica firmas MD5 y verificación de firmas RSA.

## 1. Generar Par de Claves RSA

Los comerciantes deben generar su propio par de claves RSA, utilizar la clave privada para firmar las solicitudes y enviar la clave pública a la plataforma.

### 1.1 Generar Par de Claves usando OpenSSL

Ejecute los siguientes comandos en Mac, Linux o Git Bash/WSL/Cygwin:

```bash
# Generar clave privada de 2048 bits
openssl genrsa -out rsa_private_key.pem 2048

# Generar clave pública a partir de la clave privada
openssl rsa -in rsa_private_key.pem -out rsa_public_key.pem -pubout
```

### 1.2 Extraer Cadenas de Claves

A la clave pública generada se le deben quitar las líneas `-----BEGIN PUBLIC KEY-----` / `-----END PUBLIC KEY-----` y los caracteres de nueva línea, y debe convertirse en una cadena de una sola línea antes de enviarla a la plataforma.

**Extraer en Mac/Linux/Git Bash:**

```bash
# Extraer la cadena de la clave privada
grep -v '^-----' rsa_private_key.pem | tr -d '\n'; echo

# Extraer la cadena de la clave pública
grep -v '^-----' rsa_public_key.pem | tr -d '\n'; echo
```

**Extraer en Windows PowerShell:**

```powershell
# Extraer clave privada
Write-Output ((Get-Content rsa_private_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")

# Extraer clave pública
Write-Output ((Get-Content rsa_public_key.pem | Where-Object {$_ -notmatch "^-----"}) -join "")
```

> **⚠️ Advertencia:** La clave privada extraída debe almacenarse de forma segura en su servidor local (completada en el campo `RsaPrivateKey` de `config.yaml`) y **nunca debe filtrarse**.

---

## 2. Definición de Cabeceras de Validación

Al realizar una solicitud HTTP, se deben incluir los siguientes parámetros de autenticación en la cabecera (Header):

| Nombre del Parámetro | Restricción | Ejemplo | Descripción |
| :--- | :--- | :--- | :--- |
| `key` | Longitud: 64 | `vratson2i5hjxgkd` | Clave API asignada por la plataforma después del registro del comerciante |
| `timestamp` | Longitud: 32 | `1725076567682` | Marca de tiempo de la solicitud (Unidad: milisegundos) |
| `sign` | Longitud: 32 | `0592dc64d480fb119d1e07ce0601db` | Firma generada mediante el algoritmo MD5 sobre el contenido de la solicitud |
| `clientSign` | Longitud: 512 | `9e0ccfe3915e94bcc5bfbBsC...` | Firma generada mediante la clave privada RSA del comerciante sobre el contenido de la solicitud |

---

## 3. Algoritmo de Firma Detallado

El SDK maneja internamente toda la lógica de firma de forma automática. Si necesita desarrollar de forma independiente una versión en otro idioma, o comprender los principios subyacentes de la firma, consulte las siguientes instrucciones.

### 3.1 Reglas del Campo `sign` (Firma MD5)

1. Obtenga el `Secret` de la plataforma.
2. Ordene el cuerpo JSON de la solicitud en orden alfabético ASCII ascendente por clave, y concaténelo en una cadena formateada como `key1=value1&key2=value2...`, anotada como `dataStr`.
3. Obtenga la marca de tiempo actual (milisegundos).
4. Empalme el texto sin formato que se va a cifrar: `strToHash = Secret + dataStr + timestamp`.
5. Realice el cifrado MD5 en `strToHash`, y el resultado generado es el `sign`.

### 3.2 Reglas del Campo `clientSign` (Firma RSA)

1. Ordene los parámetros de la solicitud en orden alfabético ASCII ascendente por clave.
2. Conecte todos los valores de la matriz utilizando el carácter `&`, por ejemplo:
   `address=0x038B8...&amount=10.001&coin=eth&trade_id=2022013101`
3. Utilice la clave privada RSA del comerciante para realizar la firma `RSA-MD5` en esta cadena concatenada. El resultado generado es el `clientSign`.

---

## 4. Verificación de Firma de Respuesta de la Plataforma

Los datos devueltos por la plataforma también contendrán un campo `sign`, que es el resultado del cifrado de los datos de respuesta por parte de la plataforma utilizando su clave privada RSA.

Después de que el comerciante reciba la respuesta, debe utilizar `PlatformPubKey` (la clave pública de la plataforma) en `config.yaml` para verificar esta firma y garantizar que los datos no hayan sido manipulados.

> **💡 Consejo:** El Rust SDK proporciona el método `verify_rsa_signature(&self, map_data: &HashMap<String, String>, sign: &str) -> Result<()>` para que los desarrolladores verifiquen rápidamente los datos devueltos por la plataforma.