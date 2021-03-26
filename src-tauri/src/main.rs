#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ai;
mod api_types;
mod app_state;
mod cmd;
mod game_board_state;

use tauri::api::dialog;
use tauri::{execute_promise, Webview};

use app_state::AppState;
use cmd::Cmd;

fn about_dialog_message() {
    dialog::message(
        "\
        Four Dots\n\n\
        A Connect Four clone built with a Rust-Tauri-Svelte stack\n\n\
        by Kris Scott\n\
        @okayscott | github.com/krscott | okayscott.itch.io",
        "Four Dots",
    )
}

fn main() {
    let mut state = AppState::default();
    let mut ai_brain = ai::AiBrain::new();

    tauri::AppBuilder::new()
        // .setup(move |_webview, _source| {})
        .invoke_handler(move |webview, arg| {
            println!("{}", arg);

            match serde_json::from_str::<'_, Cmd>(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    println!("{:?}", command);
                    let res = command.handle(&mut state, &mut ai_brain);

                    update_webview_state(webview, &state);

                    match res {
                        Ok(()) => Ok(()),
                        Err(e) => Err(e.to_string()),
                    }
                }
            }
        })
        .build()
        .run();
}

fn update_webview_state(webview: &mut Webview, state: &AppState) {
    let state = state.clone();

    execute_promise(
        webview,
        move || Ok(state),
        "rust_set_state".into(),
        "rust_error_handler".into(),
    );
}
