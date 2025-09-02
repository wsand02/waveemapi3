# waveemapi3
![GitHub Release](https://img.shields.io/github/v/release/wsand02/waveemapi3)
[![Rust](https://github.com/wsand02/waveemapi3/actions/workflows/rust.yml/badge.svg)](https://github.com/wsand02/waveemapi3/actions/workflows/rust.yml)
[![Clippy check](https://github.com/wsand02/waveemapi3/actions/workflows/clippy.yml/badge.svg)](https://github.com/wsand02/waveemapi3/actions/workflows/clippy.yml)
[![Publish Docker image](https://github.com/wsand02/waveemapi3/actions/workflows/docker-publish.yml/badge.svg)](https://hub.docker.com/r/wsand02/waveemapi3)
![GitHub repo size](https://img.shields.io/github/repo-size/wsand02/waveemapi3)
![GitHub License](https://img.shields.io/github/license/wsand02/waveemapi3)

**waveemapi3** is yet another wav to mp3 api, but at least it's not a ffmpeg wrapper.

This service is intended for internal API-to-API communication with simple API authentication.
And only really exists because LAME is licensed with the lame LGPL license :D

## Installation

Only one method exists for now.

### Building from source

```bash
git clone https://github.com/wsand02/waveemapi3.git
cd waveemapi3
cargo build --release
```

Create the data directory. (Make sure the user running waveemapi has the right set of permissions for the directory.)

```bash
mkdir data
```

Copy `waveemapi.toml.example` into `waveemapi.toml`.

If your data folder is not in the same directory as `Cargo.toml` you must declare it here under `data_path`.

### Docker compose

Copy `docker-compose.yml.example` into `docker-compose.yml`.

Edit the tokens and other environment variables to your liking.

Run docker compose
```bash
docker-compose up -d
```


## API Routes

### `(GET) /status/api`

Returns a simple JSON status response.

### `(POST) /status/upload`

Accepts a multipart form upload:
+ wav: The WAV file to convert.
+ Requires a bearer token, if auth is enabled.

## Configuration

**waveemapi** uses a configuration file named `waveemapi.toml` and supports environment variable overrides.

**Note:** waveemapi inherits all of rocket's [available settings](https://rocket.rs/guide/v0.5/configuration/#configuration).

Below are the key configuration options:

### `waveemapi.toml`

```toml
[default]
# Limits for form uploads
limits = { form = "1 GiB", data-form = "1 GB", file = "500 MB" }

# API authentication tokens. Add tokens to enable bearer token authentication.
auth_tokens = [
  "your_secret_token"
]

# Enable or disable authentication. Set to `false` to bypass authentication.
auth_enabled = true
```

### Environment Variables

You can override the configuration using environment variables. The following variables are supported:

- `WAVEEMAPI_DATA_PATH`: Path to the directory where data files are stored.
- `WAVEEMAPI_AUTH_ENABLED`: Set to `true` or `false` to enable or disable authentication.
- `WAVEEMAPI_PROFILE`: Specify the configuration profile to use (e.g., `default`).
- `WAVEEMAPI_AUTH_TOKENS`: A list of api tokens.

Make sure to adjust these settings according to your environment and requirements.

