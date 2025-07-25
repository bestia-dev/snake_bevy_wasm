use core::f32;
use std::f32::consts::PI;

use bevy::prelude::*;

use crate::state_in_game_mod::{Bird, DebugText, Direction, PointsText, SnakeHead, SnakeSegment};

pub fn render_snake_head(mut snake_head_query: Query<(&mut SnakeHead, &mut Transform)>) {
    if let Ok((mut snake_head, mut transform)) = snake_head_query.single_mut() {
        if snake_head.updated {
            transform.translation.x = snake_head.position.to_bevy_x();
            transform.translation.y = snake_head.position.to_bevy_y();

            // TODO: hot to flip using look_at???
            // rotate and/or flip head
            match snake_head.direction {
                Direction::Up => {
                    transform.look_at(
                        Vec3 {
                            x: snake_head.position.to_bevy_x(),
                            y: snake_head.position.to_bevy_y(),
                            z: f32::INFINITY,
                        },
                        Vec3::X,
                    );
                }
                Direction::Right => {
                    transform.look_at(
                        Vec3 {
                            x: snake_head.position.to_bevy_x(),
                            y: snake_head.position.to_bevy_y(),
                            z: f32::INFINITY,
                        },
                        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
                    );
                    transform.rotate_x(PI);
                }
                Direction::Down => {
                    transform.look_at(
                        Vec3 {
                            x: snake_head.position.to_bevy_x(),
                            y: snake_head.position.to_bevy_y(),
                            z: f32::INFINITY,
                        },
                        -Vec3::X,
                    );
                }
                Direction::Left => {
                    transform.look_at(
                        Vec3 {
                            x: snake_head.position.to_bevy_x(),
                            y: snake_head.position.to_bevy_y(),
                            z: f32::INFINITY,
                        },
                        Vec3::Y,
                    );
                }
            };

            snake_head.updated = false;
        }
    }
}
pub fn render_bird(mut queried_entities: Query<(&mut Bird, &mut Transform)>) {
    if let Ok((mut bird, mut transform)) = queried_entities.single_mut() {
        if bird.updated {
            transform.translation.x = bird.position.to_bevy_x();
            transform.translation.y = bird.position.to_bevy_y();

            bird.updated = false;
        }
    }
}

pub fn render_segment(mut segment_query: Query<(&mut SnakeSegment, &mut Transform, &mut Sprite)>, asset_server: Res<AssetServer>) {
    let segment_len = segment_query.iter().len();
    for (mut segment, mut transform, mut sprite) in segment_query.iter_mut() {
        // how to change sprite if direction!=last_direction
        if segment.index == segment_len - 1 {
            sprite.image = asset_server.load("segment_tail.png");
        }
        if segment.updated {
            if segment.direction != segment.last_direction {
                sprite.image = asset_server.load("segment_corner.png");
            } else {
                sprite.image = asset_server.load("segment_horizontal.png");
            }

            transform.translation.x = segment.position.to_bevy_x();
            transform.translation.y = segment.position.to_bevy_y();
            match segment.direction {
                Direction::Up => {
                    transform.look_at(
                        Vec3 {
                            x: segment.position.to_bevy_x(),
                            y: segment.position.to_bevy_y(),
                            z: f32::INFINITY,
                        },
                        Vec3::X,
                    );
                }
                Direction::Right => {
                    transform.look_at(
                        Vec3 {
                            x: segment.position.to_bevy_x(),
                            y: segment.position.to_bevy_y(),
                            z: f32::INFINITY,
                        },
                        Vec3 { x: 0.0, y: -1.0, z: 0.0 },
                    );
                }
                Direction::Down => {
                    transform.look_at(
                        Vec3 {
                            x: segment.position.to_bevy_x(),
                            y: segment.position.to_bevy_y(),
                            z: f32::INFINITY,
                        },
                        -Vec3::X,
                    );
                }
                Direction::Left => {
                    transform.look_at(
                        Vec3 {
                            x: segment.position.to_bevy_x(),
                            y: segment.position.to_bevy_y(),
                            z: f32::INFINITY,
                        },
                        Vec3::Y,
                    );
                }
            }
            segment.updated = false;
        }
    }
}

pub fn render_debug_text(mut debug_text_query: Query<(&mut DebugText, &mut Text)>) {
    if let Ok((debug_text, mut text)) = debug_text_query.single_mut() {
        *text = Text::new(&debug_text.bird_position);
    }
}

pub fn render_points_text(snake_head_query: Query<&SnakeHead>, mut debug_text_query: Query<(&mut PointsText, &mut Text)>) {
    if let Ok(snake_head) = snake_head_query.single() {
        if let Ok((_points_text, mut text)) = debug_text_query.single_mut() {
            *text = Text::new(format!("Moves:{} Points:{}", snake_head.moves, snake_head.points));
        }
    }
}
