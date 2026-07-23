use syncify::library::integration::wire;

// Single Track
const TRACK: &str = "https://open.spotify.com/track/7H7NyZ3G075GqPx2evsfeb?si=1489fd2c2f1b4719";

// Playlist
const URL: &str = "https://open.spotify.com/playlist/2DIftVgibXX2uaTEf6L4a3?si=e6a1d51749ef47d5";

const TEST: &str = "https://open.spotify.com/playlist/6Jc86A5z2Hb1RoXfvGxTaU?si=636466358237496a";

#[tokio::main]
async fn main() {
    let audio_dir = dirs::audio_dir().unwrap();
    wire(TEST, &audio_dir).await;
}
