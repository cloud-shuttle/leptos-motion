use leptos::prelude::*;

#[component]
pub fn BasicDemo() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (is_visible, set_visible) = signal(true);
    
    view! {
        <div style="padding: 20px; text-align: center; font-family: Arial, sans-serif;">
            <h1 style="color: #333; margin-bottom: 20px;">"Basic Leptos Demo"</h1>
            
            <div style="margin: 20px 0;">
                <p style="font-size: 18px; color: #666;">"Count: " {count}</p>
                <button 
                    on:click=move |_| set_count.set(count.get() + 1)
                    style="padding: 10px 20px; font-size: 16px; background: #4ecdc4; color: white; border: none; border-radius: 5px; cursor: pointer; margin: 5px;"
                >
                    "Increment"
                </button>
                <button 
                    on:click=move |_| set_count.set(count.get() - 1)
                    style="padding: 10px 20px; font-size: 16px; background: #ff6b6b; color: white; border: none; border-radius: 5px; cursor: pointer; margin: 5px;"
                >
                    "Decrement"
                </button>
            </div>
            
            <div style="margin: 20px 0;">
                <button 
                    on:click=move |_| set_visible.set(!is_visible.get())
                    style="padding: 10px 20px; font-size: 16px; background: #45b7d1; color: white; border: none; border-radius: 5px; cursor: pointer;"
                >
                    "Toggle Visibility"
                </button>
            </div>
            
            <div style="margin: 20px 0;">
                {move || if is_visible.get() {
                    view! {
                        <div style="width: 200px; height: 200px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 10px; margin: 20px auto; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; font-size: 18px; transition: all 0.3s ease;">
                            "Hello Leptos!"
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }}
            </div>
            
            <div style="margin: 20px 0; padding: 15px; background: #f8f9fa; border-radius: 8px; border-left: 4px solid #4ecdc4;">
                <h3 style="margin: 0 0 10px 0; color: #333;">"Status"</h3>
                <p style="margin: 0; color: #666;">
                    "✅ Leptos is working!" <br/>
                    "✅ Reactive signals are functional!" <br/>
                    "✅ Event handling works!" <br/>
                    "✅ Conditional rendering works!" <br/>
                    "✅ No time API issues!"
                </p>
            </div>
        </div>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(BasicDemo);
}
