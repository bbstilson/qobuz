use anyhow::Context;
use reqwest::header::HeaderMap;

use crate::api::models::{AlbumPage, ArtistPage, NewPlaylist, Track, Tracks};

const USER_AGENT: &str = "QobuzBot/0.1 (+bbmusic@fastmail.com; API-access)";
const DEFAULT_API_BASE: &str = "https://www.qobuz.com/api.json/0.2";

// API Paths
const ARTIST_PAGE: &str = "artist/page";
const ALBUM_GET: &str = "album/get";

pub struct Api {
    api_base: String,
    client: reqwest::Client,
}

impl Api {
    pub fn new(auth_token: &str, app_id: &str) -> anyhow::Result<Self> {
        let api_base = std::env::var("QOBUZ_API_BASE").unwrap_or(DEFAULT_API_BASE.to_string());
        let headers = HeaderMap::from_iter([
            ("User-Agent".parse()?, USER_AGENT.parse()?),
            ("X-User-Auth-Token".parse()?, auth_token.parse()?),
            ("X-App-Id".parse()?, app_id.parse()?),
        ]);

        let client = reqwest::Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()?;

        Ok(Self { api_base, client })
    }

    pub async fn get_artist_page(&self, artist_id: u32) -> anyhow::Result<ArtistPage> {
        let request = self
            .client
            .get(format!("{}/{ARTIST_PAGE}", self.api_base))
            .query(&[("artist_id", artist_id.to_string())]);

        let response = request.send().await?.json::<ArtistPage>().await?;
        Ok(response)
    }

    pub async fn get_release_tracks(&self, release_id: &str) -> anyhow::Result<Vec<Track>> {
        let query = &[
            ("album_id", release_id),
            ("offset", "0"),
            ("limit", "50"),
            ("extra", "track_ids"),
        ];

        let request = self
            .client
            .get(format!("{}/{ALBUM_GET}", self.api_base))
            .query(query);

        let response = request.send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            // Sometimes Qobuz makes an album that they themselves cannot find.
            return Ok(vec![]);
        }

        let response = response.json::<serde_json::Value>().await?;
        let AlbumPage { tracks } =
            serde_json::from_value::<AlbumPage>(response).context("decoding album page")?;
        let Tracks { items } = tracks;

        Ok(items)
    }

    pub async fn create_playlist(&self, name: &str, track_ids: Vec<u32>) -> anyhow::Result<u32> {
        let form = [
            ("name", name),
            ("description", ""),
            ("is_public", "false"),
            ("is_collaborative", "false"),
        ];
        let request = self
            .client
            .post(format!("{}/playlist/create", self.api_base))
            .form(&form);

        let NewPlaylist { id } = request.send().await?.json::<NewPlaylist>().await?;
        let playlist_id = id.to_string();

        let track_ids = track_ids
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let form = [
            ("no_duplicate", "true"),
            ("playlist_id", playlist_id.as_str()),
            ("track_ids", track_ids.as_str()),
        ];
        let request = self
            .client
            .post(format!("{}/playlist/addTracks", self.api_base))
            .form(&form);

        request.send().await?;

        Ok(id)
    }
}
