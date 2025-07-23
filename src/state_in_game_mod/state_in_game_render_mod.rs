use std::f32::consts::PI;

use bevy::prelude::*;

use crate::state_in_game_mod::{Bird, DebugText, Direction, PointsText, SnakeHead, SnakeSegment};

pub fn render_snake_head(mut snake_head_query: Query<(&mut SnakeHead, &mut Transform)>) {
    if let Ok((mut snake_head, mut transform)) = snake_head_query.single_mut() {
        if snake_head.updated {
            transform.translation.x = snake_head.position.to_bevy_x();
            transform.translation.y = snake_head.position.to_bevy_y();

            // rotate and/or flip head
            // rotate_x is flip
            match snake_head.direction {
                Direction::Up => match snake_head.last_direction {
                    Direction::Up => (),
                    Direction::Right => {
                        transform.rotate_x(PI);
                        transform.rotate_z(PI * 0.5);
                    }
                    Direction::Down => transform.rotate_z(PI),
                    Direction::Left => transform.rotate_z(-PI * 0.5),
                },
                Direction::Right => match snake_head.last_direction {
                    Direction::Up => {
                        transform.rotate_z(-PI * 0.5);
                        transform.rotate_x(PI);
                    }
                    Direction::Right => (),
                    Direction::Down => {
                        transform.rotate_z(PI * 0.5);
                        transform.rotate_x(PI);
                    }
                    Direction::Left => {
                        transform.rotate_z(PI);
                        transform.rotate_x(PI);
                    }
                },
                Direction::Down => match snake_head.last_direction {
                    Direction::Up => transform.rotate_z(PI),
                    Direction::Right => {
                        transform.rotate_x(PI);
                        transform.rotate_z(-PI * 0.5);
                    }
                    Direction::Down => (),
                    Direction::Left => transform.rotate_z(PI * 0.5),
                },
                Direction::Left => match snake_head.last_direction {
                    Direction::Up => transform.rotate_z(PI * 0.5),
                    Direction::Right => {
                        transform.rotate_x(PI);
                        transform.rotate_z(PI);
                    }
                    Direction::Down => transform.rotate_z(-PI * 0.5),
                    Direction::Left => (),
                },
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

pub fn render_segment(mut queried_entities: Query<(&mut SnakeSegment, &mut Transform)>) {
    for (mut segment, mut transform) in queried_entities.iter_mut() {
        if segment.updated {
            transform.translation.x = segment.position.to_bevy_x();
            transform.translation.y = segment.position.to_bevy_y();

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
