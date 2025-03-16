use anyhow::Result;
use rusqlite::Connection;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("music.db3")?;

        Ok(Self { conn })
    }
}

pub const INIT_DB: &str = "
begin;

create table if not exists artists (
    id integer primary key,
    name text not null
);

create table if not exists releases (
    id text primary key,
    title text not null,
    release_type_id text not null,
    foreign key (release_type_id) references release_type (variant)
);

create table if not exists artists_2_releases (
    artist_id integer not null,
    release_id text not null,
    primary key (artist_id, release_id),
    foreign key (artist_id) references artists (id),
    foreign key (release_id) references releases (id)
);

create table if not exists release_type (
    variant text primary key
);

insert into release_type (variant) values
    ('Album'),
    ('Compilation'),
    ('Download'),
    ('EpSingle'),
    ('Live'),
    ('Other')
on conflict (variant) do nothing;

commit;
";

pub fn init(db: &Db) -> Result<()> {
    db.conn.execute_batch(INIT_DB)?;
    Ok(())
}
