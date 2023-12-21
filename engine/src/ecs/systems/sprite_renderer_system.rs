use crate::assets::texture::TextureAsset;
use crate::core::extensions::WindowCanvasExt;
use crate::ecs::components::common::TransformComponent;
use crate::ecs::components::graphics::SpriteComponent;
use crate::ecs::systems::system::{Context, System};
use crate::Result;
use sdl2::rect::Rect;

pub struct SpriteRendererSystem {}

impl Default for SpriteRendererSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SpriteRendererSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for SpriteRendererSystem {
    fn update(&mut self, ctx: Context, _dt: f32) -> Result<()> {
        let Context {
            reg,
            assets: asset_reg,
            canvas,
            ..
        } = ctx;

        for entity in reg.entities::<SpriteComponent>() {
            let trans_comp = reg.component_for_entity::<TransformComponent>(entity);
            let sprite_comp = reg.component_for_entity::<SpriteComponent>(entity);

            let sprite = asset_reg.get::<TextureAsset>(&sprite_comp.sprite);
            let dest = Rect::new(
                trans_comp.translate.x.round() as i32,
                trans_comp.translate.y.round() as i32,
                (sprite.width as f32 * trans_comp.scale.x).round() as u32,
                (sprite.height as f32 * trans_comp.scale.y).round() as u32,
            );

            canvas.copy_full(&sprite.data, dest, trans_comp.rotation, sprite_comp.flip)?;
        }
        Ok(())
    }

    fn start(&mut self, _ctx: Context) -> Result<()> {
        Ok(())
    }
}
