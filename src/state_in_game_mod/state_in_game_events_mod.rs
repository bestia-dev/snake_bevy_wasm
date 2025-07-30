use bevy::prelude::*;

use crate::{
    AppState,
    state_in_game_mod::{Direction, SnakeHead},
};

pub fn handle_movement_input(keys: Res<ButtonInput<KeyCode>>, mut queried_entities: Query<&mut SnakeHead>, mut next_state: ResMut<NextState<AppState>>) {
    if let Ok(mut snake_head) = queried_entities.single_mut() {
        if (keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW)) && snake_head.direction != Direction::Down {
            snake_head.new_direction = Direction::Up;
        } else if (keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD)) && snake_head.direction != Direction::Left {
            snake_head.new_direction = Direction::Right;
        } else if (keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS)) && snake_head.direction != Direction::Up {
            snake_head.new_direction = Direction::Down;
        } else if (keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA)) && snake_head.direction != Direction::Right {
            snake_head.new_direction = Direction::Left;
        } else if keys.pressed(KeyCode::KeyX) {
            next_state.set(AppState::MainMenu);
        }
    }
}
