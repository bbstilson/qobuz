use rusqlite::Connection;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new(db_path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)?;

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
    created_at timestamp default (datetime('now', 'localtime')) not null,
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

create table if not exists tracks (
    id integer primary key,
    title text not null
);

create table if not exists tracks_2_releases (
    release_id text not null,
    track_id integer not null,
    primary key (release_id, track_id),
    foreign key (track_id) references tracks (id),
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

pub fn init(db: &Db) -> anyhow::Result<()> {
    db.conn.execute_batch(INIT_DB)?;
    Ok(())
}
