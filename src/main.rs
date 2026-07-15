use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div style="font-family: sans-serif; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background-color: #f0f2f5; color: #333;">
            <div style="background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); text-align: center;">
                <h1 style="color: #4f46e5; margin-bottom: 1rem;">"Hello World"</h1>
                <p style="color: #666; font-size: 1.1rem;">"This is a Client-Side Rendered (CSR) Leptos application!"</p>
                <div style="margin-top: 1.5rem;">
                    <span style="font-weight: bold; font-size: 0.9rem; color: #888;">"Built with Rust & WebAssembly"</span>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
