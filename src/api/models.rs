use serde::Deserialize;

use crate::types::ReleaseType;

#[derive(Deserialize, Debug)]
pub struct ArtistPage {
    pub id: u32,
    pub name: ArtistName,
    pub releases: Vec<ArtistRelease>,
}

#[derive(Deserialize, Debug)]
pub struct ArtistName {
    pub display: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ArtistRelease {
    #[serde(rename(deserialize = "type"))]
    pub release_type: ReleaseType,
    pub items: Vec<Release>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Release {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewPlaylist {
    pub id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AlbumPage {
    pub tracks: Tracks,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tracks {
    pub items: Vec<Track>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Track {
    pub id: i32,
    pub title: String,
}
