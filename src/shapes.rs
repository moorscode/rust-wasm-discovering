#[allow(dead_code)]

use std::f64;
use rgb::RGB;
use web_sys::CanvasRenderingContext2d;
use crate::game::View;

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
        let u = (x.powf(2.) + y.powf(2.)).sqrt();
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
        let from = if a.x < b.x { &a } else { &b };
        let to = if a.x < b.x { &b } else { &a };

        Box::new(Line { from: *from, to: *to, color })
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

pub trait Draw {
    fn draw(&self, context: &CanvasRenderingContext2d, view: &View) -> ();
}

pub struct Shapes {
    pub items: Vec<Box<dyn Draw>>,
}

impl Draw for Shapes {
    fn draw(&self, context: &CanvasRenderingContext2d, view: &View) -> () {
        for item in self.items.iter() {
            item.draw(context, view);
        }
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
}
