# waveemapi3

**waveemapi3** is yet another wav to mp3 api, but at least it's not a ffmpeg wrapper.

This service is intended for internal API-to-API communication with static api authentication.
And only really exists because LAME is licensed with the lame LGPL license :D

## Installation

Only one method exists for now.

### Building from source

```bash
git clone https://github.com/wsand02/waveemapi3.git
cd waveemapi3
cargo build --release
```

## Config

### Token
Set the `WAVEEMAPI_TOKEN` (environment variable) to whatever you want.

### File limit
Remove the `.example` from the `Rocket.toml.example` and change it to your liking.

## API Routes

### `(GET) /status/api`

Returns a simple JSON status response.

### `(POST) /status/upload`

Accepts a multipart form upload:
+ wav: The WAV file to convert.
+ Requires an `x-api-key` header matching `WAVEEMAPI_TOKEN`.

