// src/lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # snake_bevy_wasm
//!
//! **Simple snake game with Bevy, Rust and Wasm**  
//! ***version: 1.0.160 date: 2025-07-30 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/snake_bevy_wasm)***
//!
//!  ![maintained](https://img.shields.io/badge/maintained-green)
//!  ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
//!  ![rustlang](https://img.shields.io/badge/rustlang-orange)
//!  ![bevy](https://img.shields.io/badge/bevy-orange)
//!
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/blob/master/LICENSE)
//!  ![snake_bevy_wasm](https://bestia.dev/webpage_hit_counter/get_svg_image/1481465721.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-792-green.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-78-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-81-purple.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
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
//! rotate least and mirror when upside down
//! When dead, I want to see the snake.
//! Sprites for segments and tail  
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

#[derive(Resource)]
struct GameBoardCanvas {
    client_width: i32,
    client_height: i32,
    board_canvas_width: i32,
    board_canvas_height: i32,
    sprite_width: f32,
    sprite_height: f32,
}

#[wasm_bindgen]
pub fn main() {
    // check viewport and define sizes
    let client_width = wsm::get_client_width();
    let client_height = wsm::get_client_height();
    wsm::debug_write(&format!("{client_width} x {client_height}"));

    // landscape for PC monitor viewport is around 1280 x 712px
    let game_board_canvas = if client_width > client_height {
        GameBoardCanvas {
            client_width,
            client_height,
            board_canvas_width: client_height,
            board_canvas_height: client_height,
            sprite_width: client_height as f32 / BOARD_WIDTH as f32,
            sprite_height: client_height as f32 / BOARD_HEIGHT as f32,
        }
    } else {
        // portrait for mobile phone it is around 360px x 649px
        GameBoardCanvas {
            client_width,
            client_height,
            board_canvas_width: client_width,
            board_canvas_height: client_width,
            sprite_width: client_width as f32 / BOARD_WIDTH as f32,
            sprite_height: client_width as f32 / BOARD_HEIGHT as f32,
        }
    };

    // rust has `Raw string literals` that are great!
    // just add r# before the starting double quotes and # after the ending double quotes.
    let html = r#"
<canvas id="snake_bevy_canvas"></canvas>
"#;

    // WARNING for HTML INJECTION! Never put user provided strings in set_html_element_inner_html.
    // Only correctly html encoded strings can use this function.
    wsm::set_html_element_inner_html("div_for_wasm_html_injecting", html);

    // bevy initiation
    let mut app = bevy::app::App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // provide the ID selector string here
                    canvas: Some("#snake_bevy_canvas".into()),
                    // all client area
                    resolution: bevy::window::WindowResolution::new(client_width as f32, client_height as f32), //.with_scale_factor_override(1.0),
                    resizable: false,
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
            }),
        bevy_kira_audio::AudioPlugin,
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
