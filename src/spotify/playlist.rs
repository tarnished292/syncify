use std::error::Error;

use scraper::Html;

pub async fn get_playlist_metadata(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);
    let selector = scraper::Selector::parse(r#"meta[name="music:song"]"#).unwrap();
    let tracks: Vec<String> = document
        .select(&selector)
        .filter_map(|el| el.value().attr("content"))
        .map(|s| s.to_string())
        .collect();

    Ok(tracks)
}
