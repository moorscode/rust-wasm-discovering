use std::f64;
use rgb::RGB;
use web_sys::CanvasRenderingContext2d;

fn rgb(rgb: RGB<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
}

#[derive(Clone, Copy)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy)]
pub struct Line {
    pub from: Point2d,
    pub to: Point2d,
    pub color: RGB<u8>,
}

#[derive(Clone, Copy)]
pub struct Circle {
    pub center_point: Point2d,
    pub radius: u8,
    pub color: RGB<u8>,
}

pub trait Draw {
    fn draw(&self, context: &CanvasRenderingContext2d) -> ();
}

pub struct Shapes {
    pub items: Vec<Box<dyn Draw>>,
}

impl Draw for Shapes {
    fn draw(&self, context: &CanvasRenderingContext2d) -> () {
        for item in self.items.iter() {
            item.draw(context);
        }
    }
}

impl Draw for Line {
    fn draw(&self, context: &CanvasRenderingContext2d) -> () {
        let color = rgb(self.color);
        context.begin_path();
        context.set_stroke_style(&color.into());
        context.move_to(self.from.x, self.from.y);
        context.line_to(self.to.x, self.to.y);
        context.stroke();
    }
}

impl Draw for Circle {
    fn draw(&self, context: &CanvasRenderingContext2d) -> () {
        let color = rgb(self.color);
        context.begin_path();
        context.set_stroke_style(&color.into());
        context.move_to(
            self.center_point.x + self.radius as f64,
            self.center_point.y,
        );
        context
            .arc(
                self.center_point.x,
                self.center_point.y,
                self.radius as f64,
                0.,
                f64::consts::PI * 2.0,
            )
            .unwrap();
        context.stroke();
    }
}
