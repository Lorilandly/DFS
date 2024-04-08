//! The naming service is responsible for storing the locations of the other services in the system.
//!
//! # Naming server
//!
//! You can run the naming server by running the following command:
//! ```bash
//! cargo run --bin naming <service_port> <registration_port>
//! ```
//!
//! # Failure senarios
//!
//! 1. When making requests to the storage servers, the response content is not checked.
//!    The storage servers may be down and the naming server will do nothing about it.
//! 2. When unlocking a file, naming server does not check if the file is previously locked.
//!    The result of doing so is undefined.
//! 3. The selection of storage servers is only partially random, so the files are not
//!    going to be distributed evenly across.
//! 4. When handling requests that involve modifying the fs tree, the entire storage is
//!    locked exclusively for the entire process. The storage should only be locked for the
//!    one command that perform the update.

mod exception_return;
mod logging;
mod models;
mod requests;
mod routes;

use axum::middleware;
use logging::print_request_response;
use std::{future::IntoFuture, sync::Arc};
use tokio::sync::RwLock;
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

    let dfs = Arc::new(RwLock::new(models::dfs::Dfs::default()));

    let service_app =
        routes::service_routes(dfs.clone()).layer(middleware::from_fn(print_request_response));
    let service_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", service_port)).await?;

    let registration_app =
        routes::registration_routes(dfs.clone()).layer(middleware::from_fn(print_request_response));
    let registration_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", registration_port)).await?;

    let _ = tokio::join!(
        axum::serve(service_listener, service_app).into_future(),
        axum::serve(registration_listener, registration_app).into_future(),
    );

    Ok(())
}
