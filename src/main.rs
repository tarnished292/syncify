use syncify::test::integration::wire;

// Single Track
const TRACK: &str = "https://open.spotify.com/track/1cQ3LIkZE68pMSnPhZWXLu?si=a1c0be0991b0417b";

// Playlist
const URL: &str = "https://open.spotify.com/playlist/2DIftVgibXX2uaTEf6L4a3?si=e6a1d51749ef47d5";

const TEST: &str = "https://open.spotify.com/playlist/6XIAWqIZ2oGU3b2g6nX2O0?si=84d0a92aeb424e0c";

#[tokio::main]
async fn main() {
    wire(TRACK).await;
}
