use std::io::Error;
use std::{path::PathBuf, process::Command};

use crate::dlp::search::Candidate;

pub async fn download(best: &Candidate, output: PathBuf) -> Result<PathBuf, Error> {
    let download_arg = format!("https://www.youtube.com/watch?v={}", best.video_id);
    let output_template = output.join("%(title)s.%(ext)s");
    let result = Command::new("yt-dlp")
        .args([
            "-f",
            "bestaudio",
            "--no-playlist",
            "--concurrent-fragments",
            "4",
            "--extract-audio",
            "--audio-format",
            "mp3",
            "--audio-quality",
            "0",
            "--print",
            "after_move:filepath",
            "-o",
        ])
        .arg(&output_template)
        .arg(&download_arg)
        .output()?;

    let path_str = String::from_utf8_lossy(&result.stdout).trim().to_string();
    println!("{path_str}");
    Ok(PathBuf::from(path_str))
}
