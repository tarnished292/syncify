use std::error::Error;

use scraper::Html;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Song {
    pub title: String,
    pub description: Description,
    pub duration: String,
    pub image: String,
}

#[derive(Deserialize, Debug)]
pub struct Description {
    pub artist: String,
    pub album: String,
    pub year: String,
}

pub fn get_meta_content(document: &Html, attr: &str, value: &str) -> Result<String, Box<dyn Error>> {
    let selector_body = format!(r#"meta[{}="{}"]"#, attr, value);
    let selector = scraper::Selector::parse(&selector_body).unwrap();
    let element = document.select(&selector).next().ok_or("meta tag not found")?;
    let res = element.value().attr("content").ok_or("content attribute not found")?;

    Ok(res.to_string())
}

pub async fn get_track_metadata(url: &str) -> Result<Song, Box<dyn Error>> {
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);

    let title = get_meta_content(&document, "property", "og:title")?;
    let description = get_meta_content(&document, "property", "og:description")?;
    let duration = get_meta_content(&document, "name", "music:duration")?;
    let image = get_meta_content(&document, "property", "og:image")?;

    let parts: Vec<&str> = description.split(" · ").collect();
    let artist = parts[0];
    let album = parts[1];
    let year = parts[3];

    let description = Description {
        artist: artist.to_string(),
        album: album.to_string(),
        year: year.to_string(),
    };

    let song = Song {
        title: title.to_string(),
        description,
        duration: duration.to_string(),
        image: image.to_string(),
    };
    Ok(song)
}
