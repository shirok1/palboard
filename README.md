# PalBoard

[![Build Frontend](https://github.com/shirok1/palboard/actions/workflows/frontend.yml/badge.svg)](https://github.com/shirok1/palboard/actions/workflows/frontend.yml) [![Build Gateway](https://github.com/shirok1/palboard/actions/workflows/gateway.yml/badge.svg)](https://github.com/shirok1/palboard/actions/workflows/gateway.yml) [![Build Docker Image](https://github.com/shirok1/palboard/actions/workflows/docker-image.yml/badge.svg)](https://github.com/shirok1/palboard/actions/workflows/docker-image.yml)

A [Palworld](https://www.pocketpair.jp/palworld) dashboard, built with [Nuxt](https://nuxtjs.org/) and [Axum](https://github.com/tokio-rs/axum).

![Screenshot of "Server Info"](./assets/server_info.webp)

## Development

### Docker Compose

```bash
docker compose build --build-arg VERSION=$(git describe --tags --always --broken)
docker compose up # `docker compose up gateway` for gateway only
```

### Local

You need to install [Node.js](https://nodejs.org/) and [Bun](https://bun.sh/) to build frontend and [Rust](https://www.rust-lang.org/learn/get-started) to build gateway. Other Node.js package managers (e.g. `npm`, `yarn`) won't utilize `bun.lockb` and the build result may be different, so no guarantee.

```bash
# Allow .envrc if you use direnv
direnv allow
# or manually set environment variables, see .envrc for details

# Frontend
cd frontend
bun install
bun dev

# Gateway
cd gateway
RUST_LOG=debug cargo run
```

## Features

### SteamCMD Integration

If you are using the provided Docker compose file, installing or updating the game server is as simple as clicking a button on the web interface.

![Screenshot of updating](./assets/updating.gif)

There are also options to skip verification or updating only Steam itself if you are in a hurry.

### Configuration Editor

A [Monaco Editor](https://microsoft.github.io/monaco-editor/) is embedded to edit `PalWorldSettings.ini` on the web interface.

![Screenshot of "Config Editor"](./assets/config_editor.webp)

### RCON Commands

![Screenshot of "Player List"](./assets/player_list.webp)

See description on <https://tech.palworldgame.com/server-commands>.

Following table is corresponding to v0.1.2.0.

| Command                             | Frontend | Gateway |
| ----------------------------------- | -------- | ------- |
| `/Shutdown {Seconds} {MessageText}` | 🚧       | ✔️      |
| `/DoExit`                           | 🚧       | ✔️      |
| `/Broadcast {MessageText}`          | 🚧       | ✔️      |
| `/KickPlayer {SteamID}`             | ✔️       | ✔️      |
| `/BanPlayer {SteamID}`              | ✔️       | ✔️      |
| `/TeleportToPlayer {SteamID}`       | 🚫       | 🚫      |
| `/TeleportToMe {SteamID}`           | 🚫       | 🚫      |
| `/ShowPlayers`                      | ✔️       | ✔️      |
| `/Info`                             | ✔️       | ✔️      |
| `/Save`                             | ✔️       | ✔️      |

Symbols: ✔️(Complete), 🚧 (Work in Progress), ❌ (Incomplete), 🚫 (Impossible)

Teleport commands are not available because they need player context. If further update allows teleporting players from RCON context, they would be implemented.

### Future Plans

- Container management (enabling server start/restart)
- Configuration management
  - Parsing `DefaultPalWorldSettings.ini` is implemented (see `gateway/src/unreal_struct.rs`)
  - A form-based editor for `PalWorldSettings.ini` is planned

## License

SSPL-1.0. In short, you can use, modify, and distribute the software freely under the same license. However, if you provide the software as a service to others (which is the case if you are running game servers as a service _commercially_), you must release the complete source code, including all necessary components, under the SSPL. Contact me for alternative licensing.

## Disclaimer

This project is not affiliated with Pocketpair, Inc.
