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

const CANVAS_WIDTH: usize = 1000;
const CANVAS_HEIGHT: usize = 1000;

const BOARD_WIDTH: usize = 20;
const BOARD_HEIGHT: usize = 20;
const BOARD_CENTER: usize = 10;
const SPRITE_WIDTH: i32 = 50;
const SPRITE_HEIGHT: i32 = 50;

const SNAKE_Z_LAYER: f32 = 1.0;
const OTHER_Z_LAYER: f32 = 0.0;

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
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
}

// one component can spawn many entities
#[derive(Component)]
struct SnakeSegment {
    position: Position,
    index: usize,
    updated: bool,
}

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

    // Set the Fixed Timestep interval for game logic to 0.5 seconds
    app.insert_resource(Time::<Fixed>::from_seconds(0.5));
    // only once on startup
    app.add_systems(Startup, startup);
    // game logic
    app.add_systems(FixedUpdate, fixed_update_snake_head_move);
    app.add_systems(FixedUpdate, handle_eat_food.after(fixed_update_snake_head_move));
    app.add_systems(FixedUpdate, move_segments.after(handle_eat_food));

    // render frame and react to events
    app.add_systems(Update, (update_render_snake_head, update_render_bird, update_render_segment, handle_movement_input));
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
            position: bird_position,
            updated: false,
        },
    ));
}

impl Position {
    /// transform GameCoordinates to BevyCoordinates
    pub fn to_bevy_x(&self) -> f32 {
        self.x as f32 * SPRITE_WIDTH as f32 - (BOARD_CENTER as f32 * SPRITE_WIDTH as f32)
    }
    /// transform GameCoordinates to BevyCoordinates
    pub fn to_bevy_y(&self) -> f32 {
        -(self.y as f32) * SPRITE_HEIGHT as f32 + (BOARD_CENTER as f32 * SPRITE_HEIGHT as f32)
    }
}

// fixed time every 0.5 seconds
fn fixed_update_snake_head_move(_time: Res<Time>, mut queried_entities: Query<&mut SnakeHead>) {
    if let Ok(mut snake_head) = queried_entities.single_mut() {
        snake_head.last_position = snake_head.position.clone();
        match &snake_head.direction {
            Direction::Up => snake_head.position.y -= 1,
            Direction::Down => snake_head.position.y += 1,
            Direction::Left => snake_head.position.x -= 1,
            Direction::Right => snake_head.position.x += 1,
        }
        snake_head.updated = true;
        // move also all the segments
    }
}
fn handle_eat_food(_time: Res<Time>, mut snake_query: Query<&mut SnakeHead>, mut bird_query: Query<&mut Bird>) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        if let Ok(mut bird) = bird_query.single_mut() {
            if snake_head.position.x == bird.position.x && snake_head.position.y == bird.position.y {
                snake_head.just_eating = true;
                // food: point, longer body
                // new random position
                bird.position.x = ops::floor(js_sys::Math::random() as f32 * BOARD_WIDTH as f32) as usize;
                bird.position.y = ops::floor(js_sys::Math::random() as f32 * BOARD_HEIGHT as f32) as usize;
                bird.updated = true;

                // new segment entity
            }
            snake_head.updated = true;
        }
    }
}

/// first segment is after the snake head
fn move_segments(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_query: Query<&mut SnakeHead>,
    mut segment_query: Query<&mut SnakeSegment>,
) {
    if let Ok(mut snake_head) = snake_query.single_mut() {
        // Sort according to `usize index`.
        let sorted_snake_segments = segment_query.iter_mut().sort_by::<&SnakeSegment>(|value_1, value_2| value_1.index.cmp(&value_2.index));
        //move position from segment to segment
        let mut last_position = snake_head.last_position.clone();
        for mut snake_segment in sorted_snake_segments {
            let prev_pos = snake_segment.position.clone();
            snake_segment.position = last_position;
            last_position = prev_pos;
            snake_segment.updated = true;
        }
        // I will use the last_position to spawn the new segment - tail
        if snake_head.just_eating {
            snake_head.just_eating = false;

            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
                Transform::from_xyz(last_position.to_bevy_x(), last_position.to_bevy_y(), OTHER_Z_LAYER),
                MeshMaterial2d(materials.add(Color::hsl(250., 0.95, 0.7))),
                SnakeSegment {
                    position: last_position.clone(),
                    index: snake_head.segment_len,
                    updated: true,
                },
            ));
            snake_head.segment_len += 1;
        }
    }
}

fn update_render_snake_head(mut queried_entities: Query<(&mut SnakeHead, &mut Transform)>) {
    if let Ok((mut snake_head, mut transform)) = queried_entities.single_mut() {
        if snake_head.updated {
            transform.translation.x = snake_head.position.to_bevy_x();
            transform.translation.y = snake_head.position.to_bevy_y();

            snake_head.updated = false;
        }
    }
}
fn update_render_bird(mut queried_entities: Query<(&mut Bird, &mut Transform)>) {
    if let Ok((mut bird, mut transform)) = queried_entities.single_mut() {
        if bird.updated {
            transform.translation.x = bird.position.to_bevy_x();
            transform.translation.y = bird.position.to_bevy_y();

            bird.updated = false;
        }
    }
}

fn update_render_segment(mut queried_entities: Query<(&mut SnakeSegment, &mut Transform)>) {
    for (mut segment, mut transform) in queried_entities.iter_mut() {
        if segment.updated {
            transform.translation.x = segment.position.to_bevy_x();
            transform.translation.y = segment.position.to_bevy_y();

            segment.updated = false;
        }
    }
}

fn handle_movement_input(keys: Res<ButtonInput<KeyCode>>, mut queried_entities: Query<&mut SnakeHead>) {
    if let Ok(mut snake_head) = queried_entities.single_mut() {
        if keys.pressed(KeyCode::ArrowUp) && snake_head.direction != Direction::Down {
            snake_head.direction = Direction::Up;
        } else if keys.pressed(KeyCode::ArrowDown) && snake_head.direction != Direction::Up {
            snake_head.direction = Direction::Down;
        } else if keys.pressed(KeyCode::ArrowLeft) && snake_head.direction != Direction::Right {
            snake_head.direction = Direction::Left;
        } else if keys.pressed(KeyCode::ArrowRight) && snake_head.direction != Direction::Left {
            snake_head.direction = Direction::Right;
        }
    }
}
