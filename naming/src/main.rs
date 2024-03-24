mod routes;

use std::future::IntoFuture;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let service_app = routes::service_routes();
    let service_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", service_port)).await?;

    let registration_app = routes::registration_routes();
    let registration_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", registration_port)).await?;

    let _ = tokio::join!(
        axum::serve(service_listener, service_app).into_future(),
        axum::serve(registration_listener, registration_app).into_future(),
    );

    Ok(())
}
