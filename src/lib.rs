// src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # snake_bevy_wasm
//!
//! **Simple snake game with Bevy, Rust and Wasm**  
//! ***version: 0.0.17 date: 2025-07-15 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/snake_bevy_wasm)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!  ![bevy](https://img.shields.io/badge/bevy-orange)
//!
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/blob/master/LICENSE)
//!  ![snake_bevy_wasm](https://bestia.dev/webpage_hit_counter/get_svg_image/1481465721.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-164-green.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-104-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-21-purple.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//!
//! Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//! I recommend using the [CRUSTDE - Containerized Rust Development Environment](https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod) to write Rust projects on Linux, isolated from your system.  
//!
//! ## The game
//!
//! Everybody knows this game. This is an educational example.
//!
//! ## Faster builds
//!
//! Into Cargo.toml I added:
//!
//! ```toml
//! [package.metadata.wasm-pack.profile.profiling]
//! # for faster builds
//! wasm-opt = false
//! ```
//!
//! Now I can build faster in development using the mode `--profiling`
//!
//! ```bash
//! wasm-pack build --target web --profiling
//! ```
//!
//! For the release mode, wasm-opt was still too slow: 10 minutes.
//! I copied the files from <https://github.com/WebAssembly/binaryen/releases/download/version_123/binaryen-version_123-x86_64-linux.tar.gz> from the bin/ folder into ~/bin. Then make them all executable `chmod +x *.*`. wasm-pack was able to find the executable wasm-opt in this folder.  
//!
//! Now I have wasm-opt version 123.
//! The wasm-opt now takes only 30 seconds! Much less than 10 minutes.
//! Size reduction of the wasm file is from 65MB to 40MB.
//!
//! ## Bevy prerequisites
//!
//! Bevy needs some prerequisites to compile.  
//! Without them I get the cryptic error "alsa.pc not found".  
//! Run from container host:
//!
//! ```bash
//! podman exec --user=root crustde_vscode_cnt apt install libasound2-dev
//! podman exec --user=root crustde_vscode_cnt apt-get install libudev-dev
//! ```
//!
//!
//!
//! ## Development details
//!
//! Read the development details in a separate md file:
//! [DEVELOPMENT.md](DEVELOPMENT.md)
//!
//! ## Releases changelog
//!
//! Read the releases changelog in a separate md file:
//! [RELEASES.md](RELEASES.md)
//!
//! ## TODO
//!
//! And code happily ever after...
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod web_sys_mod;
use web_sys_mod as wsm;
mod events_mod;
use events_mod::*;
mod game_logic_mod;
use game_logic_mod::*;
mod render_mod;
use render_mod::*;

const CANVAS_WIDTH: i32 = 1000;
const CANVAS_HEIGHT: i32 = 1000;
const STEP_DURATION: f64 = 0.2;
const BOARD_WIDTH: i32 = 20;
const BOARD_HEIGHT: i32 = 20;
const BOARD_CENTER: i32 = 10;
const SPRITE_WIDTH: i32 = 50;
const SPRITE_HEIGHT: i32 = 50;

const SNAKE_Z_LAYER: f32 = 1.0;
const OTHER_Z_LAYER: f32 = 0.0;

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

#[wasm_bindgen]
pub fn main() {
    // rust has `Raw string literals` that are great!
    // just add r# before the starting double quotes and # after the ending double quotes.
    let html = format!(
        r#"
<canvas id="snake_bevy_canvas" width="{CANVAS_WIDTH}" height="{CANVAS_HEIGHT}"></canvas>
"#
    );

    // WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
    // Only correctly html encoded strings can use this function.
    wsm::set_html_element_inner_html("div_for_wasm_html_injecting", &html);

    // bevy initiation
    let mut app = bevy::app::App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // provide the ID selector string here
            canvas: Some("#snake_bevy_canvas".into()),
            resolution: bevy::window::WindowResolution::new(CANVAS_WIDTH as f32, CANVAS_HEIGHT as f32).with_scale_factor_override(1.0),
            resizable: false,
            // ... any other window properties ...
            ..default()
        }),
        ..default()
    }));

    // Set the Fixed Timestep interval for game logic to 0.x seconds
    app.insert_resource(Time::<Fixed>::from_seconds(STEP_DURATION));
    // only once on startup
    app.add_systems(Startup, startup);
    // game logic is sequential
    app.add_systems(
        FixedUpdate,
        (snake_head_move, eat_bird.after(snake_head_move), move_segments.after(eat_bird), check_dead.after(move_segments)),
    );
    // render frame and react to events
    app.add_systems(Update, (render_snake_head, render_bird, render_segment, handle_movement_input, render_dead, render_debug_text));
    app.run();
}

// run on startup
fn startup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);
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
