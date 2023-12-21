use crate::assets::texture::TextureAsset;
use crate::assets::tilemap::TilemapAsset;
use crate::ecs::components::common::TransformComponent;
use crate::ecs::components::graphics::{TileComponent, TilemapComponent};
use crate::ecs::systems::system::{Context, System};
use sdl2::rect::Rect;
use sdl2::sys::SDL_RendererFlip;

pub struct TilemapRendererSystem {}

impl Default for TilemapRendererSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TilemapRendererSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for TilemapRendererSystem {
    fn update(&mut self, ctx: Context, _dt: f32) -> crate::Result<()> {
        let Context {
            reg,
            assets,
            canvas,
            ..
        } = ctx;

        let tile_entities = reg.entities::<TileComponent>();

        for tilemap_entity in reg.entities::<TilemapComponent>() {
            let trans_comp = reg.component_for_entity::<TransformComponent>(tilemap_entity);
            let tilemap_comp = reg.component_for_entity::<TilemapComponent>(tilemap_entity);

            let tilemap_asset = assets.get::<TilemapAsset>(&tilemap_comp.tilemap);

            for tile_entity in &tile_entities {
                let tile_comp = reg.component_for_entity::<TileComponent>(*tile_entity);
                if tile_comp.tilemap != tilemap_comp.tilemap
                    && !tilemap_asset.tilesets.contains(&tile_comp.tileset)
                {
                    continue;
                }

                let x = tile_comp.offset_x as i32 + trans_comp.translate.x as i32;
                let y = tile_comp.offset_y as i32 + trans_comp.translate.y as i32;

                let size = tilemap_asset.tile_size;

                let dest = Rect::new(x * size as i32, y * size as i32, size, size);

                let source = Rect::new(
                    tile_comp.row as i32 * size as i32,
                    tile_comp.col as i32 * size as i32,
                    size,
                    size,
                );

                let tileset_asset = assets.get::<TextureAsset>(&tile_comp.tileset);

                canvas.copy_ex(
                    &tileset_asset.data,
                    source,
                    dest,
                    0.0,
                    None,
                    tile_comp.flip == SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
                    tile_comp.flip == SDL_RendererFlip::SDL_FLIP_VERTICAL,
                )?;
            }
        }
        Ok(())
    }

    fn start(&mut self, _ctx: Context) -> crate::Result<()> {
        Ok(())
    }
}
