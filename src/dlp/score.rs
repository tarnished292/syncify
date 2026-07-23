use crate::{
    dlp::search::Candidate,
    spotify::{track::Track},
};

fn core_title(t: &str) -> String {
    t.split(['(', '[', '-'])
        .next()
        .unwrap_or(t)
        .trim()
        .to_lowercase()
}

pub fn score(results: &Vec<Candidate>, track: &Track) -> Option<Candidate> {
    let mut best_score = i32::MIN;
    let mut best_candidate = None;

    let primary_artist = track
        .description
        .artist
        .split(",")
        .next()
        .unwrap_or("")
        .trim()
        .to_lowercase();
    let spotify_title = track.title.to_lowercase();
    let clean_spotify_title = core_title(&spotify_title);
    let spotify_duration: u32 = match track.duration.parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Warning Failed to parse Spotify Duration {}", track.duration);
            return None;
        }
    };

    for candidate in results {
        let yt_title = candidate.title.to_lowercase();
        let title = core_title(&yt_title);
        let uploader = candidate.uploader.to_lowercase();
        let yt_duration: u32 = match candidate.duration.parse() {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Warning Failed to parse YT Duration {}", candidate.duration);
                continue;
            }
        };
        let diff = yt_duration.abs_diff(spotify_duration);

        let mut score = 0;

        if title.contains("1 hour") {
            score -= 5;
        }
        if title.contains("slowed") {
            score -= 5;
        }
        if title.contains("8d audio") {
            score -= 5;
        }
        if title.contains("sped up") {
            score -= 10;
        }
        if title.contains("live") {
            score -= 3;
        }

        if title.contains(&clean_spotify_title) {
            score += 5;
        }

        if title.contains(&track.description.artist.to_lowercase()) {
            score += 3;
        }

        if uploader.contains(&primary_artist) {
            score += 2;
        } else {
            score -= 1;
        }

        if uploader.contains(&track.description.artist) {
            score += 1;
        }

        if diff <= 2 {
            score -= 2;
        } else if diff <= 5 {
            score -= 4;
        } else if diff <= 10 {
            score -= 5;
        } else if diff <= 20 {
            score -= 8;
        } else {
            continue;
        }

        if score > best_score {
            best_score = score;
            best_candidate = Some(candidate.clone());
        }
    }

    best_candidate
}
