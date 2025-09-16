use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"Leptos test starting".into());

    let _handle = mount_to(
        || {
            web_sys::console::log_1(&"Leptos component rendering".into());
            view! {
                <div>
                    <h1>"Simple Leptos Test"</h1>
                    <p>"This is a basic Leptos test without motion components."</p>
                    <SimpleCounter />
                </div>
            }
        },
        "app",
    );

    web_sys::console::log_1(&"Leptos mount completed".into());
}

#[component]
fn SimpleCounter() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let increment = move |_| {
        set_count.update(|c| *c += 1);
        web_sys::console::log_1(&format!("Count: {}", count.get()).into());
    };

    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=increment>"Increment"</button>
        </div>
    }
}
