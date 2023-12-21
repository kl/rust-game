use crate::ecs::components::common::TransformComponent;
use crate::ecs::components::physics::RigidBodyComponent;
use crate::ecs::systems::system::{Context, System};
use crate::math::vec2::{Vec2f};

pub struct RigidBodySystem {}

impl Default for RigidBodySystem {
    fn default() -> Self {
        Self::new()
    }
}

impl RigidBodySystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for RigidBodySystem {
    fn update(&mut self, ctx: Context, dt: f32) -> crate::Result<()> {
        let Context { reg, .. } = ctx;

        for e in reg.entities::<RigidBodyComponent>() {
            let rb = reg.component_for_entity_mut::<RigidBodyComponent>(e);
            if !rb.enabled {
                continue;
            }

            let g = Vec2f::new(0.0, rb.body.gravity_scale * 9.8);
            rb.body.velocity += (rb.body.force + g) * dt;
            let rbv = rb.body.velocity * dt;

            let trans = reg.component_for_entity_mut::<TransformComponent>(e);
            trans.translate += rbv;
        }

        Ok(())
    }

    fn start(&mut self, _ctx: Context) -> crate::Result<()> {
        Ok(())
    }
}
