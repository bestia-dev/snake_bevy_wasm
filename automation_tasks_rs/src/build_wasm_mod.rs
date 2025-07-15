// build_wasm_mod.rs

//! Functions to build a WASM library.
//!
//! Don't change this code, so it can be updated regularly with
//! cargo auto update_automation_tasks_rs
//! If you want to customize it, copy the code into main.rs and modify it there.

use crate::cl;

use cargo_auto_lib::CargoTomlPublicApiMethods;
use cargo_auto_lib::ShellCommandLimitedDoubleQuotesSanitizerTrait;
#[allow(unused_imports)]
use cl::{BLUE, GREEN, RED, RESET, YELLOW};

#[allow(dead_code)]
/// wasm-pack build --profiling
pub fn task_build() -> cl::CargoToml {

    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo clippy --no-deps").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("wasm-pack build --target web --profiling").unwrap_or_else(|e| panic!("{e}"));

    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"rsync -a --delete-after pkg/ "web_server_folder/{package_name}/pkg/" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    cargo_toml
}

#[allow(dead_code)]
/// wasm-pack build --release
pub fn task_release() -> cl::CargoToml {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo clippy --no-deps").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("wasm-pack build --target web --release").unwrap_or_else(|e| panic!("{e}"));

    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"rsync -a --delete-after pkg/ "web_server_folder/{package_name}/pkg/" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    cargo_toml
}