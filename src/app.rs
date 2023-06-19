use std::path::PathBuf;

use leptos::*;
use leptos::{ev::MouseEvent, leptos_dom::ev::SubmitEvent};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use tauri_sys::event;

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

async fn listen_open_files_event() -> Vec<PathBuf> {
    let ev = event::once::<Vec<PathBuf>>("open-files").await.unwrap();
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

    
    let (playlist, set_playlist) = create_signal::<Vec<PathBuf>>(cx, vec![]);
    let playlist_resource = create_local_resource(cx, move ||(), listen_open_files_event());


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
            let mut new_playlist: Vec<PathBuf> = playlist.get();
            let mut paths_to_add: Playlist =
                match from_value<Playlist>(invoke("open_file_dialog", JsValue::NULL).await) {
                    Ok(pl) => pl,
                    Err(_) => {
                        log!("error opening file dialog");
                        Playlist { paths: Vec::new() }
                    }
                };
            new_playlist.append(&mut paths_to_add.paths);
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

            <For
                each=playlist
                key=|pl| pl
                view=move |cx, path: PathBuf|{
                    view! {
                        cx,
                        <p>"path:" {move || path.to_str()}</p>
                    }
                }
            />

        </main>
    }
}
