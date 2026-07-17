use std::{i32, path::PathBuf};

use id3::{Tag, TagLike, Version, frame::Picture};

use crate::spotify::track::Track;

pub async fn write_metadata(track: &Track, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut tag = Tag::new();
    tag.set_title(&track.title);
    tag.set_album(&track.description.album);
    tag.set_artist(&track.description.artist);
    
    if let Ok(year) = track.description.year.parse::<i32>() {
        tag.set_year(year);
    }
    
    if let Ok(response) = reqwest::get(&track.image).await {
           if let Ok(bytes) = response.bytes().await {
               tag.add_frame(Picture {
                   mime_type: "image/jpeg".to_string(),
                   picture_type: id3::frame::PictureType::CoverFront,
                   description: "Cover".to_string(),
                   data: bytes.to_vec(),
               });
           }
       }

    tag.write_to_path(path, Version::Id3v24)?;
    Ok(())
}
