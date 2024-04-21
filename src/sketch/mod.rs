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

pub const STRIP_W: f32 = 400.0;
pub const STRIP_W2: f32 = STRIP_W * 2.0;

pub const MAX_FREQ: f32 = 3.0;

pub struct Model {
    frequency: f32,
    phases: Vec<f32>,
    speeds: Vec<f32>,
    palette: Vec<Hsl>,
    capture_next_frame: AtomicBool,
}

impl Default for Model {
    fn default() -> Self {
        let frequency = Self::random();
        let palette = Self::new_palette(frequency as usize);
        let phases = Self::phases(&palette);
        let speeds = Self::speeds(&palette);

        Self {
            phases,
            speeds,
            frequency,
            palette,
            capture_next_frame: AtomicBool::new(false),
        }
    }
}

impl Model {
    fn random() -> f32 {
        (random_f32() * MAX_FREQ / 2.0 + MAX_FREQ / 2.0).floor()
    }

    fn new_palette(count: usize) -> Vec<Hsl> {
        let palette = palettes::split_complement();
        palette.take(count).collect()
    }

    fn phases(palette: &[Hsl]) -> Vec<f32> {
        palette.iter().map(|_| random_f32()).collect()
    }

    fn speeds(palette: &[Hsl]) -> Vec<f32> {
        palette.iter().map(|_| random_f32() / 100.0).collect()
    }
}

impl Model {
    pub fn draw(&self, app: &App, draw: &Draw) {
        (0..1000).map(|i| (i as f32) / 1000.0).for_each(|t| {
            let x = lerp(0.0, WIDTH as f32, t);
            // TODO: Generate Palette
            self.palette
                .iter()
                .zip(self.phases.iter())
                .enumerate()
                .for_each(|(idx, (color, phase))| {
                    let y = ((t) * self.frequency * TAU).sin() * 100.0 * (t + *phase).cos();
                    let offset = calc_offset(self.palette.len(), idx);
                    // let color = Hsla::new(
                    //     color.hue,
                    //     color.saturation,
                    //     color.lightness,
                    //     (0.5 + (t * self.frequency * TAU + *phase).cos().abs()).clamp(0.25, 0.75),
                    // );
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
            _model.phases = Model::phases(&_model.palette);
            _model.speeds = Model::speeds(&_model.palette);
        }
        Key::S => {
            _model
                .capture_next_frame
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
        _ => {}
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model
        .phases
        .iter_mut()
        .zip(model.speeds.iter())
        .for_each(|(phase, speed)| {
            *phase += speed;
        });
}

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
