// state_in_game_mod.rs

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_kira_audio::{AudioControl, AudioInstance, AudioTween};

use crate::{AppState, CANVAS_HEIGHT, CANVAS_WIDTH};
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
    updated: bool,
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
    updated: bool,
    moves: i32,
    points: i32,
}

// one component can spawn many entities
#[derive(Component, Clone, Debug)]
struct SnakeSegment {
    position: Position,
    index: usize,
    direction: Direction,
    last_direction: Direction,
    updated: bool,
}

// Component to identify the audio entity
#[derive(Component)]
struct Music;

// Marker struct to help identify the color-changing Text component
#[derive(Component)]
struct AnimatedText;

#[derive(Resource)]
struct InstanceHandle(Handle<AudioInstance>);

const STEP_DURATION: f64 = 0.2;
const BOARD_WIDTH: i32 = 20;
const BOARD_HEIGHT: i32 = 20;
const BOARD_CENTER: i32 = BOARD_HEIGHT / 2;
pub const SPRITE_WIDTH: i32 = CANVAS_WIDTH / BOARD_WIDTH;
pub const SPRITE_HEIGHT: i32 = CANVAS_HEIGHT / BOARD_HEIGHT;

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
        )
            .chain(),
    );

    app.add_systems(
        // render frame and react to events
        Update,
        (
            render_snake_head.run_if(in_state(AppState::InGame)),
            render_bird.run_if(in_state(AppState::InGame)),
            render_segment.run_if(in_state(AppState::InGame)),
            handle_movement_input.run_if(in_state(AppState::InGame)),
            render_points_text.run_if(in_state(AppState::InGame)),
            render_debug_text.run_if(in_state(AppState::InGame)),
        ),
    );
}

// run on enter in state in_game
fn on_enter_in_game(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<bevy_kira_audio::Audio>) {
    let handle = audio.play(asset_server.load("snake_hiss.mp3")).looped().handle();
    commands.insert_resource(InstanceHandle(handle));

    commands.spawn(Camera2d);

    // snake head
    let snake_head_position = Position { x: 10, y: 10 };
    commands.spawn((
        StateScoped(AppState::InGame),
        Sprite::from_image(asset_server.load("snake_head_left.png")),
        Transform::from_xyz(snake_head_position.to_bevy_x(), snake_head_position.to_bevy_y(), SNAKE_Z_LAYER).with_rotation(Quat::from_rotation_z(PI * 0.5)),
        SnakeHead {
            position: snake_head_position.clone(),
            direction: Direction::Down,
            new_direction: Direction::Down,
            last_direction: Direction::Down,
            last_position: snake_head_position,
            just_eating: false,
            updated: false,
            moves: 0,
            points: 0,
        },
    ));

    // first and only segment
    let segment_position = Position { x: 10, y: 9 };
    commands.spawn((
        StateScoped(AppState::InGame),
        Sprite::from_image(asset_server.load("segment_horizontal.png")),
        Transform::from_xyz(segment_position.to_bevy_x(), segment_position.to_bevy_y(), OTHER_Z_LAYER).with_rotation(Quat::from_rotation_z(PI * 0.5)),
        SnakeSegment {
            position: segment_position,
            index: 0,
            direction: Direction::Down,
            last_direction: Direction::Down,
            updated: false,
        },
    ));

    // spawn entity bird
    let bird_position = Position { x: 9, y: 9 };
    commands.spawn((
        StateScoped(AppState::InGame),
        Sprite::from_image(asset_server.load("bird.png")),
        Transform::from_xyz(bird_position.to_bevy_x(), bird_position.to_bevy_y(), BIRD_Z_LAYER),
        Bird {
            position: bird_position.clone(),
            updated: false,
        },
    ));

    commands.spawn((
        StateScoped(AppState::InGame),
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
        StateScoped(AppState::InGame),
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
    pub fn to_bevy_x(&self) -> f32 {
        self.x as f32 * SPRITE_WIDTH as f32 - (BOARD_CENTER as f32 * SPRITE_WIDTH as f32) + (SPRITE_WIDTH as f32 / 2.)
    }
    /// transform GameCoordinates to BevyCoordinates. Strange because it goes in the opposite direction.
    pub fn to_bevy_y(&self) -> f32 {
        -(self.y as f32) * SPRITE_HEIGHT as f32 + (BOARD_CENTER as f32 * SPRITE_HEIGHT as f32) - (SPRITE_HEIGHT as f32 / 2.)
    }
}
