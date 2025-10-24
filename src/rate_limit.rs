use axum::body::Body;
use governor::middleware::NoOpMiddleware;
use std::env;
use tower_governor::{
    GovernorLayer,
    governor::GovernorConfigBuilder,
    key_extractor::{GlobalKeyExtractor, SmartIpKeyExtractor},
};

const RATE_LIMITER_PER_SECOND: u64 = 10;
const RATE_LIMITER_BURST_SIZE: u32 = 20;

pub fn is_production() -> bool {
    env::var("ENV")
        .unwrap_or_else(|_| "development".to_string())
        .to_lowercase()
        == "production"
}

pub fn create_dev_limiter() -> GovernorLayer<GlobalKeyExtractor, NoOpMiddleware, Body> {
    println!("Rate Limiter: GlobalKeyExtractor (dev mode)");
    GovernorLayer::new(
        GovernorConfigBuilder::default()
            .key_extractor(GlobalKeyExtractor)
            .per_second(RATE_LIMITER_PER_SECOND)
            .burst_size(RATE_LIMITER_BURST_SIZE)
            .finish()
            .unwrap(),
    )
}

pub fn create_prod_limiter() -> GovernorLayer<SmartIpKeyExtractor, NoOpMiddleware, Body> {
    println!("Rate Limiter: SmartIpKeyExtractor (production mode)");
    GovernorLayer::new(
        GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(RATE_LIMITER_PER_SECOND)
            .burst_size(RATE_LIMITER_BURST_SIZE)
            .finish()
            .unwrap(),
    )
}

#[macro_export]
macro_rules! apply_rate_limiter {
    ($router:expr) => {{
        if rate_limit::is_production() {
            $router.layer(rate_limit::create_prod_limiter())
        } else {
            $router.layer(rate_limit::create_dev_limiter())
        }
    }};
}
