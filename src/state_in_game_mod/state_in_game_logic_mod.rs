use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_kira_audio::AudioControl;

use crate::{
    AppState,
    state_in_game_mod::{BOARD_HEIGHT, BOARD_WIDTH, Bird, DebugText, Direction, OTHER_Z_LAYER, SnakeHead, SnakeSegment},
};

// fixed time every 0.5 seconds
pub fn move_snake_head(mut snake_query: Query<&mut SnakeHead>) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        snake_head.last_direction = snake_head.direction.clone();
        snake_head.direction = snake_head.new_direction.clone();

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
pub fn check_dead(
    mut snake_query: Query<&mut SnakeHead>,
    segment_query: Query<&mut SnakeSegment>,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    audio: Res<bevy_kira_audio::Audio>,
) {
    if let Ok(snake_head) = snake_query.single_mut() {
        if snake_head.position.x < 0 || snake_head.position.y < 0 || snake_head.position.x >= BOARD_WIDTH || snake_head.position.y >= BOARD_HEIGHT {
            next_state.set(AppState::Dead);
            audio.play(asset_server.load("game_over.mp3"));
        }
        for segment in segment_query {
            if segment.position == snake_head.position {
                next_state.set(AppState::Dead);
                audio.play(asset_server.load("game_over.mp3"));
                break;
            }
        }
    }
}

pub fn eat_bird(
    _time: Res<Time>,
    mut snake_query: Query<&mut SnakeHead>,
    mut bird_query: Query<&mut Bird>,
    mut debug_text_query: Query<&mut DebugText>,
    asset_server: Res<AssetServer>,
    audio: Res<bevy_kira_audio::Audio>,
) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        if let Ok(mut bird) = bird_query.single_mut() {
            if snake_head.position == bird.position {
                audio.play(asset_server.load("bird_chirp.mp3"));

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
pub fn move_segments(mut commands: Commands, mut snake_query: Query<&mut SnakeHead>, mut segment_query: Query<&mut SnakeSegment>, asset_server: Res<AssetServer>) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        let segment_len = segment_query.iter().len();

        // Sort according to `usize index`.
        let mut sorted_snake_segments: Vec<_> = segment_query.iter_mut().sort_by::<&SnakeSegment>(|value_1, value_2| value_1.index.cmp(&value_2.index)).collect();

        for snake_segment in sorted_snake_segments.iter_mut() {
            snake_segment.index += 1;
        }

        // the last segment becomes the first (zero) to avoid new spawn()
        let new_first_segment = sorted_snake_segments.last_mut().unwrap();
        // clone the old values, they will be used to make the tail longer if eating
        let last_segment_clone = new_first_segment.clone();

        new_first_segment.index = 0;
        new_first_segment.position = snake_head.last_position.clone();
        new_first_segment.direction = snake_head.direction.clone();
        new_first_segment.last_direction = snake_head.last_direction.clone();

        new_first_segment.updated = true;

        // I will use the last_segment_clone to spawn the new segment, index is segment_len
        if snake_head.just_eating {
            snake_head.just_eating = false;

            let rotation = match last_segment_clone.direction {
                Direction::Up => Quat::from_rotation_z(-PI * 0.5),
                Direction::Right => Quat::from_rotation_z(PI),
                Direction::Down => Quat::from_rotation_z(PI * 0.5),
                Direction::Left => Quat::from_rotation_z(0.),
            };
            commands.spawn((
                StateScoped(AppState::InGame),
                Sprite::from_image(asset_server.load("segment_tail.png")),
                Transform::from_xyz(last_segment_clone.position.to_bevy_x(), last_segment_clone.position.to_bevy_y(), OTHER_Z_LAYER).with_rotation(rotation),
                SnakeSegment {
                    position: last_segment_clone.position.clone(),
                    index: segment_len,
                    direction: last_segment_clone.direction.clone(),
                    last_direction: last_segment_clone.last_direction.clone(),
                    updated: false,
                },
            ));
        }
    }
}
