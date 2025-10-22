mod db;

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Redirect},
    routing::get,
};
use db::init_pool;
use deadpool_postgres::Pool;
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

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
    Html("
        <style>
            body{background:#24252a;color:white;display:grid;height:100svh;place-content:center;text-align:center;font-family:sans-serif;}
            a{color:lightblue;text-decoration:none}
        </style>
        <h1>sURL - ProDeSquare</h1>
        <a href=\"https://github.com/ProDeSquare/sURL\">View on Github</a>
    ")
}

async fn test_slug(
    State(pool): State<Pool>,
    Path(slug): Path<String>,
) -> Result<Redirect, StatusCode> {
    let client = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = client
        .query_opt(
            "SELECT id, url, enabled FROM shorts WHERE hash = $1",
            &[&slug],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(row) => {
            let enabled: bool = row.get("enabled");
            let short_id: i64 = row.get("id");
            let url: String = row.get("url");

            if !enabled {
                Err(StatusCode::NOT_FOUND)
            } else {
                let _ = client
                    .execute("INSERT INTO clicks (short_id, created_at, updated_at) VALUES ($1, now(), now())", &[&short_id])
                .await;

                Ok(Redirect::temporary(&url))
            }
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
