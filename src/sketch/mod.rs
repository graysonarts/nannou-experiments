use std::{
    os::unix::thread,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    usize::MAX,
};

use nannou::{
    color::{self, Gradient, IntoColor, IntoLinSrgba},
    draw,
    prelude::*,
    winit::event::StartCause,
};

use crate::utils::{captured_frame_path, lerp};

use self::walker::{
    moveablew::{MoveableWalker, ENERGETIC},
    staticw::ATTRACTOR,
    Walker,
};
mod palettes;
mod walker;

pub const WIDTH: u32 = 1200 / 2;
pub const HEIGHT: u32 = WIDTH * 9 / 16;

pub const STRIP_W: f32 = 100.0;
pub const STRIP_W2: f32 = STRIP_W * 2.0;

pub const MAX_FREQ: f32 = 10.0;

pub const MAX_PATH_LENGTH: usize = 10000;
pub const MIN_WALKERS: usize = 3;
pub const MAX_WALKERS: usize = 50;

pub struct Model {
    walkers: Vec<Walker>,
    ticks: u64,
    capture_next_frame: AtomicBool,
}

static RENDER_TICKS: AtomicU64 = AtomicU64::new(0);

fn should_render(model: &Model) -> bool {
    let render_ticks = RENDER_TICKS.load(Ordering::SeqCst);
    let should_render = model.ticks != render_ticks;
    if should_render {
        RENDER_TICKS.fetch_add(model.ticks - render_ticks, Ordering::SeqCst);
    }
    should_render
}

fn render(model: &mut Model) {
    model.ticks += 1;
}

impl Default for Model {
    fn default() -> Self {
        let mut ret = Self {
            ticks: 10,
            walkers: Vec::new(),
            capture_next_frame: AtomicBool::new(false),
        };
        ret.reset(random_range::<usize>(MIN_WALKERS, MAX_WALKERS));

        ret
    }
}

impl Model {
    fn reset(&mut self, size: usize) {
        let palette = Self::new_palette(size);
        let mut walkers: Vec<_> = palette
            .iter()
            .enumerate()
            .map(|(idx, color)| {
                Walker::Moveable(MoveableWalker::new(
                    idx,
                    color.into_lin_srgba(),
                    Point2::new(
                        random_range(0.0, WIDTH as f32),
                        // random_range(-(HEIGHT as f32 / 2.0), HEIGHT as f32 / 2.0),
                        map_range(idx, 0, size, HEIGHT as f32 / -2.0, HEIGHT as f32 / 2.0),
                    ),
                    Some(Vec2::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0))),
                ))
            })
            .collect();
        // walkers.push(ATTRACTOR.clone());
        self.walkers = walkers;
    }
    fn new_palette(count: usize) -> Vec<Hsl> {
        let palette = palettes::split_complement();
        palette.take(count).collect()
    }
}

impl Model {
    pub fn draw(&self, app: &App, draw: &Draw) {}
}

pub fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH * 2, HEIGHT * 2)
        .view(view)
        .key_released(key)
        .build()
        .unwrap();
    Model::default()
}

fn key(_app: &App, _model: &mut Model, key: Key) {
    render(_model);
    match key {
        Key::Space => {
            _model.reset(random_range::<usize>(MIN_WALKERS, MAX_WALKERS));
        }
        Key::S => {
            _model
                .capture_next_frame
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
        _ => {}
    }
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    // if app.elapsed_frames() % 60 != 0 {
    //     return;
    // }

    model.walkers.iter_mut().for_each(|walker| {
        walker.start_rebound();
    });

    let walker_count = model.walkers.len();

    for i in 0..walker_count {
        for j in i + 1..walker_count {
            let repel_force = model.walkers[i].rebound(&model.walkers[j]);

            model.walkers[i].apply_force(-repel_force);
            model.walkers[j].apply_force(repel_force * ENERGETIC);
        }
    }

    model.walkers.iter_mut().for_each(|walker| {
        walker.update(app, update);
    });
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let capture_frame = model
        .capture_next_frame
        .load(std::sync::atomic::Ordering::Relaxed);
    let draw = app
        .draw()
        .translate(vec3(-(WIDTH as f32) / 2.0, 0.0, 0.0))
        // .scale_y(-1.0)
        ;

    draw.background().color(WHITESMOKE);
    model.walkers.iter().enumerate().for_each(|(idx, walker)| {
        walker.draw(app, &draw, idx, !capture_frame);
    });

    // if (!capture_frame) {
    //     let walker_count = model.walkers.len();

    //     for i in 0..walker_count {
    //         for j in i + 1..walker_count {
    //             draw.line()
    //                 .start(model.walkers[i].position())
    //                 .end(model.walkers[j].position())
    //                 .color(BLACK);
    //         }
    //     }
    // }

    draw.to_frame(app, &frame).unwrap();

    if capture_frame {
        model
            .capture_next_frame
            .store(false, std::sync::atomic::Ordering::Relaxed);
        app.main_window()
            .capture_frame(captured_frame_path(app, &frame));
    }
}
