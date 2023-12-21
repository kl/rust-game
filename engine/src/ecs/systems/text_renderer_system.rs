use crate::assets::font::FontAsset;
use crate::core::extensions::WindowCanvasExt;
use crate::ecs::components::common::TransformComponent;
use crate::ecs::components::graphics::TextComponent;
use crate::ecs::systems::system::{Context, System};
use crate::Result;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

pub struct TextRendererSystem {}

impl Default for TextRendererSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TextRendererSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for TextRendererSystem {
    fn update(&mut self, ctx: Context, _dt: f32) -> Result<()> {
        let Context {
            reg,
            assets: asset_reg,
            canvas,
            ..
        } = ctx;

        for entity in reg.entities::<TextComponent>() {
            let transform_comp = reg.component_for_entity::<TransformComponent>(entity);
            let text_comp = reg.component_for_entity::<TextComponent>(entity);

            let font = asset_reg.get::<FontAsset>(&text_comp.font);

            let surface = font.data.render(&text_comp.text).blended(text_comp.color)?;
            let texture = asset_reg
                .texture_creator
                .create_texture_from_surface(surface)?;

            let TextureQuery { width, height, .. } = texture.query();

            let dest = Rect::new(
                transform_comp.translate.x.round() as i32,
                transform_comp.translate.y.round() as i32,
                (width as f32 * transform_comp.scale.x).round() as u32,
                (height as f32 * transform_comp.scale.y).round() as u32,
            );

            canvas.copy_full(&texture, dest, transform_comp.rotation, text_comp.flip)?;
        }
        Ok(())
    }

    fn start(&mut self, _ctx: Context) -> Result<()> {
        Ok(())
    }
}
