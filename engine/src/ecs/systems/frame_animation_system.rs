use crate::assets::animation::AnimationAsset;
use crate::assets::texture::TextureAsset;
use crate::core::extensions::WindowCanvasExt;
use crate::ecs::components::common::TransformComponent;
use crate::ecs::components::graphics::{AnimationComponent};
use crate::ecs::systems::system::{Context, System};

pub struct FrameAnimationSystem {}

impl Default for FrameAnimationSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameAnimationSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for FrameAnimationSystem {
    fn update(&mut self, ctx: Context, _dt: f32) -> crate::Result<()> {
        let Context {
            reg,
            assets: asset_reg,
            canvas,
            timer,
        } = ctx;

        for entity in reg.entities::<AnimationComponent>() {
            let trans_comp = reg.component_for_entity::<TransformComponent>(entity);
            let an_comp = reg.component_for_entity::<AnimationComponent>(entity);

            let anim = asset_reg.get::<AnimationAsset>(&an_comp.animation);

            let index = (timer.ticks64() / anim.speed as u64) % anim.frames.len() as u64;
            let frame = asset_reg.get::<TextureAsset>(&anim.frames[index as usize]);

            canvas.copy_full(
                &frame.data,
                trans_comp.rect(frame.width as f32, frame.height as f32),
                trans_comp.rotation,
                an_comp.flip,
            )?;
        }
        Ok(())
    }

    fn start(&mut self, _ctx: Context) -> crate::Result<()> {
        Ok(())
    }
}
