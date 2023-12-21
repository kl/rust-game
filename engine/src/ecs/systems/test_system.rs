use crate::ecs::components::common::TransformComponent;
use crate::ecs::systems::system::{Context, System};
use crate::Result;

pub struct TestSystem {}

impl Default for TestSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TestSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl System for TestSystem {
    fn update(&mut self, ctx: Context, _dt: f32) -> Result<()> {
        let Context { reg, .. } = ctx;

        for entity in reg.entities::<TransformComponent>() {
            let component = reg.component_for_entity_mut::<TransformComponent>(entity);
            println!("id[{}], x: {}", entity, component.translate.x);
            component.translate.x += 1.0;
        }
        Ok(())
    }

    fn start(&mut self, _ctx: Context) -> Result<()> {
        Ok(())
    }
}
