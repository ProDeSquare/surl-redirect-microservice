mod db;

use axum::{extract::{Path, State}, http::StatusCode, response::{Html, Redirect}, routing::get, Router};
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

async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}

async fn test_slug(State(pool): State<Pool>, Path(slug): Path<String>) -> Result<Redirect, StatusCode> {
    let client = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = client
        .query_opt("SELECT id, url, enabled FROM shorts WHERE hash = $1", &[&slug])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => {
            let enabled: bool = row.get("enabled");
            let url: String = row.get("url");

            if !enabled {
                Err(StatusCode::NOT_FOUND)
            } else {
                Ok(Redirect::temporary(&url))
            }
        }
        None => Err(StatusCode::NOT_FOUND)
    }
}
