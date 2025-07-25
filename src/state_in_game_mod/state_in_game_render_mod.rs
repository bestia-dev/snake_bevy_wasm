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
        // the last segment is the tail
        if segment.index == segment_len - 1 {
            sprite.image = asset_server.load("segment_tail.png");
        }
        if segment.updated {
            transform.translation.x = segment.position.to_bevy_x();
            transform.translation.y = segment.position.to_bevy_y();

            if segment.direction != segment.last_direction {
                sprite.image = asset_server.load("segment_corner.png");
            } else {
                sprite.image = asset_server.load("segment_horizontal.png");
            }
            rotate_segment(&segment, transform);
            segment.updated = false;
        }
    }
}

// corner can be clockwise and counterclockwise
fn rotate_segment(segment: &Mut<'_, SnakeSegment>, mut transform: Mut<'_, Transform>) {
    let forward = Vec3 {
        x: segment.position.to_bevy_x(),
        y: segment.position.to_bevy_y(),
        z: f32::INFINITY,
    };

    match segment.direction {
        Direction::Up => {
            transform.look_at(forward, Vec3::X);
            // if anti-clockwise
            if segment.last_direction == Direction::Right {
                transform.rotate_y(PI);
            }
        }
        Direction::Right => {
            transform.look_at(forward, -Vec3::Y);
            // if anti-clockwise
            if segment.last_direction == Direction::Down {
                transform.rotate_x(PI);
            }
        }
        Direction::Down => {
            transform.look_at(forward, -Vec3::X);
            // if anti-clockwise
            if segment.last_direction == Direction::Left {
                transform.rotate_y(PI);
            }
        }
        Direction::Left => {
            transform.look_at(forward, Vec3::Y);
            // if anti-clockwise
            if segment.last_direction == Direction::Up {
                transform.rotate_x(PI);
            }
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
