use std::{cell::RefCell, cell::Ref, rc::Rc, f64, cell::RefMut};
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{Browser, Draw, Point2d};
use crate::particle_system::ParticleSystem;
use crate::shapes::Shapes;

struct Inner {
    mouse: Option<Point2d>,
}

impl Default for Inner {
    fn default() -> Self {
        Inner {
            mouse: None
        }
    }
}

#[derive(Clone)]
pub struct View {
    pub offset: Point2d,
    pub center: Point2d,
}

impl View {
    pub fn new(point: Point2d) -> Self {
        Self {
            offset: point,
            center: Point2d { x: 0., y: 0. },
        }
    }
    pub fn transform(&self, point: &Point2d) -> Point2d {
        Point2d { x: point.x + self.offset.x, y: point.y + self.offset.y }
    }
}

#[derive(Clone)]
pub struct Game {
    inner: Rc<RefCell<Inner>>,
    view: Rc<RefCell<View>>,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    particle_system: ParticleSystem,
}

impl Game {
    pub fn create(id: &str) -> Self {
        let canvas: HtmlCanvasElement = Browser::canvas(id);
        let context: CanvasRenderingContext2d = Browser::context(&canvas);

        let offset: Point2d = Point2d { x: canvas.width() as f64 / 2., y: canvas.height() as f64 / 2. };
        let view: View = View::new(offset);

        let particle_system = ParticleSystem::default();

        Self {
            inner: Rc::new(RefCell::new(Inner::default())),
            view: Rc::new(RefCell::new(view)),
            context,
            canvas,
            particle_system,
        }
    }

    pub fn draw(&self, shapes: &Shapes) -> () {
        shapes.draw(&self.context, &self.view.borrow());
        self.particle_system.tick(self);
    }

    pub fn clear(&self) -> () {
        self.context.clear_rect(0., 0., self.canvas.width() as f64, self.canvas.height() as f64);
    }

    pub fn set_mouse<T: Into<Point2d>>(&self, mouse: T) -> () {
        let mut inner: RefMut<Inner> = self.inner.borrow_mut();
        inner.mouse = Some(mouse.into());
    }

    pub fn mouse(&self) -> Option<Point2d> {
        self.inner.borrow().mouse
    }

    pub fn view(&self) -> Ref<View> {
        self.view.borrow()
    }

    pub fn context(&self) -> &CanvasRenderingContext2d {
        &self.context
    }

    pub fn shift_view_by(&self, offset: Point2d) -> () {
        let mut view = self.view.borrow_mut();
        view.offset = Point2d { x: view.offset.x + offset.x, y: view.offset.y + offset.y };
        view.center = Point2d { x: view.center.x - offset.x, y: view.center.y - offset.y };
    }

    pub fn reset_view(&self) -> () {
        let mut view = self.view.borrow_mut();
        view.offset = Point2d { x: self.canvas.width() as f64 / 2., y: self.canvas.height() as f64 / 2. };
    }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    pub fn particle_system(&self) -> &ParticleSystem {
        &self.particle_system
    }

    pub fn run(&self, tick: fn(game: &Game)) {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let game = self.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            (tick)(&game);

            Browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        Browser::request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
