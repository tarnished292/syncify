use crate::{dlp::search::Candidate, spotify::track::Track};

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
    let core = core_title(&track.title);
    let spotify_duration: u32 = track.duration.parse().unwrap_or(0);

    for candidate in results {
        let title = candidate.title.to_lowercase();
        let uploader = candidate.uploader.to_lowercase();
        let yt_duration: u32 = candidate.duration.parse().unwrap_or(0);
        let diff = yt_duration.abs_diff(spotify_duration);

        let mut score = 0;

        if title.contains("1 hour") {
            score -= 2;
        }
        if title.contains("slowed") {
            score -= 2
        }
        if title.contains("8d audio") {
            score -= 2
        }
        if title.contains("sped up") {
            score -= 14
        }
        if title.contains("live") {
            score -= 2
        }
        if uploader.contains(&primary_artist) {
            score += 2
        } else {
            score -= 1
        }

       
        if !core.is_empty() && title.contains(&core) {
            score += 4
        }

        if diff <= 2 {
            score += 4;
        } else if diff <= 5 {
            score -= 1;
        } else if diff <= 10 {
            score -= 3;
        } else if diff <= 20 {
            score -= 8;
        } else {
            score -= 15;
        }

        if score > best_score {
            best_score = score;
            best_candidate = Some(candidate.clone());
        }
    }

    best_candidate
}
