use syncify::{get_track_metadata, playlist_metadata, search_candidates};

const URL: &str = "https://open.spotify.com/track/5RBOcBpJXaNnHCGViJmYhh?si=ad75636ff2b34458";

#[tokio::main]
async fn main() {

    let song = get_track_metadata(
        "https://open.spotify.com/track/1UGD3lW3tDmgZfAVDh6w7r"
    ).await.unwrap();
    
    search_candidates(song).await.unwrap();
    
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
