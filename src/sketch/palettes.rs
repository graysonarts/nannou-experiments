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
