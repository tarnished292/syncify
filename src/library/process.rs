use std::{path::PathBuf, time::Duration};
use macro_colors::red_println;
use rand::Rng;
use tokio::time::sleep;
use crate::{
    dlp::{download::download, score::score, search::search_candidate},
    lyrics::lyrics::get_lyrics,
    metadata::{tag::write_metadata, writer::write_lrc},
    spotify::track::get_song_details,
};
use macro_colors::Colorize;

pub async fn process_song(song: &str, dir: &PathBuf) {
    let jitter = rand::thread_rng().gen_range(50..300);
    sleep(Duration::from_millis(jitter)).await;

    let song = match get_song_details(song).await {
        Ok(song) => song,
        Err(e) => {
            red_println!("Failed to fetch metadata {:?}", e);
            return;
        }
    };
    let yt_data = search_candidate(&song);

    let best = match score(&yt_data, &song) {
        Some(best) => best,
        None => {
            red_println!("No matching YouTube candidate found for {}", song.title);
            return;
        }
    };

    let (mp3, lyrics) = tokio::join!(download(&best, &song, &dir), get_lyrics(&song));
    let output = match mp3 {
        Ok(path) => path,
        Err(e) => {
            red_println!("Download failed: {e:?}");
            return;
        }
    };

    if let Err(e) = write_metadata(&song, &output).await {
        red_println!("Failed to write metadata: {e:?}");
    }

    match lyrics {
        Some(lyrics) => {
            if let Err(e) = write_lrc(&output, &lyrics) {
                eprintln!("Failed to write Lrc: {}", e);
            }
        }
        None => red_println!("No Lyrics Found for {}", song.title),
    }
}
