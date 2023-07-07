use std::ops::Deref;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use js_sys::Math::log;
use leptos::*;
use leptos::{ev::MouseEvent, leptos_dom::ev::SubmitEvent};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use tauri_sys::{tauri, event};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct EmptyArgs;

#[derive(Serialize, Deserialize, Clone)]
struct Playlist {
    paths: Vec<PathBuf>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Track {
    id: usize,
    path: RwSignal<Arc<str>>
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

            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = tauri::invoke("greet", &GreetArgs { name: &name.get() }).await.unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let open_files = move |ev: MouseEvent| {
        spawn_local(async move {
            match tauri::invoke::<(), ()>("open_file_dialog", &()).await {
                Ok(_) => log!("Open file diaglog success"),
                Err(_) => log!("Failed to open dialog"),
            };
            
            let mut paths_to_add: Vec<Track> =
            match event::once::<Playlist>("open-files").await {
                Ok(ev) => {
                    ev.payload.paths
                    .iter()
                    .enumerate()
                    .map(|(pos, it)| {
                        Track{
                            id: playlist.get().len() + pos,
                            path: create_rw_signal(cx, Arc::from(match it.to_str(){
                                Some(string) => string,
                                None => {log!("failed to convert to string"); ""},
                            }))
                        }
                    }).collect::<Vec<Track>>()
                },
                Err(_) => {
                    log!("error opening file dialog");
                    Vec::new()
                }
            };
            let mut new_playlist: Vec<Track> = playlist.get();
            new_playlist.append(&mut paths_to_add);
            log!("new len: {}", new_playlist.len());
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
                        <p>"path:" {move || String::from(track.path.get().deref())}</p>
                    }
                }
            />

        </main>
    }
}
