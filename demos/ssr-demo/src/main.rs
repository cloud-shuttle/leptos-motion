use leptos::prelude::*;  // Brings in get_configuration
use leptos_axum::{generate_route_list, LeptosRoutes};
use axum::{
    Router,
    response::IntoResponse,
};
use ssr_demo::App;

#[tokio::main]
async fn main() {
    console_error_panic_hook::set_once();
    
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .fallback(leptos_axum::handle_server_fns_with_context(
            leptos_options.clone(),
            || {},
            move || view! { <App/> },
        ))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 SSR Demo server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn file_and_error_handler() -> impl IntoResponse {
    use axum::http::StatusCode;
    
    // Simple 404 response for now
    (StatusCode::NOT_FOUND, "Page not found")
}
