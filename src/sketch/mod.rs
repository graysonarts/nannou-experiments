use nannou::prelude::*;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 800;

pub struct Model {}

impl Default for Model {
    fn default() -> Self {
        Self {}
    }
}

pub fn model(app: &App) -> Model {
    Model::default()
}

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

pub fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app
        .draw()
        .translate(vec3(-(WIDTH as f32) / 2.0, (HEIGHT as f32) / 2.0, 0.0))
        .scale_y(-1.0);

    draw.to_frame(app, &frame).unwrap();
}
