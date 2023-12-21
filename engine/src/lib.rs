pub mod assets;
pub mod core;
pub mod ecs;
pub mod event;
pub mod math;
pub mod physics;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
