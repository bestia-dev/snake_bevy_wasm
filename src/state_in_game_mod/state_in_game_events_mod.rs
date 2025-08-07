use bevy::prelude::*;

use crate::{
    AppState,
    state_in_game_mod::{ButtonEnum, Direction, SnakeHead},
};

pub fn handle_movement_input(keys: Res<ButtonInput<KeyCode>>, mut snake_head: Single<&mut SnakeHead>, mut next_state: ResMut<NextState<AppState>>) {
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

#[allow(clippy::type_complexity)]
pub fn button_interaction_system(
    mut snake_head: Single<&mut SnakeHead>,
    interaction_query: Query<(&ButtonEnum, &Interaction), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in interaction_query {
        if *interaction.1 == Interaction::Pressed {
            if *interaction.0 == ButtonEnum::KeyUp {
                snake_head.new_direction = Direction::Up;
            } else if *interaction.0 == ButtonEnum::KeyRight {
                snake_head.new_direction = Direction::Right;
            } else if *interaction.0 == ButtonEnum::KeyDown {
                snake_head.new_direction = Direction::Down;
            } else if *interaction.0 == ButtonEnum::KeyLeft {
                snake_head.new_direction = Direction::Left;
            } else if *interaction.0 == ButtonEnum::KeyX {
                next_state.set(AppState::MainMenu);
            }
        }
    }
}
