use std::time::Duration;

use axum::{http::StatusCode, response::IntoResponse, routing::get, BoxError, Router};
use tokio::{net::TcpListener, time::sleep};
use tower::ServiceBuilder;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::DEBUG)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // The timeout can be handled with a 504 like this
    // let services = tower::ServiceBuilder::new()
    //     .layer(TraceLayer::new_for_http())
    //     .layer(axum::error_handling::HandleErrorLayer::new(handle_error))
    //     .timeout(Duration::from_secs(3));

    let services = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(3)));

    let app = Router::new().route("/", get(index)).layer(services);

    let address = "127.0.0.1:3000".to_string();
    println!("Hosting on http://{address}");

    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn index() -> impl IntoResponse {
    sleep(Duration::from_secs(5)).await;

    "Hello world!".to_string()
}

async fn handle_error(err: BoxError) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::GATEWAY_TIMEOUT, "Request Timeout".to_string());
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error".to_string(),
    )
}
