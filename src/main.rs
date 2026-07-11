use syncify::{get_track_metadata, playlist_metadata};

const URL: &str = "https://open.spotify.com/playlist/2DIftVgibXX2uaTEf6L4a3?si=e85c4ac4e0374a8b";

#[tokio::main]
async fn main() {
    let data = playlist_metadata(URL).await.unwrap();
    println!("Found {} tracks\n", data.len());
    for music_url in data {
        match get_track_metadata(&music_url).await {
            Ok(song) => {
                println!("════════════════════════════════════");
                println!("Title    : {}", song.title);
                println!("Artist   : {}", song.description.artist);
                println!("Album    : {}", song.description.album);
                println!("Year     : {}", song.description.year);
                println!("Duration : {}", song.duration);
                println!("Cover    : {}", song.image);
                println!("URL      : {}", music_url);
                println!("════════════════════════════════════\n");
            }
            Err(e) => {
                println!("Failed to fetch {}: {}", music_url, e);
            }
        }
    }
}
