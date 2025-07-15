// automation_tasks_rs for snake_bevy_wasm

// region: library and modules with basic automation tasks

mod build_cli_bin_mod;
mod build_lib_mod;
mod build_wasm_mod;
mod cargo_auto_github_api_mod;
mod encrypt_decrypt_with_ssh_key_mod;
mod generic_functions_mod;
mod tasks_mod;

pub use cargo_auto_lib as cl;

use crate::cargo_auto_github_api_mod as cgl;
use crate::encrypt_decrypt_with_ssh_key_mod as ende;
use crate::generic_functions_mod as gn;
use crate::tasks_mod as ts;

pub use cl::{BLUE, GREEN, RED, RESET, YELLOW};

// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;
use cl::ShellCommandLimitedDoubleQuotesSanitizerTrait;

// region: library with basic automation tasks

fn main() {
    std::panic::set_hook(Box::new(gn::panic_set_hook));
    gn::tracing_init();
    cl::exit_if_not_run_in_rust_project_root_directory();
    ende::github_api_token_with_oauth2_mod::github_api_config_initialize();
    ende::crates_io_api_token_mod::crates_io_config_initialize();
    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("  {YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
                } else if &task == "github_new_release" {
                    task_github_new_release();
                } else {
                    eprintln!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
  {YELLOW}Welcome to cargo-auto !{RESET}
  {YELLOW}This program automates your custom tasks when developing a Rust project.{RESET}

  {YELLOW}User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto build{RESET} - {YELLOW}builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto release{RESET} - {YELLOW}builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET} - {YELLOW}builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET} - {YELLOW}runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET} - {YELLOW}commits with message and push with mandatory message{RESET}
  {YELLOW}It is preferred to use SSH for git push to GitHub.{RESET}
  {YELLOW}<https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod/blob/main/ssh_easy.md>{YELLOW}
  {YELLOW}On the very first commit, this task will initialize a new local git repository and create a remote GitHub repo.{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto publish_to_web{RESET} - {YELLOW}publish to web, git tag{RESET}
  {YELLOW}It is preferred to use SSH to publish to web and remotely manage the web server.{RESET}
  {YELLOW}<https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod/blob/main/ssh_easy.md>{YELLOW}
{GREEN}cargo auto github_new_release{RESET} - {YELLOW}creates new release on GitHub{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto update_automation_tasks_rs{RESET} - {YELLOW}updates the files in automation_tasks_rs{RESET}
  {YELLOW}Some files are fixed and the update is straight forward, other files need manual diff.{RESET}

  {YELLOW}© 2025 bestia.dev  MIT License github.com/automation--tasks--rs/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd() {
    /*
        println!(
            r#"
    {YELLOW}run examples:{RESET}
    {GREEN}cargo run --example plantuml1{RESET}
    "#
        );
    */
}

/// Sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`.
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec![
            "build",
            "release",
            "doc",
            "test",
            "commit_and_push",
            "publish_to_web",
            "github_new_release",
            "update_automation_tasks_rs",
        ];
        cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// wasm-pack build
fn task_build() {
    let cargo_toml = crate::build_wasm_mod::task_build();
    println!(
        r#"
  {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server{RESET}
  {YELLOW}in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
  {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/{package_name}{RESET}
  {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto release{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// wasm-pack build --release
fn task_release() {
    let cargo_toml = crate::build_wasm_mod::task_release();

    println!(
        r#"
  {YELLOW}After `cargo auto build`, open port 4000 in VSCode and run the basic web server{RESET}
  {YELLOW}in a separate VSCode bash terminal, so it can serve constantly in the background.{RESET}
{GREEN}basic-http-server -a 0.0.0.0:4000 ./web_server_folder{RESET}
  {YELLOW}and open the browser on{RESET}
{GREEN}http://localhost:4000/{package_name}{RESET}    
  {YELLOW}If all is fine, run{RESET}
{GREEN}cargo auto doc{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a GitHub standard folder
fn task_doc() {
    ts::task_doc();
    // message to help user with next move
    println!(
        r#"
  {YELLOW}After `cargo auto doc`, ctrl-click on `docs/index.html`. 
    It will show the index.html in VSCode Explorer, then right-click and choose "Show Preview".
    This works inside the CRUSTDE container, because of the extension "Live Preview" 
    <https://marketplace.visualstudio.com/items?itemName=ms-vscode.live-server>
    If ok then run the tests in code and the documentation code examples.{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
}

/// cargo test
fn task_test() {
    println!(r#"  {YELLOW}Wasm is a cdylib and therefore doc-tests are not run !{RESET}"#);
    cl::run_shell_command_static("cargo test").unwrap_or_else(|e| panic!("{e}"));
    println!(
        r#"
  {YELLOW}After `cargo auto test`. If ok then {RESET}
  {YELLOW}(commit message is mandatory){RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    ts::task_commit_and_push(arg_2);
    println!(
        r#"
  {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_web{RESET}
"#
    );
}

/// publish to web
fn task_publish_to_web() {
    let cargo_toml = cl::CargoToml::read();
    let version = cargo_toml.package_version();
    // take care of tags
    let _tag_name_version = cl::git_tag_sync_check_create_push(&version);

    // rsync to copy to server over ssh into a temporary installation folder
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
r#"rsync -e ssh -a --info=progress2 --delete-after "web_server_folder/{package_name}/" "{server__username}@{web__server__domain}:/var/www/transfer_folder/{package_name}" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .arg("{server__username}", "luciano_bestia").unwrap_or_else(|e| panic!("{e}"))
    .arg("{web__server__domain}", "bestia.dev").unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    // rsync to copy to server over ssh the installation script
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
r#"rsync -e ssh -a --info=progress2 --delete-after "publish_script/{package_name}_publish.sh" "{server__username}@{web__server__domain}:/var/www/scripts/{package_name}/" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .arg("{server__username}", "luciano_bestia").unwrap_or_else(|e| panic!("{e}"))
    .arg("{web__server__domain}", "bestia.dev").unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    //make the bash script executable
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
r#"ssh "{server__username}@{web__server__domain}" chmod +x  "/var/www/scripts/{package_name}/{package_name}_publish.sh" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .arg("{server__username}", "luciano_bestia").unwrap_or_else(|e| panic!("{e}"))
    .arg("{web__server__domain}", "bestia.dev").unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    // run installation script over ssh on the server to copy from the installation folder to production folder
    cl::ShellCommandLimitedDoubleQuotesSanitizer::new(
r#"ssh "{server__username}@{web__server__domain}" "/var/www/scripts/{package_name}/{package_name}_publish.sh" "#).unwrap_or_else(|e| panic!("{e}"))
    .arg("{package_name}", &cargo_toml.package_name()).unwrap_or_else(|e| panic!("{e}"))
    .arg("{server__username}", "luciano_bestia").unwrap_or_else(|e| panic!("{e}"))
    .arg("{web__server__domain}", "bestia.dev").unwrap_or_else(|e| panic!("{e}"))
    .run().unwrap_or_else(|e| panic!("{e}"));

    println!(
        r#"
  {YELLOW}After `cargo auto publish_to_web`check {RESET}
{GREEN}https://bestia.dev/{package_name}{RESET}
  {YELLOW}Now, write the content of the release in the RELEASES.md in the `## Unreleased` section, then{RESET}
  {YELLOW}Next, create the GitHub Release.{RESET}
{GREEN}cargo auto github_new_release{RESET}
"#,
        package_name = cargo_toml.package_name()
    );
}

/// create a new release on github
fn task_github_new_release() {
    ts::task_github_new_release();
    println!(
        r#"
  {YELLOW}No more automation tasks. {RESET}
"#
    );
}
// endregion: tasks
