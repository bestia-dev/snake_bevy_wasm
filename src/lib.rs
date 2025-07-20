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

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use wasm_bindgen::prelude::*;

mod web_sys_mod;
use web_sys_mod as wsm;

const TIME_STEP: f64 = 1. / 9.;

#[wasm_bindgen]
pub fn main() {
    // rust has `Raw string literals` that are great!
    // just add r# before the starting double quotes and # after the ending double quotes.
    let html = r#"
<canvas id="snake_bevy_canvas" width="1000" height="1000"></canvas>
"#;

    // WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
    // Only correctly html encoded strings can use this function.
    wsm::set_html_element_inner_html("div_for_wasm_html_injecting", html);

    // bevy initiation
    let mut app = bevy::app::App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // provide the ID selector string here
            canvas: Some("#snake_bevy_canvas".into()),
            resolution: bevy::window::WindowResolution::new(1000., 1000.).with_scale_factor_override(1.0),
            // ... any other window properties ...
            ..default()
        }),
        ..default()
    }));

    // Set the Fixed Timestep interval for game logic to 0.5 seconds
    app.insert_resource(Time::<Fixed>::from_seconds(0.5));

    app.add_systems(Startup, setup)
        .add_systems(FixedUpdate, (snake_move,))
        .add_systems(
            Update,
            (
                snake_render,
                event_up.run_if(input_just_pressed(KeyCode::ArrowUp)),
                event_down.run_if(input_just_pressed(KeyCode::ArrowDown)),
                event_left.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                event_right.run_if(input_just_pressed(KeyCode::ArrowRight)),
            ),
        )
        .run();
}

const BOARD_WIDTH: usize = 20;
const BOARD_HEIGHT: usize = 20;
const BOARD_CENTER: usize = 10;
const SPRITE_WIDTH: i32 = 50;
const SPRITE_HEIGHT: i32 = 50;

struct Position {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct Bird {
    position: Position,
}

enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Snake {
    snake_vec: Vec<Position>,
    snake_direction: SnakeDirection,
    rendered: bool,
}

struct BevyPosition {
    x: f32,
    y: f32,
}

// run on startup
fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);

    let snake_position = Position { x: 10, y: 10 };
    let bevy_snake_position = transform_coordinates(&snake_position);

    // first snake
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        Transform::from_xyz(bevy_snake_position.x, bevy_snake_position.y, 1.),
        MeshMaterial2d(materials.add(Color::hsl(300., 0.95, 0.7))),
        Snake {
            snake_vec: vec![snake_position],
            snake_direction: SnakeDirection::Down,
            rendered: true,
        },
    ));

    let bird_position = Position { x: 9, y: 9 };
    let bevy_bird_position = transform_coordinates(&bird_position);

    // first bird
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::hsl(2., 0.95, 0.7))),
        Transform::from_xyz(bevy_bird_position.x, bevy_bird_position.y, 0.),
        Bird { position: bird_position },
    ));
}

/// transform GameCoordinates to BevyCoordinates
fn transform_coordinates(position: &Position) -> BevyPosition {
    let x = position.x as f32 * SPRITE_WIDTH as f32 - (BOARD_CENTER as f32 * SPRITE_WIDTH as f32);
    let y = -1. * position.y as f32 * SPRITE_HEIGHT as f32 + (BOARD_CENTER as f32 * SPRITE_HEIGHT as f32);
    BevyPosition { x, y }
}

// fixed time every 0.5 seconds
fn snake_move(_time: Res<Time>, mut snake_entity: Query<&mut Snake>) {
    if let Ok(mut snake) = snake_entity.single_mut() {
        let snake_position = match &snake.snake_direction {
            SnakeDirection::Up => Position {
                x: snake.snake_vec[0].x,
                y: snake.snake_vec[0].y - 1,
            },
            SnakeDirection::Down => Position {
                x: snake.snake_vec[0].x,
                y: snake.snake_vec[0].y + 1,
            },
            SnakeDirection::Left => Position {
                x: snake.snake_vec[0].x - 1,
                y: snake.snake_vec[0].y,
            },
            SnakeDirection::Right => Position {
                x: snake.snake_vec[0].x + 1,
                y: snake.snake_vec[0].y,
            },
        };
        snake.snake_vec[0] = snake_position;
        snake.rendered = false;
    }
}

fn snake_render(time: Res<Time>, mut snake_entity: Query<(&mut Snake, &mut Transform)>) {
    if let Ok((mut snake, mut transform)) = snake_entity.single_mut() {
        if !snake.rendered {
            let bevy_position = transform_coordinates(&snake.snake_vec[0]);
            transform.translation.x = bevy_position.x;
            transform.translation.y = bevy_position.y;

            snake.rendered = true;
        }
    }
}

fn event_up(mut snake: Query<&mut Snake>) {
    if let Ok(mut snake) = snake.single_mut() {
        snake.snake_direction = SnakeDirection::Up;
    }
}
fn event_down(mut snake: Query<&mut Snake>) {
    if let Ok(mut snake) = snake.single_mut() {
        snake.snake_direction = SnakeDirection::Down;
    }
}
fn event_left(mut snake: Query<&mut Snake>) {
    if let Ok(mut snake) = snake.single_mut() {
        snake.snake_direction = SnakeDirection::Left;
    }
}
fn event_right(mut snake: Query<&mut Snake>) {
    if let Ok(mut snake) = snake.single_mut() {
        snake.snake_direction = SnakeDirection::Right;
    }
}
