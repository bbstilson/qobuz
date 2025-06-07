use std::rc::Rc;

use crate::types::ReleaseType;

use crate::data::db::Db;

#[derive(Debug, Clone)]
#[allow(clippy::struct_field_names)]
pub struct Release {
    pub id: String,
    pub title: String,
    pub release_type: ReleaseType,
}

const INSERT_RELEASE: &str = "
insert into releases (
    id,
    title,
    release_type_id
) values (?1, ?2, ?3)
on conflict (id) do nothing;
";

const INSERT_ARTIST_2_RELEASE: &str = "
insert into artists_2_releases (artist_id, release_id)
values (?1, ?2)
on conflict (artist_id, release_id) do nothing;
";

#[tracing::instrument(skip(db))]
pub fn insert_batch(db: &Db, artist_id: u32, releases: &[Release]) -> anyhow::Result<()> {
    let mut release_stmt = db.conn.prepare(INSERT_RELEASE)?;
    let mut artist_2_release_stmt = db.conn.prepare(INSERT_ARTIST_2_RELEASE)?;
    for release in releases {
        release_stmt.execute((
            release.id.clone(),
            release.title.clone(),
            release.release_type,
        ))?;
        artist_2_release_stmt.execute((artist_id, release.id.clone()))?;
    }
    Ok(())
}

const GET_ALL_IDS_FOR_ARTIST: &str = "
select id from releases r
join artists_2_releases a2r on a2r.release_id = r.id 
where a2r.artist_id = ?1;
";

#[tracing::instrument(skip(db))]
pub fn get_all_ids_for_artist(db: &Db, artist_id: u32) -> anyhow::Result<Vec<String>> {
    let mut stmt = db.conn.prepare(GET_ALL_IDS_FOR_ARTIST)?;
    let releases = stmt.query_map((artist_id,), |row| row.get(0))?;
    let result = releases.map(|a| a.unwrap()).collect();
    Ok(result)
}

const BULK_VERIFY: &str = "
update releases
set verified = true
where id in (
    select value from rarray(?1)
);
";

#[tracing::instrument(skip(db))]
pub fn bulk_verify(db: &Db, release_ids: &[String]) -> anyhow::Result<()> {
    let mut stmt = db.conn.prepare(BULK_VERIFY)?;
    let values = Rc::new(
        release_ids
            .iter()
            .cloned()
            .map(rusqlite::types::Value::from)
            .collect::<Vec<_>>(),
    );
    stmt.execute([values])?;
    Ok(())
}
