#![allow(dead_code)]
#![allow(unused)]
use crate::sketch::{HEIGHT, WIDTH};
use nannou::{app::ModelFn, App};

mod sketch;
mod utils;

macro_rules! map_range {
    ($value:expr, $from_min:expr, $from_max:expr, $to_min:expr, $to_max:expr) => {
        if $from_min == $from_max {
            ($to_max + $to_min) / 2
        } else {
            map_range($value, $from_min, $from_max, $to_min, $to_max)
        }
    };
}
pub fn height() -> u32 {
    #[cfg(feature = "lazy_height")]
    {
        *HEIGHT
    }
    #[cfg(not(feature = "lazy_height"))]
    {
        HEIGHT
    }
}

fn model_wrapper(app: &App) -> sketch::Model {
    #[cfg(feature = "once")]
    {
        app.set_loop_mode(nannou::app::LoopMode::NTimes {
            number_of_updates: 1,
        });
    }
    sketch::model(app)
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    #[cfg(feature = "simple")]
    {
        nannou::app(model_wrapper)
            .update(sketch::update)
            .size(WIDTH, height())
            .simple_window(sketch::view)
            .run();
    }
    #[cfg(not(feature = "simple"))]
    {
        nannou::app(model_wrapper).update(sketch::update).run();
    }
}
