use reqwest::header::HeaderMap;

use crate::api::models::ArtistPage;

use super::models::{AlbumPage, NewPlaylist, Track, Tracks};

const USER_AGENT: &str = "QobuzBot/0.1 (+bbmusic@fastmail.com; API-access)";
const API_BASE: &str = "https://www.qobuz.com/api.json/0.2";
const ARTIST_PAGE: &str = "artist/page";
const ALBUM_GET: &str = "album/get";

pub struct Api {
    client: reqwest::Client,
}

impl Api {
    pub fn new(auth_token: &str, app_id: &str) -> anyhow::Result<Self> {
        let headers = HeaderMap::from_iter([
            ("User-Agent".parse()?, USER_AGENT.parse()?),
            ("X-User-Auth-Token".parse()?, auth_token.parse()?),
            ("X-App-Id".parse()?, app_id.parse()?),
        ]);
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_artist_page(&self, artist_id: u32) -> anyhow::Result<ArtistPage> {
        let request = self
            .client
            .get(format!("{API_BASE}/{ARTIST_PAGE}"))
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
            .get(format!("{API_BASE}/{ALBUM_GET}"))
            .query(query);
        let AlbumPage { tracks } = request.send().await?.json::<AlbumPage>().await?;
        let Tracks { items } = tracks;

        Ok(items)
    }

    pub async fn create_playlist(&self, track_ids: Vec<i32>) -> anyhow::Result<String> {
        let now = chrono::Local::now().date_naive().to_string();
        let form = [
            ("name", now.as_str()),
            ("description", ""),
            ("is_public", "false"),
            ("is_collaborative", "false"),
        ];
        let request = self
            .client
            .post(format!("{API_BASE}/playlist/create"))
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
            .post(format!("{API_BASE}/playlist/addTracks"))
            .form(&form);

        request.send().await?;
        Ok(now)
    }
}
