use std::{io::Error, process::Command, str};

use crate::scrapper::Song;

pub struct YtDlpResult {
    pub id: String,
    pub title: String,
    pub duration: u32,
    pub uploader: String,
}

pub async fn search_candidates(song: &Song) -> Result<Vec<YtDlpResult>, Error> {
    let query = format!("ytsearch5: {} {}", song.title, song.description.artist);
    let output_url = Command::new("yt-dlp")
        .arg("--skip-download")
        .arg("--print")
        .arg("%(id)s|%(title)s|%(duration)s|%(uploader)s")
        .arg(&query)
        .output()?;

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
    Ok(results)
}

pub fn score_candidate<'a>(song: &Song, results: &'a Vec<YtDlpResult>) -> Option<&'a YtDlpResult> {
    let mut best_score = i32::MIN;
    let mut best: Option<&YtDlpResult> = None;

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
