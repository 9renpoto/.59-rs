use application::GreetingResponse;
use gloo_net::http::Request;
use leptos::*;
use wasm_bindgen_futures::spawn_local;

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (message, set_message) = create_signal(None::<String>);
    let (error, set_error) = create_signal(None::<String>);

    let submit = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();
        let name = name.get_untracked();
        set_message.set(None);
        set_error.set(None);

        spawn_local(async move {
            let body = serde_json::json!({ "name": name }).to_string();
            let request = match Request::post("/api/greetings")
                .header("Content-Type", "application/json")
                .body(body)
            {
                Ok(request) => request,
                Err(_) => {
                    set_error.set(Some("リクエストを作成できませんでした。".to_owned()));
                    return;
                }
            };
            let response = request.send().await;

            match response {
                Ok(response) if response.ok() => match response.json::<GreetingResponse>().await {
                    Ok(response) => set_message.set(Some(response.message)),
                    Err(_) => set_error.set(Some("応答を読み取れませんでした。".to_owned())),
                },
                Ok(response) => {
                    let message = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "入力を確認してください。".to_owned());
                    set_error.set(Some(message));
                }
                Err(_) => set_error.set(Some("API に接続できませんでした。".to_owned())),
            }
        });
    };

    view! {
        <main class="page">
            <section class="card">
                <p class="eyebrow">"DDD + Cloudflare Workers"</p>
                <h1>"Greeting sample"</h1>
                <p>"名前を送信すると、Rust の domain/application 層で挨拶を生成します。"</p>
                <form on:submit=submit>
                    <label for="name">"Name"</label>
                    <input
                        id="name"
                        name="name"
                        placeholder="Ada"
                        prop:value=name
                        on:input=move |event| set_name.set(event_target_value(&event))
                    />
                    <button type="submit">"Greet"</button>
                </form>
                {move || message.get().map(|message| view! { <p class="success">{message}</p> })}
                {move || error.get().map(|error| view! { <p class="error">{error}</p> })}
            </section>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
