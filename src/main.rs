mod db;

use axum::{Router, extract::{Path, State}, response::Html, routing::get};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use deadpool_postgres::Pool;
use db::init_pool;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = init_pool().await;

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/l/{slug}", get(test_slug))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Server up and running on port http://127.0.0.1:{}", port);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn root_handler(State(pool): State<Pool>) -> Html<String> {
    match pool.get().await {
        Ok(_) => Html("<h1>Database connected</h1>".to_string()),
        Err(err) => Html(format!("<h1>Database Error</h1><p>{}</p>", err)),
    }
}

async fn test_slug(Path(slug): Path<String>) -> Html<String> {
    Html(format!("<p>{}</p>", slug))
}
