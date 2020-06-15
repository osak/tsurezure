# Deployment

## Set password in env var
Admin APIs are protected by Basic auth. Set `ADMIN_BASIC_AUTH` env var to Base64 encoded credential.
```
$ echo -n "user:pass" | base64
```