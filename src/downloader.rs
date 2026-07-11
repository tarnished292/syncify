use std::{
    io::Error,
    process::{Command, Output},
    str,
};

use crate::scrapper::Song;

pub struct YtDlpResult {
    id: String,
    title: String,
    duration: u32,
    uploader: String,
}

pub async fn search_candidates(song: Song) -> Result<YtDlpResult, Error> {
    let query = format!("ytsearch5: {} {}", song.title, song.description.artist);
    let output_url = Command::new("yt-dlp")
        .arg("--skip-download")
        .arg("--print")
        .arg("%(id)s|%(title)s|%(duration)s|%(uploader)s")
        .arg(&query)
        .output()?;

    let output = String::from_utf8_lossy(&output_url.stdout);
    for line in output.lines() {
        println!("{}", line);
        let parts: Vec<&str> = line.split("|").collect();
    }
    todo!()
}
