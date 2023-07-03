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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Track {
    id: usize,
    path: RwSignal<String>
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

    let (playlist, set_playlist) = create_signal::<Vec<Track>>(cx, vec![]);

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
            let mut new_playlist: Vec<Track> = playlist.get();
            let mut paths_to_add: Vec<Track> =
                match from_value::<Playlist>(invoke("open_file_dialog", JsValue::NULL).await) {
                    Ok(pl) => {
                        pl.paths
                            .iter()
                            .enumerate()
                            .map(|(pos, it)| {
                                Track{
                                    id: playlist.get().len()+pos,
                                    path: create_rw_signal(cx, it.to_str().unwrap().to_string())
                                }
                            }).collect::<Vec<Track>>()
                    },
                    Err(_) => {
                        log!("error opening file dialog");
                        Vec::new()
                    }
                };
            new_playlist.append(&mut paths_to_add);
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
                key=|pl| pl.id
                view=move |cx, track: Track|{
                    view! {
                        cx,
                        <p>"path:" {move || track.path.get()}</p>
                    }
                }
            />

        </main>
    }
}
