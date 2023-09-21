use bevy::prelude::*;

use super::utils::{color_linear_interpolation, get_weight};



pub struct Gradient {
    positions: Vec<(f32, Color)>
}

impl Gradient {
    pub fn new(positions: Vec<(f32, Color)>) -> Self {
        Self { positions }
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            positions: vec![
                (0.0, Color::WHITE),
                (1.0, Color::BLACK)
            ]
        }
    }
}

impl Gradient {
    pub fn get_color(&self, weight: f32) -> Color {
        if self.positions.len() == 0 {
            return Color::default();
        }

        let weight = weight.clamp(0.0, 1.0);

        let mut last_pos = self.positions[0];
        for (color_weight, color) in self.positions.clone() {
            if weight <= color_weight {
                return color_linear_interpolation(
                    last_pos.1,
                    color,
                    get_weight(last_pos.0, color_weight, weight)
                )
            }

            last_pos = (color_weight, color);
        }

        last_pos.1
    }
}
