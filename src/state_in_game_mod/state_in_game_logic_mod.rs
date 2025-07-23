use bevy::prelude::*;

use crate::{
    AppState,
    state_in_game_mod::{BOARD_HEIGHT, BOARD_WIDTH, Bird, DebugText, Direction, OTHER_Z_LAYER, SPRITE_HEIGHT, SPRITE_WIDTH, SnakeHead, SnakeSegment},
};

// fixed time every 0.5 seconds
pub fn move_snake_head(mut snake_query: Query<&mut SnakeHead>) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        snake_head.last_position = snake_head.position.clone();

        match &snake_head.direction {
            Direction::Up => snake_head.position.y -= 1,
            Direction::Down => snake_head.position.y += 1,
            Direction::Left => snake_head.position.x -= 1,
            Direction::Right => snake_head.position.x += 1,
        }
        snake_head.updated = true;
        snake_head.moves += 1;
    }
}

// this is executed after snake_head_move
pub fn check_dead(mut snake_query: Query<&mut SnakeHead>, segment_query: Query<&mut SnakeSegment>, mut next_state: ResMut<NextState<AppState>>) {
    if let Ok(snake_head) = snake_query.single_mut() {
        if snake_head.position.x < 0 || snake_head.position.y < 0 || snake_head.position.x >= BOARD_WIDTH || snake_head.position.y >= BOARD_HEIGHT {
            next_state.set(AppState::Dead);
        }
        for segment in segment_query {
            if segment.position == snake_head.position {
                next_state.set(AppState::Dead);
                break;
            }
        }
    }
}

pub fn eat_bird(_time: Res<Time>, mut snake_query: Query<&mut SnakeHead>, mut bird_query: Query<&mut Bird>, mut debug_text_query: Query<&mut DebugText>) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        if let Ok(mut bird) = bird_query.single_mut() {
            if snake_head.position == bird.position {
                snake_head.just_eating = true;
                snake_head.points += 1;
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

/// first segment is after the snake head
pub fn move_segments(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_query: Query<&mut SnakeHead>,
    mut segment_query: Query<&mut SnakeSegment>,
) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        // Sort according to `usize index`.
        let sorted_snake_segments = segment_query.iter_mut().sort_by::<&SnakeSegment>(|value_1, value_2| value_1.index.cmp(&value_2.index));
        //move position from segment to segment
        let mut position = snake_head.last_position.clone();
        let mut direction = snake_head.last_direction.clone();
        // dummy direction will be overwritten
        let mut last_direction = Direction::Down;
        let mut last_position;
        for mut snake_segment in sorted_snake_segments {
            last_position = snake_segment.position.clone();
            last_direction = snake_segment.direction.clone();

            snake_segment.position = position;
            snake_segment.direction = direction;
            snake_segment.last_direction = last_direction.clone();

            position = last_position;
            direction = last_direction.clone();
            snake_segment.updated = true;
        }
        // I will use the last_position to spawn the new segment - tail
        if snake_head.just_eating {
            snake_head.just_eating = false;

            commands.spawn((
                StateScoped(AppState::InGame),
                Mesh2d(meshes.add(Rectangle::new(SPRITE_WIDTH as f32, SPRITE_HEIGHT as f32))),
                Transform::from_xyz(position.to_bevy_x(), position.to_bevy_y(), OTHER_Z_LAYER),
                MeshMaterial2d(materials.add(Color::hsl(250., 0.95, 0.7))),
                SnakeSegment {
                    position: position.clone(),
                    index: snake_head.segment_len,
                    direction: direction.clone(),
                    last_direction: last_direction.clone(),
                    updated: true,
                },
            ));
            snake_head.segment_len += 1;
        }
    }
}
