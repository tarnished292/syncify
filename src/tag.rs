use crate::scrapper::Song;
use id3::frame::Picture;
use id3::{Tag, TagLike, Version};
use std::path::Path;

pub async fn write_metadata(
    audio_path: &Path,
    song: &Song,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tag = Tag::new();

    tag.set_title(&song.title);
    tag.set_artist(&song.description.artist);
    tag.set_album(&song.description.album);

    if let Ok(year) = song.description.year.parse::<i32>() {
        tag.set_year(year);
    }

    if let Ok(response) = reqwest::get(&song.image).await {
        if let Ok(bytes) = response.bytes().await {
            tag.add_frame(Picture {
                mime_type: "image/jpeg".to_string(),
                picture_type: id3::frame::PictureType::CoverFront,
                description: "Cover".to_string(),
                data: bytes.to_vec(),
            });
        }
    }

    tag.write_to_path(audio_path, Version::Id3v24)?;
    Ok(())
}
