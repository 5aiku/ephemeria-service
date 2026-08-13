# Endpoints to be implemented in near future

## GET /api/v1/server/manifest

Returns information about the server and game instance in JSON format

```json
{
  "game\_version": "1.20.1",
  "java\_version": "17",
  "mods\_hash": "sha256:...",
  "server\_ip": "mc.ephemeria.fun",
  "server\_port": 25565
}
```

## GET /api/v1/server/status

Returns current status information about the server in JSON format

```json
{
  "online": true,
  "players": 7,
  "max\_players": 30,
  "motd": "Ephimeria — выживание с модами",
  "version": "1.20.1"
}
```

## GET /api/v1/launcher/version

Returns information about latest launcher release

```json
{
  "version": "0.2.0",
  "hash": "sha256:...",
  "release\_notes": "Добавлено автообновление, исправлена авторизация"
}
```

## GET /api/v1/launcher/download

Hosts latest launcher to be downloaded.

# Endpoints to be implemented later, just ideas

## Endpoints for authorization, so only authorized user can use protected enpoints like RCON commands

## POST /api/v1/server/command

RCON command for authorized only
