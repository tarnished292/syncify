use syncify::download;

const URL: &str = "https://open.spotify.com/track/1o82DwNisONAd2mu1RcGE6?si=d71132a2d1a34cd6";

#[tokio::main]
async fn main() {
    download(URL).await;
}
