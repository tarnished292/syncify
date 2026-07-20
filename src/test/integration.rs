use crate::dlp::download::download;
use crate::dlp::score::score;
use crate::dlp::search::search_candidate;
use crate::lyrics::lyrics::get_lyrics;
use crate::metadata::tag::write_metadata;
use crate::metadata::writer::write_lrc;
use crate::spotify::{playlist::get_playlist_metadata, track::get_song_details};
use macro_colors::green_println;
use macro_colors::{Colorize, red_println};
use rand::Rng;
use std::path::PathBuf;
use std::ptr::dangling;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

pub async fn wire(url: &str) {
    let start = Instant::now();
    let audio_dir = dirs::audio_dir().unwrap();

    if url.contains("/track/") {
        process_song(url, &audio_dir).await;
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
            process_song(&track, &audio_dir).await;
        }
    }

    println!("Time Taken {:?}", start.elapsed());
    println!("======================");
}

async fn process_song(song: &str, dir: &PathBuf) {
    let jitter = rand::thread_rng().gen_range(50..300);
    sleep(Duration::from_millis(jitter)).await;

    let song = match get_song_details(song).await {
        Ok(song) => song,
        Err(e) => {
            red_println!("Failed to fetch metadata {:?}", e);
            return;
        }
    };
    println!("Data From Spotify");
    green_println!("Title: {:?}", song.title);
    green_println!("Artist: {:?}", song.description.artist);
    green_println!("Album: {:?}", song.description.album);
    green_println!("Year: {:?}", song.description.year);
    green_println!("Duartion: {:?}", song.duration);
    green_println!("======================");

    let yt_data = search_candidate(&song);
    for data in &yt_data {
        
    red_println!("VIDEO ID: {:?}", data.video_id);
    red_println!("Title: {:?}", data.title);
    red_println!("UPLOADER: {:?}", data.uploader);
    red_println!("Duration: {:?}", data.duration);
    }

    let best = match score(&yt_data, &song) {
        Some(best) => best,
        None => {
            red_println!("No matching YouTube candidate found for {}", song.title);
            return;
        }
    };

    println!("Best Data from Yt-DLP");
    red_println!("Title: {:?}", best.video_id);
    red_println!("Title: {:?}", best.title);
    red_println!("Title: {:?}", best.uploader);
    red_println!("Title: {:?}", best.duration);

    let (mp3, lyrics) = tokio::join!(download(&best, &song, &dir), get_lyrics(&song));
    let output = match mp3{
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
