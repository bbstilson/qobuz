use anyhow::Result;
use reqwest::header::HeaderMap;

use crate::api::models::ArtistPage;

const USER_AGENT: &str = "QobuzBot/0.1 (+bbmusic@fastmail.com; API-access)";
const API_BASE: &str = "https://www.qobuz.com/api.json/0.2";
const ARTIST_PAGE: &str = "artist/page";

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

    pub async fn get_artist_page(&self, artist_id: u32) -> Result<ArtistPage> {
        let request = self
            .client
            .get(format!("{API_BASE}/{ARTIST_PAGE}"))
            .query(&[("artist_id", artist_id.to_string())]);

        let response = request.send().await?.json::<ArtistPage>().await?;
        Ok(response)
    }
}
