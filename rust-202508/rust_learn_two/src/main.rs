mod logger;
use tracing::log::logger;
use axum::{debug_handler, routing, Router};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    logger::init();
    let router = Router::new().route("/", routing::get(async || "Hello, rust!"))
        .route("/hello", routing::get(async_hello));

    let listing = TcpListener::bind(("127.0.0.1", 8080)).await.unwrap();
    // println!("Serving on http://{}", listing.local_addr().unwrap());
    tracing::info!("Serving on http://{}", listing.local_addr().unwrap());
    axum::serve(listing,router).await.unwrap();


}

#[debug_handler]
async fn async_hello() -> &'static str {
    "Hello, zengtian!"
}