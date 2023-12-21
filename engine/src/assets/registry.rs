use crate::assets::asset::{Asset, AssetId, AssetTypeId};
use crate::assets::font::FontAsset;
use crate::assets::texture::TextureAsset;
use crate::Result;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use std::any;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::path::Path;

pub struct AssetRegistry {
    assets: HashMap<AssetTypeId, Vec<Box<dyn Any>>>,
    pub texture_creator: &'static TextureCreator<WindowContext>,
    pub ttf_context: &'static Sdl2TtfContext,
}

impl AssetRegistry {
    pub fn new(
        texture_creator: &'static TextureCreator<WindowContext>,
        ttf_context: &'static Sdl2TtfContext,
    ) -> AssetRegistry {
        AssetRegistry {
            assets: HashMap::new(),
            texture_creator,
            ttf_context,
        }
    }
}

impl AssetRegistry {
    pub fn clear(&mut self) {
        self.assets.clear();
    }

    pub fn get<T: Asset + 'static>(&self, id: &AssetId) -> &T {
        let type_id = TypeId::of::<T>();
        self.assets
            .get(&type_id)
            .and_then(|vec| {
                for asset in vec {
                    match asset.downcast_ref::<T>() {
                        Some(downcast) => {
                            if downcast.id() == id {
                                return Some(downcast);
                            }
                        }
                        None => panic!("wrong asset type for T: {}", any::type_name::<T>()),
                    }
                }
                None
            })
            .unwrap_or_else(|| panic!("did not find asset id {id} in registry"))
    }

    pub fn add<T: Asset + 'static>(&mut self, asset: T) -> AssetId {
        let type_id = TypeId::of::<T>();
        let asset_id = *asset.id();
        self.assets
            .entry(type_id)
            .or_default()
            .push(Box::new(asset));
        asset_id
    }

    pub fn load_texture(
        &mut self,
        source: impl AsRef<Path>,
        name: impl Into<String>,
    ) -> Result<AssetId> {
        let texture = self.texture_creator.load_texture(source)?;
        let query = texture.query();
        let asset = TextureAsset::new(name, texture, query.width, query.height);
        Ok(self.add(asset))
    }

    pub fn load_font(
        &mut self,
        source: impl AsRef<Path>,
        name: impl Into<String>,
        size: u16,
    ) -> Result<AssetId> {
        let font = self.ttf_context.load_font(source, size)?;
        let asset = FontAsset::new(font, name.into(), size);
        Ok(self.add(asset))
    }
}
