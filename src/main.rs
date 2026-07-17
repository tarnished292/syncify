use syncify::test::integration::wire;

// Single Track
const TRACK: &str = "https://open.spotify.com/track/4wajJ1o7jWIg62YqpkHC7S?si=6e61d641b1444fe7";

// Playlist
const URL: &str = "https://open.spotify.com/playlist/2DIftVgibXX2uaTEf6L4a3?si=e6a1d51749ef47d5";

const TEST: &str = "https://open.spotify.com/playlist/6XIAWqIZ2oGU3b2g6nX2O0?si=84d0a92aeb424e0c";

#[tokio::main]
async fn main() {
    wire(TRACK).await;
}
