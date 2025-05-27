use std::collections::{HashMap, HashSet};

use crate::{
    api::Api,
    data::{self, artists, db::Db, releases, tracks},
};

const DEFAULT_DB_NAME: &str = "music.db3";

pub struct App {
    db: Db,
    api: Api,
}

impl App {
    pub fn init() -> anyhow::Result<Self> {
        let db_path = std::env::var("QOBUZ_DB_PATH").unwrap_or(DEFAULT_DB_NAME.to_string());
        let auth_token = std::env::var("QOBUZ_AUTH_TOKEN")?;
        let app_id = std::env::var("QOBUZ_APP_ID")?;

        let db = Db::new(&db_path)?;
        data::db::init(&db)?;

        let api = Api::new(&auth_token, &app_id)?;
        Ok(Self { db, api })
    }

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

    pub async fn check_for_new_releases(&self) -> anyhow::Result<()> {
        let all_artists = artists::get_all(&self.db)?;
        println!("Checking {} artists", all_artists.len());
        let mut new_music_found = false;
        for artist in all_artists {
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
                new_music_found = true;
                println!(
                    "Found {} new release(s) for {}!",
                    new_releases.len(),
                    artist.name
                );
                let rels = new_releases
                    .into_iter()
                    .map(|(release_type, release)| releases::Release {
                        id: release.id,
                        title: release.title,
                        release_type,
                    })
                    .collect::<Vec<_>>();

                for new_release in &rels {
                    println!("\t{}", &new_release.title);
                }

                releases::insert_batch(&self.db, artist.id, &rels)?;

                for release in &rels {
                    let tracks = self.api.get_release_tracks(&release.id).await?;
                    tracks::insert_batch(&self.db, &release.id, tracks)?;
                }
            }
        }

        if !new_music_found {
            println!("No new music found");
        }

        Ok(())
    }

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

    pub async fn gen_playlist(&self) -> anyhow::Result<()> {
        let now = chrono::Local::now().date_naive().to_string();
        let latest = tracks::get_after(&self.db, now)?;
        let name = self.api.create_playlist(latest).await?;
        println!("Created playlist: {name}");
        Ok(())
    }
}
