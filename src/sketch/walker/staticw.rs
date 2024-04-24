use nannou::{
    color::{IntoColor, IntoLinSrgba},
    prelude::*,
};
use once_cell::sync::Lazy;

use crate::sketch::WIDTH;

use super::Walker;

pub const ATTRACTOR: Lazy<Walker> =
    Lazy::new(|| Walker::Static(StaticWalker::new(Point2::new(WIDTH as f32 / 2.0, 0.0))));

#[derive(Clone)]
pub struct StaticWalker {
    pub position: Point2,
}

impl StaticWalker {
    pub fn new(position: Point2) -> Self {
        Self { position }
    }

    pub fn draw(&self, app: &App, draw: &Draw) {
        #[cfg(debug)]
        {
            draw.ellipse().xy(self.position).radius(5.0).color(BLACK);
        }
    }
}
