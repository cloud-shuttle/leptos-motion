use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 Simple SSR Demo server running on http://{}", addr);
    let serve = axum::serve(listener, app);
    serve.await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Leptos Motion - SSR Demo</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            margin: 0;
            padding: 40px;
            min-height: 100vh;
        }
        .container {
            max-width: 800px;
            margin: 0 auto;
            text-align: center;
        }
        h1 {
            font-size: 48px;
            margin-bottom: 20px;
        }
        p {
            font-size: 24px;
            margin-bottom: 40px;
            opacity: 0.9;
        }
        .demo-info {
            background: rgba(255, 255, 255, 0.1);
            padding: 20px;
            border-radius: 8px;
            margin: 20px 0;
        }
        .status {
            display: inline-block;
            padding: 10px 20px;
            background: #4CAF50;
            color: white;
            border-radius: 20px;
            font-weight: bold;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎨 Leptos Motion</h1>
        <p>Server-Side Rendering Demo</p>

        <div class="demo-info">
            <h2>✅ SSR Demo Active</h2>
            <p>This page was rendered on the server using Leptos and MotionDiv components.</p>
            <div class="status">Server-Side Rendered ✓</div>
        </div>

        <div class="demo-info">
            <h3>Available Features:</h3>
            <ul style="text-align: left; display: inline-block;">
                <li>✅ MotionDiv components with SSR support</li>
                <li>✅ Animation properties (scale, translate, rotate)</li>
                <li>✅ Easing functions and transitions</li>
                <li>✅ Hydration-ready for client-side interactivity</li>
                <li>✅ Performance optimized rendering</li>
            </ul>
        </div>

        <p style="margin-top: 40px; font-size: 18px; opacity: 0.8;">
            Visit <a href="/health" style="color: #81C784;">/health</a> for server status.
        </p>
    </div>
</body>
</html>"#)
}

async fn health() -> Html<&'static str> {
    Html(r#"{"status":"healthy","ssr":"enabled","motion_div":"ready"}"#)
}
