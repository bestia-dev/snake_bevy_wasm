use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    AppState,
    state_in_game_mod::{Direction, SnakeHead},
};

pub fn handle_movement_input(keys: Res<ButtonInput<KeyCode>>, mut queried_entities: Query<&mut SnakeHead>, mut next_state: ResMut<NextState<AppState>>) {
    if let Ok(mut snake_head) = queried_entities.single_mut() {
        if keys.pressed(KeyCode::ArrowUp) && snake_head.direction != Direction::Down {
            // old direction
            match snake_head.direction {
                Direction::Up => snake_head.rotate = 0.,
                Direction::Right => snake_head.rotate = PI * 0.5,
                Direction::Down => snake_head.rotate = PI,
                Direction::Left => snake_head.rotate = PI * 1.5,
            }
            snake_head.direction = Direction::Up;
            snake_head.updated = true;
        } else if keys.pressed(KeyCode::ArrowRight) && snake_head.direction != Direction::Left {
            // old direction
            match snake_head.direction {
                Direction::Up => snake_head.rotate = PI * 1.5,
                Direction::Right => snake_head.rotate = 0.,
                Direction::Down => snake_head.rotate = PI * 0.5,
                Direction::Left => snake_head.rotate = PI,
            }
            snake_head.direction = Direction::Right;
            snake_head.updated = true;
        } else if keys.pressed(KeyCode::ArrowDown) && snake_head.direction != Direction::Up {
            // old direction
            match snake_head.direction {
                Direction::Up => snake_head.rotate = PI,
                Direction::Right => snake_head.rotate = PI * 1.5,
                Direction::Down => snake_head.rotate = 0.,
                Direction::Left => snake_head.rotate = PI * 0.5,
            }
            snake_head.direction = Direction::Down;
            snake_head.updated = true;
        } else if keys.pressed(KeyCode::ArrowLeft) && snake_head.direction != Direction::Right {
            // old direction
            match snake_head.direction {
                Direction::Up => snake_head.rotate = PI * 0.5,
                Direction::Right => snake_head.rotate = PI,
                Direction::Down => snake_head.rotate = PI * 1.5,
                Direction::Left => snake_head.rotate = 0.,
            }
            snake_head.direction = Direction::Left;
            snake_head.updated = true;
        } else if keys.pressed(KeyCode::KeyX) {
            debug!("keys.pressed X");
            next_state.set(AppState::MainMenu);
        }
    }
}
