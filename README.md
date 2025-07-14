
added wasm-opt = false in Cargo.toml
for faster building

wasm-pack build --target web --profiling

rsync -a --delete-after pkg/ web_server_folder/snake_bevy_wasm/pkg


for missing apsa.pc
from container host
podman exec --user=root crustde_vscode_cnt apt install libasound2-dev

podman exec --user=root crustde_vscode_cnt apt-get install libudev-dev