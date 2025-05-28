use crate::data::db::Db;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Artist {
    pub id: u32,
    pub name: String,
}

const INSERT: &str = "
insert into artists (id, name) values (?1, ?2)
on conflict (id) do nothing;
";

pub fn insert(db: &Db, artist: &Artist) -> anyhow::Result<()> {
    db.conn.execute(INSERT, (&artist.id, &artist.name))?;

    Ok(())
}

const GET_ALL: &str = "select id, name from artists;";

pub fn get_all(db: &Db) -> anyhow::Result<Vec<Artist>> {
    let mut stmt = db.conn.prepare(GET_ALL)?;
    let artists = stmt.query_map([], |row| {
        Ok(Artist {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let result = artists.map(|a| a.unwrap()).collect();

    Ok(result)
}
