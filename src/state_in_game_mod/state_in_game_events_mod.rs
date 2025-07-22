use bevy::prelude::*;

use crate::{
    AppState,
    state_in_game_mod::{Direction, SnakeHead},
};

pub fn handle_movement_input(keys: Res<ButtonInput<KeyCode>>, mut queried_entities: Query<&mut SnakeHead>, mut next_state: ResMut<NextState<AppState>>) {
    if let Ok(mut snake_head) = queried_entities.single_mut() {
        if keys.pressed(KeyCode::ArrowUp) && snake_head.direction != Direction::Down {
            snake_head.direction = Direction::Up;
        } else if keys.pressed(KeyCode::ArrowDown) && snake_head.direction != Direction::Up {
            snake_head.direction = Direction::Down;
        } else if keys.pressed(KeyCode::ArrowLeft) && snake_head.direction != Direction::Right {
            snake_head.direction = Direction::Left;
        } else if keys.pressed(KeyCode::ArrowRight) && snake_head.direction != Direction::Left {
            snake_head.direction = Direction::Right;
        } else if keys.pressed(KeyCode::KeyX) {
            debug!("keys.pressed X");
            next_state.set(AppState::MainMenu);
        }
    }
}
