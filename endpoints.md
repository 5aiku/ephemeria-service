# Endpoints to be implemented in near future

## GET /api/v1/server/manifest
Returns information about the server and game instance in JSON format
```json
{
  "game_version": "1.20.1",
  "java_version": "17",
  "mods_hash": "sha256:...",
  "server_ip": "mc.ephemeria.fun",
  "server_port": 25565
}
```

## GET /api/v1/server/instance
Hosts game's instance file to be downloaded. For example 'default.zip'

## GET /api/v1/server/status
Returns current status information about the server in JSON format
```json
{
  "online": true,
  "players": 7,
  "max_players": 30,
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
  "release_notes": "Добавлено автообновление, исправлена авторизация"
}
```

## GET /api/v1/launcher/download
Hosts latest launcher to be downloaded.

# Endpoints to be implemented later, just ideas

## Endpoints for authorization, so only authorized user can use protected enpoints like RCON commands

## POST /api/v1/server/command
RCON command for authorized only
