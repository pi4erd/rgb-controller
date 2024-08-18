use openrgb::data::Color;
use rand::{rngs::OsRng, Rng};

use crate::shared::*;

use super::PixelFunction;

pub struct DefaultBlinks {
    colors: [Color; Self::POINT_COUNT],
    delay_counter: usize,
    rng: OsRng,
}

impl DefaultBlinks {
    const POINT_COUNT: usize = 360;
    const STAR_DELAY: usize = 2;
}

impl Default for DefaultBlinks {
    fn default() -> Self {
        Self {
            colors: [Color::new(1, 0, 0); 360],
            delay_counter: Self::STAR_DELAY,
            rng: OsRng {},
        }
    }
}

impl PixelFunction for DefaultBlinks {
    fn init(&mut self) {
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
        "Default Blinks"
    }

    fn update(&mut self, screen: &mut [Color]) {
        for i in 0..screen.len() {
            screen[i] = lerp_color(screen[i], self.colors[0..screen.len()][i], 0.02);
        }

        if self.delay_counter == 0 {
            self.delay_counter = Self::STAR_DELAY;
            let gen_idx = self.rng.gen_range(0..screen.len());
            screen[gen_idx] = invert_color(screen[gen_idx]);
        }

        self.colors.rotate_right(1);

        self.delay_counter -= 1;
    }
}
