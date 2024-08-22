use std::collections::HashMap;

use openrgb::data::Color;

use crate::shared::*;

use super::PixelFunction;

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

// macro_rules! config_value {
//     ($config:expr, $name:ident, $typ:ty, $default:expr) => {
//         let $name = {
//             let tmp = $config.config_map.get(stringify!($name))
//                 .unwrap_or(&$default);
//             if let $typ(val) = tmp {
//                 val
//             } else {
//                 panic!("Value for {} was of incorrect type ({})", stringify!($name), stringify!($typ))
//             }
//         };
//     };
// }

impl PixelFunction for Cycling {
    fn init(&mut self, config: &HashMap<String, toml::Value>) {
        // TODO: Make this a macro somehow
        let wheel_intensity = {
            let tmp = config.get("wheel_intensity").unwrap_or(&toml::Value::Float(1.0));
            if let toml::Value::Float(val) = tmp {
                *val
            } else {
                panic!("Value for 'wheel_intensity' was of incorrect type.");
            }
        };
        for i in 0..self.colors.len() {
            let (r, g, b) = hsv::hsv_to_rgb(
                mapf01(i as f64, 0.0, (self.colors.len() - 1) as f64) * 360.0 * wheel_intensity % 360.0,
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
