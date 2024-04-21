use std::sync::atomic::AtomicBool;

use nannou::{
    color::{self, Gradient, IntoColor, IntoLinSrgba},
    draw,
    prelude::*,
    winit::event::StartCause,
};

use crate::utils::{captured_frame_path, lerp};
mod palettes;

pub const WIDTH: u32 = 1200;
pub const HEIGHT: u32 = WIDTH * 9 / 16;

pub const STRIP_W: f32 = 50.0;
pub const STRIP_W2: f32 = STRIP_W * 2.0;

pub struct Model {
    frequency: f32,
    palette: Vec<Hsl>,
    capture_next_frame: AtomicBool,
}

impl Default for Model {
    fn default() -> Self {
        let frequency = Self::random();
        let palette = Self::new_palette(frequency as usize);

        Self {
            frequency,
            palette,
            capture_next_frame: AtomicBool::new(false),
        }
    }
}

impl Model {
    fn random() -> f32 {
        (random_f32() * 3.0 + 3.0).floor()
    }

    fn new_palette(count: usize) -> Vec<Hsl> {
        let palette = palettes::complement();
        palette.take(count).collect()
    }
}

impl Model {
    pub fn draw(&self, app: &App, draw: &Draw) {
        (0..1000).map(|i| (i as f32) / 1000.0).for_each(|t| {
            let x = lerp(0.0, WIDTH as f32, t);
            let y = (t * self.frequency * TAU).sin() * 100.0;
            // TODO: Generate Palette
            self.palette.iter().enumerate().for_each(|(idx, color)| {
                let offset = calc_offset(self.palette.len(), idx);
                Model::draw_section(draw, idx, x, y + offset, color.into_lin_srgba());
            });
        });

        #[cfg(debug)]
        {
            self.palette.iter().enumerate().for_each(|(idx, _)| {
                let x = WIDTH as f32 / 2.0;
                let y = calc_offset(self.palette.len(), idx);

                draw.text(&format!("{}", idx))
                    .x_y(x, y)
                    .font_size(24)
                    .color(BLACK);

                draw.text(&format!("{}", idx % 2))
                    .x_y(x + 30.0, y)
                    .font_size(12)
                    .color(BLACK);
            })
        }
    }

    fn draw_section<C>(draw: &Draw, count: usize, x: f32, y: f32, color: C)
    where
        C: IntoLinSrgba<f32>,
    {
        draw.line()
            .start(pt2(x, y - STRIP_W))
            .end(pt2(x, y + STRIP_W))
            .color(color.into_lin_srgba())
            .stroke_weight(2.0);
    }
}

fn calc_offset(total: usize, idx: usize) -> f32 {
    let idx_offset = (idx as f32) - (total as f32 / 2.0);
    (idx_offset + 0.5) * STRIP_W2
}

pub fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_released(key)
        .build()
        .unwrap();
    Model::default()
}

fn key(_app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            _model.frequency = Model::random();
            _model.palette = Model::new_palette(_model.frequency as usize);
        }
        Key::S => {
            _model
                .capture_next_frame
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
        _ => {}
    }
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app
        .draw()
        .translate(vec3(-(WIDTH as f32) / 2.0, 0.0, 0.0))
        // .scale_y(-1.0)
        ;

    draw.background().color(WHITESMOKE);
    model.draw(app, &draw);

    draw.to_frame(app, &frame).unwrap();

    if model
        .capture_next_frame
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        model
            .capture_next_frame
            .store(false, std::sync::atomic::Ordering::Relaxed);
        app.main_window()
            .capture_frame(captured_frame_path(app, &frame));
    }
}
