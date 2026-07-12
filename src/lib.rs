mod scrapper;
mod playlist;
mod downloader;
mod lyrics;

pub use scrapper::get_track_metadata;
pub use playlist::playlist_metadata;
pub use downloader::search_candidates;   
pub use downloader::score_candidate;   
pub use lyrics::get_lyrics;