use crate::ecs::systems::system::{Context, System};

struct CollisionSystem {}

impl CollisionSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for CollisionSystem {
    fn update(&mut self, _ctx: Context, _dt: f32) -> crate::Result<()> {
        todo!()
    }

    fn start(&mut self, ctx: Context) -> crate::Result<()> {
        let Context {  .. } = ctx;

        Ok(())
    }
}
