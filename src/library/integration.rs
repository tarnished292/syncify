use macro_colors::Colorize;
use macro_colors::red_println;
use std::path::PathBuf;

use crate::{library::process::process_song, spotify::playlist::get_playlist_metadata};

pub async fn wire(url: &str, output_dir: &PathBuf) {
    if url.contains("/track/") {
        process_song(url, &output_dir).await;
    } else {
        let songs = match get_playlist_metadata(url).await {
            Ok(songs) => songs,
            Err(e) => {
                red_println!("Failed to fetch Playlist for {:?}", e);
                return;
            }
        };
        println!("{} Songs Found Inside the playlist", &songs.len());
        for track in &songs {
            process_song(&track, &output_dir).await;
        }
    }
}
