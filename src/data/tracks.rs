use crate::{api::models::Track, data::db::Db};

const INSERT_TRACK: &str = "
insert into tracks (id, title)
values (?1, ?2)
on conflict (id) do nothing;
";

const INSERT_TRACK_2_RELEASE: &str = "
insert into tracks_2_releases (release_id, track_id)
values (?1, ?2)
on conflict (release_id, track_id) do nothing;
";

#[tracing::instrument(skip(db))]
pub fn insert_batch(db: &Db, release_id: &str, tracks: Vec<Track>) -> anyhow::Result<()> {
    let mut track_stmt = db.conn.prepare(INSERT_TRACK)?;
    let mut track_2_release_stmt = db.conn.prepare(INSERT_TRACK_2_RELEASE)?;
    for track in tracks {
        track_stmt.execute((track.id, track.title))?;
        track_2_release_stmt.execute((release_id, track.id))?;
    }
    Ok(())
}

const GET_LATEST: &str = "
select t.id from tracks t
join tracks_2_releases t2r on t2r.track_id = t.id
join releases r on r.id = t2r.release_id
where r.created_at >= (
    select created_at from playlists
    order by created_at desc
    limit 1
);
";

/// Gets all tracks that haven't been loaded into a playlist.
#[tracing::instrument(skip(db))]
pub fn get_latest(db: &Db) -> anyhow::Result<Vec<u32>> {
    let mut stmt = db.conn.prepare(GET_LATEST)?;
    let latest_tracks = stmt.query_map([], |row| row.get(0)).unwrap();
    let latest_track_ids = latest_tracks.map(|a| a.unwrap()).collect();
    Ok(latest_track_ids)
}
