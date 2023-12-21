use crate::math::vec2::Vec2f;

pub struct RigidBody2 {
    pub gravity_scale: f32,
    pub velocity: Vec2f,
    pub force: Vec2f,
}

impl Default for RigidBody2 {
    fn default() -> Self {
        Self {
            gravity_scale: 1.0,
            velocity: Vec2f::default(),
            force: Vec2f::default(),
        }
    }
}
