use bounded_static_derive_more::ToStatic;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct HintList<'a> {
    pub title: Cow<'a, str>,
    pub hints: Vec<Hint<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
pub struct Hint<'a> {
    pub term: Cow<'a, str>,
    pub url: Cow<'a, str>,
}
