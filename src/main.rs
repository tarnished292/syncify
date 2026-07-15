use syncify::test::integration::wire;

// Single Track
const URL: &str = "https://open.spotify.com/track/3zEN0ii6s4DHHBpnTp3RP7?si=39268f955cd84950";

// Playlist
// const URL: &str = "https://open.spotify.com/playlist/6XIAWqIZ2oGU3b2g6nX2O0?si=6f8028425afb44bb";

#[tokio::main]
async fn main() {
    wire(URL).await;
}