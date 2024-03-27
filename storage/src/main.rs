mod handlers;
mod routes;
mod storage;
use std::{future::IntoFuture, path::Path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage: cargo run <client_port> <command_port> <registration_port> <root_storage_dir>"
        );
        std::process::exit(1);
    }

    let client_port = args[1]
        .parse::<u16>()
        .expect("Failed to parse service port");
    let command_port = args[2]
        .parse::<u16>()
        .expect("Failed to parse registration port");
    let registration_port = args[3]
        .parse::<u16>()
        .expect("Failed to parse registration port");
    let root_storage_dir = Path::new(&args[4]);

    let storage = storage::Storage::new(root_storage_dir);
    storage
        .initialize_storage(
            client_port,
            command_port,
            registration_port,
            root_storage_dir,
        )
        .await?;

    let client_app = routes::client_routes();
    let client_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", client_port)).await?;

    let command_app = routes::command_routes();
    let command_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", command_port)).await?;

    let _ = tokio::join!(
        axum::serve(client_listener, client_app).into_future(),
        axum::serve(command_listener, command_app).into_future(),
    );

    Ok(())
}
