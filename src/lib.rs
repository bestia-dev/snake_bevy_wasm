// src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # snake_bevy_wasm
//!
//! **Simple snake game with Bevy, Rust and Wasm**  
//! ***version: 0.0.8 date: 2025-03-18 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/snake_bevy_wasm)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!  ![wasm](https://img.shields.io/badge/wasm-orange)
//!
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/blob/master/LICENSE)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-348-green.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-130-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-70-purple.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-16-orange.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//!
//! Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//! I recommend using the [CRUSTDE - Containerized Rust Development Environment](https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod) to write Rust projects on Linux, isolated from your system.  
//!
//! ## This template
//!
//! Just like `cargo new` makes a soft and gentle introduction to Rust projects and development, I want to make the same for an in-browser WASM project with
//!
//! ```bash
//! cargo auto new_pwa_wasm
//! ```
//!
//! Extremely simple, just the basic moving parts and use cases.  
//! This simplest template does not have a PWA implementation or dedicated web server app.
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

    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                snake_move,
                event_up.run_if(input_just_pressed(KeyCode::ArrowUp)),
                event_down.run_if(input_just_pressed(KeyCode::ArrowDown)),
                event_left.run_if(input_just_pressed(KeyCode::ArrowLeft)),
                event_right.run_if(input_just_pressed(KeyCode::ArrowRight)),
            ),
        )
        .run();
}

#[derive(Component)]
struct FoodPosition {
    x: i32,
    y: i32,
}

#[derive(Component)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);

    let color = Color::hsl(300., 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        Transform::from_xyz(0., 0., 0.),
        MeshMaterial2d(materials.add(color)),
        SnakeDirection::Down,
    ));

    let color = Color::hsl(2., 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., 100.0, 0.0),
        FoodPosition { x: 50, y: 50 },
    ));
}

fn snake_move(time: Res<Time>, mut snake_position: Query<(&mut SnakeDirection, &mut Transform)>) {
    if let Ok((direction, mut transform)) = snake_position.single_mut() {
        match direction.as_ref() {
            SnakeDirection::Up => transform.translation.y += 50. * time.delta_secs(),
            SnakeDirection::Down => transform.translation.y -= 50. * time.delta_secs(),
            SnakeDirection::Left => transform.translation.x -= 50. * time.delta_secs(),
            SnakeDirection::Right => transform.translation.x += 50. * time.delta_secs(),
        }
    }
}

fn event_up(mut snake_direction: Query<&mut SnakeDirection>) {
    if let Ok(mut d) = snake_direction.single_mut() {
        *d = SnakeDirection::Up;
    }
}
fn event_down(mut snake_direction: Query<&mut SnakeDirection>) {
    if let Ok(mut d) = snake_direction.single_mut() {
        *d = SnakeDirection::Down;
    }
}
fn event_left(mut snake_direction: Query<&mut SnakeDirection>) {
    if let Ok(mut d) = snake_direction.single_mut() {
        *d = SnakeDirection::Left;
    }
}
fn event_right(mut snake_direction: Query<&mut SnakeDirection>) {
    if let Ok(mut d) = snake_direction.single_mut() {
        *d = SnakeDirection::Right;
    }
}
