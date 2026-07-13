mod scrapper;
mod playlist;
mod downloader;
mod lyrics;
mod tag;
mod wire;
mod writer;

pub use scrapper::get_track_metadata;
pub use playlist::playlist_metadata;
pub use downloader::best_candidates;    
pub use downloader::download_song;   
pub use lyrics::get_lyrics;
pub use writer::write_lrc;
pub use wire::download;
pub use tag::write_metadata;