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
     let clean_song_title = clean_title(&song.title);
      let duration_str = song.duration.to_string();
     let query = vec![
           ("track_name", clean_song_title.as_str()),
           ("artist_name", primary_artist),
           ("duration", duration_str.as_str()),
       ];

    let response = client
        .get("https://lrclib.net/api/get")
        .query(&query)
        .send()
        .await
        .ok()?;
    println!("{}", response.status());

    let lyrics = response.json::<LyricsResponse>().await.ok()?;

    Some(lyrics.synced_lyrics)
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