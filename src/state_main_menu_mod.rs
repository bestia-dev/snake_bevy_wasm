// state_main_menu_mod.rs

use bevy::prelude::*;

use crate::AppState;
use crate::state_in_game_mod::SPRITE_HEIGHT;
use bevy::color::palettes::css::{GREEN, RED, YELLOW};

pub fn add_main_menu_to_app(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), on_enter_main_menu);

    // MUST add all systems to app with run_if in_state MainMenu
    app.add_systems(Update, handle_main_menu_ui_input.run_if(in_state(AppState::MainMenu)));
}

pub fn on_enter_main_menu(mut commands: Commands) {
    commands.spawn(Camera2d);
    debug!("on_enter_main_menu");
    commands
        .spawn((
            StateScoped(AppState::MainMenu),
            Node {
                // Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                // Make node fill the entirety of its parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Set the grid to have 3 rows with sizes [20px, auto, 20px]
                grid_template_rows: vec![GridTrack::vw(33.), GridTrack::vw(33.), GridTrack::vw(33.)],
                ..default()
            },
        ))
        .with_children(|builder| {
            // Header
            builder
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    //width: Val::Percent(100.),
                    //height: Val::Percent(100.),
                    ..default()
                },))
                .with_children(|builder| {
                    // Header
                    builder.spawn((
                        Text::new("bestia.dev/snake_bevy_wasm"),
                        TextFont {
                            font_size: SPRITE_HEIGHT as f32,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor::from(GREEN),
                    ));
                });
            // middle
            builder
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    //width: Val::Percent(100.),
                    //height: Val::Percent(100.),
                    ..default()
                },))
                .with_children(|builder| {
                    // middle
                    builder.spawn((
                        Text::new("Bestia.dev tutorial\nRust+Bevy+Wasm"),
                        TextFont {
                            font_size: SPRITE_HEIGHT as f32,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor::from(YELLOW),
                    ));
                });
            // footer
            builder
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    //width: Val::Percent(100.),
                    //height: Val::Percent(100.),
                    ..default()
                },))
                .with_children(|builder| {
                    // footer
                    builder.spawn((
                        Text::new("Press N to start"),
                        TextFont {
                            font_size: SPRITE_HEIGHT as f32,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor::from(RED),
                    ));
                });
        });
}

pub fn handle_main_menu_ui_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.pressed(KeyCode::KeyN) {
        debug!("keys.pressed N");
        next_state.set(AppState::InGame);
    }
}
