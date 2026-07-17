use syncify::test::integration::wire;

// Single Track
const TRACK: &str = "https://open.spotify.com/track/37BZB0z9T8Xu7U3e65qxFy?si=c6e2629b19394986";

// Playlist
const URL: &str = "https://open.spotify.com/playlist/2DIftVgibXX2uaTEf6L4a3?si=e6a1d51749ef47d5";

const TEST: &str = "https://open.spotify.com/playlist/6XIAWqIZ2oGU3b2g6nX2O0?si=84d0a92aeb424e0c";

#[tokio::main]
async fn main() {
    wire(TRACK).await;
}