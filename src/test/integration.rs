use crate::dlp::download::download;
use crate::dlp::score::score;
use crate::dlp::search::search_candidate;
use crate::metadata::tag::write_metadata;
use crate::spotify::{playlist::get_playlist_metadata, track::get_song_details};
use futures::stream::{self, StreamExt};
use macro_colors::green_println;
use macro_colors::{Colorize, red_println};
use rand::Rng;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

pub async fn wire(url: &str) {
    let start = Instant::now();
    let audio_dir = dirs::audio_dir().unwrap();

    if url.contains("/track/") {
        process_song(url, &audio_dir).await;
    } else {
        let songs = get_playlist_metadata(url).await.unwrap();
        println!("{} Songs Found Inside the playlist", &songs.len());
        for track in &songs {
            process_song(&track, &audio_dir).await;
        }

    }

    println!("Time Taken {:?}", start.elapsed());
    println!("======================");
}

async fn process_song(song: &str, dir: &PathBuf) {
    let jitter = rand::thread_rng().gen_range(50..300);
    sleep(Duration::from_millis(jitter)).await;
    let song = get_song_details(song).await.unwrap();
    println!("Data From Spotify");
    green_println!("Title: {:?}", song.title);
    green_println!("Artist: {:?}", song.description.artist);
    green_println!("Album: {:?}", song.description.album);
    green_println!("Year: {:?}", song.description.year);
    green_println!("Duartion: {:?}", song.duration);
    green_println!("======================");

    let yt_data = search_candidate(&song);

    let best = score(&yt_data, &song);
    println!("Best Data from Yt-DLP");
    red_println!("Title: {:?}", best.video_id);
    red_println!("Title: {:?}", best.title);
    red_println!("Title: {:?}", best.uploader);
    red_println!("Title: {:?}", best.duration);

    let output = match download(&best, &song, &dir) {
        Ok(path) => path,
        Err(e) => {
            red_println!("Download failed: {e:?}");
            return;
        }
    };

    if let Err(e) = write_metadata(&song, output).await {
        red_println!("Failed to write metadata: {e:?}");
    }
}
