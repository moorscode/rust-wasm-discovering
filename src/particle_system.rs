use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use chrono::Utc;
use crate::{Draw, Game, Particle};

#[derive(Clone)]
pub struct ParticleContainer {
    particles: Vec<Box<Particle>>,
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

        if container.particles.len() > 2000 {
            container.particles.remove(0);
        }

        container.particles.push(Box::new(particle));
    }

    fn remove_particles(mut container: RefMut<ParticleContainer>, indices: Vec<Option<usize>>) -> () {
        let mut removed = 0;
        for index in indices.iter() {
            match index {
                Some(index) => {
                    let remove = *index - removed;
                    if container.particles.len() >= remove {
                        container.particles.remove(remove);
                        removed += 1;
                    }
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

        for (index, particle) in container.particles.iter().enumerate() {
            match particle.tick(time) {
                Some(_) => {
                    particle.draw(context, &view);
                }
                None => {
                    remove.push(Some(index));
                }
            }
        }

        Self::remove_particles(container, remove);
    }
}