use crate::assets::asset::{Asset, AssetId};
use std::collections::HashSet;
use uuid::Uuid;

pub struct TilemapAsset {
    pub id: AssetId,
    pub tilesets: HashSet<AssetId>,
    pub col_count: u32,
    pub row_count: u32,
    pub tile_size: u32,
}

impl Asset for TilemapAsset {
    fn id(&self) -> &AssetId {
        &self.id
    }
}

impl TilemapAsset {
    pub fn new(
        tilesets: impl Into<HashSet<AssetId>>,
        col_count: u32,
        row_count: u32,
        tile_size: u32,
    ) -> Self {
        TilemapAsset {
            id: Uuid::new_v4(),
            tilesets: tilesets.into(),
            col_count,
            row_count,
            tile_size,
        }
    }
}
