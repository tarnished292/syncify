use crate::spotify::{playlist::get_playlist_metadata, track::get_song_details};
use futures::stream::{self, StreamExt};
use macro_colors::Colorize;
use macro_colors::green_println;
use std::time::Instant;

pub async fn wire(url: &str) {
    let start = Instant::now();

    if url.contains("/track/") {
        process_song(url).await;
    } else {
        let songs = get_playlist_metadata(url).await.unwrap();


        stream::iter(&songs)
            .for_each_concurrent(3, |track| async move {
                process_song(&track).await;
            })
            .await;
        println!("{} Songs Found Inside the playlist", &songs.len());
    }

    println!("Time Taken {:?}", start.elapsed());
    println!("======================");
}

async fn process_song(song: &str) {
    let song = get_song_details(song).await.unwrap();
    println!("Data From Spotify");
    green_println!("Title: {:?}", song.title);
    green_println!("Artist: {:?}", song.description.artist);
    green_println!("Album: {:?}", song.description.album);
    green_println!("Year: {:?}", song.description.year);
    green_println!("Duartion: {:?}", song.duration);
    green_println!("======================");
}
