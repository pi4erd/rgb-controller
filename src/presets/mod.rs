mod default_blinks;

use std::fmt::Display;

pub use default_blinks::DefaultBlinks;
use openrgb::data::Color;

pub trait PixelFunction {
    fn init(&mut self);
    fn name(&self) -> &'static str {
        "Unnamed"
    }
    fn update(&mut self, screen: &mut [Color]);
}

impl Display for dyn PixelFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
