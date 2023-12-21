use engine::core::application::AppConfig;
use engine::core::entry::engine_main;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::error::Error;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let config = AppConfig {
        title: "Fuse2d".to_string(),
        version: "0.0.1".to_string(),
        width: 1080,
        height: 720,
    };
    if let Err(e) = engine_main(config, &|canvas| {
        return;
        let white = Color::RGB(255, 255, 255);
        let black = Color::RGB(0, 0, 0);
        let green = Color::RGB(0, 255, 0);

        let xoriginoffset = (canvas.window().size().0 / 2) as i32;
        let yoriginoffset = (canvas.window().size().1 / 2) as i32;

        canvas.set_draw_color(green);
        for x in -500i32..=500 {
            canvas
                .draw_point((x + xoriginoffset, yoriginoffset))
                .unwrap();
        }
        for y in -400i32..=400 {
            canvas
                .draw_point((xoriginoffset, y + yoriginoffset))
                .unwrap();
        }

        canvas.set_draw_color(white);

        for x in -500i32..=500 {
            // let y = (0.05 * x as f32) as i32 - 100;
            let y = (10.0 * x as f32) as i32 - 100;
            canvas
                .draw_point((x + xoriginoffset, y + yoriginoffset))
                .unwrap();
        }

        canvas.set_draw_color(black);
    }) {
        eprintln!("GAME ERROR: {}", e);
        process::exit(1)
    }
    Ok(())
}
