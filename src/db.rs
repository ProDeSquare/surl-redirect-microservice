use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};
use std::env;

pub async fn init_pool() -> Pool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let cfg: Config = url.parse().expect("Invalid DATABASE_URL");
    let mgr = Manager::new(cfg, NoTls);

    Pool::builder(mgr)
        .max_size(16)
        .build()
        .expect("Failed to create DB pool")
}
