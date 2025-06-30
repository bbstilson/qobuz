# Qobuz - New Music Alerts

Check for new music from artists you care about on Qobuz.

## Usage

First, get your auth token and app id. You can find these in the headers of an authenticated request.

```bash
export QOBUZ_AUTH_TOKEN="<...>"
export QOBUZ_APP_ID="<...>"
```

Load the artists you want to follow:

```bash
# The artist id can be found in the URL when you load an artist's page.
# For example: https://play.qobuz.com/artist/1373166
cargo r -- load <artist_id>
```

Then, periodically check for new music and generate a new playlist:

```bash
cargo r -- check-gen
```

## Developing

```bash
# fmt
cargo fmt

# clippy
cargo clippy --tests -- -Dclippy::all -Dclippy::pedantic -Dwarnings

# Cargo.toml lints
cargo sort
cargo +nightly udeps

# tests
cargo test
```
