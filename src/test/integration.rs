use std::time::{Instant};

use crate::spotify::{playlist::get_playlist_metadata, track::get_song_details};



pub async fn wire(url: &str) {
    let start =  Instant::now();

    if url.contains("/track/") {
    let song = get_song_details(url).await.unwrap();
    println!("Title: {:?}", song.title);
    println!("Artist: {:?}", song.description.artist);
    println!("Album: {:?}", song.description.album);
    println!("Year: {:?}", song.description.year);
    println!("Duartion: {:?}", song.duration);
    } else {
        let song = get_playlist_metadata(url).await.unwrap();
    }

    println!("======================");
    println!("Time Taken {:?}", start.elapsed());
    println!("======================");
}