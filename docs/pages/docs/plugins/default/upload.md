# upload-plugin

The `datagen-rs-upload-plugin` is a plugin
for [`datagen-rs`](https://markusjx.github.io/datagen/)
providing the ability to upload generated data to a server. The plugin provides a
serialization
interface to serialize the generated data to a string and upload it to a server.

## Parameters

The plugin has the following parameters:

| Name                             | Type                                                                        | Description                                                                                                                                                                                                                           | Default    |
| -------------------------------- |-----------------------------------------------------------------------------| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- |
| `url`                            | [`URL`](#url)                                                               | The URL to upload the data to.                                                                                                                                                                                                        | unset      |
| `method`                         | [`HttpMethod`](#httpmethod)                                                 | The HTTP method to use.                                                                                                                                                                                                               | `post`     |
| `serializer`                     | [`serializer`](https://markusjx.github.io/datagen/docs/options/#serializer) | The serializer to use.                                                                                                                                                                                                                | `json`     |
| `returnNull`                     | `bool`                                                                      | Whether to discard the serialized value. If set to `true`, an empty string will be returned by `datagen`                                                                                                                              | `false`    |
| `headers`                        | `Map<string, string>`                                                       | A map containing headers to send with the request.                                                                                                                                                                                    | `{}`       |
| `splitTopLevelArray`             | `bool`                                                                      | Whether to split the top level array into multiple requests. If set to `true`, each element in the top level array will be uploaded in a separate request.                                                                            | `false`    |
| `numParallelRequests`            | `usize`                                                                     | The number of parallel requests to send.                                                                                                                                                                                              | `1`        |
| `expectedStatus`                 | `u16`                                                                       | The expected status code. If the status code of the response is not equal to this value, an error will be returned.                                                                                                                   | `200`      |
| `timeout`                        | `u64`                                                                       | The timeout in milliseconds.                                                                                                                                                                                                          | `infinite` |
| `auth`                           | [`AuthArgs`](#authargs)                                                     | The authentication method to use.                                                                                                                                                                                                     | `none`     |
| `uploadIn`                       | [`UploadIn`](#uploadin)                                                     | The data to upload.                                                                                                                                                                                                                   | `body`     |
| `disableCertificateVerification` | `bool`                                                                      | Whether to disable SSL certificate verification. ONLY DISABLE IF YOU KNOW WHAT YOU ARE DOING                                                                                                                                          | `false`    |
| `rootCA`                         | `string \| string[]`                                                        | The root CA to use for certificate verification. If a string is provided, it will be used as the path to the root CA certificate. If an array is provided, each element in the array will be used as a path to a root CA certificate. | `unset`    |

### `URL`

The `URL` type is used to specify the URL to upload the data to.

It is defined as follows: `String | Map<String, String>`.

If a string is provided, it will be used as the URL.
If a map is provided, the first type in the generated schema
must be an [`object`](../../generators/object.mdx) where each key in the map
is a field in the object and the value is the URL to use for that field.
If `splitTopLevelArray` is set to `true`, any top-level array in the
object will be split into multiple requests, with each element in the array
being uploaded to the URL specified in the map.

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

#### `oidc`

The `oidc` authentication method is used to send a bearer token to the server
using
the [OpenID Connect Protocol](https://auth0.com/docs/authenticate/protocols/openid-connect-protocol)
for retrieving the token.

| Name           | Type                                  | Description                                | Default             |
| -------------- | ------------------------------------- | ------------------------------------------ | ------------------- |
| `type`         | `'oidc'`                              | The authentication type. Must be `'oidc'`. |                     |
| `clientId`     | `String`                              | The client ID to use.                      | unset               |
| `clientSecret` | `String`                              | The client secret to use. Optional.        | none                |
| `discoveryUrl` | `String`                              | The OIDC endpoint discovery URL.           | unset               |
| `scopes`       | `String[]`                            | The scopes to use.                         | `[]`                |
| `authFlow`     | [`OidcAuthFlow`](#oidcauthflow)       | The authentication flow to use.            | `authorizationCode` |
| `authType`     | [`OidcAuthType`](#oidcauthtype)       | The authentication type to use.            | `requestBody`       |
| `method`       | [`OidcLoginMethod`](#oidcloginmethod) | The method to use for logging in.          | unset               |

##### `OidcAuthFlow`

The `OidcAuthFlow` enum is used to specify the authentication flow to use.
The following values are available:

- `authorizationCode`: This enables the
  [authorization code flow](https://auth0.com/docs/get-started/authentication-and-authorization-flow/authorization-code-flow).
  This flow is recommended for production use.
- `implicit`: This enables
  the [implicit flow](https://auth0.com/docs/get-started/authentication-and-authorization-flow/implicit-flow-with-form-post).
  This flow is not recommended for production use.

##### `OidcAuthType`

The `OidcAuthType` enum is used to specify where the credentials will be sent to the oidc
provider.
The following values are available:

- `requestBody`: The credentials will be sent in the request body.
- `basicAuth`: The credentials will be sent as basic auth.

##### `OidcLoginMethod`

The `OidcLoginMethod` object is used to specify the method to use for logging in.
Currently, there are three methods available:

- [`clientCredentials`](#clientcredentials)
- [`password`](#password)
- [`deviceCode`](#devicecode)

###### `clientCredentials`

The `clientCredentials` method is used to authorize the client using the
[client credentials flow](https://auth0.com/docs/authenticate/login/oidc-conformant-authentication/oidc-adoption-client-credentials-flow).
This method only has a `type` field, which must be set to `'clientCredentials'`.
The client credentials flow will use the `clientId` and `clientSecret` fields
from the [`oidc`](#oidc) object to authorize the client. The `clientSecret` field
must not be unset.

| Name   | Type                  | Description                                      | Default |
| ------ | --------------------- | ------------------------------------------------ | ------- |
| `type` | `'clientCredentials'` | The login method. Must be `'clientCredentials'`. | unset   |

###### `password`

The `password` method is used to authorize the client with the users username and
password using
the [Client Credentials Flow](https://auth0.com/docs/get-started/authentication-and-authorization-flow/client-credentials-flow).

| Name       | Type         | Description                             | Default |
| ---------- | ------------ | --------------------------------------- | ------- |
| `type`     | `'password'` | The login method. Must be `'password'`. | unset   |
| `username` | `String`     | The username to use.                    | unset   |
| `password` | `String`     | The password to use.                    | unset   |

###### `deviceCode`

The `deviceCode` method is used to authorize the client using the
[Device Authorization Flow](https://auth0.com/docs/get-started/authentication-and-authorization-flow/device-authorization-flow).

| Name                     | Type           | Description                               | Default |
| ------------------------ | -------------- | ----------------------------------------- | ------- |
| `type`                   | `'deviceCode'` | The login method. Must be `'deviceCode'`. | unset   |
| `deviceAuthorizationUrl` | `String`       | The URL to use for device authorization   | unset   |

### `UploadIn`

The `UploadIn` enum is used to specify the data to upload.
The following values are available:

- `body`
- `query`
- `form`

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

### Upload data with oidc using a Keycloak instance as the oidc provider

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
          "type": "oidc",
          "clientId": "my-client-id",
          "clientSecret": "my-client-secret",
          "discoveryUrl": "http://localhost:8080/realms/my-realm",
          "scopes": ["openid", "profile", "email"],
          "method": {
            "type": "deviceCode",
            "deviceAuthorizationUrl": "http://localhost:8080/realms/my-realm/protocol/openid-connect/auth/device"
          }
        }
      }
    }
  }
}
```

Note that the `/auth` prefix at the end of the hostname is only required for
Keycloak instances with version 16 or lower. Starting from version 17, the
`/auth` prefix must be omitted.

### Upload data with basic authentication

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
          "type": "basic",
          "username": "my-username",
          "password": "my-password"
        }
      }
    }
  }
}
```

### Upload data with multiple URLs

```json
{
  "type": "object",
  "properties": {
    "a": {
      "type": "string",
      "value": "test"
    },
    "b": {
      "type": "string",
      "value": "test"
    }
  },
  "options": {
    "serializer": {
      "type": "plugin",
      "pluginName": "upload_plugin",
      "args": {
        "url": {
          "a": "http://localhost:8080/upload/a",
          "b": "http://localhost:8080/upload/b"
        }
      }
    }
  }
}
```

This will upload the content of the `a` field to `http://localhost:8080/upload/a`
and the content of the `b` field to `http://localhost:8080/upload/b`.
