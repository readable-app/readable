#[cfg(feature = "local")]
use axum::{http::HeaderValue, routing::get, Router};

#[cfg(feature = "local")]
use readable::{readable, static_content};

#[cfg(feature = "local")]
use std::net::SocketAddr;

#[cfg(feature = "local")]
use anyhow::Result;

#[cfg(feature = "local")]
#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new()
        .route(
            "/static/Crimson.woff2",
            get(|| async {
                static_content(
                    include_bytes!("../static/fonts/Crimson.woff2",),
                    HeaderValue::from_static("text/woff2"),
                )
            }),
        )
        .route(
            "/static/JetBrainsMono.woff2",
            get(|| async {
                static_content(
                    include_bytes!("../static/fonts/JetBrainsMono.woff2",),
                    HeaderValue::from_static("font/woff2"),
                )
            }),
        )
        .fallback(get(readable));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

#[cfg(not(feature = "local"))]
fn main() {
    // dummy main function if shuttle is used
}
