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

pub fn insert_batch(db: &Db, release_id: &str, tracks: Vec<Track>) -> anyhow::Result<()> {
    let mut track_stmt = db.conn.prepare(INSERT_TRACK)?;
    let mut track_2_release_stmt = db.conn.prepare(INSERT_TRACK_2_RELEASE)?;
    for track in tracks {
        track_stmt.execute((track.id, track.title))?;
        track_2_release_stmt.execute((release_id, track.id))?;
    }
    Ok(())
}

const GET_AFTER: &str = "
select t.id from tracks t
join tracks_2_releases t2r
on t2r.track_id = t.id
join releases r
on r.id = t2r.release_id
where r.created_at >= ?1;
";

/// Gets all tracks inserted into the database after the provided timestamp.
/// This timestamp *not* when the track was released. Just when it was added to
/// the database.
pub fn get_after(db: &Db, ts: String) -> anyhow::Result<Vec<i32>> {
    let mut stmt = db.conn.prepare(GET_AFTER)?;
    let releases = stmt.query_map((ts,), |row| row.get(0))?;
    let result = releases.map(|a| a.unwrap()).collect();
    Ok(result)
}
