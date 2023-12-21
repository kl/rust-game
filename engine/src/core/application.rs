use crate::ecs::scene::Scene;
use crate::event::event;
use crate::event::event::EventHandlerAction;
use crate::Result;
use sdl2::render::WindowCanvas;
use sdl2::{Sdl, TimerSubsystem};



const MAX_DELTA_TIME: f32 = 0.05;

pub struct AppConfig {
    pub title: String,
    pub version: String,
    pub width: u32,
    pub height: u32,
}

pub fn run(
    sdl: Sdl,
    config: AppConfig,
    app_callback: &dyn Fn(&mut WindowCanvas),
) -> Result<()> {
    let video_subsystem = sdl.video()?;

    let window = video_subsystem
        .window(&config.title, config.width, config.height)
        .position_centered()
        .allow_highdpi()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl.event_pump()?;

    let mut event_handlers = event::make_event_handlers();

    let ttf = Box::leak(Box::new(sdl2::ttf::init()?));
    let texture = Box::leak(Box::new(canvas.texture_creator()));

    let timer = sdl.timer()?;

    let mut scene = Scene::new(canvas, texture, ttf, timer);
    scene.start()?;

    let mut last_tick = scene.timer.ticks64();

    'game_loop: loop {
        let dt = compute_delta_time(&mut last_tick, &scene.timer);
        dbg!(dt);

        for event in event_pump.poll_iter() {
            for handler in &mut event_handlers {
                if EventHandlerAction::Quit == handler(&event) {
                    break 'game_loop;
                }
            }
        }

        scene.canvas.clear();
        scene.update(dt)?;
        app_callback(&mut scene.canvas);
        scene.canvas.present();

        // thread::sleep(Duration::from_millis(1000));
        //thread::sleep(Duration::from_millis(500))

        // The rest of the game loop goes here...
    }
    Ok(())
}

fn compute_delta_time(last_tick: &mut u64, timer: &TimerSubsystem) -> f32 {
    let tick = timer.ticks64();
    let dt = (tick - *last_tick) as f32 / 1_000.0;
    *last_tick = tick;
    dt.min(MAX_DELTA_TIME)
}
