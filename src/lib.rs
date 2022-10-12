mod game;
mod shapes;
mod ray;
mod particle;
mod particle_animation;
mod particle_system;
mod draw;

use rgb::*;
use std::cell::RefCell;
use std::rc::Rc;
use js_sys::Math::random;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, Window, KeyboardEvent};

use crate::shapes::{Line, Point2d, Shapes};
use crate::particle_animation::*;
use crate::particle::{Particle, ParticlePixel};
use game::Game;
use ray::Ray;
use crate::draw::Draw;
use crate::particle_system::ParticleSystem;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub fn console_log(log: &str) {
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(log));
    web_sys::console::log(&array);
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

const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
const GREEN: RGB8 = RGB8 { r: 0, g: 255, b: 0 };
// const WHITE: RGB8 = RGB8 { r: 255, g: 255, b: 255 };
const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };
const CENTER_POINT: Point2d = Point2d { x: 0., y: 0. };

fn draw(game: &Game) {
    let mouse: Option<Point2d> = game.mouse();
    let view: Point2d = game.view().offset;

    let line: Box<Line> = Line::new(
        Point2d { x: 100., y: -100. },
        Point2d { x: 150., y: 100. },
        BLACK,
    );

    let line2: Box<Line> = Line::new(
        Point2d { x: -100., y: -100. },
        Point2d { x: -150., y: 100. },
        BLACK,
    );

    let mut lines: Vec<Box<Line>> = vec![];

    let mut items: Vec<Box<dyn Draw>> = vec![];
    items.push(line.clone());
    items.push(line2.clone());

    lines.push(line);
    lines.push(line2);

    if mouse.is_some() {
        let mouse = mouse.unwrap();
        let view_mouse: Point2d = Point2d { x: mouse.x - view.x, y: mouse.y - view.y };
        let ray = Ray::new(CENTER_POINT, view_mouse);

        let mut intersection = None;

        for line in lines.iter() {
            intersection = ray.intersects_line(&line);
            if intersection.is_some() {
                break;
            }
        }

        match intersection {
            Some(point) => {
                items.push(Line::new(
                    CENTER_POINT,
                    point,
                    GREEN,
                ));

                let particle_system: &ParticleSystem = game.particle_system();
                particle_system.add_particle(
                    Particle::new(
                        ParticlePixel { position: point, color: RED, alpha: 1.0 },
                        Point2d { x: ray.direction().x * -1., y: ray.direction().y * -1. },
                        0.4 + random() * 0.3,
                        1500,
                        standard_increasing,
                        default_behaviour,
                    )
                );
            }
            None => {
                items.push(Line::new(
                    CENTER_POINT,
                    Point2d { x: ray.direction().x * 1000., y: ray.direction().y * 1000. },
                    BLACK,
                ));
            }
        }

        let ray_line: Box<Line> = Line::new(CENTER_POINT, Point2d { x: ray.direction().x * 10., y: ray.direction().y * 10. }, BLACK);
        items.push(ray_line);
    }

    game.clear();
    game.draw(&Shapes { items });
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

    // Mouse tracker.
    {
        let closure = Closure::<dyn FnMut(_)>::new(
            enclose!( (game ) move |event:MouseEvent| { game.set_mouse(event); } ),
        );
        game.canvas().add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Keyboard listener.
    {
        let closure = Closure::<dyn FnMut(_)>::new(
            enclose!( (game) move |event:KeyboardEvent| {
                match event.key_code() {
                    37 => { game.shift_view_by( Point2d { x: -10., y: 0. } ); } // left
                    38 => { game.shift_view_by( Point2d { x: 0., y: -10. } ); } // up
                    39 => { game.shift_view_by( Point2d { x: 10., y: 0. } ); } // right
                    40 => { game.shift_view_by( Point2d { x: 0., y: 10. } ); } // down
                    67 => { game.reset_view(); } // "C"
                    _ => ()
                }
            }),
        );
        window().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    game_loop(game);

    Ok(())
}
