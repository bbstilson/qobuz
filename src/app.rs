use std::collections::{HashMap, HashSet};

use anyhow::Context;
use indicatif::ProgressIterator;

use crate::{
    api::Api,
    data::{self, artists, db::Db, playlists, releases, tracks},
};

const DEFAULT_DB_NAME: &str = "music.db3";

pub struct App {
    db: Db,
    api: Api,
}

impl App {
    /// Initializes an App.
    /// # Errors
    /// Will return `Err` if there's an issue.
    pub fn init() -> anyhow::Result<Self> {
        let db_path = std::env::var("QOBUZ_DB_PATH").unwrap_or(DEFAULT_DB_NAME.to_string());
        let auth_token = std::env::var("QOBUZ_AUTH_TOKEN")?;
        let app_id = std::env::var("QOBUZ_APP_ID")?;

        let db = Db::new(&db_path)?;
        data::db::init(&db)?;

        let api = Api::new(&auth_token, &app_id)?;
        Ok(Self { db, api })
    }

    /// Loads an artist into the database.
    /// # Errors
    /// Will return `Err` if there's an issue.
    pub async fn load_artist(&self, artist_id: u32) -> anyhow::Result<()> {
        let artist_page = self.api.get_artist_page(artist_id).await?;

        println!("Loading data for '{}'", artist_page.name.display);

        artists::insert(
            &self.db,
            &artists::Artist {
                id: artist_page.id,
                name: artist_page.name.display,
            },
        )?;

        let rels = artist_page
            .releases
            .into_iter()
            .flat_map(|rels| {
                rels.items
                    .into_iter()
                    .map(move |release| releases::Release {
                        id: release.id,
                        title: release.title,
                        release_type: rels.release_type,
                    })
            })
            .collect::<Vec<_>>();

        let num_releases = rels.len();
        releases::insert_batch(&self.db, artist_id, &rels)?;
        println!("Loaded {num_releases} releases");

        Ok(())
    }

    /// Checks for new releases from artists in the database.
    /// # Errors
    /// Will return `Err` if there's an issue.
    pub async fn check_for_new_releases(&self) -> anyhow::Result<()> {
        let all_artists = artists::get_all(&self.db)?;
        println!("Checking {} artists\n", all_artists.len());
        let mut all_new_releases = HashMap::new();
        for artist in all_artists.iter().progress() {
            let existing_release_ids = releases::get_all_ids_for_artist(&self.db, artist.id)?
                .into_iter()
                .collect::<HashSet<_>>();

            let api_releases = self
                .api
                .get_artist_page(artist.id)
                .await?
                .releases
                .into_iter()
                .flat_map(|rels| rels.items.into_iter().map(move |r| (rels.release_type, r)))
                .map(|(r_type, r)| (r.id.clone(), (r_type, r)))
                .collect::<HashMap<_, _>>();

            let new_releases = api_releases
                .into_iter()
                .filter(|(release_id, _)| !existing_release_ids.contains(release_id))
                .map(|(_, release)| release)
                .collect::<Vec<_>>();

            if !new_releases.is_empty() {
                let rels = new_releases
                    .into_iter()
                    .map(|(release_type, release)| releases::Release {
                        id: release.id,
                        title: release.title,
                        release_type,
                    })
                    .collect::<Vec<_>>();

                all_new_releases.insert(artist, rels);
            }
        }

        if all_new_releases.is_empty() {
            println!("No new music found");
            return Ok(());
        }

        for (artist, new_releases) in all_new_releases {
            // Not all found releases are real. We need to wait until we
            // confirm the release tracks can be loaded. Sometimes releases
            // 404 or don't have tracks.
            releases::insert_batch(&self.db, artist.id, &new_releases)
                .context("releases::insert_batch")?;

            let mut loaded_releases = vec![];
            for release in new_releases {
                let tracks = self
                    .api
                    .get_release_tracks(&release.id)
                    .await
                    .context("api.get_release_tracks")?;

                if tracks.is_empty() {
                    continue;
                }

                loaded_releases.push(release.clone());
                tracks::insert_batch(&self.db, &release.id, tracks)
                    .context("tracks::insert_batch")?;
            }

            // All the releases were bogus. Go to the next artist.
            if loaded_releases.is_empty() {
                continue;
            }

            // Finalize the loaded releases.
            releases::bulk_verify(
                &self.db,
                &loaded_releases
                    .iter()
                    .map(|r| r.id.clone())
                    .collect::<Vec<_>>(),
            )
            .context("releases::bulk_verify")?;

            // Let the user know what happened.
            let num_releases = loaded_releases.len();
            let release_msg = if num_releases == 1 {
                "release"
            } else {
                "releases"
            };
            println!("Found {num_releases} new {release_msg} for {}", artist.name);
            let release_log = loaded_releases
                .iter()
                .map(|r| format!("\t • {}", r.title))
                .collect::<Vec<_>>()
                .join("\n");
            println!("{release_log}");
        }

        Ok(())
    }

    /// List artists in the database.
    /// # Errors
    /// Will return `Err` if there's an issue.
    pub fn list_artists(&self) -> anyhow::Result<()> {
        let mut artists = artists::get_all(&self.db)?
            .into_iter()
            .map(|a| a.name)
            .collect::<Vec<_>>();
        artists.sort_by_key(|a| a.to_lowercase());
        for artist in artists {
            println!("{artist}");
        }
        Ok(())
    }

    /// Generate a playlist for latest releases that haven't been put into a
    /// playlist.
    /// # Errors
    /// Will return `Err` if there's an issue.
    pub async fn gen_playlist(&self) -> anyhow::Result<()> {
        let name = chrono::Local::now().date_naive().to_string();
        let track_ids = tracks::get_latest(&self.db)?;

        if track_ids.is_empty() {
            println!("No new tracks. Skipping playlist creation");
            return Ok(());
        }

        let id = self.api.create_playlist(&name, track_ids).await?;
        playlists::insert(
            &self.db,
            &playlists::Playlist {
                id,
                name: name.clone(),
            },
        )
        .context("playlists::insert")?;

        println!("Created playlist: {name}");

        Ok(())
    }
}
