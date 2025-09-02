# waveemapi3
![GitHub Release](https://img.shields.io/github/v/release/wsand02/waveemapi3)
[![Rust](https://github.com/wsand02/waveemapi3/actions/workflows/rust.yml/badge.svg)](https://github.com/wsand02/waveemapi3/actions/workflows/rust.yml)
[![Clippy check](https://github.com/wsand02/waveemapi3/actions/workflows/clippy.yml/badge.svg)](https://github.com/wsand02/waveemapi3/actions/workflows/clippy.yml)
[![Publish Docker image](https://github.com/wsand02/waveemapi3/actions/workflows/docker-publish.yml/badge.svg)](https://hub.docker.com/r/wsand02/waveemapi3)
![GitHub repo size](https://img.shields.io/github/repo-size/wsand02/waveemapi3)
![GitHub License](https://img.shields.io/github/license/wsand02/waveemapi3)

**waveemapi3** is yet another WAV-to-MP3 API, but at least it's not an FFmpeg wrapper. (I wish it was at this point 😭😭).

This service is intended for internal API-to-API communication with simple API authentication. It only really exists because LAME is licensed with the LGPL license.

## Installation

### Building from Source

```bash
git clone https://github.com/wsand02/waveemapi3.git
cd waveemapi3
cargo build --release
```

Create the data directory. Ensure the user running waveemapi has the correct permissions for the directory.

```bash
mkdir data
```

Copy `waveemapi.toml.example` into `waveemapi.toml`.

If your data folder is not in the same directory as `Cargo.toml`, or is called anything other than `data`, please declare it under `data_path` in the configuration file. Alternatively as an environment variable, see the [environment variable](#environment-variables) section for more info.

### Docker Compose

Copy `docker-compose.yml.example` into `docker-compose.yml`.

Edit the tokens and other environment variables to your liking.

Run Docker Compose:

```bash
docker-compose up -d
```

Verify the service is running:

```bash
docker-compose ps
```

## API Routes

### `(GET) /api/status`

Returns a simple JSON status response.

#### Example Response:

```json
{
  "status": "Online"
}
```

### `(POST) /api/upload`

Accepts a multipart form upload:
- `wav`: The WAV file to convert.
- Requires a bearer token, if authentication is enabled.

#### Example Request:

```bash
curl -X POST \
  -H "Authorization: Bearer your_token" \
  -F "wav=@path/to/file.wav" \
  http://localhost:8000/api/upload
```

Returns a raw MP3 file, or a multitude of errors.

## Configuration

**waveemapi** uses a configuration file named `waveemapi.toml` and supports environment variable overrides.

**Note:** waveemapi inherits all of Rocket's [available settings](https://rocket.rs/guide/v0.5/configuration/#configuration).

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
- `WAVEEMAPI_AUTH_TOKENS`: A list of API tokens.

#### Example:

```bash
export WAVEEMAPI_DATA_PATH="/path/to/data"
export WAVEEMAPI_AUTH_ENABLED=true
```

## License

As stated earlier, this project is licensed under the LGPL license, see [LICENSE](LICENSE).
