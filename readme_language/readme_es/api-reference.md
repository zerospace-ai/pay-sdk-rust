# Referencia de API 📚

Este documento detalla todas las interfaces de API del CryptoPay Rust SDK, incluyendo parámetros de solicitud, parámetros de retorno y ejemplos.

## Registrar Nuevo Usuario (create_user)🆕🧑‍💻

### Concepto
Crear un nuevo usuario en la plataforma, requiriendo el ID único del usuario, es decir, UserOpenId.

### Solicitud HTTP
- Nombre de la Interface: create_user
- URL: https://sandbox-api.privatex.io/sdk/user/create
- Método: POST

### Parámetros de Solicitud
| Nombre del Parámetro | Requerido | Tipo   | Descripción                                                                                       |
| -------------------- | --------- | ------ | ------------------------------------------------------------------------------------------------- |
| OpenId               | Sí        | string | Se recomienda usar el prefijo estándar de la plataforma + ID único del usuario para formar OpenId |

### Parámetros de Retorno
| Nombre del Parámetro | Tipo   | Descripción              |
| -------------------- | ------ | ------------------------ |
| code                 | int    | Código de estado global  |
| msg                  | string | Descripción de estado    |
| data.OpenId          | string | OpenId único del usuario |
| sign                 | string | Firma de la plataforma   |

### Ejemplo
Ejemplo de Solicitud:
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
Ejemplo de Retorno:
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

Para Autenticación y Seguridad, consulte [🧩 authentication.md](./authentication.md)

## Crear Billetera (create_wallet) 💰

### Concepto
Crear una cuenta de billetera para el usuario en la red blockchain correspondiente.

### Solicitud HTTP
- Nombre de la Interface: create_wallet
- URL: https://sandbox-api.privatex.io/sdk/wallet/create
- Método: POST

### Parámetros de Solicitud
| Nombre del Parámetro | Requerido | Tipo   | Descripción              |
| -------------------- | --------- | ------ | ------------------------ |
| ChainID              | Sí        | string | ID de la cadena          |
| OpenId               | Sí        | string | OpenId único del usuario |

### Parámetros de Retorno
| Nombre del Parámetro | Tipo   | Descripción               |
| -------------------- | ------ | ------------------------- |
| code                 | int    | Código de estado global   |
| msg                  | string | Descripción de estado     |
| data.address         | string | Dirección de la billetera |
| data.OpenId          | string | OpenId único del usuario  |
| sign                 | string | Firma de la plataforma    |

### Ejemplo
Ejemplo de Solicitud:
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
Ejemplo de Retorno:
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

## Obtener Dirección de Depósito (get_wallet_addresses)💳

### Concepto
Obtener la dirección de depósito de la billetera blockchain del usuario.

### Solicitud HTTP
- Nombre de la Interface: get_wallet_addresses
- URL: https://sandbox-api.privatex.io/sdk/wallet/getWalletAddresses
- Método: POST

### Parámetros de Solicitud
| Nombre del Parámetro | Requerido | Tipo   | Descripción                                  |
| -------------------- | --------- | ------ | -------------------------------------------- |
| OpenId               | Sí        | string | OpenId único del usuario                     |
| ChainIDs             | Sí        | string | Múltiples IDs de cadena, separados por comas |

### Parámetros de Retorno
| Nombre del Parámetro | Tipo   | Descripción             |
| -------------------- | ------ | ----------------------- |
| code                 | int    | Código de estado global |
| msg                  | string | Descripción de estado   |
| data.Addresses       | array  | Lista de direcciones    |
| sign                 | string | Firma de la plataforma  |

### Ejemplo
Ejemplo de Solicitud:
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
Ejemplo de Retorno:
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

## Retiro de Usuario (user_withdraw_by_open_id)💸

### Concepto
Operación de retiro de usuario, transferencia desde la cuenta del socio a la dirección especificada por el usuario.

* Función: Interface de operación de retiro de usuario. Los retiros deben transferirse desde la cuenta del socio en el pool de retiro de tokens correspondiente a la dirección de billetera de retiro especificada por el usuario. Los socios pueden configurar una dirección de callback segura para verificar la legitimidad del retiro. Si se verifica como válido, el retiro se puede completar directamente desde la billetera del pool de fondos del comerciante.

* La interface de transacción de retiro verifica si la billetera caliente de retiro predeterminada tiene suficientes activos de retiro y tarifas de manejo.

* Por defecto, la interface de retiro usa un código de verificación de seguridad como el requisito de parámetro único para transacciones de retiro. Generalmente se recomienda usar el número de orden de retiro único de la plataforma de negocios como código de seguridad. Enviar un código de verificación de seguridad duplicado resultará en un error.

* Todas las solicitudes de transacción de retiro se compararán con las reglas de revisión de control de riesgos configuradas en la plataforma del canal. Si la solicitud de parámetro es válida, se aceptará la solicitud de transacción. Las transacciones de retiro que cumplan con las reglas de revisión automática se enviarán inmediatamente a la transacción de red y se devolverá la información hash de la transacción enviada (campo de retorno data). Las solicitudes de transacción de retiro que requieran revisión secundaria en el canal devolverán (code=2). No es necesario volver a enviar la solicitud de retiro. El administrador debe completar la revisión secundaria en la plataforma del canal. Después de completar la revisión secundaria, el orden de transacción callback notificará el cambio de estado de la transacción de retiro.

* Prerrequisito: El pool de fondos de la moneda correspondiente debe tener una cantidad suficiente de fondos para retirar (especialmente para retiros de tokens de red ETH, que requieren un saldo de tarifa de transacción ETH en la billetera del pool de fondos).

* ⚠️ Nota: **Para retiros de blockchain, asegúrese de que el proceso de pre-aprobación esté completo antes de llamar a la interface. Una vez iniciada una transacción de blockchain, no se puede revocar ni devolver.**

### Solicitud HTTP
- Nombre de la Interface: user_withdraw_by_open_id
- URL: https://sandbox-api.privatex.io/sdk/partner/UserWithdrawByOpenID
- Método: POST

### Parámetros de Solicitud
| Nombre del Parámetro | Requerido | Tipo   | Descripción                         |
| -------------------- | --------- | ------ | ----------------------------------- |
| OpenId               | Sí        | string | OpenId único del usuario            |
| TokenId              | Sí        | string | ID del token                        |
| Amount               | Sí        | float  | Cantidad de retiro                  |
| AddressTo            | Sí        | string | Dirección objetivo                  |
| CallBackUrl          | No        | string | URL de callback                     |
| SafeCheckCode        | No        | string | Código de verificación de seguridad |

### Parámetros de Retorno

| Nombre del Parámetro | Tipo   | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| code                 | int    | Código de estado</br>0 Error de parámetro, número de orden duplicado, formato de dirección de retiro incorrecto o tarifas de billetera de retiro insuficientes. Información detallada en msg.</br>1 La transacción de retiro se envió con éxito y se ha enviado a la red blockchain. El hash único de la transacción enviada está en data.</br>2 La transacción de retiro se envió con éxito y requiere revisión secundaria del canal antes de completarse. Después de la revisión, la información de transacción se actualizará a través de un callback.</br>-1 La transacción de retiro falló. Puede volver a enviar la solicitud de retiro. |
| msg                  | string | Descripción de estado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| data                 | string | Hash de transacción. Si el retiro inteligente está habilitado, este campo se devolverá como una cadena vacía.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| sign                 | string | Firma de la plataforma                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| timestamp            | string | Marca de tiempo actual en milisegundos convertida a cadena                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |


### Ejemplo
Ejemplo de Solicitud:
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

Ejemplo de Retorno
```json
{
    "sign": "D+VTPNiwGLzh9eIvkrscwS4UlGKzdnrBgB63RDG4HeobZT6FXqUwYCPgKojynKaxwm5PkmW0xhIASZ4asSCvnYfi0NSFehchZAtUnQIispxKcjsiudWsUznbkEIQ2h2TA/mbUZ1X9+wyh7QhNo6+RkxtgRyRpVb7ARG8pL14cdTAs OTtMLO0W1GO0M83VAv2ybBZNObncX9qy6tdwLQV/KYuNJYyMN0dL0nLKYHnj9Q4d3lEDM45AVJ0153/YIiIgcF BnOWhsQ9rVARcFeXeWd9KJ5OZpmxlxnhcJGcEUY2UDC4zKLZxtUet7CPAyehAMQ5plkpvRrR3Z6lA5zl6GQ==",
    "timestamp": "1725439986754",
    "data": "94f4c29eba73d53dcd3aa1b8cf90a98108d0acf82f38b97a4032dcdf7ff172e7",
    "msg": "ok",
    "code": 1
}
```

## Revisión Secundaria de Orden de Retiro 💳

* Función: Interface de revisión secundaria de control de riesgos de orden de retiro del comerciante
* ⚠️ Nota: **La plataforma asigna a los comerciantes una clave pública RSA de control de riesgos separada (diferente de la clave pública de notificación de callback de depósito/retiro)**
* Tiempo de Activación: Después de que el administrador configure los parámetros de URL de callback de control de riesgos en el lado del comerciante (configuraciones del sistema), el canal agregará una revisión secundaria de callback de control de riesgos adicional para cada solicitud de transacción de retiro. Solo cuando la URL de control de riesgos del lado del comerciante devuelva un código de verificación de paso correcto, la transacción será válida enviada.
* Requisitos Técnicos: Se requiere implementación técnica del lado del comerciante y configuración de la interface de callback de revisión secundaria.

#### Solicitud HTTP

La plataforma envía una solicitud de revisión de retiro al comerciante

> POST: `/withdrawal/order/check`

#### Parámetros de Solicitud

| Nombre del Parámetro | Requerido | Tipo   | Descripción                                                                                                                                                            |
| :------------------- | :-------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| safeCode             | No        | string | ID de transacción único enviado por el comerciante, generalmente correspondiente al ID de orden de retiro del comerciante (SafeCheckCode para transacciones de retiro) |
| openId               | Sí        | string | ID de usuario del comerciante que envía la transacción de retiro                                                                                                       |
| tokenId              | Sí        | string | ID de moneda, basado en el ID de moneda proporcionado por la plataforma                                                                                                |
| toAddress            | Sí        | string | Dirección de retiro                                                                                                                                                    |
| amount               | Sí        | string | Cantidad de retiro                                                                                                                                                     |
| timestamp            | Sí        | int    | Marca de tiempo actual                                                                                                                                                 |
| sign                 | Sí        | string | Firma: Solo se firman los parámetros en el campo data; la corrección de la firma debe verificarse usando la clave pública RSA de control de riesgos de la plataforma.  |

#### Descripción de Parámetros de Retorno

| Nombre del Parámetro | Tipo   | Descripción                                                                                        |
| :------------------- | :----- | :------------------------------------------------------------------------------------------------- |
| code                 | int    | Resultado de verificación. 0 indica paso; otros códigos son inválidos.                             |
| timestamp            | int    | Marca de tiempo actual, en segundos.                                                               |
| message              | string | Mensaje de retorno.                                                                                |
| sign                 | string | Firma: Firma de clave privada RSA del comerciante para el campo data en el parámetro de respuesta. |

## Notificaciones de Callback de Depósito y Retiro

1. Las transacciones de depósito y retiro activarán múltiples notificaciones de callback. Se usará la información de transacción y estado de la última notificación de callback.
2. Se requiere que el lado del negocio devuelva un mensaje de callback válido. El formato se describe en la descripción de parámetros de retorno. Un código de retorno de 0 indica que el mensaje de callback ha sido procesado y no se requieren más notificaciones. De lo contrario, el callback continuará notificando (inicialmente cada 2 segundos por 50 veces, y luego cada 10 minutos) hasta que se devuelva un mensaje de confirmación con código 0.

Contacte al proveedor de servicios para configurar la URL de callback.

> POST

* Función: Define el formato de mensaje de callback que la plataforma usa para notificar al lado de la aplicación sobre información de transacción de tokens (retiro o depósito de usuario). Este mensaje es adecuado para notificaciones de eventos del lado de la aplicación sobre estado de transacción de tokens (retiro o depósito). Las aplicaciones pueden optar por soportar la interface de notificación de callback basada en su funcionalidad de aplicación.

### Parámetros de Solicitud

| Nombre del Parámetro | Requerido | Tipo   | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :------------------- | :-------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| openid               | sí        | string | ID único de usuario del canal                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| totalvalue           | sí        | string | Valor USDT correspondiente a la transacción de depósito o retiro (calculado basado en el precio de mercado en el momento de la transacción)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| status               | sí        | int    | Estado de transacción:</br>1 La transacción está completa y se ha enviado con éxito a la red blockchain. Los detalles de la transacción se pueden consultar en cadena usando el hash.</br>-1 La transacción se ha enviado a la red blockchain, pero la transacción en cadena falló. Puede volver a revisarla en Gestión de Comerciantes --> Gestión de Transacciones --> [Enviar Código de Seguridad de Orden]. La plataforma de negocios no necesita procesar el cambio de estado y puede simplemente esperar a que el canal callback el nuevo estado de notificación.</br>-2 La solicitud de transacción de retiro fue rechazada por el backend del comerciante. La solicitud de retiro ha expirado. Se recomienda que la plataforma de negocios devuelva la solicitud de retiro del usuario después de recibir la notificación.</br>2 La transacción de retiro se ha enviado a la gestión del comerciante. Porque ha activado los requisitos de control de riesgos de seguridad de moneda configurados, el administrador necesita iniciar sesión en Gestión de Comerciantes --> Gestión de Transacciones --> Revisión de Retiro para completar el procesamiento de la solicitud de retiro.</br>3 Durante el procesamiento de blockchain de transacción de retiro, la plataforma de negocios no necesita actualizar el cambio de estado y puede simplemente esperar a que el canal reciba una nueva notificación de estado. </br>⛑️**Recordatorio Especial: Para callbacks de transacción de retiro recibidos por la plataforma de negocios, si status = -1, se ignorará el callback. Después de que el administrador inicie sesión en el backend de gestión y vuelva a enviar la transacción, se empujará una nueva notificación de estado a la plataforma simultáneamente.** |  | type | sí | int | 1 para transacciones de depósito; 2 para transacciones de retiro |
| hash                 | sí        | string | Valor hash de transacción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| confirm              | sí        | int    | Número de confirmaciones en cadena para la transacción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| createdtime          | sí        | string | Tiempo de creación                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| from                 | sí        | string | Dirección del iniciador de la transacción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| to                   | sí        | string | Dirección receptora de la transacción                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |