# Syncify

Syncify scrapes track metadata from Spotify, finds the best matching audio via `yt-dlp`, and downloads the song with embedded metadata and cover art — while saving lyrics to a separate file.

Usable both as a **Rust library** and as a **standalone CLI tool**.

## How it works

1. **Fetch metadata** — Given a Spotify track URL, Syncify scrapes the page's Open Graph / meta tags to extract:
   - Title
   - Artist
   - Album
   - Release year
   - Duration
   - Cover image
2. **Find a match** — That metadata is used to query `yt-dlp` for the best matching audio source.
3. **Download & tag** — The matched audio is downloaded, tagged with the fetched metadata, and the cover image is embedded into the file.
4. **Lyrics** — Lyrics are fetched and saved as a separate file alongside the audio.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [`yt-dlp`](https://github.com/yt-dlp/yt-dlp) installed and available on your `PATH`
- `ffmpeg` (required by `yt-dlp` for audio extraction/embedding)

## Installation

### As a CLI tool

```bash
cargo install --path .
```

### As a library

Add to your `Cargo.toml`:

```toml
[dependencies]
syncify = { path = "../syncify" } # or a version/git source once published
```

## Usage

Syncify can be used as a command-line tool, pointing it at a Spotify track URL to fetch, match, download, and tag the song. It can also be used as a library within other Rust projects by calling its metadata-fetching and download functions directly.

## Disclaimer

Syncify is intended for personal use with content you have the rights to download. Respect the terms of service of the platforms it interacts with, and applicable copyright law in your jurisdiction.

## License

Licensed under the [MIT License](License).