use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryEntry<'a> {
    #[serde(rename = "versionString")]
    pub version_string: Cow<'a, str>,
    #[serde(rename = "releaseDate")]
    pub release_date: DateTime<Utc>,
    #[serde(rename = "releaseNotes")]
    pub release_notes: Option<Cow<'a, str>>,
}
