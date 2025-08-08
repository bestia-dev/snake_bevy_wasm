// src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # snake_bevy_wasm
//!
//! **Simple snake game with Bevy, Rust and Wasm**  
//! ***version: 1.2.7 date: 2025-08-08 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/snake_bevy_wasm)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!  ![bevy](https://img.shields.io/badge/bevy-orange)
//!
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/blob/master/LICENSE)
//!  ![snake_bevy_wasm](https://bestia.dev/webpage_hit_counter/get_svg_image/1481465721.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1278-green.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-81-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-135-purple.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//!
//! Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
//!
//! ## The game
//!
//! Everybody knows this game. This is an educational example.  
//! In this project I explore the Bevy game engine, wasm and Rust.  
//! Try it here: <https://bestia.dev/snake_bevy_wasm/>  
//!
//! <!-- markdownlint-disable MD033 -->
//! <video width="300" height="350" src="https://github.com/user-attachments/assets/05340382-904b-4489-8352-682474748eee"></video>
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
//! 3 birds from 3 bevy birds (little rotated)
//! buttons a little space in between (padding)
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
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin};
use wasm_bindgen::prelude::*;

mod web_sys_mod;
use web_sys_mod as wsm;
mod state_dead_mod;
mod state_in_game_mod;
mod state_main_menu_mod;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BOARD_WIDTH: i32 = 20;
const BOARD_HEIGHT: i32 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
#[states(scoped_entities)]
enum AppState {
    MainMenu,
    InGame,
    Dead,
}

#[derive(PartialEq)]
pub enum Orientation {
    Landscape,
    Portrait,
}

#[derive(Resource)]
pub struct GameBoardCanvas {
    client_width: i32,
    client_height: i32,
    orientation: Orientation,
    board_canvas_width: i32,
    board_canvas_height: i32,
    sprite_width: f32,
    sprite_height: f32,
}

// This struct defines the data that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material2d for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "custom_material.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "custom_material.wgsl".into()
    }
}

#[wasm_bindgen]
pub fn main() {
    // rust has `Raw string literals` that are great!
    // just add r# before the starting double quotes and # after the ending double quotes.
    let html = r#"
<canvas id="snake_bevy_canvas" ></canvas>
"#;

    // WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
    // Only correctly html encoded strings can use this function.
    wsm::set_html_element_inner_html("div_for_wasm_html_injecting", html);

    // bevy initiation
    let mut app = bevy::app::App::new();
    let game_board_canvas = get_game_board_canvas();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // provide the ID selector string here
                    canvas: Some("#snake_bevy_canvas".into()),
                    // all client area
                    resolution: bevy::window::WindowResolution::new(game_board_canvas.client_width as f32, game_board_canvas.client_height as f32),
                    fit_canvas_to_parent: true,
                    // ... any other window properties ...
                    ..default()
                }),
                ..default()
            })
            .set(bevy::log::LogPlugin {
                filter: "info,wgpu_core=error,wgpu_hal=error,bevy_render=error,bevy_ecs=error,bevy_winit=error,bevy_core_pipeline=error,bevy_pbr=error,symphonia=error,snake_bevy_wasm=debug".into(),
                level: bevy::log::Level::DEBUG,
                custom_layer: |_| None,
            })
            // don't download meta files for assets
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        bevy_kira_audio::AudioPlugin,
        Material2dPlugin::<CustomMaterial>::default(),
    ));

    info!("started snake_bevy_wasm {}", VERSION);
    // initial state is MainMenu
    app.insert_state(AppState::MainMenu);

    app.insert_resource(game_board_canvas);
    state_main_menu_mod::add_main_menu_to_app(&mut app);

    state_in_game_mod::add_in_game_to_app(&mut app);

    state_dead_mod::add_dead_to_app(&mut app);

    app.run();
}

// setup fit to window size on startup
fn get_game_board_canvas() -> GameBoardCanvas {
    // check viewport and define sizes
    let client_width = wsm::get_client_width();
    let client_height = wsm::get_client_height();

    // landscape for PC monitor viewport is around 1280 x 712px
    // portrait for mobile phone it is around 360px x 649px
    // choose the smaller square to fit the window
    let (game_square_width, orientation) = if client_width > client_height {
        (client_height, Orientation::Landscape)
    } else {
        (client_width, Orientation::Portrait)
    };
    // return
    GameBoardCanvas {
        client_width,
        client_height,
        orientation,
        board_canvas_width: game_square_width,
        board_canvas_height: game_square_width,
        sprite_width: game_square_width as f32 / BOARD_WIDTH as f32,
        sprite_height: game_square_width as f32 / BOARD_HEIGHT as f32,
    }
}

/// Bevy 0.16 is not setting the correct width and height for the canvas element.
/// I don't know why. The css style is correct, but the attributes of the html element are wrong.
/// I must check this and correct this size frequently.
pub fn handle_browser_resize(mut game_board_canvas: ResMut<GameBoardCanvas>, mut window: bevy::ecs::system::Single<&mut bevy::window::Window, bevy::ecs::query::With<bevy::window::PrimaryWindow>>) {
    let client_width = wsm::get_client_width();
    let client_height = wsm::get_client_height();

    if (window.resolution.width() as i32) != client_width || (window.resolution.height() as i32) != client_height {
        // debug!("handle_browser_resize {client_width} {client_height}");
        game_board_canvas.client_width = client_width;
        game_board_canvas.client_height = client_height;
        // landscape for PC monitor viewport is around 1280 x 712px
        // portrait for mobile phone it is around 360px x 649px
        // choose the smaller square to fit the window
        let (game_square_width, orientation) = if client_width > client_height {
            (client_height, Orientation::Landscape)
        } else {
            (client_width, Orientation::Portrait)
        };
        game_board_canvas.orientation = orientation;
        game_board_canvas.board_canvas_width = game_square_width;
        game_board_canvas.board_canvas_height = game_square_width;
        game_board_canvas.sprite_width = game_square_width as f32 / BOARD_WIDTH as f32;
        game_board_canvas.sprite_height = game_square_width as f32 / BOARD_HEIGHT as f32;

        window.resolution.set(client_width as f32, client_height as f32);
    }
}

/// Portrait: left-top will be the square canvas, bottom is the remaining
pub fn portrait(game_board_canvas: &Res<'_, GameBoardCanvas>) -> Node {
    Node {
        // Use the CSS Grid algorithm for laying out this node
        display: Display::Grid,
        // Make node fill the entirety of its parent (in this case the window)
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        // Set the grid to have 2 rows
        // flex(1) means the remaining space
        grid_template_rows: vec![GridTrack::px(game_board_canvas.board_canvas_height as f32), GridTrack::flex(1.0)],
        ..default()
    }
}

/// Landscape: left-top will be the square canvas, left is the remaining
pub fn landscape(game_board_canvas: &Res<'_, GameBoardCanvas>) -> Node {
    Node {
        // Use the CSS Grid algorithm for laying out this node
        display: Display::Grid,
        // Make node fill the entirety of its parent (in this case the window)
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        // Set the grid to have 2 columns
        // flex(1) means the remaining space
        grid_template_columns: vec![GridTrack::px(game_board_canvas.board_canvas_width as f32), GridTrack::flex(1.0)],
        ..default()
    }
}
