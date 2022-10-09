mod game;
mod shapes;

use rgb::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, Window};

use crate::shapes::{Circle, Line, Point2d, Shapes};
use game::Game;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

impl From<MouseEvent> for Point2d {
    fn from(e: MouseEvent) -> Self {
        Self {
            x: e.offset_x() as f64,
            y: e.offset_y() as f64,
        }
    }
}

// const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
// const WHITE: RGB8 = RGB8 { r: 255, g: 255, b: 255 };
const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };

fn draw(game: &Game) {
    let mouse = game.mouse();
    if mouse.is_none() {
        return;
    }

    let view = game.view();
    let mouse = mouse.unwrap();
    let view_mouse: Point2d = Point2d { x: mouse.x - view.x, y: mouse.y - view.y };

    let items = Shapes {
        items: vec![
            Line::new(
                Point2d { x: 0., y: 0. },
                view_mouse,
                BLACK,
            ),
            Circle::new(
                view_mouse,
                3,
                BLACK,
            ),
        ]
    };

    game.clear();
    game.draw(&items);
}

fn game_loop(game: Game) {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        draw(&game);

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let game = Game::create("#canvas");

    {
        let closure = Closure::<dyn FnMut(_)>::new(
            enclose!( (game ) move |event:MouseEvent| { game.set_mouse(event); } ),
        );
        window().add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    game_loop(game);

    Ok(())
}
