use syncify::download;

const URL: &str = "https://open.spotify.com/track/40riOy7x9W7GXjyGp4pjAv?si=25c9e9e9fc214a72";

#[tokio::main]
async fn main() {
    download(URL).await;
}
