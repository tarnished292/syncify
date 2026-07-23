use reqwest;
use serde::Deserialize;
use std::sync::OnceLock;

use crate::spotify::track::Track;

#[derive(Deserialize)]
struct LyricsResponse {
    #[serde(rename = "syncedLyrics")]
    synced_lyrics: Option<String>,
}

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub async fn get_lyrics(song: &Track) -> Option<String> {
    let client = CLIENT.get_or_init(reqwest::Client::new);

    let duration = song.duration.parse::<u64>().ok()?;

    let response = client
        .get("https://lrclib.net/api/get")
        .query(&[
            ("track_name", song.title.as_str()),
            ("artist_name", song.description.artist.as_str()),
            ("album_name", song.description.album.as_str()),
            ("duration", &duration.to_string()),
        ])
        .send()
        .await
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let lyrics = response
        .json::<LyricsResponse>()
        .await
        .ok()?;

    lyrics.synced_lyrics
}