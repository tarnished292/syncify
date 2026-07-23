use std::io::Error;
use std::{path::PathBuf};
use tokio::process::Command;
use crate::dlp::search::Candidate;
use crate::spotify::track::Track;

pub async fn download(best: &Candidate, track: &Track, output: &PathBuf) -> Result<PathBuf, Error> {
    let download_arg = format!("https://www.youtube.com/watch?v={}", best.video_id);
    let output_template = output.join(format!("{}.%(ext)s", sanitize_filename(&track.title)));
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
            // "--cookies", 
            // "cookies.txt",
            "--print",
            "after_move:filepath",
            "-o",
        ])
        .arg(&output_template)
        .arg(&download_arg)
        .output()
        .await?;

    let path_str = String::from_utf8_lossy(&result.stdout).trim().to_string();
    Ok(PathBuf::from(path_str))
}


fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if r#"/\:*?"<>|"#.contains(c) { '_' } else { c })
        .collect()
}