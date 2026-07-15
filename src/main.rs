use syncify::download;

// const URL: &str = "https://open.spotify.com/playlist/2DIftVgibXX2uaTEf6L4a3?si=12217e758a7e41e4";

// test playlist
// const URL: &str = "https://open.spotify.com/playlist/6XIAWqIZ2oGU3b2g6nX2O0?si=528d9a9df3ca4235";

// Single Track
const URL: &str = "https://open.spotify.com/track/2JiDi0qAXsPwhPqA2qaKGt?si=62f2fa64487e41bb";

#[tokio::main]
async fn main() {
    download(URL).await;
}
