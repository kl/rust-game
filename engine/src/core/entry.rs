use crate::core::application::{self, AppConfig};
use crate::Result;
use sdl2::render::WindowCanvas;

pub fn engine_main(config: AppConfig, cb: &dyn Fn(&mut WindowCanvas)) -> Result<()> {
    let sdl = sdl2::init()?;
    application::run(sdl, config, cb)?;
    Ok(())
}
