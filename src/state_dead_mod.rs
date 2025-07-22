// state_dead_mod.rs

use bevy::prelude::*;

use crate::AppState;
use crate::state_in_game_mod::SPRITE_HEIGHT;

pub fn add_dead_to_app(app: &mut App) {
    app.add_systems(OnEnter(AppState::Dead), on_enter_dead);

    // MUST add all systems to app with run_if in_state Dead
    app.add_systems(Update, handle_dead_ui_input.run_if(in_state(AppState::Dead)));
}

pub fn on_enter_dead(mut commands: Commands) {
    commands.spawn(Camera2d);
    debug!("on_enter_dead");
    // Text with one section
    commands.spawn((
        StateScoped(AppState::Dead),
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("Snake \n is dead. \n press N to start"),
        TextFont {
            font_size: SPRITE_HEIGHT as f32,
            ..default()
        },
        TextShadow::default(),
        // Set the justification of the Text
        TextLayout::new_with_justify(JustifyText::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    ));
}

pub fn handle_dead_ui_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.pressed(KeyCode::KeyN) {
        debug!("keys.pressed N");
        next_state.set(AppState::InGame);
    }
}
