use std::{fs, io, path::PathBuf};

pub fn write_lrc(audio_path: &PathBuf, lyrics: &str) -> io::Result<()>{
    let path = audio_path.with_extension("lrc");
    let _ = fs::write(&path, lyrics);
    Ok(())
}