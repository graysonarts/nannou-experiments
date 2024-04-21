use nannou::{
    geom::Point2,
    rand::{random_f32, random_range},
    App, Frame,
};

pub struct RandomStepRange {
    end: i32,
    max_step: i32,
    value: i32,
    step: i32,
}

impl RandomStepRange {
    pub fn new(start: i32, end: i32, max_step: i32) -> Self {
        Self {
            end,
            max_step,
            step: 1,
            value: start,
        }
    }
}

impl Iterator for RandomStepRange {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.value;
        if self.value >= self.end {
            return None;
        }

        let next_step = random_range(1, self.max_step);
        self.step = if return_value + next_step > self.end {
            self.end - return_value
        } else {
            next_step
        };

        self.value += self.step;

        Some((return_value, self.step))
    }
}

pub fn random_step(start: i32, end: i32, step: i32) -> i32 {
    let rnd = random_f32();
    let size = (end - start) as f32 * rnd;
    (size / step as f32).round() as i32 * step + start
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    t * a + (1.0 - t) * b
}

pub fn lerp_points(a: &Point2, b: &Point2, t: f32) -> Point2 {
    let x = lerp(a.x, b.x, t);
    let y = lerp(a.y, b.y, t);

    Point2::new(x, y)
}

pub fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    capture_directory(app)
        // Name each file after the number of the frame.
        .join(format!("{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}

pub fn capture_directory(app: &App) -> std::path::PathBuf {
    let code = artcode::BatDate::default();
    app.project_path()
        .expect("failed to locate `project_path`")
        .join(app.exe_name().unwrap())
        .join(code.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_finishes() {
        let results: Vec<_> = RandomStepRange::new(0, 10, 4).collect();
        assert_eq!(results, vec![])
    }
}
