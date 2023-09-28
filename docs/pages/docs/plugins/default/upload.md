# upload-plugin

The `datagen-rs-upload-plugin` is a plugin for [`datagen-rs`](https://markusjx.github.io/datagen/)
providing the ability to upload generated data to a server. The plugin provides a serialization
interface to serialize the generated data to a string and upload it to a server.

## Parameters

The plugin has the following parameters:

| Name                  | Type                                                                        | Description                                                                                                                                                | Default    |
| --------------------- | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| `url`                 | `String`                                                                    | The URL to upload the data to.                                                                                                                             | unset      |
| `method`              | [`HttpMethod`](#HttpMethod)                                                 | The HTTP method to use.                                                                                                                                    | `post`     |
| `serializer`          | [`serializer`](https://markusjx.github.io/datagen/docs/options/#serializer) | The serializer to use.                                                                                                                                     | `json`     |
| `returnNull`          | `bool`                                                                      | Whether to discard the serialized value. If set to `true`, an empty string will be returned by `datagen`                                                   | `false`    |
| `headers`             | `Map<String, String>`                                                       | A map containing headers to send with the request.                                                                                                         | `{}`       |
| `splitTopLevelArray`  | `bool`                                                                      | Whether to split the top level array into multiple requests. If set to `true`, each element in the top level array will be uploaded in a separate request. | `false`    |
| `numParallelRequests` | `usize`                                                                     | The number of parallel requests to send.                                                                                                                   | `1`        |
| `expectedStatus`      | `u16`                                                                       | The expected status code. If the status code of the response is not equal to this value, an error will be returned.                                        | `200`      |
| `timeout`             | `u64`                                                                       | The timeout in milliseconds.                                                                                                                               | `infinite` |
| `auth`                | [`AuthArgs`](#AuthArgs)                                                     | The authentication method to use.                                                                                                                          | `none`     |

### `HttpMethod`

The `HttpMethod` enum is used to specify the HTTP method to use.
The following values are available:

- `post`
- `put`
- `patch`

### `AuthArgs`

The `AuthArgs` object is used to specify the authentication method to use.
The authentication type is specified by the `type` field.
The following authentication types are available:

#### `basic`

The `basic` authentication method is used to send a username and password
to the server. The `AuthArgs` object has the following fields:

| Name       | Type      | Description                                 | Default |
| ---------- | --------- | ------------------------------------------- | ------- |
| `type`     | `'basic'` | The authentication type. Must be `'basic'`. |         |
| `username` | `String`  | The username to use.                        | unset   |
| `password` | `String`  | The password to use. Optional.              | unset   |

#### `bearer`

The `bearer` authentication method is used to send a bearer token to the server.

| Name    | Type       | Description                                  | Default |
| ------- | ---------- | -------------------------------------------- | ------- |
| `type`  | `'bearer'` | The authentication type. Must be `'bearer'`. |         |
| `token` | `String`   | The token to use.                            | unset   |

#### `keycloak`

The `keycloak` authentication method is used to send a bearer token to the server
using a [Keycloak](https://www.keycloak.org/) instance for authentication.

| Name       | Type         | Description                                    | Default |
| ---------- | ------------ | ---------------------------------------------- | ------- |
| `type`     | `'keycloak'` | The authentication type. Must be `'keycloak'`. |         |
| `host`     | `String`     | The host of the Keycloak instance.             | unset   |
| `realm`    | `String`     | The realm to use.                              | unset   |
| `username` | `String`     | The username to use.                           | unset   |
| `password` | `String`     | The password to use.                           | unset   |
| `clientId` | `String`     | The client ID to use.                          | unset   |

## Examples

### Upload data without authentication

```json
{
  "type": "string",
  "value": "test",
  "options": {
    "serializer": {
      "type": "plugin",
      "pluginName": "upload_plugin",
      "args": {
        "url": "http://localhost:8080/upload",
        "method": "post",
        "serializer": {
          "type": "json",
          "pretty": true
        },
        "expectedStatus": 200,
        "timeout": 1000
      }
    }
  }
}
```

### Upload data with keycloak auth

```json
{
  "type": "string",
  "value": "test",
  "options": {
    "serializer": {
      "type": "plugin",
      "pluginName": "upload_plugin",
      "args": {
        "url": "http://localhost:8080/upload",
        "auth": {
          "type": "keycloak",
          "host": "http://localhost:8080/auth",
          "realm": "my-realm",
          "username": "my-username",
          "password": "my-password",
          "clientId": "my-client-id"
        }
      }
    }
  }
}
```

Note that the `/auth` prefix at the end of the hostname is only required for
Keycloak instances with version 16 or lower. Starting from version 17, the
`/auth` prefix must be omitted.
