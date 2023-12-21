use crate::math::vec2::Vec2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[derive(PartialEq)]
pub enum EventHandlerAction {
    None,
    Quit,
}

pub type EventHandler = Box<dyn FnMut(&Event) -> EventHandlerAction>;

pub fn make_event_handlers() -> Vec<EventHandler> {
    let mut handlers = Vec::<EventHandler>::new();

    handlers.push(Box::new(|event| match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => EventHandlerAction::Quit,
        _ => EventHandlerAction::None,
    }));

    handlers.push(Box::new(|event| match event {
        Event::MouseButtonDown { x, y, .. } => {
            let pos = Vec2::new(x, y);
            println!("{pos}");
            EventHandlerAction::None
        }
        _ => EventHandlerAction::None,
    }));

    handlers
}
