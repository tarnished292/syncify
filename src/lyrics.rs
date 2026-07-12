use reqwest;
use serde::Deserialize;
use std::sync::OnceLock;
use crate::scrapper::Song;


#[derive(Deserialize)]
struct LyricsResponse {
    #[serde(rename = "syncedLyrics")]
    synced_lyrics: String,
}

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

pub async fn get_lyrics(song: &Song) -> Option<String> {
    let client = CLIENT.get_or_init(reqwest::Client::new);

    let primary_artist = song.description.artist.split(',').next().unwrap_or(&song.description.artist);

    let response = client
        .get("https://lrclib.net/api/get")
        .query(&[
            ("track_name", song.title.as_str()),
            ("artist_name", primary_artist),
            ("album_name", song.description.album.as_str()),
        ])
        .send()
        .await
        .ok()?;

    let lyrics = response.json::<LyricsResponse>().await.ok()?;

    Some(lyrics.synced_lyrics)
}
