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
