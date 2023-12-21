use crate::assets::asset::{Asset, AssetId};
use sdl2::mixer::Chunk;
use std::path::PathBuf;
use uuid::Uuid;

pub struct AudioClipAsset {
    id: Uuid,
    data: Chunk,
    filename: PathBuf,
}

impl AudioClipAsset {
    fn new(path: impl Into<PathBuf>) -> Self {
        let path_buf = path.into();
        Self {
            id: Uuid::new_v4(),
            data: Chunk::from_file(&path_buf).unwrap(),
            filename: path_buf,
        }
    }
}

impl Asset for AudioClipAsset {
    fn id(&self) -> &AssetId {
        &self.id
    }
}
