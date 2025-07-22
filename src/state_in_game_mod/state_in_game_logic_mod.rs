use bevy::prelude::*;

use crate::state_in_game_mod::{BOARD_HEIGHT, BOARD_WIDTH, Bird, DebugText, Direction, OTHER_Z_LAYER, SnakeHead, SnakeSegment};

// fixed time every 0.5 seconds
pub fn move_snake_head(mut snake_query: Query<&mut SnakeHead>) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        if !snake_head.dead {
            snake_head.last_position = snake_head.position.clone();
            match &snake_head.direction {
                Direction::Up => snake_head.position.y -= 1,
                Direction::Down => snake_head.position.y += 1,
                Direction::Left => snake_head.position.x -= 1,
                Direction::Right => snake_head.position.x += 1,
            }
            snake_head.updated = true;
        }
    }
}

// this is executed after snake_head_move
pub fn check_dead(mut snake_query: Query<&mut SnakeHead>, segment_query: Query<&mut SnakeSegment>) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        if !snake_head.dead {
            if snake_head.position.x < 0 || snake_head.position.y < 0 || snake_head.position.x >= BOARD_WIDTH || snake_head.position.y >= BOARD_HEIGHT {
                snake_head.dead = true;
            }
            for segment in segment_query {
                if segment.position == snake_head.position {
                    snake_head.dead = true;
                    break;
                }
            }
        }
    }
}

pub fn eat_bird(_time: Res<Time>, mut snake_query: Query<&mut SnakeHead>, mut bird_query: Query<&mut Bird>, mut debug_text_query: Query<&mut DebugText>) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        if !snake_head.dead {
            if let Ok(mut bird) = bird_query.single_mut() {
                if snake_head.position == bird.position {
                    snake_head.just_eating = true;
                    // food: point, longer body
                    // new random position
                    bird.position.x = ops::floor(js_sys::Math::random() as f32 * BOARD_WIDTH as f32) as i32;
                    bird.position.y = ops::floor(js_sys::Math::random() as f32 * BOARD_HEIGHT as f32) as i32;
                    bird.updated = true;
                    if let Ok(mut debug_text) = debug_text_query.single_mut() {
                        debug_text.bird_position = format!("{:?}", &bird.position);
                    }
                }
            }
        }
    }
}

/// first segment is after the snake head
pub fn move_segments(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_query: Query<&mut SnakeHead>,
    mut segment_query: Query<&mut SnakeSegment>,
) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        if !snake_head.dead {
            // Sort according to `usize index`.
            let sorted_snake_segments = segment_query.iter_mut().sort_by::<&SnakeSegment>(|value_1, value_2| value_1.index.cmp(&value_2.index));
            //move position from segment to segment
            let mut last_position = snake_head.last_position.clone();
            for mut snake_segment in sorted_snake_segments {
                let prev_pos = snake_segment.position.clone();
                snake_segment.position = last_position;
                last_position = prev_pos;
                snake_segment.updated = true;
            }
            // I will use the last_position to spawn the new segment - tail
            if snake_head.just_eating {
                snake_head.just_eating = false;

                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
                    Transform::from_xyz(last_position.to_bevy_x(), last_position.to_bevy_y(), OTHER_Z_LAYER),
                    MeshMaterial2d(materials.add(Color::hsl(250., 0.95, 0.7))),
                    SnakeSegment {
                        position: last_position.clone(),
                        index: snake_head.segment_len,
                        updated: true,
                    },
                ));
                snake_head.segment_len += 1;
            }
        }
    }
}
