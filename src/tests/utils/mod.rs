use bevy::prelude::*;


pub fn vec_len_sum(vecs: impl IntoIterator<Item = Vec2>) -> f32 {
    let mut sum = 0.0;

    for vec in vecs.into_iter() {
        sum += vec.length();
    }

    sum
}

pub fn get_weight(min: f32, max: f32, val: f32) -> f32 {
    (val - min) / (max - min)
}

pub fn color_linear_interpolation(from: Color, to: Color, weight: f32) -> Color {
    let weight = weight.clamp(0.0, 1.0);

    Color::Rgba {
        red: to.r() * weight + from.r() * (1.0 - weight),
        green: to.g() * weight + from.g() * (1.0 - weight),
        blue: to.b() * weight + from.b() * (1.0 - weight),
        alpha: to.a() * weight + from.a() * (1.0 - weight),
    }
}
