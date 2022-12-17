extern crate core;

mod colors;
mod download;
mod http_client;
mod launch;

use clap::Parser;

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
        #[clap(help = "Password to use", long = "java")]
        java: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args {
        Cli::Download { version } => download::download(version).await.unwrap(),
        Cli::Launch {
            version,
            username,
            java,
        } => launch::launch(version, username, java).await.unwrap(),
    }
}
