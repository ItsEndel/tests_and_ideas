use bevy::prelude::*;

use super::{utils::vec_len_sum, gradient::Gradient};



#[derive(Component)]
pub struct Trail {
    pub length: f32,
    points: Vec<Vec2>,

    pub gradient: Gradient,

    last_pos: Vec2,
}

impl Trail {
    pub fn new(length: f32, gradient: Gradient) -> Self {
        Self {
            length,
            points: Vec::new(),

            gradient,

            last_pos: Vec2::ZERO,
        }
    }
}

impl Default for Trail {
    fn default() -> Self {
        Self {
            length: 1000.0,
            points: Vec::new(),

            gradient: Gradient::default(),

            last_pos: Vec2::ZERO,
        }
    }
}

impl Trail {
    pub fn setup_trail_test(app: &mut App) {
        app.add_systems(Update, (
            | // Trail System
            mut trails: Query<(&mut Trail, &Transform)>,

            mut gizmos: Gizmos
            | {
                for (mut trail, transform) in &mut trails {
                    let position = Vec2::new(transform.translation.x, transform.translation.y);
                    let moved = position - trail.last_pos;

                    let len_sum = vec_len_sum(trail.points.clone());

                    let mut positions: Vec<(Vec2, Color)> = Vec::new();
                    let mut points: Vec<Vec2> = Vec::new();
                    let mut length: f32 = 0.0;

                    for point in &trail.points {
                        length += point.length();

                        if length <= trail.length {
                            let point = point.clone() - moved;

                            positions.push((point + position, trail.gradient.get_color(length / len_sum)));
                            points.push(point);

                            continue;
                        }

                        break;
                    }

                    if moved != Vec2::ZERO {
                        positions.insert(0, (position, trail.gradient.get_color(length / len_sum)));
                        points.insert(0, Vec2::ZERO);
                    } else if positions.len() > 0 {
                        positions.remove(positions.len() - 1);
                        points.remove(points.len() - 1);
                    }

                    gizmos.linestrip_gradient_2d(positions);

                    trail.last_pos = position.clone();
                    trail.points = points;
                }
            }, // Trail System

            | // Trail Debug
            mut trail_transforms: Query<&mut Transform, With<Trail>>,

            windows: Query<&Window>,
            | {
                let window = windows.single();
                
                if let Some(cursor_position) = window.cursor_position() {
                    for mut transform in &mut trail_transforms {
                        transform.translation.x = cursor_position.x - (window.resolution.width() / 2.0);
                        transform.translation.y = -cursor_position.y + (window.resolution.height() / 2.0);
                    }
                }
            } // Trail Debug
        ));

        app.add_systems(Startup,
            |mut commands: Commands| {
                commands.spawn((
                    TransformBundle::default(),
                    Trail::new(1000.0, Gradient::new(vec![
                        (0.0, Color::rgba(1.0, 0.0, 1.0, 1.0)),
                        (1.0, Color::rgba(0.0, 1.0, 1.0, 0.0)),
                    ])),
                ));
            }
        );
    }
}
