use crate::assets::asset::{Asset, AssetId};
use uuid::Uuid;

#[derive(Debug)]
pub struct AnimationAsset {
    pub id: AssetId,
    pub frames: Vec<AssetId>,
    pub speed: u32,
}

impl Asset for AnimationAsset {
    fn id(&self) -> &AssetId {
        &self.id
    }
}

impl AnimationAsset {
    pub fn new(frames: impl Into<Vec<AssetId>>, speed: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            frames: frames.into(),
            speed,
        }
    }
}
