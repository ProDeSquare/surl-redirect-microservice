use axum::{Router, extract::Path, response::Html, routing::get};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/l/{slug}", get(test_slug));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Server up and running on port http://127.0.0.1:{}", port);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}

async fn test_slug(Path(slug): Path<String>) -> Html<String> {
    Html(format!("<p>{}</p>", slug))
}
