use crate::physics::rigidbody2::RigidBody2;

use sdl2::sys::SDL_FRect;

pub struct RigidBodyComponent {
    pub enabled: bool,
    pub body: RigidBody2,
}

impl Default for RigidBodyComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl RigidBodyComponent {
    pub fn new() -> Self {
        Self {
            body: RigidBody2::default(),
            enabled: true,
        }
    }
}

pub struct ColliderComponent {
    enabled: bool,
    collider: SDL_FRect,
}
