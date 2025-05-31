use app::App;
use clap::Parser;

mod api;
pub mod app;
mod data;
mod types;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Load an artist's releases into the database.
    Load { artist_id: u32 },
    /// Check for new music from all the artists in the database.
    Check,
    /// List all the artists in the database.
    List,
    /// Generate a playlist with all the latest releases.
    GenPlaylist,
    /// Check for new music and put all the latest releases into a playlist.
    CheckGen,
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
        Command::CheckGen => {
            app.check_for_new_releases().await?;
            app.gen_playlist().await?;
        }
    }

    Ok(())
}
