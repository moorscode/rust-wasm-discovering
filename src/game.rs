#![allow(unused, dead_code)]

use rgb::*;
use std::cell::{RefCell, Ref};
use std::rc::Rc;
use js_sys::Math::random;
use web_sys::{MouseEvent};
use crate::browser::console_log;

use crate::{Draw, GameEngine};
use crate::shapes::*;
use crate::particle_animation::*;
use crate::particle::*;
use crate::game_engine::View;
use crate::particle_system::ParticleSystem;
use crate::ray::Ray;
use std::borrow::Borrow;

const RED: RGB8 = RGB8 { r: 255, g: 0, b: 0 };
const GREEN: RGB8 = RGB8 { r: 0, g: 255, b: 0 };
const WHITE: RGB8 = RGB8 { r: 255, g: 255, b: 255 };
const BLACK: RGB8 = RGB8 { r: 0, g: 0, b: 0 };

const DIR_UP: Point2d = Point2d { x: 0., y: -1. };
const DIR_DOWN: Point2d = Point2d { x: 0., y: 1. };
const DIR_LEFT: Point2d = Point2d { x: -1., y: 0. };
const DIR_RIGHT: Point2d = Point2d { x: 1., y: 0. };

impl From<MouseEvent> for Point2d {
    fn from(e: MouseEvent) -> Self {
        Self {
            x: e.offset_x() as f64,
            y: e.offset_y() as f64,
        }
    }
}

pub fn handle_keypress(game_engine: &GameEngine, key_code: u32) -> () {
    match key_code {
        37 => { game_engine.shift_view_by(Point2d { x: 10., y: 0. }); } // left
        38 => { game_engine.shift_view_by(Point2d { x: 0., y: 10. }); } // up
        39 => { game_engine.shift_view_by(Point2d { x: -10., y: 0. }); } // right
        40 => { game_engine.shift_view_by(Point2d { x: 0., y: -10. }); } // down
        67 => { game_engine.reset_view(); } // "C"
        _ => ()
    }
}

pub fn tick(game_engine: &GameEngine) {
    let mouse: Option<Point2d> = game_engine.mouse();
    let view: Ref<View> = game_engine.view();

    let mut lines: Vec<Box<Line>> = vec![];
    let mut items: Vec<Box<dyn Draw>> = vec![];

    let line: Box<Line> = Line::new(
        Point2d { x: 200., y: -200. },
        Point2d { x: 200., y: 200. },
        BLACK,
    );
    let line2: Box<Line> = Line::new(
        Point2d { x: -200., y: -200. },
        Point2d { x: -200., y: 200. },
        BLACK,
    );
    let line3: Box<Line> = Line::new(
        Point2d { x: -200., y: 200. },
        Point2d { x: 200., y: 200. },
        BLACK,
    );
    let line4: Box<Line> = Line::new(
        Point2d { x: -200., y: -200. },
        Point2d { x: 200., y: -200. },
        BLACK,
    );

    items.push(line.clone());
    items.push(line2.clone());
    items.push(line3.clone());
    items.push(line4.clone());

    items.push(
        Circle::new(
            view.center,
            10,
            BLACK,
        )
    );

    lines.push(line);
    lines.push(line2);
    lines.push(line3);
    lines.push(line4);

    if mouse.is_some() {
        let mouse = mouse.unwrap();
        let view_mouse: Point2d = Point2d { x: mouse.x - view.offset.x, y: mouse.y - view.offset.y };
        let ray = Ray::new(view.center, view_mouse);

        let mut intersection = None;

        for line in lines.iter() {
            intersection = ray.intersects_line(&line);
            if intersection.is_some() {
                break;
            }
        }

        match intersection {
            Some(intersection) => {
                items.push(Line::new(
                    view.center,
                    intersection.point,
                    GREEN,
                ));

                let dir_towards_center = Point2d { x: ray.direction().x * -1., y: ray.direction().y * -1. };

                items.push(
                    Line::new(
                        intersection.point,
                        Point2d {
                            x: intersection.point.x + intersection.angle_direction.x * 10.,
                            y: intersection.point.y + intersection.angle_direction.y * 10.,
                        },
                        BLACK,
                    ),
                );

                let particle_system: &ParticleSystem = game_engine.particle_system();
                particle_system.add_particle(
                    Particle::new(
                        ParticlePixel { position: intersection.point, color: RED, alpha: 1.0 },
                        intersection.angle_direction,
                        0.4 + random() * 0.3,
                        1500,
                        particle_velocity_increasing,
                        particle_tick_move_and_fade_out,
                    )
                );
            }
            None => {
                items.push(Line::new(
                    view.center,
                    Point2d { x: view.center.x + ray.direction().x * 1000., y: view.center.y + ray.direction().y * 1000. },
                    BLACK,
                ));
            }
        }

        let ray_line: Box<Line> = Line::new(view.center, Point2d { x: view.center.x + ray.direction().x * 10., y: view.center.y + ray.direction().y * 10. }, BLACK);
        items.push(ray_line);
    }

    game_engine.clear();
    game_engine.draw(&Shapes { items });
}