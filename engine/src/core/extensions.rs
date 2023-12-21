use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::sys::SDL_RendererFlip;

pub trait WindowCanvasExt {
    fn copy_full(
        &mut self,
        texture: &Texture,
        dst: impl Into<Option<Rect>>,
        angle: f64,
        flip: SDL_RendererFlip,
    ) -> Result<(), String>;
}

impl WindowCanvasExt for WindowCanvas {
    fn copy_full(
        &mut self,
        texture: &Texture,
        dst: impl Into<Option<Rect>>,
        angle: f64,
        flip: SDL_RendererFlip,
    ) -> Result<(), String> {
        self.copy_ex(
            texture,
            None,
            dst,
            angle,
            None,
            flip == SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
            flip == SDL_RendererFlip::SDL_FLIP_VERTICAL,
        )
    }
}
