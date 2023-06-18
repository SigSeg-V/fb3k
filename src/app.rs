use std::path::PathBuf;

use leptos::*;
use leptos::{ev::MouseEvent, leptos_dom::ev::SubmitEvent};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize, Clone)]
struct Playlist {
    paths: Vec<PathBuf>,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());
    let (playlist, set_playlist) = create_signal(cx, Playlist { paths: Vec::new() });

    let update_name = move |ev: SubmitEvent| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            if name.get().is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let open_files = move |ev: MouseEvent| {
        spawn_local(async move {
            let mut new_playlist: Playlist = playlist.get();
            let mut paths_to_add: Playlist =
                match from_value(invoke("open_file_dialog", JsValue::NULL).await) {
                    Ok(pl) => pl,
                    Err(_) => {
                        log!("error opening file dialog");
                        Playlist { paths: Vec::new() }
                    }
                };
            new_playlist.paths.append(&mut paths_to_add.paths);
            set_playlist.set(new_playlist)
        })
    };

    view! { cx,
        <main class="container">
            <div class="row">
                <button on:click=open_files>
                    "Open Files"
                </button>
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <p>
                "Recommended IDE setup: "
                <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
                " + "
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
                " + "
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
            </p>
        </main>
    }
}