use std::cell::{RefCell, RefMut};
use std::ops::{Sub};
use std::rc::Rc;
use chrono::{DateTime, Duration, Utc};
use js_sys::Math::random;
use rgb::RGB;
use web_sys::CanvasRenderingContext2d;
use crate::game::View;
use crate::{Draw, Game, Point2d};
use crate::shapes::rgb;

trait Tick {
    fn tick(&self, time: u64) -> Point2d;
}

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
// todo: I want this to receive a function that determines its behaviour..
pub struct Particle {
    start_pixel: ParticlePixel,
    start_time: DateTime<Utc>,
    render: Rc<RefCell<Render>>,
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.start_pixel.position == other.start_pixel.position && self.start_time == other.start_time
    }
}

#[derive(Copy, Clone)]
pub struct ParticlePixel {
    pub position: Point2d,
    pub color: RGB<u8>,
}

impl Draw for Particle {
    fn draw(&self, context: &CanvasRenderingContext2d, view: &View) -> () {
        let pixel = self.render.borrow().pixel;
        match pixel {
            Some(pixel) => {
                let color = rgb(pixel.color);
                let coords = view.transform(&pixel.position);
                context.set_fill_style(&color.into());
                context.fill_rect(coords.x, coords.y, 2., 2.);
            }
            None => ()
        }
    }
}

impl Particle {
    pub fn new(pixel: ParticlePixel, start_time: DateTime<Utc>) -> Self {
        Self {
            render: Rc::new(RefCell::new(Render { pixel: Some(pixel) })),
            start_pixel: pixel,
            start_time,
        }
    }

    fn delta(&self, time: DateTime<Utc>) -> Duration {
        time.sub(self.start_time)
    }

    fn start_pixel(&self) -> ParticlePixel {
        self.start_pixel
    }

    fn set_pixel(&self, pixel: ParticlePixel) -> () {
        let mut render: RefMut<Render> = self.render.borrow_mut();
        render.pixel = Some(pixel);
    }

    fn pixel(&self) -> Option<ParticlePixel> {
        self.render.borrow().pixel
    }
}

#[derive(Clone)]
pub struct ParticleContainer {
    pub particles: Vec<Box<Particle>>,
}

#[derive(Clone)]
pub struct ParticleSystem {
    pub container: Rc<RefCell<ParticleContainer>>,
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self {
            container: Rc::new(RefCell::new(ParticleContainer { particles: vec![] })),
        }
    }
}

impl ParticleSystem {
    pub fn add_particle(&self, particle: Particle) -> () {
        let mut container = self.container.borrow_mut();
        container.particles.push(Box::new(particle));

        // Trigger initial animation to make sure the pixel exists.
        // self.animate(&container.particles.last().unwrap(), Utc::now());
    }

    pub fn remove_particles(mut container: RefMut<ParticleContainer>, indices: Vec<Option<usize>>) -> () {
        for index in indices.iter() {
            match index {
                Some(index) => {
                    container.particles.remove(*index);
                }
                None => ()
            }
        }
    }

    pub fn tick(&self, game: &Game) -> () {
        let context = game.context();
        let view = game.view();
        let time = Utc::now();

        let container = self.container.borrow_mut();
        let mut remove: Vec<Option<usize>> = vec![];

        for particle in container.particles.iter() {
            self.animate(particle, time);
            match particle.pixel() {
                Some(_) => {
                    particle.draw(context, &view);
                }
                None => {
                    let index = container.particles.iter().position(|r| r == particle);
                    remove.push(index);
                }
            }
        }

        ParticleSystem::remove_particles(container, remove);
    }

    // todo: This should be on a particle itself.
    fn animate(&self, particle: &Particle, time: DateTime<Utc>) -> () {
        let pixel = particle.start_pixel();
        let delta = particle.delta(time).num_milliseconds();

        if delta.rem_euclid(250) == 0 {
            particle.set_pixel(
                ParticlePixel {
                    position: Point2d {
                        x: pixel.position.x + (random() * 10.) - 5.,
                        y: pixel.position.y + (random() * 10.) - 5.,
                    },
                    ..pixel
                }
            );
        }
    }
}