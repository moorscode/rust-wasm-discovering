#[allow(dead_code)]
use std::f64;
use js_sys::Math::{max, min};
use rgb::RGB;
use web_sys::CanvasRenderingContext2d;
use crate::browser::console_log;
use crate::Draw;
use crate::game_engine::View;

pub fn rgb(rgb: RGB<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
}

#[derive(Clone, Copy, PartialEq)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Point2d {
    pub fn normalize(x: f64, y: f64) -> Self {
        let u: f64 = (x.powi(2) + y.powi(2)).sqrt();
        Point2d { x: x / u, y: y / u }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Line {
    pub from: Point2d,
    pub to: Point2d,
    pub color: RGB<u8>,
}

impl Line {
    pub fn new(a: Point2d, b: Point2d, color: RGB<u8>) -> Box<Line> {
        Box::new(Line { from: a, to: b, color })
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Circle {
    pub center_point: Point2d,
    pub radius: u8,
    pub color: RGB<u8>,
}

impl Circle {
    pub fn new(center: Point2d, radius: u8, color: RGB<u8>) -> Box<Circle> {
        Box::new(Circle { center_point: center, radius, color })
    }
}

pub struct CollisionRectangle {
    pub top_left: Point2d,
    pub bottom_right: Point2d,
}

impl CollisionRectangle {
    pub fn new(a: Point2d, b: Point2d) -> Self {
        let top_left = Point2d { x: min(a.x, b.x), y: min(a.y, b.y) };
        let bottom_right = Point2d { x: max(a.x, b.x), y: max(a.y, b.y) };

        Self {
            top_left,
            bottom_right,
        }
    }

    pub fn collides_with(&self, other: CollisionRectangle) -> bool {
        // To the left of the other.
        if self.bottom_right.x < other.top_left.x {
            return false;
        }

        // To the right of the other.
        if self.top_left.x > other.bottom_right.x {
            return false;
        }

        // To the top of the other.
        if self.bottom_right.y < other.top_left.y {
            return false;
        }

        if self.top_left.y > other.bottom_right.y {
            return false;
        }

        true
    }
}

pub struct Shapes {
    pub items: Vec<Box<dyn Draw>>,
}

impl Draw for Shapes {
    fn draw(&self, context: &CanvasRenderingContext2d, view: &View) -> () {
        for item in self.items.iter() {
            if item.in_view(view) {
                item.draw(context, view);
            }
        }
    }

    fn in_view(&self, _view: &View) -> bool {
        true
    }
}

impl Draw for Line {
    fn draw(&self, context: &CanvasRenderingContext2d, view: &View) -> () {
        let color = rgb(self.color);
        let from = view.transform(&self.from);
        let to = view.transform(&self.to);

        context.begin_path();
        context.set_stroke_style(&color.into());
        context.move_to(from.x, from.y);
        context.line_to(to.x, to.y);
        context.stroke();
    }

    fn in_view(&self, view: &View) -> bool {
        let from = view.transform(&self.from);
        let to = view.transform(&self.to);

        let r: CollisionRectangle = CollisionRectangle::new(from, to);
        let v: CollisionRectangle = CollisionRectangle::new(Point2d { x: 0., y: 0. }, view.size);

        r.collides_with(v)
    }
}

impl Draw for Circle {
    fn draw(&self, context: &CanvasRenderingContext2d, view: &View) -> () {
        let color = rgb(self.color);
        let center = view.transform(&self.center_point);

        context.begin_path();
        context.set_stroke_style(&color.into());
        context.move_to(
            center.x + self.radius as f64,
            center.y,
        );
        context
            .arc(
                center.x,
                center.y,
                self.radius as f64,
                0.,
                f64::consts::PI * 2.0,
            )
            .unwrap();
        context.stroke();
    }

    fn in_view(&self, view: &View) -> bool {
        let center = view.transform(&self.center_point);
        let offset = center.x - self.radius as f64 / 2.;
        let top_left = Point2d { x: center.x - offset, y: center.y - offset };
        let bottom_right = Point2d { x: center.x + offset, y: center.y + offset };

        let r: CollisionRectangle = CollisionRectangle::new(top_left, bottom_right);
        let v: CollisionRectangle = CollisionRectangle::new(Point2d { x: 0., y: 0. }, view.size);

        r.collides_with(v)
    }
}
