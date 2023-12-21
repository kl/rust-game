use crate::assets::animation::AnimationAsset;
use crate::assets::registry::AssetRegistry;
use crate::assets::tilemap::TilemapAsset;
use crate::ecs::components::common::{InfoComponent, TransformComponent};
use crate::ecs::components::graphics::{
    AnimationComponent, SpriteComponent, TextComponent, TileComponent, TilemapComponent,
};
use crate::ecs::components::physics::RigidBodyComponent;
use crate::ecs::ecs::EntityId;
use crate::ecs::registry::Registry;
use crate::ecs::systems::rigidbody_system::RigidBodySystem;
use crate::ecs::systems::sprite_renderer_system::SpriteRendererSystem;
use crate::ecs::systems::system::{Context, System};
use sdl2::pixels::Color;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use sdl2::TimerSubsystem;

pub struct Scene {
    pub registry: Registry,
    pub asset_registry: AssetRegistry,
    pub systems: Vec<Box<dyn System>>,
    pub canvas: WindowCanvas,
    pub timer: TimerSubsystem,
}

impl Scene {
    pub fn new(
        canvas: WindowCanvas,
        texture_creator: &'static TextureCreator<WindowContext>,
        ttf_context: &'static Sdl2TtfContext,
        timer: TimerSubsystem,
    ) -> Self {
        Self {
            registry: Registry::new(),
            asset_registry: AssetRegistry::new(texture_creator, ttf_context),
            systems: vec![
                Box::new(SpriteRendererSystem::new()),
                // Box::new(TextRendererSystem::new()),
                // Box::new(FrameAnimationSystem::new()),
                // Box::new(TilemapRendererSystem::new()),
                Box::new(RigidBodySystem::new()),
            ],
            canvas,
            timer,
        }
    }

    fn add_entity(&mut self, name: impl Into<String>) -> EntityId {
        let entity = self.registry.add_entity();
        self.registry
            .add_component(entity, InfoComponent::with_name(name));
        entity
    }

    pub fn start(&mut self) -> crate::Result<()> {
        let sp = self.asset_registry.load_texture("assets/obj1.png", "")?;
        let entity = self.add_entity("rigidbody");
        self.registry
            .add_component(entity, SpriteComponent::new(sp));
        self.registry
            .add_component(entity, RigidBodyComponent::new());
        self.registry
            .add_component(entity, TransformComponent::default());

        let tileset = self
            .asset_registry
            .load_texture("assets/tile.png", "tileset")?;

        let tma = self
            .asset_registry
            .add(TilemapAsset::new([tileset], 32, 32, 12));

        let tilemap = self.add_entity("tilemap");
        self.registry
            .add_component(tilemap, TilemapComponent::new(tma));

        self.registry
            .add_component(tilemap, TransformComponent::default());

        for row in 0..32 {
            for col in 0..32 {
                let e = self.add_entity("tile");
                let mut tc = TileComponent::new(tilemap, tileset);
                tc.offset_x = col;
                tc.offset_y = row;
                tc.row = col;
                tc.col = row;
                self.registry.add_component(e, tc)
            }
        }

        let anim_entity = self.add_entity("anim_entity");
        let a1 = self.asset_registry.load_texture("assets/a1.jpg", "a1")?;
        let a2 = self.asset_registry.load_texture("assets/a2.png", "a2")?;
        let a3 = self.asset_registry.load_texture("assets/a3.png", "a3")?;

        let animation = self
            .asset_registry
            .add(AnimationAsset::new(vec![a1, a2, a3], 300));

        self.registry
            .add_component(anim_entity, TransformComponent::translate(0, 300));
        self.registry
            .add_component(anim_entity, AnimationComponent::new(animation));

        let entity = self.add_entity("test_entity");
        let sprite = self
            .asset_registry
            .load_texture("assets/test.png", "test")?;
        self.registry
            .add_component(entity, SpriteComponent::new(sprite));
        self.registry
            .add_component(entity, TransformComponent::default());

        let font_entity = self.add_entity("font entity");
        let font = self.asset_registry.load_font("assets/font.ttf", "ft", 30)?;
        self.registry
            .add_component(font_entity, TextComponent::new(font, "Hell world!"));
        self.registry
            .add_component(font_entity, TransformComponent::translate(0, 200));

        for system in &mut self.systems {
            let c = Context {
                reg: &mut self.registry,
                assets: &mut self.asset_registry,
                canvas: &mut self.canvas,
                timer: &self.timer,
            };
            system.start(c)?;
        }
        Ok(())
    }

    pub fn update(&mut self, dt: f32) -> crate::Result<()> {
        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        Context {
            reg: &mut self.registry,
            assets: &mut self.asset_registry,
            canvas: &mut self.canvas,
            timer: &self.timer,
        };
        for system in &mut self.systems {
            let c = Context {
                reg: &mut self.registry,
                assets: &mut self.asset_registry,
                canvas: &mut self.canvas,
                timer: &self.timer,
            };
            system.update(c, dt)?;
        }
        Ok(())
    }
}
