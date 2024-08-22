mod default_blinks;
mod cycling;

use openrgb::data::Color;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

pub use default_blinks::DefaultBlinks;
pub use cycling::Cycling;

pub fn all_presets() -> Vec<Box<dyn PixelFunction>> {
    vec![
        Box::new(DefaultBlinks::default()),
        Box::new(Cycling::default()),
    ]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FunctionConfigField {
    Str(String),
    Int(i64),
    Float(f64),
}

#[derive(Deserialize, Serialize)]
pub struct FunctionConfig {
    pub is_default: bool,
    pub config_map: HashMap<String, FunctionConfigField>,
}

impl Default for FunctionConfig {
    fn default() -> Self {
        Self { is_default: true, config_map: HashMap::new() }
    }
}

pub trait PixelFunction: Send + 'static {
    fn init(&mut self, config: &FunctionConfig);
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
