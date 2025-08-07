// state_in_game_mod.rs

use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_kira_audio::{AudioControl, AudioInstance, AudioTween};

use crate::{AppState, GameBoardCanvas, Orientation};
mod state_in_game_events_mod;
use state_in_game_events_mod::*;
mod state_in_game_logic_mod;
use state_in_game_logic_mod::*;
mod state_in_game_render_mod;
use state_in_game_render_mod::*;

#[derive(Component)]
struct DebugText {
    bird_position: String,
}

#[derive(Component)]
struct PointsText {}

#[derive(Clone, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

// only one bird at any time
#[derive(Component)]
struct Bird {
    position: Position,
    color: Color,
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// only one snake_head
#[derive(Component)]
struct SnakeHead {
    position: Position,
    direction: Direction,
    // keyboard events are too often to have game logic inside
    new_direction: Direction,
    last_direction: Direction,
    last_position: Position,
    just_eating: bool,
    moves: i32,
    points: i32,
}

// one component can spawn many entities
#[derive(Component, Clone, Debug)]
struct SnakeSegment {
    position: Position,
    direction: Direction,
    last_direction: Direction,
    is_tail: bool,
}

#[derive(Component, Clone, Debug)]
struct SnakeSegmentIndex {
    index: usize,
}

// Component to identify the audio entity
#[derive(Component)]
struct Music;

// Marker struct to help identify the color-changing Text component
#[derive(Component)]
struct AnimatedText;

#[derive(Resource)]
struct InstanceHandle(Handle<AudioInstance>);

#[derive(Component, PartialEq)]
enum ButtonEnum {
    KeyUp,
    KeyRight,
    KeyDown,
    KeyLeft,
    KeyX,
}

const STEP_DURATION: f64 = 0.2;

const SNAKE_Z_LAYER: f32 = 2.0;
const BIRD_Z_LAYER: f32 = 1.0;
const OTHER_Z_LAYER: f32 = 0.0;

pub fn add_in_game_to_app(app: &mut App) {
    // Set the Fixed Timestep interval for game logic to 0.x seconds
    app.insert_resource(Time::<Fixed>::from_seconds(STEP_DURATION));
    app.add_systems(OnEnter(AppState::InGame), on_enter_in_game);
    app.add_systems(OnExit(AppState::InGame), on_exit_in_game);
    app.add_systems(
        FixedUpdate,
        (
            move_snake_head.run_if(in_state(AppState::InGame)),
            eat_bird.run_if(in_state(AppState::InGame)),
            move_segments.run_if(in_state(AppState::InGame)),
            check_dead.run_if(in_state(AppState::InGame)),
            render_snake_head.run_if(in_state(AppState::InGame)),
            render_segment.run_if(in_state(AppState::InGame)),
        )
            .chain(),
    );

    app.add_systems(
        // render frame and react to events
        Update,
        (
            crate::handle_browser_resize.run_if(in_state(AppState::InGame)),
            button_interaction_system.run_if(in_state(AppState::InGame)),
            // draw_axis.run_if(in_state(AppState::InGame)),
            render_bird.run_if(in_state(AppState::InGame)),
            handle_movement_input.run_if(in_state(AppState::InGame)),
            render_points_text.run_if(in_state(AppState::InGame)),
            render_debug_text.run_if(in_state(AppState::InGame)),
        ),
    );
}

// run on enter in state in_game
fn on_enter_in_game(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<bevy_kira_audio::Audio>, game_board_canvas: Res<GameBoardCanvas>) {
    let handle = audio.play(asset_server.load("snake_hiss.mp3")).looped().handle();
    commands.insert_resource(InstanceHandle(handle));

    commands.spawn(Camera2d);

    let mut client = if game_board_canvas.orientation == Orientation::Landscape {
        commands.spawn(crate::landscape(&game_board_canvas))
    } else {
        commands.spawn(crate::portrait(&game_board_canvas))
    };

    client.with_children(|client| {
        client.spawn((
            Node {
                // Make node fill the entirety of its parent (in this case the window)
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                grid_template_rows: vec![GridTrack::percent(33.), GridTrack::percent(33.), GridTrack::percent(34.)],
                ..default()
            },
            Outline {
                width: Val::Px(2.),
                offset: Val::Px(-1.),
                color: Color::BLACK,
            },
        ));
        // second cell
        if game_board_canvas.orientation == Orientation::Portrait {
            let mut keys = client.spawn((
                // the UI buttons will despawn, but the rest of the game
                // will remain in the background after death
                StateScoped(AppState::InGame),
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
                    // Make this node span 1 grid columns so that it takes up half row
                    grid_column: GridPlacement::span(2),
                    ..default()
                },));
                keys.spawn((
                    Button,
                    ButtonEnum::KeyUp,
                    ImageNode {
                        image: asset_server.load("key_up.png"),
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

                keys.spawn((Node {
                    display: Display::Grid,
                    // Make this node span 1 grid columns so that it takes up the second half row
                    grid_column: GridPlacement::span(2),
                    ..default()
                },));

                keys.spawn((Node {
                    display: Display::Grid,
                    // Make this node span 1 grid columns so that it takes up one column
                    grid_column: GridPlacement::span(1),
                    ..default()
                },));

                keys.spawn((
                    Button,
                    ButtonEnum::KeyLeft,
                    ImageNode {
                        image: asset_server.load("key_left.png"),
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

                keys.spawn((
                    Button,
                    ButtonEnum::KeyDown,
                    ImageNode {
                        image: asset_server.load("key_down.png"),
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

                keys.spawn((
                    Button,
                    ButtonEnum::KeyRight,
                    ImageNode {
                        image: asset_server.load("key_right.png"),
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

    // snake head
    let snake_head_position = Position { x: 10, y: 10 };
    commands.spawn((
        Sprite::from_image(asset_server.load("snake_head_left.png")),
        Transform::from_xyz(snake_head_position.to_bevy_x(&game_board_canvas), snake_head_position.to_bevy_y(&game_board_canvas), SNAKE_Z_LAYER).with_rotation(Quat::from_rotation_z(PI * 0.5)),
        SnakeHead {
            position: snake_head_position.clone(),
            direction: Direction::Down,
            new_direction: Direction::Down,
            last_direction: Direction::Down,
            last_position: snake_head_position,
            just_eating: false,
            moves: 0,
            points: 0,
        },
    ));

    // first (zero) segment
    let segment_position = Position { x: 10, y: 9 };
    commands.spawn((
        Sprite::from_image(asset_server.load("segment_horizontal.png")),
        Transform::from_xyz(segment_position.to_bevy_x(&game_board_canvas), segment_position.to_bevy_y(&game_board_canvas), OTHER_Z_LAYER).with_rotation(Quat::from_rotation_z(PI * 0.5)),
        SnakeSegment {
            position: segment_position,
            direction: Direction::Down,
            last_direction: Direction::Down,
            is_tail: false,
        },
        SnakeSegmentIndex { index: 0 },
    ));

    // last (1) segment
    let segment_position = Position { x: 10, y: 8 };
    commands.spawn((
        Sprite::from_image(asset_server.load("segment_tail.png")),
        Transform::from_xyz(segment_position.to_bevy_x(&game_board_canvas), segment_position.to_bevy_y(&game_board_canvas), OTHER_Z_LAYER).with_rotation(Quat::from_rotation_z(PI * 0.5)),
        SnakeSegment {
            position: segment_position,
            direction: Direction::Down,
            last_direction: Direction::Down,
            is_tail: true,
        },
        SnakeSegmentIndex { index: 1 },
    ));

    // spawn entity bird
    let bird_position = Position { x: 9, y: 9 };
    commands.spawn((
        Sprite::from_image(asset_server.load("bird.png")),
        Transform::from_xyz(bird_position.to_bevy_x(&game_board_canvas), bird_position.to_bevy_y(&game_board_canvas), BIRD_Z_LAYER),
        Bird {
            position: bird_position.clone(),
            color: WHITE.into(),
        },
    ));

    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        PointsText {},
    ));

    // DebugText
    commands.spawn((
        Visibility::Hidden,
        Text::new("Debug text: "),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(25.0),
            left: Val::Px(300.0),
            ..default()
        },
        DebugText {
            bird_position: format!("{:?}", &bird_position),
        },
    ));
}

fn on_exit_in_game(handle: Res<InstanceHandle>, mut audio_instances: ResMut<Assets<AudioInstance>>) {
    // stop only the music, not the other sound effects
    if let Some(instance) = audio_instances.get_mut(&handle.0) {
        instance.stop(AudioTween::default());
    }
}

impl Position {
    /// transform GameCoordinates to BevyCoordinates
    pub fn to_bevy_x(&self, game_board_canvas: &GameBoardCanvas) -> f32 {
        self.x as f32 * game_board_canvas.sprite_width - (game_board_canvas.client_width as f32 / 2.0) + (game_board_canvas.sprite_width / 2.)
    }
    /// transform GameCoordinates to BevyCoordinates. Strange because it goes in the opposite direction.
    pub fn to_bevy_y(&self, game_board_canvas: &GameBoardCanvas) -> f32 {
        -(self.y as f32) * game_board_canvas.sprite_height + (game_board_canvas.client_height as f32 / 2.0) - (game_board_canvas.sprite_height / 2.)
    }
}
