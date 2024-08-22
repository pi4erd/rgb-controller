use openrgb::data::Color;

use crate::shared::*;

use super::{FunctionConfig, PixelFunction};

pub struct Cycling {
    colors: [Color; Self::POINT_COUNT],
}

impl Cycling {
    const POINT_COUNT: usize = 360;
}

impl Default for Cycling {
    fn default() -> Self {
        Self {
            colors: [Color::new(1, 0, 0); 360],
        }
    }
}

impl PixelFunction for Cycling {
    fn init(&mut self, config: &FunctionConfig) {
        for i in 0..self.colors.len() {
            let (r, g, b) = hsv::hsv_to_rgb(
                mapf01(i as f64, 0.0, (self.colors.len() - 1) as f64) * 360.0,
                1.0,
                0.5,
            );
            self.colors[i] = Color::new(r, g, b);
        }
    }

    fn name(&self) -> &'static str {
        "Cycling Colors"
    }

    fn update(&mut self, screen: &mut [Color]) {
        for i in 0..screen.len() {
            screen[i] = lerp_color(screen[i], self.colors[0..screen.len()][i], 0.02);
        }

        self.colors.rotate_right(1);
    }
}
