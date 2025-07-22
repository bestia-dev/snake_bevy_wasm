# Development details

## CRUSTDE - Containerized Rust Development Environment

I recommend using the CRUSTDE - Containerized Rust Development Environment to write Rust projects. Follow the instructions here <https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod>.  

It is an isolated development environment that will not mess with you system.
It will work on Linux (tested on Debian) and inside WSL (Windows Subsystem for Linux).

You just need to install the newer alternative to Docker: [podman](https://podman.io/). Then you download the prepared container image from DockerHub (3GB). And then a little juggling with ssh keys. All this is simplified by running a few bash scripts. Just follow the easy instructions.  

The container image contains cargo, rustc, wasm-pack, basic-http-server, cargo-auto and other utils that a Rust project needs.  

## Workflow with automation_tasks_rs and cargo-auto

For easy workflow, use the automation tasks that are already coded in the sub-project `automation_tasks_rs`. This is a basic workflow:

```bash
cargo auto build
cargo auto release
cargo auto doc
cargo auto test
cargo auto commit_and_push
cargo auto publish_to_crates_io
cargo auto github_new_release
```

Every task finishes with instructions how to proceed.  
The [cargo-auto](https://github.com/automation-tasks-rs/cargo-auto) and [dev_bestia_cargo_completion](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion) are already installed inside the CRUSTDE container.

You can open the automation sub-project in VSCode and then code your own tasks in Rust.

```bash
code automation_tasks_rs
```

## HTML, CSS

The simple static HTML and CSS files are in `web_server_folder/snake_bevy_wasm`.  
Then the Rust code injects html elements into the DOM.  

## Web server and wasm

The browser security does not allow the loading of WASM modules from local files. It needs to be loaded from a web server. The CRUSTDE container has the [basic-http-server](https://github.com/brson/basic-http-server) already installed.  

Run the server in a second VSCode terminal, so it can keep running all the time.  

```bash
basic-http-server -a 0.0.0.0:4000 ./web_server_folder
```

In the first VSCode terminal, we can build the project.  
Then in the browser, we can refresh the page <http://localhost:4000/snake_bevy_wasm> with F5 to see the changes.  

## Rust and wasm

In the `Cargo.toml` it is important to define the output as wasm library and the required dependencies to web-sys, js-sys, and wasm-bindgen. Wasm starts from the `src/lib.rs`. The `main` function is decorated with the attribute `#[wasm_bindgen]`.

## GitHub

This template contains GitHub actions to build the project on commit and publish the documentation on GitHub pages.  

## Bevy prerequisites

[Bevy](https://bevy.org/) is s refreshingly simple data-driven game engine built in Rust. Free and Open Source Forever!  
Bevy needs some prerequisites to compile.  
Without them I get the cryptic error "alsa.pc not found".  
Run from container host:

```bash
podman exec --user=root crustde_vscode_cnt apt install libasound2-dev
podman exec --user=root crustde_vscode_cnt apt-get install libudev-dev
```


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
I copied the files from <https://github.com/WebAssembly/binaryen/releases/latest/> and find `binaryen-version_xxx-x86_64-linux.tar.gz` from the bin/ folder into `~/bin`. Then make them all executable `chmod +x *.*`. wasm-pack was able to find the executable wasm-opt in this folder.  

Now I have wasm-opt version 123. This version should be faster.
The wasm-opt now takes only 30 seconds! Much less than 10 minutes.
Size reduction of the wasm file is from 65MB to 40MB.

## Graphics

The canvas will be 1000x1000 pixels. The sprite will be 50x50 pixels. So the game-board will be 20x20 cells.  
The movements are based on elapsed time - tick.  
Bevy coordinate system is 0,0 in the middle of the canvas. Right is x+, Left is x-, Up is y+, Down in y-.  
For the game grid I will use the coordinate system 0,0 is in the up-left corner. Right is x+, Left is x-, Down in y+, Up is y-.  
Using this simplified coordinate system the array index is the same as the coordinate of the sprite.  
The game data will be in the game coordinate system. Then the renderer will transform that into the bevy coordinate system.  
 


