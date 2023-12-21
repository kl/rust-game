use crate::assets::registry::AssetRegistry;
use crate::ecs::registry::Registry;
use crate::Result;
use sdl2::render::WindowCanvas;
use sdl2::TimerSubsystem;
use std::any;

pub struct Context<'a> {
    pub reg: &'a mut Registry,
    pub assets: &'a mut AssetRegistry,
    pub canvas: &'a mut WindowCanvas,
    pub timer: &'a TimerSubsystem,
}

pub trait System {
    fn update(&mut self, ctx: Context, dt: f32) -> Result<()>;
    fn start(&mut self, ctx: Context) -> Result<()>;
    fn name(&self) -> &'static str {
        any::type_name::<Self>()
    }
}
