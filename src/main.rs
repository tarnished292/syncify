use syncify::{get_lyrics, get_track_metadata, score_candidate, search_candidates};

const URL: &str = "https://open.spotify.com/track/0JmnkIqdlnUzPaf8sqBRs3?si=09d8e0cd9d534b94";

#[tokio::main]
async fn main() {
    let song = get_track_metadata(&URL)
    .await
    .unwrap();

    if let Some(lyrics) = get_lyrics(&song).await {
     println!("{}", lyrics);
    }

    println!("Spotify:");
    println!("Title    : {}", song.title);
    println!("Artist   : {}", song.description.artist);
    println!("Duration : {}\n", song.duration);

    let results = search_candidates(&song).await.unwrap();

    println!("Top candidates:");
    println!("════════════════════════════════════");
    for (i, candidate) in results.iter().enumerate() {
        println!("Candidate #{}", i + 1);
        println!("ID       : {}", candidate.id);
        println!("Title    : {}", candidate.title);
        println!("Uploader : {}", candidate.uploader);
        println!("Duration : {}", candidate.duration);
        println!("────────────────────────────────────");
    }

    let top = score_candidate(&song, &results);

    println!("\n>>> Best match:");
    match top {
        Some(candidate) => {
            println!("════════════════════════════════════");
            println!("ID       : {}", candidate.id);
            println!("Title    : {}", candidate.title);
            println!("Uploader : {}", candidate.uploader);
            println!("Duration : {}", candidate.duration);
            println!("════════════════════════════════════");
        }
        None => {
            println!("No good match found for this song");
        }
    }
}
