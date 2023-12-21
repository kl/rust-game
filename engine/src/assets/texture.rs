use crate::assets::asset::{Asset, AssetId};
use sdl2::render::Texture;
use sdl2::sys::SDL_DestroyTexture;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use uuid::Uuid;

pub struct TextureAsset {
    pub id: AssetId,
    pub name: String,
    pub data: Texture<'static>,
    pub filename: PathBuf,
    pub height: u32,
    pub width: u32,
}

impl Debug for TextureAsset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TextureAsset")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("filename", &self.filename)
            .field("height", &self.height)
            .field("width", &self.width)
            .finish()
    }
}

impl TextureAsset {
    pub fn new(
        name: impl Into<String>,
        texture: Texture<'static>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            data: texture,
            filename: Default::default(),
            width,
            height,
        }
    }
}

impl Asset for TextureAsset {
    fn id(&self) -> &AssetId {
        &self.id
    }
}

impl Drop for TextureAsset {
    fn drop(&mut self) {
        // SAFETY: the TextureCreator that created this texture
        // lives as long as the application does so it is safe to
        // call destroy.
        unsafe {
            SDL_DestroyTexture(self.data.raw());
        }
    }
}
