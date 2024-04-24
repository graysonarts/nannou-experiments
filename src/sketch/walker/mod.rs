use nannou::{
    color::{IntoColor, IntoLinSrgba},
    prelude::*,
};
use once_cell::sync::Lazy;

use self::{moveablew::MoveableWalker, staticw::StaticWalker};

use super::{HEIGHT, MAX_PATH_LENGTH, WIDTH};

pub mod moveablew;
pub mod staticw;

const LEFT: Lazy<Vec2> = Lazy::new(|| Vec2::new(-1.0, 0.0));
const RIGHT: Lazy<Vec2> = Lazy::new(|| Vec2::new(1.0, 0.0));
const UP: Lazy<Vec2> = Lazy::new(|| Vec2::new(0.0, 1.0));
const DOWN: Lazy<Vec2> = Lazy::new(|| Vec2::new(0.0, -1.0));
const TOP_LEFT: Lazy<Vec2> = Lazy::new(|| Vec2::new(0.0, HEIGHT as f32 / -2.0));
const BOTTOM_RIGHT: Lazy<Vec2> = Lazy::new(|| Vec2::new(WIDTH as f32, HEIGHT as f32 / 2.0));

const PERSONAL_SPACE: f32 = 20.0;
const MAX_EFFECTIVE_DISTANCE: f32 = 100.0;

#[derive(Clone)]
pub enum Walker {
    Moveable(MoveableWalker),
    Static(StaticWalker),
}

impl Walker {
    pub fn position(&self) -> Point2 {
        match self {
            Walker::Moveable(w) => w.position,
            Walker::Static(w) => w.position,
        }
    }

    pub fn velocity(&self) -> Vec2 {
        match self {
            Walker::Moveable(w) => w.velocity,
            Walker::Static(_) => Vec2::new(0.0, 0.0),
        }
    }

    pub(crate) fn start_rebound(&mut self) {
        if let Walker::Moveable(w) = self {
            w.start_rebound();
        }
    }

    pub fn rebound(&self, other: &Walker) -> Vec2 {
        let distance = self.position().distance(other.position());
        if distance > MAX_EFFECTIVE_DISTANCE {
            return Vec2::new(0.0, 0.0);
        }

        let (repel_force, repel_dir) = if distance > PERSONAL_SPACE {
            (
                // 1000.0 / distance,
                distance,
                (-self.velocity() + other.velocity()).normalize(),
            )
        } else {
            (
                (PERSONAL_SPACE - distance),
                // distance * -1000.0,
                (-self.velocity() + other.velocity()).normalize(),
            )
        };

        (repel_force * repel_dir).clamp_length_max(1.0)
    }

    pub(crate) fn apply_force(&mut self, repel_force: Vec2) {
        if let Walker::Moveable(w) = self {
            w.apply_force(repel_force);
        }
    }

    pub(crate) fn update(&mut self, app: &App, update: Update) {
        if let Walker::Moveable(w) = self {
            w.update(app, update);
        }
    }

    pub(crate) fn draw(&self, app: &App, draw: &Draw, idx: usize, include_debug: bool) {
        match self {
            Walker::Moveable(w) => w.draw(app, draw, idx, include_debug),
            Walker::Static(w) => w.draw(app, draw),
        }
    }
}
