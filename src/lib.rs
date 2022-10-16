mod game;
mod shapes;
mod ray;
mod particle;
mod particle_animation;
mod particle_system;
mod draw;
mod browser;
mod game_engine;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, KeyboardEvent};

use game_engine::GameEngine;
use crate::browser::Browser;
use crate::draw::Draw;
use crate::game::{handle_keypress, tick};
use crate::shapes::Point2d;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let game_engine = GameEngine::create("#canvas");

    // Mouse tracker.
    {
        let closure = Closure::<dyn FnMut(_)>::new(
            enclose!( (game_engine ) move |event:MouseEvent| {
                game_engine.set_mouse(event);
            } ),
        );
        game_engine.canvas().add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Keyboard listener.
    {
        let closure = Closure::<dyn FnMut(_)>::new(
            enclose!( (game_engine) move |event:KeyboardEvent| {
                handle_keypress( &game_engine, event.key_code() );
            }),
        );
        Browser::window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }


    game_engine.run(tick);

    Ok(())
}
