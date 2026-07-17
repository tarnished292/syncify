use crate::spotify::track::Track;
use std::process::Command;


#[derive(Clone)]
pub struct Candidate {
    pub video_id: String,
    pub title: String,
    pub uploader: String,
    pub duration: String,
}

pub fn search_candidate(song: &Track) -> Vec<Candidate> {
    let search_format = format!("ytsearch2: {}", song.title);
    println!("Search Query: {search_format}");

    let command = Command::new("yt-dlp")
        .arg("--skip-download")
        .arg("--print")
        .arg("%(id)s|%(title)s|%(duration)s|%(uploader)s")
        .arg(&search_format)
        .output()
        .expect("failed to execute yt-dlp");

    let yt_metadata = String::from_utf8_lossy(&command.stdout);

    let mut results = Vec::new();
    for output in yt_metadata.lines() {
        let parts: Vec<&str> = output.split("|").collect();
        let video_id = parts[0].to_string();
        let title = parts[1].to_string();
        let duration = parts[2].to_string();
        let uploader = parts[3].to_string();

        results.push(Candidate {
            video_id,
            title,
            duration,
            uploader,
        });
    }

    results
}
