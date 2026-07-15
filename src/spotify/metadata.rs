use std::error::Error;

use scraper::{Html, Selector};

pub fn get_meta_content(html: &Html, attribute: &str, content_type: &str) -> Result<String, Box<dyn Error>> {
    let selector_format = format!(r#"meta[{}="{}"]"#, attribute, content_type);
    let selector = Selector::parse(&selector_format).unwrap();
    let element = html
        .select(&selector)
        .next()
        .ok_or("Meta Tag Not Found")?;
    let res = element
        .value()
        .attr("content")
        .ok_or("Content attribute not found")?;

    Ok(res.to_string())
}
