use std::cell::RefCell;
use std::rc::Rc;
use crate::{Line, Point2d};

pub trait Intersects {
    fn intersects(&self, line: &Line) -> Option<Point2d>;
}

struct Direction {
    direction: Point2d,
}

pub struct Ray {
    source: Point2d,
    direction: Rc<RefCell<Direction>>,
}

impl Intersects for Ray {
    fn intersects(&self, line: &Line) -> Option<Point2d> {
        let direction = self.direction.borrow().direction;

        let Point2d { x: x1, y: y1 } = line.from;
        let Point2d { x: x2, y: y2 } = line.to;
        let Point2d { x: x3, y: y3 } = self.source;
        let x4 = x3 + direction.x;
        let y4 = y3 + direction.y;

        let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denominator == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denominator;
        if t <= 0.0 || t >= 1.0 {
            return None;
        }

        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denominator;
        if u <= 0.0 {
            return None;
        }

        Some(
            Point2d {
                x: x1 + t * (x2 - x1),
                y: y1 + t * (y2 - y1),
            }
        )
    }
}

impl Ray {
    pub fn new(source: Point2d, direction: Point2d) -> Self {
        let direction = Ray::create_direction(source, direction);
        let direction = Rc::new(RefCell::new(Direction { direction }));
        Self {
            source,
            direction,
        }
    }

    fn create_direction(source: Point2d, point: Point2d) -> Point2d {
        let look_at = Point2d { x: point.x - source.x, y: point.y - source.y };
        Point2d::normalized(look_at.x, look_at.y)
    }

    pub fn look_at(&self, point: Point2d) -> () {
        let mut direction = self.direction.borrow_mut();
        direction.direction = Ray::create_direction(self.source, point);
    }

    pub fn direction(&self) -> Point2d {
        self.direction.borrow().direction
    }
}