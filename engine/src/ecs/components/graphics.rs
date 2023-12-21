use crate::assets::asset::AssetId;
use sdl2::pixels::Color;
use sdl2::sys::SDL_RendererFlip::{self, *};

#[derive(Debug)]
pub struct SpriteComponent {
    pub sprite: AssetId,
    pub flip: SDL_RendererFlip,
}

impl SpriteComponent {
    pub fn new(sprite: AssetId) -> Self {
        Self {
            sprite,
            flip: SDL_FLIP_NONE,
        }
    }
}

#[derive(Debug)]
pub struct TextComponent {
    pub font: AssetId,
    pub flip: SDL_RendererFlip,
    pub color: Color,
    pub text: String,
}

impl TextComponent {
    pub fn new(font: AssetId, text: impl Into<String>) -> Self {
        Self {
            font,
            flip: SDL_FLIP_NONE,
            color: Color::RGB(0, 0, 0),
            text: text.into(),
        }
    }
}

#[derive(Debug)]
pub struct AnimationComponent {
    pub animation: AssetId,
    pub flip: SDL_RendererFlip,
}

impl AnimationComponent {
    pub fn new(animation: AssetId) -> Self {
        Self {
            animation,
            flip: SDL_FLIP_NONE,
        }
    }
}

#[derive(Debug)]
pub struct TilemapComponent {
    pub tilemap: AssetId,
}

impl TilemapComponent {
    pub fn new(tilemap: AssetId) -> Self {
        Self { tilemap }
    }
}

#[derive(Debug)]
pub struct TileComponent {
    pub tilemap: AssetId,
    pub tileset: AssetId,
    pub offset_x: u32,
    pub offset_y: u32,
    pub row: u32,
    pub col: u32,
    pub flip: SDL_RendererFlip,
}

impl TileComponent {
    pub fn new(tilemap: AssetId, tileset: AssetId) -> Self {
        Self {
            tilemap,
            tileset,
            offset_x: 0,
            offset_y: 0,
            row: 0,
            col: 0,
            flip: SDL_FLIP_NONE,
        }
    }
}
