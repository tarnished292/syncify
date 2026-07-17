use crate::{dlp::search::Candidate, spotify::track::Track};


pub fn score(results: &Vec<Candidate>, track: &Track) -> Candidate {
    let mut best_score = i32::MIN;
    let mut best_candidate = None;

    for candidate in results {
        let title = candidate.title.to_lowercase();
        let uploader = candidate.uploader.to_lowercase();
         let primary_artist = track.description.artist.split(",").next().unwrap_or("");

        let yt_duration: u32 = candidate.duration.parse().unwrap_or(0);
        let spotify_duration: u32 = track.duration.parse().unwrap_or(0);

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
            score -= 4
        }
        if title.contains("live") {
            score -= 2
        }
        if uploader.contains(&primary_artist.to_lowercase()) {
            score += 2
        }
        if title.contains(&track.title.to_lowercase()) {
            score += 4
        }
        if diff <= 3 {
            score += 5;
        } else if diff <= 10 {
            score += 2;
        } else if diff > 20 {
            score -= 5;
        }

        if score > best_score {
            best_score = score;
            best_candidate = Some(candidate.clone());
        }
    }

    let best_candidate = best_candidate.expect("No Best candidate Found");

    best_candidate
}
