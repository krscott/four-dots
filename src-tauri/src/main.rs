#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cmd;
mod state;

use tauri::{execute_promise, Webview};

use cmd::Cmd;
use state::State;

fn main() {
    let mut state = State::new(7, 6).unwrap();

    let setup_state = state.clone();

    tauri::AppBuilder::new()
        .setup(move |webview, source| {
            println!("Source: {}", source);
            update_webview_state(webview, &setup_state);
        })
        .invoke_handler(move |webview, arg| {
            println!("{}", arg);

            match serde_json::from_str::<'_, Cmd>(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    let res = command.handle(&mut state);
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

fn update_webview_state(webview: &mut Webview, state: &State) {
    let state = state.clone();

    execute_promise(
        webview,
        move || Ok(state),
        "rust_set_state".into(),
        "rust_error_handler".into(),
    );
}
