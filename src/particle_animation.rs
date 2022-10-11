use chrono::{Duration};
use crate::{ParticlePixel, Point2d};

pub fn default_behaviour(start_pixel: &ParticlePixel, current_pixel: Option<ParticlePixel>, direction: &Point2d, velocity: &f64, delta: Duration) -> Option<ParticlePixel> {
    let lifetime = 1500.;

    let delta_milliseconds = delta.num_milliseconds() as f64;
    if delta_milliseconds > lifetime {
        return None;
    }

    let pixel = if current_pixel.is_some() { current_pixel.unwrap() } else { *start_pixel };
    let percentage = 1. - if delta_milliseconds > 0. { delta_milliseconds / lifetime as f64 } else { 0. };

    let new = ParticlePixel {
        position: Point2d {
            x: pixel.position.x + (direction.x * velocity),
            y: pixel.position.y + (direction.y * velocity),
        },
        color: pixel.color,
        alpha: start_pixel.alpha * percentage as f64,
    };

    Some(new)
}

pub fn standard_increasing(
    velocity: &f64,
    delta: Duration,
) -> f64 {
    let multiplier: f64 = 1.10;
    let modulo = delta.num_milliseconds() / 100;
    return velocity * (multiplier.powf(modulo as f64 + 1.));
}

pub fn standard_decreasing(
    velocity: &f64,
    delta: Duration,
) -> f64 {

    let multiplier: f64 = 0.95;
    let modulo = delta.num_milliseconds() / 100;
    return velocity * (multiplier.powf(modulo as f64 + 1.));
}
