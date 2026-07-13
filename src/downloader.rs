use std::{io::Error, path::PathBuf, process::Command, str};

use crate::scrapper::Song;

#[derive(Debug, Clone)]
pub struct YtDlpResult {
    pub id: String,
    pub title: String,
    pub duration: u32,
    pub uploader: String,
}

pub async fn best_candidates(song: &Song) -> Option<YtDlpResult> {
    let query = format!("ytsearch2: {} {}", song.title, song.description.artist);
    let output_url = Command::new("yt-dlp")
        .arg("--flat-playlist")
        .arg("--skip-download")
        .arg("--print")
        .arg("%(id)s|%(title)s|%(duration)s|%(uploader)s")
        .arg(&query)
        .output()
        .ok()?;

    let mut results = Vec::new();
    let output = String::from_utf8_lossy(&output_url.stdout);
    for line in output.lines() {
        let parts: Vec<&str> = line.split("|").collect();
        if parts.len() < 4 {
            continue;
        }
        let id = parts[0].to_string();
        let title = parts[1].to_string();
        let duration = parts[2].parse::<u32>().unwrap_or(0);
        let uploader = parts[3].to_string();

        results.push(YtDlpResult {
            id,
            title,
            duration,
            uploader,
        })
    }
    let mut best_score = i32::MIN;
    let mut best: Option<YtDlpResult> = None;

    for candidates in results {
        let mut score = 0;
        if candidates
            .title
            .to_lowercase()
            .contains(&song.title.to_lowercase())
        {
            score += 4;
        }

        if candidates.title.to_lowercase().contains("remix") {
            score -= 2;
        }

        if candidates.title.to_lowercase().contains("live") {
            score -= 2;
        }

        if candidates.title.to_lowercase().contains("slowed") {
            score -= 2;
        }
        if candidates.title.to_lowercase().contains("1 hour") {
            score -= 5;
        }
        if candidates.title.to_lowercase().contains("sped up") {
            score -= 5;
        }

        if candidates
            .uploader
            .to_lowercase()
            .contains(&song.description.artist.to_lowercase())
        {
            score += 2;
        }
        let spotify_duration: u32 = song.duration.parse().unwrap();
        let diff = candidates.duration.abs_diff(spotify_duration);

        if diff <= 2 {
            score += 4;
        } else if diff <= 5 {
            score += 3;
        } else if diff <= 10 {
            score += 1;
        }

        if score > best_score {
            best_score = score;
            best = Some(candidates);
        }
    }
    best
}

pub fn download_song(
    candidate: &YtDlpResult,
    song: &Song,
    output_dir: &str,
) -> Result<PathBuf, Error> {
    let url = format!("https://youtube.com/watch?v={}", candidate.id);
    let output_template = format!("{}/{}.%(ext)s", output_dir, song.title);

    let output = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestaudio")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--postprocessor-args")
        .arg("ffmpeg:-preset ultrafast")
        .arg("-o")
        .arg(output_template)
        .arg("--print")
        .arg("after_move:filepath")
        .arg(&url)
        .output()?;
    eprintln!(
        "--- stderr ---\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if path_str.is_empty() {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "yt-dlp did not return a file path",
        ));
    }

    Ok(PathBuf::from(path_str))
}
