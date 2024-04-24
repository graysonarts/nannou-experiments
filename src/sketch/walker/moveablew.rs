use nannou::{
    color::{blend::PreAlpha, Blend, IntoColor, IntoLinSrgba},
    prelude::*,
};
use once_cell::sync::Lazy;

const DAMP: f32 = 0.75;
pub const ENERGETIC: f32 = 0.5;

pub const ALPHA: Lazy<LinSrgba> = Lazy::new(|| LinSrgba::new(0.0, 0.0, 0.0, 0.25));

use super::{
    Walker, BOTTOM_RIGHT, DOWN, HEIGHT, MAX_EFFECTIVE_DISTANCE, MAX_PATH_LENGTH, PERSONAL_SPACE,
    TOP_LEFT, WIDTH,
};

#[derive(Clone)]
pub struct MoveableWalker {
    pub id: usize,
    pub color: LinSrgba,
    pub position: Point2,
    direction_bias: Vec2,
    pub velocity: Vec2,
    acceleration: Vec2,
    path: Vec<Point2>,
}
impl MoveableWalker {
    pub fn new<C>(id: usize, color: C, position: Point2, initial_v: Option<Vec2>) -> Self
    where
        C: IntoLinSrgba<f32>,
    {
        Self {
            id,
            color: color.into_lin_srgba(),
            position,
            velocity: Vec2::new(0.0, 0.0),
            direction_bias: initial_v.unwrap_or_default(),
            acceleration: Vec2::new(0.0, 0.0),
            path: vec![position],
        }
    }

    pub fn start_rebound(&mut self) {
        self.acceleration = self.direction_bias;
        // println!("{} a:{:?}", self.id, self.acceleration);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force * 1.0; // MASS
                                          // println!("{} a:{:?} += f:{:?}", self.id, self.acceleration, force);
    }

    pub fn apply_edge_repulsion(&mut self) {
        let distance_to_left = self.position.x - TOP_LEFT.x;
        let distance_to_right = BOTTOM_RIGHT.x - self.position.x;
        let distance_to_top = self.position.y - TOP_LEFT.y;
        let distance_to_bottom = BOTTOM_RIGHT.y - self.position.y;

        let mut force = -Vec2::new(distance_to_left, distance_to_top)
            + Vec2::new(distance_to_right, distance_to_bottom);
        force = force.normalize() * 0.01;
        self.apply_force(force);
    }

    pub fn update(&mut self, app: &App, update: Update) {
        // self.acceleration = self.acceleration.clamp_length(0.0, 25.0);
        // self.velocity += self.acceleration;
        // self.velocity = self.velocity.clamp_length(0.0, 5.0);
        // self.position += self.velocity;

        // self.path = vec![self.position];
        // println!(
        //     "{} a:{:?} v:{:?} p:{:?}",
        //     self.id, self.acceleration, self.velocity, self.path
        // );

        // let mouse = app.mouse.position() + Vec2::new(WIDTH as f32 / 2.0, 0.0);
        // self.acceleration += (mouse - self.position) * 0.01;
        self.apply_edge_repulsion();
        self.acceleration = self.acceleration.clamp_length(0.0, 3.0);
        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(0.0, 5.0);

        let possible_new_position = (self.position);

        if (possible_new_position.x < 0.0) || (possible_new_position.x > WIDTH as f32) {
            self.velocity.x *= -1.0;
            self.direction_bias.x *= -1.0;
        }

        if (possible_new_position.y < HEIGHT as f32 / -2.0)
            || (possible_new_position.y > HEIGHT as f32 / 2.0)
        {
            self.velocity.y *= -1.0;
            self.direction_bias.y *= -1.0;
        }

        self.position += self.velocity;
        self.velocity *= DAMP;

        self.path = self
            .path
            .iter()
            .rev()
            .cloned()
            .take(MAX_PATH_LENGTH)
            .rev()
            .collect();
        self.path.push(self.position);
    }

    pub fn draw(&self, app: &App, draw: &Draw, idx: usize, include_debug: bool) {
        // let color = self.color.multiply(*ALPHA);
        let color = self.color;
        self.path.iter().fold(None, |prev, &point| {
            if let Some(prev) = prev {
                draw.line().start(prev).end(point).color(color);
            }
            Some(point)
        });

        if include_debug {
            self.path.last().map(|&point| {
                // draw.ellipse().xy(point).radius(5.0).color(RED);
                draw.text(&format!("{}", idx))
                    .xy(point)
                    .color(RED)
                    .font_size(24);
                // draw.ellipse()
                //     .xy(point)
                //     .radius(PERSONAL_SPACE / 2.0)
                //     .no_fill()
                //     .stroke_weight(1.0)
                //     .color(RED);
                // draw.ellipse()
                //     .radius(MAX_EFFECTIVE_DISTANCE / 2.0)
                //     .xy(point)
                //     .no_fill()
                //     .stroke_weight(1.0)
                //     .color(RED);
                // let direction = self.velocity.normalize() * PERSONAL_SPACE / 2.0;
                // draw.line()
                //     .start(point + direction)
                //     .end(point + direction + direction.normalize() * self.velocity.length() * 50.0)
                //     .stroke_weight(1.0)
                //     .color(RED);
                // draw.line()
                //     .start(point)
                //     .end(point + self.direction_bias * 30.0)
                //     .color(RED)
                //     .stroke_weight(3.0);

                // draw.line()
                //     .start(point)
                //     .end(point + self.velocity)
                //     .color(BLACK)
                //     .stroke_weight(3.0);
            });
        }
    }
}
