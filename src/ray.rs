use std::cell::RefCell;
use std::rc::Rc;
use crate::{Line, Point2d};

struct Direction {
    vector: Point2d,
}

pub struct Ray {
    source: Point2d,
    direction: Rc<RefCell<Direction>>,
}

impl Ray {
    fn create_direction(source: Point2d, point: Point2d) -> Point2d {
        let look_at = Point2d { x: point.x - source.x, y: point.y - source.y };
        Point2d::normalize(look_at.x, look_at.y)
    }

    pub fn new(source: Point2d, direction: Point2d) -> Self {
        let direction = Ray::create_direction(source, direction);
        let direction = Rc::new(RefCell::new(Direction { vector: direction }));
        Self {
            source,
            direction,
        }
    }

    pub fn look_at(&self, point: Point2d) -> () {
        let mut direction = self.direction.borrow_mut();
        direction.vector = Ray::create_direction(self.source, point);
    }

    pub fn direction(&self) -> Point2d {
        self.direction.borrow().vector
    }

    pub fn intersects_line(&self, line: &Line) -> Option<Point2d> {
        let direction = self.direction.borrow().vector;

        let Point2d { x: x1, y: y1 } = line.from;
        let Point2d { x: x2, y: y2 } = line.to;
        let Point2d { x: x3, y: y3 } = self.source;

        let a = x1 - x2;
        let b = y1 - y2;
        let c = -direction.y;
        let d = -direction.x;

        let e = x1 - x3;
        let f = y1 - y3;

        let denominator = a * c - b * d;
        if denominator == 0.0 {
            return None;
        }

        let t = (e * c - f * d) / denominator;
        // It should be on the target line.
        if t <= 0.0 || t >= 1.0 {
            return None;
        }

        let u = -(a * f - b * e) / denominator;
        // As we're using a direction, we invalidate any intersection "behind" the source point.
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
