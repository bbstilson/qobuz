use std::collections::HashMap;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers};

use rand::Rng;

const ARTIST_PAGE_0_RESPONSE: &str = include_str!("responses/artist_page_0.json");
const ARTIST_PAGE_1_RESPONSE: &str = include_str!("responses/artist_page_1.json");
const ALBUM_PAGE_RESPONSE: &str = include_str!("responses/album_page.json");
const PLAYLIST_CREATE_RESPONSE: &str = include_str!("responses/playlist_create.json");

pub struct Test {
    db_path: String,
    pub vars: HashMap<&'static str, String>,
    _mock_server: MockServer,
}

impl Test {
    pub async fn init() -> Self {
        let mock_server = init_server().await;
        let api_base = mock_server.uri();
        let db_path = mk_db_name();
        let rand_str = mk_rand_str();
        Self {
            db_path: db_path.clone(),
            vars: HashMap::from([
                ("QOBUZ_DB_PATH", db_path.clone()),
                ("QOBUZ_AUTH_TOKEN", rand_str.clone()),
                ("QOBUZ_APP_ID", rand_str),
                ("QOBUZ_API_BASE", api_base),
            ]),
            _mock_server: mock_server,
        }
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        _ = std::fs::remove_file(self.db_path.clone());
    }
}

async fn init_server() -> MockServer {
    let mock_server = MockServer::start().await;

    // GET Artist Page. Responds once.
    let artist_page_response_body = load_json_response(ARTIST_PAGE_0_RESPONSE);
    let artist_page_response = ResponseTemplate::new(200).set_body_json(artist_page_response_body);
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/artist/page"))
        .and(matchers::query_param("artist_id", "13925362"))
        .respond_with(artist_page_response)
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // GET Artist Page Update. Responds once.
    let artist_page_response_body = load_json_response(ARTIST_PAGE_1_RESPONSE);
    let artist_page_response = ResponseTemplate::new(200).set_body_json(artist_page_response_body);
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/artist/page"))
        .and(matchers::query_param("artist_id", "13925362"))
        .respond_with(artist_page_response)
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // GET Album Page. Responds once.
    let album_page_response_body = load_json_response(ALBUM_PAGE_RESPONSE);
    let album_page_response = ResponseTemplate::new(200).set_body_json(album_page_response_body);
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/album/get"))
        .and(matchers::query_param_contains("album_id", "na99v5xa7s26a"))
        .respond_with(album_page_response)
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // POST Playlist Create. Responds once.
    let playlist_create_response_body = load_json_response(PLAYLIST_CREATE_RESPONSE);
    let playlist_create_response =
        ResponseTemplate::new(200).set_body_json(playlist_create_response_body);
    Mock::given(matchers::method("POST"))
        .and(matchers::path("/playlist/create"))
        .and(matchers::header(
            "Content-Type",
            "application/x-www-form-urlencoded",
        ))
        .respond_with(playlist_create_response)
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    mock_server
}

fn load_json_response(path: &str) -> serde_json::Value {
    serde_json::from_str(path).unwrap()
}

pub fn mk_db_name() -> String {
    format!("{}.db3", mk_rand_str())
}

pub fn mk_rand_str() -> String {
    rand::rng()
        .sample_iter(rand::distr::Alphabetic)
        .take(10)
        .map(char::from)
        .collect()
}
