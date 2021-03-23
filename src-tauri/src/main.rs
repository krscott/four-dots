#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app_state;
mod cmd;
mod game_board_state;

use tauri::{execute_promise, Webview};

use app_state::AppState;
use cmd::Cmd;

fn main() {
    // let mut state = GameBoardState::new(7, 6).unwrap();
    let mut state = AppState::default();

    tauri::AppBuilder::new()
        // .setup(move |_webview, _source| {})
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

fn update_webview_state(webview: &mut Webview, state: &AppState) {
    let state = state.clone();

    execute_promise(
        webview,
        move || Ok(state),
        "rust_set_state".into(),
        "rust_error_handler".into(),
    );
}
