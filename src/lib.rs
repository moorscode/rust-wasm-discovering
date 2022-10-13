mod game;
mod shapes;
mod ray;
mod particle;
mod particle_animation;
mod particle_system;
mod draw;
mod browser;
mod logic;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, KeyboardEvent};

use game::Game;
use logic::*;
use crate::browser::Browser;
use crate::draw::Draw;
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
    let game = Game::create("#canvas");

    // Mouse tracker.
    {
        let closure = Closure::<dyn FnMut(_)>::new(
            enclose!( (game ) move |event:MouseEvent| {
                handle_mouse_move( &game, event );
            } ),
        );
        game.canvas().add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Keyboard listener.
    {
        let closure = Closure::<dyn FnMut(_)>::new(
            enclose!( (game) move |event:KeyboardEvent| {
                handle_keypress( &game, event.key_code() );
            }),
        );
        Browser::window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    game.run(game_logic);

    Ok(())
}
