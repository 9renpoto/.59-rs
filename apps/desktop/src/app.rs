use application::GreetingResponse;
use leptos::task::spawn_local;
use leptos::{ev::SubmitEvent, prelude::*};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Deserialize)]
struct CommandErrorPayload {
    message: String,
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (greet_msg, set_greet_msg) = signal(None::<String>);
    let (error_msg, set_error_msg) = signal(None::<String>);

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            set_greet_msg.set(None);
            set_error_msg.set(None);

            let args = match serde_wasm_bindgen::to_value(&GreetArgs { name: &name }) {
                Ok(args) => args,
                Err(err) => {
                    set_error_msg.set(Some(err.to_string()));
                    return;
                }
            };

            match invoke("greet", args).await {
                Ok(value) => {
                    if let Ok(res) = serde_wasm_bindgen::from_value::<GreetingResponse>(value) {
                        set_greet_msg.set(Some(res.message));
                    } else {
                        set_error_msg.set(Some("応答を読み取れませんでした。".to_owned()));
                    }
                }
                Err(err) => {
                    if let Ok(err_payload) =
                        serde_wasm_bindgen::from_value::<CommandErrorPayload>(err.clone())
                    {
                        set_error_msg.set(Some(err_payload.message));
                    } else if let Some(err_str) = err.as_string() {
                        set_error_msg.set(Some(err_str));
                    } else {
                        set_error_msg.set(Some("エラーが発生しました。".to_owned()));
                    }
                }
            }
        });
    };

    view! {
        <main class="container">
            <h1>"Welcome to Tauri + Leptos"</h1>

            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>
            <p>"Tauri IPC 経由で共通の application / domain 層を呼び出します。"</p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    prop:value=name
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>
            {move || greet_msg.get().map(|msg| view! { <p class="success">{msg}</p> })}
            {move || error_msg.get().map(|err| view! { <p class="error">{err}</p> })}
        </main>
    }
}
