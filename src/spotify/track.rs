use scraper::Html;
use std::error::Error;

use crate::spotify::metadata::get_meta_content;

pub struct Track {
    pub title: String,
    pub description: Description,
    pub duration: String,
}

pub struct Description {
    pub artist: String,
    pub album: String,
    pub year: String,
}

pub async fn get_song_details(url: &str) -> Result<Track, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let html = response.text().await?;

    let document = Html::parse_document(&html);
    let title = get_meta_content(&document, "property", "og:title")?;
    let description = get_meta_content(&document, "property", "og:description")?;
    let duration = get_meta_content(&document, "name", "music:duration")?;

    let description: Vec<&str> = description.split(" · ").collect();
    let artist = description[0].to_string();
    let album = description[1].to_string();
    let year = description[3].to_string();

    let description = Description {
        artist,
        album,
        year,
    };

    Ok(Track {
        title,
        description,
        duration,
    })
}
