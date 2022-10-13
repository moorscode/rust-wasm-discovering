use chrono::{Duration};
use crate::particle::ParticlePixel;
use crate::Point2d;

#[allow(dead_code)]
pub fn particle_tick_move(start_pixel: &ParticlePixel, current_pixel: Option<ParticlePixel>, direction: &Point2d, velocity: &f64, delta: Duration, lifetime: &u32) -> Option<ParticlePixel> {
    let delta_milliseconds: f64 = delta.num_milliseconds() as f64;
    if delta_milliseconds > *lifetime as f64 {
        return None;
    }

    let pixel: ParticlePixel = if current_pixel.is_some() { current_pixel.unwrap() } else { *start_pixel };

    let new = ParticlePixel {
        position: Point2d {
            x: pixel.position.x + (direction.x * velocity),
            y: pixel.position.y + (direction.y * velocity),
        },
        color: pixel.color,
        alpha: 1.0,
    };

    Some(new)
}

#[allow(dead_code)]
pub fn particle_tick_move_and_fade_out(start_pixel: &ParticlePixel, current_pixel: Option<ParticlePixel>, direction: &Point2d, velocity: &f64, delta: Duration, lifetime: &u32) -> Option<ParticlePixel> {
    let pixel = particle_tick_move(start_pixel, current_pixel, direction, velocity, delta, lifetime);

    if pixel.is_none() {
        return pixel;
    }

    let delta_milliseconds: f64 = delta.num_milliseconds() as f64;
    let percentage: f64 = if delta_milliseconds > 0. { 1. - ( delta_milliseconds / *lifetime as f64 ) } else { 1. };
    let alpha: f64 = start_pixel.alpha * percentage;

    let pixel = ParticlePixel {
        alpha,
        ..pixel.unwrap()
    };

    Some(pixel)
}

#[allow(dead_code)]
pub fn particle_velocity_increasing(
    velocity: &f64,
    delta: Duration,
) -> f64 {
    let multiplier: f64 = 1.10;
    let division: i64 = delta.num_milliseconds() / 100;
    return velocity * (multiplier.powi(division as i32 + 1));
}

#[allow(dead_code)]
pub fn particle_velocity_decreasing(
    velocity: &f64,
    delta: Duration,
) -> f64 {
    let multiplier: f64 = 0.95;
    let division: i64 = delta.num_milliseconds() / 100;
    return velocity * (multiplier.powi(division as i32 + 1));
}
