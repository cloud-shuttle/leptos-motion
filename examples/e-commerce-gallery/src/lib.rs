use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="ecommerce-container">
            <header class="ecommerce-header">
                <h1>"E-commerce Product Gallery"</h1>
                <div class="cart-icon">
                    <span class="cart-count">"0"</span>
                    "🛒"
                </div>
            </header>

            <main class="ecommerce-main">
                <ProductGallery />
                <ProductDetails />
            </main>
        </div>
    }
}

#[component]
fn ProductGallery() -> impl IntoView {
    let (current_image, set_current_image) = signal(0);
    let (is_zoomed, set_zoomed) = signal(false);

    let images = vec![
        "https://via.placeholder.com/400x400/667eea/ffffff?text=Product+1",
        "https://via.placeholder.com/400x400/764ba2/ffffff?text=Product+2",
        "https://via.placeholder.com/400x400/f093fb/ffffff?text=Product+3",
        "https://via.placeholder.com/400x400/f5576c/ffffff?text=Product+4",
    ];
    let images_len = images.len();
    let images_clone = images.clone();

    view! {
        <div class="product-gallery">
            <div class="main-image-container">
                <MotionDiv
                    class="main-image".to_string()
                    animate={
                        let mut target = HashMap::new();
                        target.insert("scale".to_string(), AnimationValue::Number(if is_zoomed.get() { 1.5 } else { 1.0 }));
                        target
                    }
                    transition=Transition {
                        duration: Some(0.3),
                        ease: Easing::EaseInOut,
                        ..Default::default()
                    }
                >
                    <img
                        src=images[current_image.get()]
                        alt="Product image"
                        on:click=move |_| set_zoomed.set(!is_zoomed.get())
                    />
                </MotionDiv>

                <div class="image-controls">
                    <button
                        class="nav-btn prev"
                        on:click=move |_| {
                            let new_index = if current_image.get() == 0 {
                                images_len - 1
                            } else {
                                current_image.get() - 1
                            };
                            set_current_image.set(new_index);
                        }
                    >
                        "‹"
                    </button>
                    <button
                        class="nav-btn next"
                        on:click=move |_| {
                            let new_index = (current_image.get() + 1) % images_len;
                            set_current_image.set(new_index);
                        }
                    >
                        "›"
                    </button>
                </div>
            </div>

            <div class="thumbnail-grid">
                {images_clone.into_iter().enumerate().map(|(i, src)| {
                    let is_active = move || current_image.get() == i;
                    view! {
                        <MotionDiv
                            class="thumbnail".to_string()
                            class:active=is_active
                            on:click=move |_| set_current_image.set(i)
                            while_hover={
                                let mut target = HashMap::new();
                                target.insert("scale".to_string(), AnimationValue::Number(1.1));
                                target
                            }
                            transition=Transition {
                                duration: Some(0.2),
                                ease: Easing::EaseOut,
                                ..Default::default()
                            }
                        >
                            <img src=src alt=format!("Thumbnail {}", i + 1) />
                        </MotionDiv>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[component]
fn ProductDetails() -> impl IntoView {
    let (quantity, set_quantity) = signal(1);
    let (is_added_to_cart, set_added_to_cart) = signal(false);
    let (price, set_price) = signal(99.99);

    let add_to_cart = move |_| {
        set_added_to_cart.set(true);
        // Reset after animation
        set_timeout_with_handle(
            move || set_added_to_cart.set(false),
            std::time::Duration::from_millis(2000),
        )
        .expect("Failed to set timeout");
    };

    view! {
        <div class="product-details">
            <h2>"Premium Product"</h2>
            <p class="description">
                "This is a high-quality product with amazing features. Perfect for everyday use and designed with user experience in mind."
            </p>

            <div class="price-section">
                <span class="price">"${price}"</span>
                <span class="original-price">"$129.99"</span>
                <span class="discount">"-23%"</span>
            </div>

            <div class="quantity-selector">
                <label>"Quantity:"</label>
                <div class="quantity-controls">
                    <button
                        on:click=move |_| {
                            if quantity.get() > 1 {
                                set_quantity.set(quantity.get() - 1);
                            }
                        }
                    >
                        "-"
                    </button>
                    <span class="quantity">{quantity}</span>
                    <button
                        on:click=move |_| set_quantity.set(quantity.get() + 1)
                    >
                        "+"
                    </button>
                </div>
            </div>

            <MotionDiv
                class="add-to-cart-btn".to_string()
                class:added=is_added_to_cart
                on:click=add_to_cart
                while_hover={
                    let mut target = HashMap::new();
                    target.insert("scale".to_string(), AnimationValue::Number(1.05));
                    target.insert("boxShadow".to_string(), AnimationValue::String("0 8px 25px rgba(102, 126, 234, 0.4)".to_string()));
                    target
                }
                while_tap={
                    let mut target = HashMap::new();
                    target.insert("scale".to_string(), AnimationValue::Number(0.95));
                    target
                }
                animate={
                    let mut target = HashMap::new();
                    target.insert("backgroundColor".to_string(), AnimationValue::String(
                        if is_added_to_cart.get() { "#4CAF50".to_string() } else { "#667eea".to_string() }
                    ));
                    target
                }
                transition=Transition {
                    duration: Some(0.3),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                {move || if is_added_to_cart.get() { "✓ Added to Cart!" } else { "Add to Cart" }}
            </MotionDiv>

            <div class="features">
                <h3>"Features"</h3>
                <ul>
                    <li>"High-quality materials"</li>
                    <li>"Durable construction"</li>
                    <li>"Easy to use"</li>
                    <li>"30-day warranty"</li>
                </ul>
            </div>
        </div>
    }
}

/// Mount the application
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");

    mount_to_body(App);
}
