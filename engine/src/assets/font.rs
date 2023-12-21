use crate::assets::asset::{Asset, AssetId};
use sdl2::ttf::Font;
use uuid::Uuid;

pub struct FontAsset {
    pub id: Uuid,
    pub data: Font<'static, 'static>,
    pub name: String,
    pub size: u16,
}

impl FontAsset {
    pub fn new(data: Font<'static, 'static>, name: String, size: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            data,
            name,
            size,
        }
    }
}

impl Asset for FontAsset {
    fn id(&self) -> &AssetId {
        &self.id
    }
}
