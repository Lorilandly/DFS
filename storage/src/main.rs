mod handlers;
mod routes;
use std::fs;
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

    let client_app = routes::client_routes();
    let client_listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", client_port)).await?;

    let command_app = routes::command_routes();
    let command_listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", command_port)).await?;

    let client = reqwest::blocking::Client::new();
    // {
    //     "storage_ip": "localhost",
    //     "client_port": 1111,
    //     "command_port": 2222,
    //     "files": [
    //         "/fileA",
    //         "/path/to/fileA",
    //         "/path/to/fileB",
    //         "/path/to/another/fileA"
    //     ]
    // }

    let paths = fs::read_dir(root_storage_dir).unwrap();
    let mut files = vec![];
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    let body = format!(
        "{{\"storage_ip\": \"localhost\", \"client_port\": {}, \"command_port\": {}, \"files\": {:?}}}",
        client_port, command_port, files
    );

    println!("\nn\n\n\n\nbody: {}", body);

    let _res = client
        .post(format!("http://localhost:{}/register", registration_port))
        .body(format!(
            "{{\"storage_ip\": \"localhost\", \"client_port\": {}, \"command_port\": {}, \"files\": {:?}}}",
            client_port, command_port, files
        ))
        .send()
        .unwrap();

    let _ = tokio::join!(
        axum::serve(client_listener, client_app).into_future(),
        axum::serve(command_listener, command_app).into_future(),
    );

    Ok(())
}
