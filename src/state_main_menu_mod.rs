// state_main_menu_mod.rs

use bevy::prelude::*;

use crate::AppState;

pub fn add_main_menu_to_app(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), on_enter_main_menu);

    // MUST add all systems to app with run_if in_state MainMenu
    app.add_systems(Update, handle_main_menu_ui_input.run_if(in_state(AppState::MainMenu)));
}

pub fn on_enter_main_menu(mut commands: Commands) {
    commands.spawn(Camera2d);
    debug!("on_enter_main_menu");
    // Text with one section
    commands.spawn((
        StateScoped(AppState::MainMenu),
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("Snake Bevy Wasm \n game \n press N to start"),
        TextFont { font_size: 67.0, ..default() },
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

pub fn handle_main_menu_ui_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.pressed(KeyCode::KeyN) {
        debug!("keys.pressed N");
        next_state.set(AppState::InGame);
    }
}
