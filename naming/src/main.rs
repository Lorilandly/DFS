mod handlers;
mod logging;
mod routes;

use axum::middleware;
use logging::print_request_response;
use std::future::IntoFuture;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "naming=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run <service_port> <registration_port>");
        std::process::exit(1);
    }

    let service_port = args[1]
        .parse::<u16>()
        .expect("Failed to parse service port");
    let registration_port = args[2]
        .parse::<u16>()
        .expect("Failed to parse registration port");

    let service_app = routes::service_routes().layer(middleware::from_fn(print_request_response));
    let service_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", service_port)).await?;

    let registration_app =
        routes::registration_routes().layer(middleware::from_fn(print_request_response));
    let registration_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", registration_port)).await?;

    let _ = tokio::join!(
        axum::serve(service_listener, service_app).into_future(),
        axum::serve(registration_listener, registration_app).into_future(),
    );

    Ok(())
}
