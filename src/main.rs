use clap::{Parser, Subcommand};
use anyhow::Result;
use std::sync::Arc;
use axum::Router as AxumRouter;
use ultralisk::db::{Repository, DbStorage};
use ultralisk::core::interfaces::Storage;
use std::path::PathBuf;

fn default_database_path() -> String {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".openzerg")
        .join("openzerg.db")
        .to_string_lossy()
        .to_string()
}

#[derive(Parser)]
#[command(name = "ultralisk")]
#[command(about = "OpenZerg Agent Server")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Start the server")]
    Serve {
        #[arg(short = 'H', long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value_t = 15317)]
        port: u16,
        #[arg(short, long, default_value_t = default_database_path())]
        database: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { host, port, database } => {
            let repository = Repository::new(&database).await?;
            let db = repository.connection().clone();
            let storage: Arc<dyn Storage> = Arc::new(DbStorage::new(Arc::new(db)));
            
            let connect_router = ultralisk::api::create_router(storage).await;

            let app = AxumRouter::new().fallback_service(connect_router.into_axum_service());

            let addr = format!("{}:{}", host, port);
            let listener = tokio::net::TcpListener::bind(&addr).await?;

            println!("Starting OpenZerg server...");
            println!("[Connect] Server running at http://{}", addr);
            println!("[Connect] API: http://{}/openzerg.Agent/", addr);
            println!("Server ready. Press Ctrl+C to stop.");

            axum::serve(listener, app).await?;
            println!("\nShutting down...");
        }
    }

    Ok(())
}