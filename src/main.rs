extern crate core;

mod colors;
mod config;
mod download;
mod http_client;
mod launch;

use clap::Parser;
use std::process::exit;

#[derive(Parser)]
#[clap(
    name = "watercraft",
    about = "A simple CLI minecraft launcher",
    long_about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
enum Cli {
    #[clap(about = "Download a minecraft version")]
    Download {
        #[clap(help = "The version to download")]
        version: String,
    },
    #[clap(about = "Launches the game")]
    Launch {
        #[clap(help = "Version to launch")]
        version: String,
        #[clap(help = "Username to use")]
        username: String,
        #[clap(help = "Path to java to use", long = "java")]
        java: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args {
        Cli::Download { version } => {
            if let Err(e) = download::download(version).await {
                eprintln!("{red}{error}", red = colors::RED, error = e);
                exit(1);
            }
        }
        Cli::Launch {
            version,
            username,
            java,
        } => {
            if let Err(e) = launch::launch(version, username, java).await {
                eprintln!("{red}Error: {error}", red = colors::RED, error = e);
                exit(1);
            }
        }
    }
}
