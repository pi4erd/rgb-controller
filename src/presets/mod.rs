mod default_blinks;
mod cycling;

use openrgb::data::Color;
use std::{collections::HashMap, fmt::Display};

pub use default_blinks::DefaultBlinks;
pub use cycling::Cycling;

pub fn all_presets() -> Vec<Box<dyn PixelFunction>> {
    vec![
        Box::new(DefaultBlinks::default()),
        Box::new(Cycling::default()),
    ]
}

pub trait PixelFunction: Send + 'static {
    fn init(&mut self, config: &HashMap<String, toml::Value>);
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
