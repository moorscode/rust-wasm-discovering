use std::cell::{RefCell, RefMut};
use std::ops::{Sub};
use std::rc::Rc;
use chrono::{DateTime, Duration, Utc};
use rgb::RGB;
use web_sys::CanvasRenderingContext2d;

use crate::game::View;
use crate::{Draw, Point2d};
use crate::shapes::rgb;

struct Render {
    pixel: Option<ParticlePixel>,
}

impl Default for Render {
    fn default() -> Self {
        Render {
            pixel: None
        }
    }
}

#[derive(Clone)]
pub struct Particle {
    start_pixel: ParticlePixel,
    start_time: DateTime<Utc>,
    start_velocity: f64,
    direction: Point2d,
    velocity: Rc<RefCell<f64>>,
    render: Rc<RefCell<Render>>,
    tick: fn(
        start_pixel: &ParticlePixel,
        current_pixel: Option<ParticlePixel>,
        direction: &Point2d,
        velocity: &f64,
        delta: Duration,
    ) -> Option<ParticlePixel>,
    update_velocity: fn(
        velocity: &f64,
        delta: Duration,
    ) -> f64,
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.start_pixel.position == other.start_pixel.position
            && self.start_time == other.start_time
            && self.direction == other.direction
            && self.velocity == other.velocity
            && self.start_pixel.color == other.start_pixel.color
    }
}

#[derive(Copy, Clone)]
pub struct ParticlePixel {
    pub position: Point2d,
    pub color: RGB<u8>,
    pub alpha: f64,
}

impl Draw for Particle {
    fn draw(&self, context: &CanvasRenderingContext2d, view: &View) -> () {
        let pixel = self.render.borrow().pixel;
        match pixel {
            Some(pixel) => {
                let color = rgb(pixel.color);
                let coords = view.transform(&pixel.position);
                context.set_global_alpha(pixel.alpha);
                context.set_fill_style(&color.into());
                context.fill_rect(coords.x, coords.y, 2., 2.);
                context.set_global_alpha(1.0);
            }
            None => ()
        }
    }
}

impl Particle {
    pub fn new(
        pixel: ParticlePixel,
        direction: Point2d,
        velocity: f64,
        update_velocity: fn(
            velocity: &f64,
            delta: Duration,
        ) -> f64,
        tick: fn(
            start_pixel: &ParticlePixel,
            current_pixel: Option<ParticlePixel>,
            direction: &Point2d,
            velocity: &f64,
            delta: Duration,
        ) -> Option<ParticlePixel>,
    ) -> Self {
        Self {
            render: Rc::new(RefCell::new(Render { pixel: Some(pixel) })),
            start_pixel: pixel,
            start_time: Utc::now(),
            tick,
            direction,
            velocity: Rc::new(RefCell::new(velocity)),
            start_velocity: velocity,
            update_velocity,
        }
    }

    pub fn delta(&self, time: DateTime<Utc>) -> Duration {
        time.sub(self.start_time)
    }

    pub fn start_pixel(&self) -> ParticlePixel {
        self.start_pixel
    }

    pub fn pixel(&self) -> Option<ParticlePixel> {
        self.render.borrow().pixel
    }

    fn set_velocity(&self, velocity: f64) -> () {
        *self.velocity.borrow_mut() = velocity;
    }

    pub fn velocity(&self) -> f64 {
        *self.velocity.borrow()
    }

    pub fn tick(&self, time: DateTime<Utc>) -> Option<ParticlePixel> {
        self.set_velocity((self.update_velocity)(&self.start_velocity, self.delta(time)));
        let pixel = (self.tick)(&self.start_pixel(), self.pixel(), &self.direction, &self.velocity(), self.delta(time));

        let mut render: RefMut<Render> = self.render.borrow_mut();
        render.pixel = pixel;
        pixel
    }
}
