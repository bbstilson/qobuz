use crate::data::db::Db;

#[derive(Debug)]
pub struct Playlist {
    pub id: u32,
    pub name: String,
}

const INSERT: &str = "
insert into playlists (id, name)
values (?1, ?2)
on conflict (id) do nothing;
";

pub fn insert(db: &Db, playlist: &Playlist) -> anyhow::Result<()> {
    let Playlist { id, name, .. } = playlist;
    db.conn.execute(INSERT, (id, name))?;

    Ok(())
}
