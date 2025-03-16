use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ReleaseType {
    Album,
    Compilation,
    Download,
    EpSingle,
    Live,
    Other,
}

impl ToSql for ReleaseType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let rep = match self {
            Self::Album => "Album",
            Self::Compilation => "Compilation",
            Self::Download => "Download",
            Self::EpSingle => "EpSingle",
            Self::Live => "Live",
            Self::Other => "Other",
        };
        Ok(ToSqlOutput::from(rep))
    }
}

impl FromSql for ReleaseType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "Album" => Ok(Self::Album),
            "Compilation" => Ok(Self::Compilation),
            "Download" => Ok(Self::Download),
            "EpSingle" => Ok(Self::EpSingle),
            "Live" => Ok(Self::Live),
            "Other" => Ok(Self::Other),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
