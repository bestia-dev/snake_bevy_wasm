
added wasm-opt = false in Cargo.toml
for faster building

wasm-pack build --target web --profiling

rsync -a --delete-after pkg/ web_server_folder/snake_bevy/pkg