use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum ReleaseType {
    Album,
    Compilation,
    Download,
    EpSingle,
    Live,
    AwardedReleases,
    Other,
}

impl ReleaseType {
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Album => "Album",
            Self::Compilation => "Compilation",
            Self::Download => "Download",
            Self::EpSingle => "EpSingle",
            Self::Live => "Live",
            Self::AwardedReleases => "AwardedReleases",
            Self::Other => "Other",
        }
    }
}

impl ToSql for ReleaseType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.to_str()))
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
            "AwardedReleases" => Ok(Self::AwardedReleases),
            "Other" => Ok(Self::Other),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
