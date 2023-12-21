use std::any::TypeId;
use uuid::Uuid;

pub type AssetId = Uuid;
pub type AssetTypeId = TypeId;

pub enum AssetType {}

// pub struct Asset {
//     pub uuid: Uuid,
//     name: String,
// }

pub trait Asset {
    fn id(&self) -> &AssetId;
}
