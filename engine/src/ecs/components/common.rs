use crate::math::vec2::Vec2;
use sdl2::rect::Rect;
use uuid::Uuid;

pub struct InfoComponent {
    pub uuid: Uuid,
    pub name: String,
    pub tag: Option<String>,
}

impl InfoComponent {
    pub fn with_name(name: impl Into<String>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: name.into(),
            tag: None,
        }
    }
}

#[derive(Debug)]
pub struct TransformComponent {
    pub translate: Vec2<f32>,
    pub scale: Vec2<f32>,
    pub rotation: f64,
}

impl TransformComponent {
    pub fn rect(&self, width: f32, height: f32) -> Rect {
        Rect::new(
            self.translate.x.round() as i32,
            self.translate.y.round() as i32,
            (width * self.scale.x).round() as u32,
            (height * self.scale.y).round() as u32,
        )
    }
}

impl Default for TransformComponent {
    fn default() -> Self {
        Self {
            translate: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
        }
    }
}

impl TransformComponent {
    pub fn translate(x: i32, y: i32) -> Self {
        let mut t = TransformComponent::default();
        t.translate = Vec2::new(x as f32, y as f32);
        t
    }
}
