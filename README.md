# waveemapi3
[![Rust](https://github.com/wsand02/waveemapi3/actions/workflows/rust.yml/badge.svg)](https://github.com/wsand02/waveemapi3/actions/workflows/rust.yml)
[![Clippy check](https://github.com/wsand02/waveemapi3/actions/workflows/clippy.yml/badge.svg)](https://github.com/wsand02/waveemapi3/actions/workflows/clippy.yml)
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

Create the data directory `data`.

Make sure the user running waveemapi has the right set of permissions for it.

Copy `waveemapi.toml.example` into `waveemapi.toml`.

Remove the `.example` from the `waveemapi.toml.example` and change it to your liking.

If your data folder is not in the same directory as `Cargo.toml` you must declare it here under `data_path`.

## API Routes

### `(GET) /status/api`

Returns a simple JSON status response.

### `(POST) /status/upload`

Accepts a multipart form upload:
+ wav: The WAV file to convert.
+ Requires a bearer token, if auth is enabled.

