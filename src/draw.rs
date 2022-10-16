use web_sys::CanvasRenderingContext2d;
use crate::game_engine::View;

pub trait Draw {
    fn draw(&self, context: &CanvasRenderingContext2d, view: &View) -> ();
}
