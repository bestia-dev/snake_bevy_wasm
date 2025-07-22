use bevy::prelude::*;

use crate::AppState;
mod state_in_game_events_mod;
use state_in_game_events_mod::*;
mod state_in_game_logic_mod;
use state_in_game_logic_mod::*;
mod state_in_game_render_mod;
use state_in_game_render_mod::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct InGameSet;

#[derive(Component)]
struct DebugText {
    bird_position: String,
}

#[derive(Clone, PartialEq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Bird {
    position: Position,
    updated: bool,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct SnakeHead {
    position: Position,
    direction: Direction,
    last_position: Position,
    segment_len: usize,
    just_eating: bool,
    updated: bool,
    dead: bool,
}

// one component can spawn many entities
#[derive(Component)]
struct SnakeSegment {
    position: Position,
    index: usize,
    updated: bool,
}

// Marker struct to help identify the color-changing Text component
#[derive(Component)]
struct AnimatedText;

const STEP_DURATION: f64 = 0.2;
const BOARD_WIDTH: i32 = 20;
const BOARD_HEIGHT: i32 = 20;
const BOARD_CENTER: i32 = 10;
const SPRITE_WIDTH: i32 = 50;
const SPRITE_HEIGHT: i32 = 50;

const SNAKE_Z_LAYER: f32 = 1.0;
const OTHER_Z_LAYER: f32 = 0.0;

pub fn add_in_game_to_app(app: &mut App) {
    // Set the Fixed Timestep interval for game logic to 0.x seconds
    app.insert_resource(Time::<Fixed>::from_seconds(STEP_DURATION));

app.add_systems(OnEnter(AppState::InGame), startup_in_game);
    // game logic is sequential
    app.add_systems(FixedUpdate, (move_snake_head, eat_bird, move_segments, check_dead).chain());
    // render frame and react to events
    app.add_systems(Update, (render_snake_head, render_bird, render_segment, handle_movement_input, render_dead, render_debug_text));

    app.configure_sets(Update, (MainMenuSet.run_if(in_state(AppState::MainMenu)),));
    // now we can easily add our different systems
    app.add_systems(Update, handle_main_menu_ui_input.in_set(MainMenuSet));
}

// run on startup
fn startup_in_game(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // snake head
    let snake_head_position = Position { x: 10, y: 10 };
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        Transform::from_xyz(snake_head_position.to_bevy_x(), snake_head_position.to_bevy_y(), SNAKE_Z_LAYER),
        MeshMaterial2d(materials.add(Color::hsl(300., 0.95, 0.7))),
        SnakeHead {
            position: snake_head_position.clone(),
            direction: Direction::Down,
            last_position: snake_head_position,
            just_eating: false,
            segment_len: 1,
            updated: false,
            dead: false,
        },
    ));

    // first and only segment
    let segment_position = Position { x: 10, y: 9 };
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        Transform::from_xyz(segment_position.to_bevy_x(), segment_position.to_bevy_y(), OTHER_Z_LAYER),
        MeshMaterial2d(materials.add(Color::hsl(250., 0.95, 0.7))),
        SnakeSegment {
            position: segment_position,
            index: 0,
            updated: false,
        },
    ));

    // spawn entity bird
    let bird_position = Position { x: 9, y: 9 };
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::hsl(2., 0.95, 0.7))),
        Transform::from_xyz(bird_position.to_bevy_x(), bird_position.to_bevy_y(), OTHER_Z_LAYER),
        Bird {
            position: bird_position.clone(),
            updated: false,
        },
    ));

    // Text with one section
    commands.spawn((
        Visibility::Hidden,
        // Accepts a `String` or any type that converts into a `String`, such as `&str`
        Text::new("The snake\nis dead!"),
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
        AnimatedText,
    ));

    commands.spawn((
        // Visibility::Hidden,
        Text::new("Debug text: "),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        DebugText {
            bird_position: format!("{:?}", &bird_position),
        },
    ));
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