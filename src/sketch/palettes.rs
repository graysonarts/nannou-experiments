use nannou::{color::Gradient, prelude::*};

pub fn complement() -> Gradient<Hsl> {
    let start = hsl(random_f32(), 0.5, random_f32());
    let end = hsl(
        (start.hue.to_positive_degrees() + 180.0) / 360.0,
        0.5,
        random_f32() * 0.5 + 0.5,
    );
    Gradient::new(vec![start, end])
}

pub fn split_complement() -> Gradient<Hsl> {
    let start = hsl(random_f32(), 0.5, random_f32() * 0.5 + 0.5);
    let mid = hsl(
        (start.hue.to_positive_degrees() + 150.0) / 360.0,
        0.5,
        random_f32() * 0.5 + 0.5,
    );
    let end = hsl(
        (start.hue.to_positive_degrees() + 210.0) / 360.0,
        0.5,
        random_f32() * 0.5 + 0.5,
    );

    Gradient::new(vec![start, mid, end])
}
