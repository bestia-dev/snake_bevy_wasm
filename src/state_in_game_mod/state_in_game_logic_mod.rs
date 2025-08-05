use bevy::prelude::*;
use bevy_kira_audio::AudioControl;

use crate::{
    AppState, BOARD_HEIGHT, BOARD_WIDTH, GameBoardCanvas,
    state_in_game_mod::{Bird, DebugText, Direction, OTHER_Z_LAYER, SnakeHead, SnakeSegment, SnakeSegmentIndex},
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
        } else {
            for segment in segment_query {
                if segment.position == snake_head.position {
                    next_state.set(AppState::Dead);
                    audio.play(asset_server.load("game_over.mp3"));
                    break;
                }
            }
        }
    }
}

pub fn eat_bird(
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
    mut snake_query: Query<&mut SnakeHead>,
    mut segment_query: Query<(&mut SnakeSegment, &mut SnakeSegmentIndex)>,
    asset_server: Res<AssetServer>,
    game_board_canvas: Res<GameBoardCanvas>,
) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        // Sort according to `usize index`.
        let mut sorted_snake_segments: Vec<_> = segment_query
            .iter_mut()
            .sort_by::<(&SnakeSegment, &SnakeSegmentIndex)>(|value_1, value_2| value_1.1.index.cmp(&value_2.1.index))
            .collect();

        for snake_segment in sorted_snake_segments.iter_mut() {
            snake_segment.1.index += 1;
        }

        // if eating, spawn the new index zero, all rest of segments remain equal
        if snake_head.just_eating {
            snake_head.just_eating = false;

            commands.spawn((
                StateScoped(AppState::InGame),
                Sprite::from_image(asset_server.load("segment_horizontal.png")),
                Transform::from_xyz(
                    snake_head.last_position.to_bevy_x(&game_board_canvas),
                    snake_head.last_position.to_bevy_y(&game_board_canvas),
                    OTHER_Z_LAYER,
                ),
                SnakeSegment {
                    position: snake_head.last_position.clone(),
                    direction: snake_head.direction.clone(),
                    last_direction: snake_head.last_direction.clone(),
                    is_tail: false,
                },
                SnakeSegmentIndex { index: 0 },
            ));
        } else {
            // the last segment becomes the first (zero) to avoid new spawn()
            let new_first_segment = sorted_snake_segments.last_mut().unwrap();
            // new values for the new first segment
            new_first_segment.1.index = 0;
            new_first_segment.0.position = snake_head.last_position.clone();
            new_first_segment.0.direction = snake_head.direction.clone();
            new_first_segment.0.last_direction = snake_head.last_direction.clone();
            new_first_segment.0.is_tail = false;

            // the new last segment becomes the tail
            // The segments are always at least 2 elements.
            let segment_len = sorted_snake_segments.len();
            let new_last_segment = sorted_snake_segments.get_mut(segment_len - 2).unwrap();
            new_last_segment.0.is_tail = true;
        }
    }
}
