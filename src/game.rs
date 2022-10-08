use std::{cell::RefCell, rc::Rc, f64};
use std::cell::RefMut;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window, Document};
use crate::Point2d;
use crate::shapes::{Shapes, Draw};

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn canvas(id: &str) -> HtmlCanvasElement {
    document()
        .query_selector(id)
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .expect("No canvas found.")
}

fn context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("No context created.")
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("Could not be fetched as internal object.")
}

struct Inner {
    mouse: Point2d,
}

impl Default for Inner {
    fn default() -> Self {
        Inner {
            mouse: Point2d { x: 0., y: 0. },
        }
    }
}

pub trait Create {
    fn create(id: &str) -> Self;
}

#[derive(Clone)]
pub struct Game {
    inner: Rc<RefCell<Inner>>,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl Create for Game {
    fn create(id: &str) -> Self {
        let canvas: HtmlCanvasElement = canvas(id);
        let context: CanvasRenderingContext2d = context(&canvas);

        Game {
            inner: Rc::new(RefCell::new(Inner::default())),
            context,
            canvas,
        }
    }
}

impl Game {
    pub fn draw(&self, shapes: &Shapes) -> () {
        shapes.draw(&self.context);
    }

    pub fn clear(&self) -> () {
        self.context.clear_rect(0., 0., self.canvas.width() as f64, self.canvas.height() as f64);
    }

    pub fn set_mouse<T: Into<Point2d>>(&self, mouse: T) -> () {
        let mut inner: RefMut<Inner> = self.inner.borrow_mut();
        inner.mouse = mouse.into();
    }

    pub fn mouse(&self) -> Point2d {
        self.inner.borrow().mouse
    }
}
