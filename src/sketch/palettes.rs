use nannou::{color::Gradient, prelude::*};

pub fn complement() -> Gradient<Hsl> {
    let sat = random_f32() * 0.5 + 0.5;
    let start = hsl(random_f32(), sat, random_f32() * 0.5 + 0.5);
    let end = hsl(
        (start.hue.to_positive_degrees() + 180.0) / 360.0,
        sat,
        random_f32() * 0.5 + 0.5,
    );
    Gradient::new(vec![start, end])
}

pub fn split_complement() -> Gradient<Hsl> {
    let sat = random_f32() * 0.5 + 0.5;
    let light = random_f32() * 0.5 + 0.25;

    let start = hsl(random_f32(), sat, light);
    let mid = hsl(
        (start.hue.to_positive_degrees() + 150.0) / 360.0,
        start.saturation,
        start.lightness * light,
    );
    let end = hsl(
        (start.hue.to_positive_degrees() + 210.0) / 360.0,
        start.saturation,
        start.lightness * light,
    );

    Gradient::new(vec![start, mid, end])
}
