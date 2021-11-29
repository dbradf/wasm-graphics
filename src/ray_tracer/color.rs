use auto_ops::*;
use serde::Deserialize;

use super::util::f64_is_equal;


#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: u8,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn as_u8_array(&self) -> [u8; 4] {
        [
            channel_to_u8(self.r), 
            channel_to_u8(self.g), 
            channel_to_u8(self.b), 
            self.a,
            ]
    }
}

fn channel_to_u8(channel: f64) -> u8 {
    let mut value = channel;
    if value > 1.0 {
        value = 1.0;
    } else if value < 0.0 {
        value = 0.0;
    }
    (255.0 * value) as u8
}

impl_op_ex!(+ |lhs: &Color, rhs: &Color| -> Color {
    Color {
            r: lhs.r + rhs.r,
            g: lhs.g + rhs.g,
            b: lhs.b + rhs.b,
            a: lhs.a,
        }
});
impl_op_ex!(- |lhs: &Color, rhs: &Color| -> Color {
    Color {
            r: lhs.r - rhs.r,
            g: lhs.g - rhs.g,
            b: lhs.b - rhs.b,
            a: lhs.a,
        }
});
impl_op_ex!(* |lhs: &Color, rhs: &Color| -> Color {
    Color {
            r: lhs.r * rhs.r,
            g: lhs.g * rhs.g,
            b: lhs.b * rhs.b,
            a: lhs.a,
        }
});
impl_op_ex!(* |lhs: &Color, rhs: &f64| -> Color {
    Color {
            r: lhs.r * rhs,
            g: lhs.g * rhs,
            b: lhs.b * rhs,
            a: lhs.a,
        }
});

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        f64_is_equal(self.r, other.r) &&
        f64_is_equal(self.g, other.g) &&
        f64_is_equal(self.b, other.b) &&
        self.a == other.a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_multiplying_a_color_by_a_scaler() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
