use scraper::Html;
use std::error::Error;

pub async fn playlist_metadata(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let html = reqwest::get(url).await?.text().await?;
    println!(
           "music:song tags found in raw html: {}",
           html.matches(r#"name="music:song""#).count()
       );
    let document = Html::parse_document(&html);
    let selector = scraper::Selector::parse(r#"meta[name="music:song"]"#).unwrap();
    let track_url: Vec<String> = document
        .select(&selector)
        .filter_map(|el| el.value().attr("content"))
        .map(|s| s.to_string())
        .collect();
    println!("scraper found {}", track_url.len());

    Ok(track_url)
}
