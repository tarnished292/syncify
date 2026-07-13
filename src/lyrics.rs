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
async fn try_fetch(
    client: &reqwest::Client,
    title: &str,
    artist: &str,
    duration: Option<&str>,
) -> Option<String> {
    let mut request = client
        .get("https://lrclib.net/api/get")
        .query(&[
            ("track_name", title),
            ("artist_name", artist),
        ]);

    if let Some(duration) = duration {
        request = request.query(&[
            ("duration", duration)
        ]);
    }

    let response = request.send().await.ok()?;

    if !response.status().is_success() {
        return None;
    }

    let lyrics = response.json::<LyricsResponse>().await.ok()?;

    Some(lyrics.synced_lyrics)
}

pub async fn get_lyrics(song: &Song) -> Option<String> {
    let client = CLIENT.get_or_init(reqwest::Client::new);

    let original_title = song.title.as_str();
    let clean_title = clean_title(original_title);

    let full_artist = song.description.artist.as_str();

    let primary_artist = full_artist
        .split(',')
        .next()
        .unwrap_or(full_artist)
        .trim();

    let duration = song.duration.as_str();

    let attempts = [
        (original_title, full_artist, Some(duration)),
        (clean_title.as_str(), full_artist, Some(duration)),
        (clean_title.as_str(), primary_artist, Some(duration)),
        (clean_title.as_str(), primary_artist, None),
    ];

    for (title, artist, duration) in attempts {
        println!("Trying: {} - {}", artist, title);

        if let Some(lyrics) = try_fetch(
            client,
            title,
            artist,
            duration,
        ).await {
            println!("Lyrics matched using: {} - {}", artist, title);
            return Some(lyrics);
        }
    }

    None
}

fn clean_title(title: &str) -> String {
   
    let no_parens = if let Some(idx) = title.find(['(', '[']) {
        title[..idx].trim()
    } else {
        title.trim()
    };

    
    let lower = no_parens.to_lowercase();
    let cutoff = lower.find(" feat.")
        .or_else(|| lower.find(" ft."))
        .or_else(|| lower.find(" featuring"));

    match cutoff {
        Some(idx) => no_parens[..idx].trim().to_string(),
        None => no_parens.to_string(),
    }
}