// state_main_menu_mod.rs

use bevy::prelude::*;

use crate::{AppState, GameBoardCanvas, Orientation, VERSION};
use bevy::color::palettes::css::{GREEN, RED, WHITE, YELLOW};

#[derive(Component, PartialEq)]
enum ButtonEnum {
    KeyN,
}

pub fn add_main_menu_to_app(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), on_enter_main_menu);
    app.add_systems(OnExit(AppState::MainMenu), on_exit_dead);
    // MUST add all systems to app with run_if in_state MainMenu
    app.add_systems(
        Update,
        (
            handle_main_menu_ui_input.run_if(in_state(AppState::MainMenu)),
            crate::handle_browser_resize.run_if(in_state(AppState::MainMenu)),
            button_interaction_system.run_if(in_state(AppState::MainMenu)),
        ),
    );
}

pub fn on_enter_main_menu(
    mut commands: Commands,
    game_board_canvas: Res<GameBoardCanvas>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<crate::CustomMaterial>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        //MeshMaterial2d(materials.add(Color::from(RED))),
        MeshMaterial2d(materials.add(crate::CustomMaterial { color: LinearRgba::BLUE })),
        Transform::default().with_scale(Vec3::splat(400.)),
    ));

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
                color: WHITE.into(),
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
                        Text::new("github.com/bestia-dev/snake_bevy_wasm"),
                        TextFont {
                            font_size: game_board_canvas.sprite_height,
                            ..default()
                        },
                        TextLayout::new_with_justify(JustifyText::Center),
                        TextColor::from(GREEN),
                    ));
                    // debug line to see where is the coordinating system
                    header_box.spawn(());
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
                        Text::new(format!("Bestia.dev tutorial\nRust+Bevy+Wasm v{VERSION}")),
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

        if game_board_canvas.orientation == Orientation::Portrait {
            let mut keys = client.spawn((
                Node {
                    // Use the CSS Grid algorithm for laying out this node
                    display: Display::Grid,
                    // Make node fill the entirety of its parent (in this case the window)
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
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
                    color: WHITE.into(),
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

pub fn handle_main_menu_ui_input(keys: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
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
