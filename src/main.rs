mod api;
mod app;
mod data;
mod types;

use app::App;
use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Load an artist's releases into the database.
    Load { artist_id: u32 },
    /// Check for new music from all the artists in the database.
    Check,
    /// List all the artists in the database.
    List,
    /// Generate a playlist with all the latest releases.
    GenPlaylist,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let app = App::init()?;

    match args.command {
        Command::Load { artist_id } => app.load_artist(artist_id).await?,
        Command::Check => app.check_for_new_releases().await?,
        Command::List => app.list_artists()?,
        Command::GenPlaylist => app.gen_playlist().await?,
    }

    Ok(())
}
