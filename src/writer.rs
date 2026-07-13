use std::{fs, io, path::Path};

pub fn write_lrc(audio_path: &Path, lyrics: &str) -> io::Result<()>{
    let path = audio_path.with_extension("lrc");
    let _ = fs::write(&path, lyrics);
    println!("Song Path: {:?}", path);
    Ok(())
}