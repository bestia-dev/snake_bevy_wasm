// state_dead_mod.rs

use bevy::color::palettes::css::{GREEN, RED, YELLOW};
use bevy::prelude::*;

use crate::{AppState, GameBoardCanvas};

pub fn add_dead_to_app(app: &mut App) {
    app.add_systems(OnEnter(AppState::Dead), on_enter_dead);

    // MUST add all systems to app with run_if in_state Dead
    app.add_systems(
        Update,
        (handle_dead_ui_input.run_if(in_state(AppState::Dead)), crate::handle_browser_resize.run_if(in_state(AppState::Dead))),
    );
}

pub fn on_enter_dead(mut commands: Commands, game_board_canvas: Res<GameBoardCanvas>) {
    commands.spawn(Camera2d);

    let mut client = if game_board_canvas.client_width > game_board_canvas.client_height {
        commands.spawn((StateScoped(AppState::Dead), crate::landscape(&game_board_canvas)))
    } else {
        commands.spawn((StateScoped(AppState::Dead), crate::portrait(&game_board_canvas)))
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
        {
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
                            TextColor::from(GREEN),
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
                            TextColor::from(YELLOW),
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
                            TextColor::from(RED),
                        ));
                    });
                }
            });
        }
    });
}

pub fn handle_dead_ui_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keys.pressed(KeyCode::KeyN) {
        next_state.set(AppState::InGame);
    }
}
