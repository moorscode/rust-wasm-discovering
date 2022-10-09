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
    mouse: Option<Point2d>,
}

impl Default for Inner {
    fn default() -> Self {
        Inner {
            mouse: None
        }
    }
}

pub struct View {
    pub offset: Point2d,
}

#[derive(Clone)]
pub struct Game {
    inner: Rc<RefCell<Inner>>,
    view: Rc<RefCell<View>>,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

impl Game {
    pub fn create(id: &str) -> Self {
        let canvas: HtmlCanvasElement = canvas(id);
        let context: CanvasRenderingContext2d = context(&canvas);

        let offset: Point2d = Point2d { x: canvas.width() as f64 / 2., y: canvas.height() as f64 / 2. };
        let view: View = View { offset };

        Game {
            inner: Rc::new(RefCell::new(Inner::default())),
            view: Rc::new(RefCell::new(view)),
            context,
            canvas,
        }
    }

    pub fn draw(&self, shapes: &Shapes) -> () {
        shapes.draw(&self.context, &self.view.borrow());
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

    pub fn view(&self) -> Point2d {
        self.view.borrow().offset
    }
}
