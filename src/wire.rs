use crate::best_candidates;
use crate::download_song;
use crate::get_lyrics;
use crate::get_track_metadata;
use crate::write_lrc;
use crate::write_metadata;
use macro_colors::Colorize;
use macro_colors::bold_green_println;
use macro_colors::bold_red_println;
use macro_colors::green_println;
use macro_colors::red_println;

pub async fn download(url: &str) {
    let song = get_track_metadata(url).await.unwrap();
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

            let download_path = match download_song(&candidate, &song, "Download") {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Download failed: {}", e);
                    return;
                }
            };
            if let Err(e) = write_metadata(&download_path, &song).await {
                eprintln!("Failed to write metadata: {}", e);
            }
            match get_lyrics(&song).await {
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
