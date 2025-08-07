// state_dead_mod.rs

use bevy::color::palettes::css::{GREEN, RED, YELLOW};
use bevy::{color, prelude::*};

use crate::{AppState, GameBoardCanvas, Orientation};

#[derive(Component, PartialEq)]
enum ButtonEnum {
    KeyN,
}

pub fn add_dead_to_app(app: &mut App) {
    app.add_systems(OnEnter(AppState::Dead), on_enter_dead);
    app.add_systems(OnExit(AppState::Dead), on_exit_dead);
    // MUST add all systems to app with run_if in_state Dead
    app.add_systems(
        Update,
        (
            handle_dead_ui_input.run_if(in_state(AppState::Dead)),
            crate::handle_browser_resize.run_if(in_state(AppState::Dead)),
            button_interaction_system.run_if(in_state(AppState::Dead)),
        ),
    );
}

pub fn on_enter_dead(mut commands: Commands, game_board_canvas: Res<GameBoardCanvas>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let mut client = if game_board_canvas.orientation == Orientation::Landscape {
        commands.spawn(crate::landscape(&game_board_canvas))
    } else {
        commands.spawn(crate::portrait(&game_board_canvas))
    };

    client.with_children(|client| {
        let mut grid = client.spawn((
            Node {
                // Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                // Make node fill the entirety of its parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Set the grid to have 3 rows with sizes
                grid_template_rows: vec![GridTrack::percent(33.), GridTrack::percent(33.), GridTrack::percent(34.)],

                ..default()
            },
            Outline {
                width: Val::Px(1.),
                offset: Val::Px(0.),
                color: Color::WHITE,
            },
        ));

        grid.with_children(|grid| {
            // Header
            let mut header_box = grid.spawn((Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },));
            {
                header_box.with_children(|header_box| {
                    // Header
                    header_box.spawn((
                        Text::new("bestia.dev/snake_bevy_wasm"),
                        TextFont {
                            font_size: game_board_canvas.sprite_height,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor::from(color::Color::Srgba(GREEN).with_alpha(0.7)),
                    ));
                });
            }
            // middle
            let mut middle_box = grid.spawn((Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },));
            {
                middle_box.with_children(|middle_box| {
                    // middle
                    middle_box.spawn((
                        Text::new("Snake is dead."),
                        TextFont {
                            font_size: game_board_canvas.sprite_height,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor::from(color::Color::Srgba(YELLOW).with_alpha(0.7)),
                    ));
                });
            }
            // footer
            let mut footer_box = grid.spawn((Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },));
            {
                footer_box.with_children(|footer_box| {
                    // footer
                    footer_box.spawn((
                        Text::new("Press N to start"),
                        TextFont {
                            font_size: game_board_canvas.sprite_height,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor::from(color::Color::Srgba(RED).with_alpha(0.7)),
                    ));
                });
            }
        });
        if game_board_canvas.orientation == Orientation::Portrait {
            let mut keys = client.spawn((
                Node {
                    // Use the CSS Grid algorithm for laying out this node
                    display: Display::Grid,
                    // Make node fill the entirety of its parent (in this case the window)
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    // grid 5 col x 4 rows
                    grid_template_columns: vec![
                        GridTrack::percent(20.),
                        GridTrack::percent(20.),
                        GridTrack::percent(20.),
                        GridTrack::percent(20.),
                        GridTrack::percent(20.),
                    ],
                    grid_template_rows: vec![GridTrack::percent(25.), GridTrack::percent(25.), GridTrack::percent(25.), GridTrack::percent(25.)],
                    ..default()
                },
                Outline {
                    width: Val::Px(1.),
                    offset: Val::Px(-2.),
                    color: Color::from(RED),
                },
            ));
            keys.with_children(|keys| {
                keys.spawn((Node {
                    display: Display::Grid,
                    // Make this node span 5 grid columns so that it takes up the entire top row
                    grid_column: GridPlacement::span(5),
                    ..default()
                },));
                keys.spawn((Node {
                    display: Display::Grid,
                    // Make this node span 2 grid columns so that it takes up half row
                    grid_column: GridPlacement::span(2),
                    ..default()
                },));
                keys.spawn((
                    Button,
                    ButtonEnum::KeyN,
                    ImageNode {
                        image: asset_server.load("key_n.png"),
                        ..default()
                    },
                    Interaction::None,
                    Node {
                        width: Val::Px(80.),
                        height: Val::Px(80.),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ));
            });
        }
    });
}

// despawn all entities
pub fn on_exit_dead(mut commands: Commands, entities: Query<Entity, With<Visibility>>) {
    for entity in entities {
        commands.entity(entity).despawn();
    }
}

pub fn handle_dead_ui_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.pressed(KeyCode::KeyN) {
        next_state.set(AppState::InGame);
    }
}

#[allow(clippy::type_complexity)]
fn button_interaction_system(interaction_query: Query<(&ButtonEnum, &Interaction), (Changed<Interaction>, With<Button>)>, mut next_state: ResMut<NextState<AppState>>) {
    for interaction in interaction_query {
        if *interaction.0 == ButtonEnum::KeyN && *interaction.1 == Interaction::Pressed {
            next_state.set(AppState::InGame);
        }
    }
}
