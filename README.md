<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# snake_bevy_wasm

[//]: # (auto_cargo_toml_to_md start)

**Simple snake game with Bevy, Rust and Wasm**  
***version: 0.0.19 date: 2025-07-20 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/snake_bevy_wasm)***

 ![maintained](https://img.shields.io/badge/maintained-green)
 ![work-in-progress](https://img.shields.io/badge/work_in_progress-yellow)
 ![rustlang](https://img.shields.io/badge/rustlang-orange)
 ![bevy](https://img.shields.io/badge/bevy-orange)

[//]: # (auto_cargo_toml_to_md end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/blob/master/LICENSE)
 ![snake_bevy_wasm](https://bestia.dev/webpage_hit_counter/get_svg_image/1481465721.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-174-green.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-104-blue.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-25-purple.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/snake_bevy_wasm/)

[//]: # (auto_lines_of_code end)

Hashtags: #maintained #ready-for-use #rustlang #automation #workflow  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).  
I recommend using the [CRUSTDE - Containerized Rust Development Environment](https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod) to write Rust projects on Linux, isolated from your system.  

## The game

Everybody knows this game. This is an educational example.

## Graphics

The canvas will be 1000x1000 pixels. The sprite will be 50x50 pixels. So the game-board will be 20x20 cells.
The movements are based on elapsed time - tick.
Bevy coordinate system is 0,0 in the middle of the canvas. Right is x+, Left is x-, Up is y+, Down in y-.
For the game grid I will use the coordinate system 0,0 is in the up-left corner. Right is x+, Left is x-, Down in y+, Up is y-.
Using this simplified coordinate system the array index is the same as the coordinate of the sprite.
The game data will be in the game coordinate system. Then the renderer will transform that into the bevy coordinate system.
 

## Faster builds

I use the mold linker to speed up the build. In ~/.cargo/config.toml I already have:

```toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-B/home/rustdevuser/.cargo/bin/mold"]
```

But rust-analyzer does not read that file. In .vscode/settings.json I added

```json
    "rust-analyzer.cargo.extraEnv": {
        "RUSTFLAGS": "-Clink-arg=-B/home/rustdevuser/.cargo/bin/mold"
    },
```

Hopefully it will help but I don't really know for sure.

The build step `wasm-opt` was really slow so, into Cargo.toml I added:

```toml
[package.metadata.wasm-pack.profile.profiling]
# for faster builds
wasm-opt = false
```

Now I can build faster in development using the mode `--profiling`

```bash
wasm-pack build --target web --profiling
```

For the release mode, wasm-opt was still too slow: 10 minutes.
I copied the files from <https://github.com/WebAssembly/binaryen/releases/download/version_123/binaryen-version_123-x86_64-linux.tar.gz> from the bin/ folder into ~/bin. Then make them all executable `chmod +x *.*`. wasm-pack was able to find the executable wasm-opt in this folder.  

Now I have wasm-opt version 123. This version should be faster.
The wasm-opt now takes only 30 seconds! Much less than 10 minutes.
Size reduction of the wasm file is from 65MB to 40MB.

## Bevy prerequisites

Bevy needs some prerequisites to compile.  
Without them I get the cryptic error "alsa.pc not found".  
Run from container host:

```bash
podman exec --user=root crustde_vscode_cnt apt install libasound2-dev
podman exec --user=root crustde_vscode_cnt apt-get install libudev-dev
```



## Development details

Read the development details in a separate md file:
[DEVELOPMENT.md](DEVELOPMENT.md)

## Releases changelog

Read the releases changelog in a separate md file:
[RELEASES.md](RELEASES.md)

## TODO

And code happily ever after...

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
