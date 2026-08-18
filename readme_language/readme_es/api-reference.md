# Referencia de API

Este documento detalla todos los puntos finales de la API del CryptoPay Rust SDK, incluidos los parámetros de solicitud, los parámetros de retorno y los ejemplos.

## Tabla de Contenido

1. [Información Pública Global](#1-información-pública-global)
2. [Crear Nuevo Usuario (create_user)](#2-crear-nuevo-usuario-create_user)
3. [Crear Billetera (create_wallet)](#3-crear-billetera-create_wallet)
4. [Obtener Direcciones de Depósito (get_wallet_addresses)](#4-obtener-direcciones-de-depósito-get_wallet_addresses)
5. [Retiro de Usuario (user_withdraw_by_open_id)](#5-retiro-de-usuario-user_withdraw_by_open_id)
6. [Revisión Secundaria de Orden de Retiro (Webhook)](#6-revisión-secundaria-de-orden-de-retiro-webhook)
7. [Notificación de Devolución de Llamada de Depósito y Retiro (Webhook)](#7-notificación-de-devolución-de-llamada-de-depósito-y-retiro-webhook)
8. [Crear orden de caja (new_order)](#8-crear-orden-de-caja-new_order)
9. [Callback de éxito de pago de orden de caja (Webhook)](#9-callback-de-éxito-de-pago-de-orden-de-caja-webhook)
10. [Consultar Saldo de Billetera (wallet_balance)](#10-consultar-saldo-de-billetera-wallet_balance)

---

## 1. Información Pública Global

Todos los datos de respuesta JSON devueltos por la plataforma siguen esta estructura pública:

| Nombre del Campo | Tipo | Ejemplo | Descripción |
| :--- | :--- | :--- | :--- |
| `code` | integer | `1` | Código de estado global. `1` representa éxito, `0` o `-1` representa falla |
| `msg` | string | `ok` | Descripción de texto devuelta |
| `data` | object | `{"OpenId":"PT00001"}` | Contenido de datos específico devuelto, variando según la API |
| `timestamp` | string | `"1725439986754"` | Marca de tiempo UTC (milisegundos) |
| `sign` | string | `9e0ccfe3915e...` | Firma generada por la plataforma utilizando su clave privada RSA sobre los datos de respuesta |

---

## 2. Crear Nuevo Usuario (create_user)

### Descripción de la API
Crea un nuevo usuario de plataforma, requiriendo el ID único del usuario, que es el UserOpenId.

### Solicitud HTTP
* **URL:** `https://sandbox-api.privatex.io/sdk/user/create`
* **Método:** `POST`

### Parámetros de Solicitud
| Nombre del Parámetro | Requerido | Tipo | Descripción |
| :--- | :--- | :--- | :--- |
| `OpenId` | Sí | string | Se recomienda utilizar el prefijo estándar de la plataforma + el ID único del usuario para formar el OpenId. |

### Parámetros de Respuesta
*(Incluye Información Global)*
| Nombre del Parámetro | Tipo | Descripción |
| :--- | :--- | :--- |
| `data.OpenId` | string | OpenId único del usuario |

### Código de Ejemplo (cURL)
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

## 3. Crear Billetera (create_wallet)

### Descripción de la API
Crea una cuenta de billetera para el usuario en la red blockchain especificada.

### Solicitud HTTP
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/create`
* **Método:** `POST`

### Parámetros de Solicitud
| Nombre del Parámetro | Requerido | Tipo | Descripción |
| :--- | :--- | :--- | :--- |
| `ChainID` | Sí | string | ID de Cadena (Ver Apéndice) |
| `OpenId` | Sí | string | OpenId único del usuario |

### Parámetros de Respuesta
*(Incluye Información Global)*
| Nombre del Parámetro | Tipo | Descripción |
| :--- | :--- | :--- |
| `data.address` | string | Dirección de la Billetera |
| `data.OpenId` | string | OpenId único del usuario |

### Código de Ejemplo (cURL)
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

## 4. Obtener Direcciones de Depósito (get_wallet_addresses)

### Descripción de la API
Obtiene las direcciones de depósito de billeteras blockchain para un usuario (admite consultas por lotes a través de múltiples cadenas).

### Solicitud HTTP
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses`
* **Método:** `POST`

### Parámetros de Solicitud
| Nombre del Parámetro | Requerido | Tipo | Descripción |
| :--- | :--- | :--- | :--- |
| `OpenId` | Sí | string | OpenId único del usuario |
| `ChainIDs` | Sí | string | Múltiples ID de Cadena, separados por comas |

### Parámetros de Respuesta
*(Incluye Información Global)*
| Nombre del Parámetro | Tipo | Descripción |
| :--- | :--- | :--- |
| `data.Addresses` | array | Lista de objetos de dirección |

### Código de Ejemplo (cURL)
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

## 5. Retiro de Usuario (user_withdraw_by_open_id)

### Descripción de la API
Inicia una operación de retiro, transfiriendo fondos desde el fondo común del socio a la dirección especificada por el usuario.

> **⚠️ Advertencia:**
> * Una vez que se inicia una transacción en la red blockchain, **no se puede revertir ni reembolsar**. Asegúrese de realizar los controles de riesgo adecuados antes de llamar a esta API.
> * Debe asegurarse de que el fondo de retiro tenga un saldo suficiente del token y tarifas de Gas (como ETH/TRX).

### Lógica de Negocio
1. Por lo general, se utiliza un número de pedido único de la plataforma comercial como `SafeCheckCode` para evitar retiros duplicados.
2. La transacción se compara con las reglas de control de riesgos de la plataforma:
   * **Aprobación automática:** Entra directamente a la cadena (on-chain), y `data` devolverá el hash de la transacción.
   * **Revisión Secundaria (`code=2`):** Requiere revisión manual por parte de un administrador en la plataforma del canal. Una vez revisado, el estado se notificará de forma asíncrona a través del Webhook.

### Solicitud HTTP
* **URL:** `https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID`
* **Método:** `POST`

### Parámetros de Solicitud
| Nombre del Parámetro | Requerido | Tipo | Descripción |
| :--- | :--- | :--- | :--- |
| `OpenId` | Sí | string | OpenId único del usuario |
| `TokenId` | Sí | string | ID del Token (Ver Apéndice)|
| `Amount` | Sí | float | Monto de retiro |
| `AddressTo` | Sí | string | Dirección de retiro de destino |
| `CallBackUrl` | No | string | URL de devolución de llamada de cambio de estado |
| `SafeCheckCode` | No | string | Código de verificación de seguridad (ID de Pedido Comercial) |

### Descripción del Código de Estado de Respuesta
| `code` | Descripción |
| :--- | :--- |
| `1` | Transacción exitosa y enviada on-chain, `data` devuelve el hash. |
| `2` | Transacción enviada, pero desencadenó el control de riesgos de la plataforma, requiriendo revisión secundaria. |
| `0` o `-1` | Error de parámetro, saldo insuficiente o transacción fallida. Verifique `msg` para más detalles. |

### Código de Ejemplo (cURL)
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

## 6. Revisión Secundaria de Orden de Retiro (Webhook)

### Descripción de la Devolución de Llamada
Cuando un administrador configura una "URL de devolución de llamada de control de riesgos" en el backend comercial, la plataforma iniciará activamente una solicitud HTTP a esta URL al procesar una transacción de retiro para preguntar al extremo comercial del comerciante si debe liberar el retiro.
**La plataforma solo liberará los fondos si la API del comerciante devuelve `code=0`.**

> **⚠️ Nota:** La plataforma asignará una **Clave Pública RSA de Control de Riesgos (`PlatformRiskPubKey`)** independiente al comerciante. El comerciante debe utilizar esta clave pública para verificar la autenticidad de la firma de esta devolución de llamada.

### Solicitud HTTP Iniciada por la Plataforma
* **Método:** `POST`
* **URL:** URL de Webhook configurada por el comerciante `/withdrawal/order/check`

### Parámetros de Devolución de Llamada
| Nombre del Parámetro | Requerido | Tipo | Descripción |
| :--- | :--- | :--- | :--- |
| `safeCode` | No | string | ID de pedido de retiro enviado por el comerciante (`SafeCheckCode`) |
| `openId` | Sí | string | ID del usuario que retira |
| `tokenId` | Sí | string | ID del Token |
| `toAddress` | Sí | string | Dirección de destino del retiro |
| `amount` | Sí | string | Monto del Retiro |
| `timestamp` | Sí | int | Marca de tiempo en milisegundos |
| `sign` | Sí | string | Firma generada por la plataforma utilizando su clave privada de control de riesgos |

### Respuesta Esperada del Comerciante
Después del procesamiento, la API del comerciante debe devolver el siguiente JSON:

| Nombre del Parámetro | Tipo | Descripción |
| :--- | :--- | :--- |
| `code` | int | Resultado de la verificación. `0` significa aceptar la liberación; otros números significan rechazar. |
| `timestamp` | int | Marca de tiempo actual en segundos |
| `message` | string | Descripción personalizada |
| `sign` | string | El comerciante firma los datos de respuesta utilizando su propia clave privada RSA |

---

## 7. Notificación de Devolución de Llamada de Depósito y Retiro (Webhook)

### Descripción de la Devolución de Llamada
La plataforma envía notificaciones asíncronas sobre el estado de las transacciones de tokens (éxito/fracaso de retiro o abono de depósito de usuario) al extremo comercial.

1. **Mecanismo de Reintento:** El extremo comercial debe responder con el código de retorno `0` para que se considere recibido correctamente. De lo contrario, la plataforma reintentará el envío continuamente según la estrategia (inicialmente 50 veces cada 2 segundos, luego una vez cada 10 minutos).
2. **Anulación de Estado:** Una transacción puede desencadenar múltiples notificaciones. Confíe en el estado más reciente.
3. **Manejo de Errores:** Si la devolución de llamada de retiro es `status = -1`, después de que el administrador reenvíe la transacción en el backend, el sistema enviará nuevamente una nueva notificación de estado.

### Solicitud HTTP Iniciada por la Plataforma
* **Método:** `POST`
* **URL:** La URL de devolución de llamada proporcionada por el comerciante al proveedor de servicios.

### Parámetros de Devolución de Llamada
| Nombre del Parámetro | Requerido | Tipo | Descripción |
| :--- | :--- | :--- | :--- |
| `type` | Sí | int | Tipo de Negocio: `1` para depósito; `2` para retiro. |
| `status` | Sí | int | Estado de la Transacción:<br/> `1`: Exitoso on-chain, se puede consultar vía hash.<br/> `-1`: Fallido on-chain, procesable en el backend.<br/> `2`: Control de riesgos activado, requiere revisión manual en el backend de administración.<br/> `-2`: Solicitud de retiro rechazada por la API de control de riesgos del comerciante. |
| `openid` | Sí | string | ID único del Usuario del Canal |
| `totalvalue` | Sí | string | Valor USDT equivalente calculado en función del precio de mercado en el momento de la transacción |
| `hash` | Sí | string | Hash de la Transacción On-chain |
| `confirm` | Sí | int | Recuento de Confirmaciones On-chain |
| `from` | Sí | string | Dirección del Iniciador de la Transacción |
| `to` | Sí | string | Dirección de Recepción de la Transacción |
| `amount` | Sí | string | Monto de la Transacción |
| `chainid` | Sí | string | ID de Cadena |
| `tokenid` | Sí | string | ID del Token |
| `tokenaddress` | Sí | string | Dirección del Contrato Inteligente |
| `safecode` | No | string | Código de Verificación de Seguridad del Pedido de Retiro (OrderID) |
| `createdtime` | Sí | string | Tiempo de Creación de la Transacción |
| `timestamp` | Sí | string | Marca de tiempo de envío (Push) |

### Respuesta Esperada del Comerciante
Si se recibe con éxito, devuelva un cuerpo de respuesta JSON que contenga `{"code": 0}`.

---

## 8. Crear orden de caja (new_order)

### Descripción de la API
Esta interfaz es utilizada por los comerciantes para iniciar solicitudes de pago o recarga. El sistema devolverá una dirección de pago (URL de caja) y el usuario puede visitar esta dirección para completar el proceso de pago posterior.

### Solicitud HTTP
* **URL:** `https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder`
* **Método:** `POST`

### Parámetros de solicitud
| Nombre del parámetro | Requerido | Tipo | Descripción |
| :--- | :--- | :--- | :--- |
| `outOrderNo` | Sí | string | ID de pedido único en el lado del comerciante (utilizado para evitar envíos duplicados y para conciliación) |
| `tokenId` | Sí | integer | ID de token |
| `quantity` | Sí | decimal | Monto/cantidad del pedido |
| `notifyUrl` | No | string | Dirección de notificación de callback asíncrono después de un pago exitoso |

### Parámetros de respuesta
*(Incluye información global)*
| Nombre del parámetro | Tipo | Descripción |
| :--- | :--- | :--- |
| `data.orderNo` | string | ID de pedido único generado por la plataforma |
| `data.outOrderNo` | string | ID de pedido original pasado por el comerciante |
| `data.cashierUrl` | string | **Campo central**: Dirección de la página de caja. Los comerciantes deben guiar a los usuarios a este enlace para el pago. |

### Ejemplo de código (cURL)
```bash
curl --location 'https://vapi.dogpay.ai/sdk/api/v2/exchange/cashier/newOrder' \
--header 'key: your_api_key' \
--header 'sign: your_md5_sign' \
--header 'clientSign: your_rsa_sign' \
--header 'Content-Type: application/json' \
--header 'timestamp: 1770606706659' \
--data '{  
  "outOrderNo": "1234623",   
  "tokenId": 6,   
  "quantity": 0.01,   
  "notifyUrl": "https://your-domain.com/callback"   
}'
```

---

## 9. Callback de éxito de pago de orden de caja (Webhook)

### Descripción del callback
Después de que el usuario completa el pago en la página de caja, DogPay enviará una notificación de callback asíncrona a la `notifyUrl` proporcionada por el comerciante.

### Solicitud HTTP iniciada por la plataforma
* **Método:** `POST`
* **URL:** `notifyUrl` proporcionada por el comerciante

### Parámetros del callback
| Nombre del parámetro | Tipo | Descripción |
| :--- | :--- | :--- |
| `orderId` | string | ID de pedido del sistema de la plataforma |
| `outOrderId` | string | ID de pedido original del comerciante |
| `orderStatus` | string | **Estado del pedido**: 2 generalmente representa pago exitoso |
| `tokenId` | string | ID de token |
| `quantityPaid` | string | **Cantidad real pagada por el usuario** |
| `sign` | string | **Firma de datos** (utilizada para la verificación del comerciante para evitar callbacks falsificados) |

### Respuesta esperada del comerciante
Si se recibe correctamente, devuelva el siguiente JSON:
```json
{
  "code": "1",
  "message": "success"
}
```

---

## 10. Consultar Saldo de Billetera (wallet_balance)

### Descripción de la API
Consulta el saldo de tokens o monedas para una dirección de billetera especificada en una red blockchain determinada.

### Solicitud HTTP
* **URL:** `https://sandbox-api.privatex.io/sdk/wallet/balance`
* **Método:** `POST`

### Parámetros de Solicitud
| Nombre del Parámetro | Requerido | Tipo | Descripción |
| :--- | :--- | :--- | :--- |
| `address` | Sí | string | Dirección de la billetera |
| `contractAddress` | Sí | string | Dirección del contrato o símbolo del token (p. ej., `"XRP"`, `"USDT"`) |
| `chainId` | Sí | integer | ID de la cadena (p. ej., `5` para XRP, `1` para Ethereum, `56` para BNB Chain) |

### Parámetros de Respuesta
*(Incluye Información Global)*
| Nombre del Parámetro | Tipo | Descripción |
| :--- | :--- | :--- |
| `code` | integer | Código de estado global (`1` para éxito) |
| `msg` | string | Descripción del estado |
| `data` | string | Cantidad de saldo de token/moneda (en unidades mínimas) |
| `timestamp` | string | Marca de tiempo de respuesta (milisegundos) |
| `sign` | string | Firma de datos de la plataforma |

### Código de Ejemplo (cURL)
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

### Ejemplo de Respuesta
```json
{
  "sign" : "",
  "timestamp" : "1725432397796",
  "data" : "1979984",
  "msg" : "ok",
  "code" : 1
}
```