use std::time::Duration;
use std::time::Instant;

use crate::best_candidates;
use crate::download_song;
use crate::get_lyrics;
use crate::get_track_metadata;
use crate::playlist_metadata;
use crate::scrapper::Song;
use crate::write_lrc;
use crate::write_metadata;
use macro_colors::Colorize;
use macro_colors::bold_green_println;
use macro_colors::bold_red_println;
use macro_colors::green_println;
use macro_colors::red_println;

pub async fn download(url: &str) {
    let start = Instant::now();
    if url.contains("/track/") {
        println!("This is a Track");
        let song = get_track_metadata(url).await.unwrap();
        process_song(&song).await;
    } else if url.contains("/playlist/") {
        let results = playlist_metadata(url).await.unwrap();
        println!(
            "Playlist metadata returned {} track URLs (expected )",
            results.len()
        );
        for url in results {
            let song = get_track_metadata(&url).await.unwrap();

            process_song(&song).await;
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
    let elapsed = start.elapsed();
    println!("============================");
    println!("Total time: {:.2?}", elapsed);
    println!("============================");
}

async fn process_song(song: &Song) {
    println!("============================");
    bold_green_println!("Data from Spotify");
    green_println!("Song Title: {}", song.title);
    green_println!("Song Artist: {}", song.description.artist);
    green_println!("Song album: {}", song.description.album);
    green_println!("Song Duration: {}", song.duration);
    println!("============================");

    let top_result = best_candidates(&song).await;
    match top_result {
        Some(candidate) => {
            bold_red_println!("Data from Youtube");
            red_println!("Video Id: {}", candidate.id);
            red_println!("Video Title: {}", candidate.title);
            red_println!("Video Uploader: {}", candidate.uploader);
            red_println!("Video Duration: {}", candidate.duration);
            println!("============================");

            let song_for_download = song.clone();
            let candidate_for_download = candidate.clone();
            let song_for_lyrics = song.clone();

            let download_task = tokio::task::spawn_blocking(move || {
                download_song(&candidate_for_download, &song_for_download, "Download")
            });

            let lyrics_task = tokio::spawn(async move { get_lyrics(&song_for_lyrics).await });

            let (download_result, lyrics_result) = tokio::join!(download_task, lyrics_task);

            let download_path = match download_result {
                Ok(Ok(path)) => path,
                Ok(Err(e)) => {
                    eprintln!("Download failed: {}", e);
                    return;
                }
                Err(e) => {
                    eprintln!("Download task panicked: {}", e);
                    return;
                }
            };

            if let Err(e) = write_metadata(&download_path, &song).await {
                eprintln!("Failed to write metadata: {}", e);
            }
            let lyrics = match lyrics_result {
                Ok(inner) => inner,
                Err(e) => {
                    eprintln!("Lyrics task panicked: {}", e);
                    None
                }
            };
            match lyrics {
                Some(lyrics) => {
                    if let Err(e) = write_lrc(&download_path, &lyrics) {
                        eprintln!("Failed to write Lrc: {}", e);
                    }
                }
                None => println!("No Lyrics Found for the song"),
            }
        }
        None => println!("No Best Match Found"),
    }
}
