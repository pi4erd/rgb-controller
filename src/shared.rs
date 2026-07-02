use openrgb::data::Color;

pub fn mapf01(n: f64, start: f64, finish: f64) -> f64 {
    (n - start) / (finish - start)
}

pub fn clampf(v: f64, a: f64, b: f64) -> f64 {
    if v > b {
        b
    } else if v < a {
        a
    } else {
        v
    }
}

pub fn lerp(a: f64, b: f64, k: f64) -> f64 {
    k * (b - a) + a
}

pub fn lerp_color(a: Color, b: Color, k: f64) -> Color {
    Color::new(
        clampf(lerp(a.r as f64, b.r as f64, k), 0.0, 255.0) as u8,
        clampf(lerp(a.g as f64, b.g as f64, k), 0.0, 255.0) as u8,
        clampf(lerp(a.b as f64, b.b as f64, k), 0.0, 255.0) as u8,
    )
}

pub fn invert_color(color: Color) -> Color {
    Color::new(255 - color.r, 255 - color.g, 255 - color.b)
}

/* For the below function, thanks to James Bromley:
 * MIT License
 
 Copyright (c) 2023 James Bromley
 
 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:
 
 The above copyright notice and this permission notice shall be included in all
 copies or substantial portions of the Software.
 
 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 SOFTWARE.
 */
// https://github.com/jayber/hsv/blob/main/src/lib.rs
pub fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> (u8, u8, u8) {
    fn is_between(value: f64, min: f64, max: f64) -> bool {
        min <= value && value < max
    }

    check_bounds(hue, saturation, value);

    let c = value * saturation;
    let h = hue / 60.0;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = value - c;

    let (r, g, b): (f64, f64, f64) = if is_between(h, 0.0, 1.0) {
        (c, x, 0.0)
    } else if is_between(h, 1.0, 2.0) {
        (x, c, 0.0)
    } else if is_between(h, 2.0, 3.0) {
        (0.0, c, x)
    } else if is_between(h, 3.0, 4.0) {
        (0.0, x, c)
    } else if is_between(h, 4.0, 5.0) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn check_bounds(hue: f64, saturation: f64, value: f64) {
    fn panic_bad_params(name: &str, from_value: &str, to_value: &str, supplied: f64) -> ! {
        panic!(
            "param {} must be between {} and {} inclusive; was: {}",
            name, from_value, to_value, supplied
        )
    }

    if !(0.0..=360.0).contains(&hue) {
        panic_bad_params("hue", "0.0", "360.0", hue)
    } else if !(0.0..=1.0).contains(&saturation) {
        panic_bad_params("saturation", "0.0", "1.0", saturation)
    } else if !(0.0..=1.0).contains(&value) {
        panic_bad_params("value", "0.0", "1.0", value)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::mapf01;

    #[test]
    fn lerp_test() {
        use crate::shared::lerp;

        assert_eq!(lerp(0.0, 1.0, 0.5), 0.5);
        assert_eq!(lerp(1.0, 0.0, 0.5), 0.5);
        assert_eq!(lerp(2.0, 3.0, 0.5), 2.5);
        assert_eq!(lerp(10.0, 0.0, 0.2), 8.0);
        assert_eq!(lerp(-10.0, 0.0, 0.2), -8.0);
    }

    #[test]
    fn invert_test() {
        use crate::shared::invert_color;
        use openrgb::data::Color;

        assert_eq!(invert_color(Color::new(0, 0, 0)), Color::new(255, 255, 255));
    }

    #[test]
    fn map_test() {
        assert_eq!(mapf01(25.0, 0.0, 100.0), 0.25);
        assert_eq!(mapf01(0.0, -100.0, 100.0), 0.5);
    }
}
