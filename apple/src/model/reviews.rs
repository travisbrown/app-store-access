use crate::request::params::review::SortOrder;
use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc};
use serde_field_attributes::integer_str;
use std::borrow::Cow;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid sort order")]
    InvalidSortOrder(String),
    #[error("Missing self link")]
    MissingSelfLink,
    #[error("Invalid self link")]
    InvalidSelfLink(String),
    #[error("Missing last link")]
    MissingLastLink,
    #[error("Invalid last link")]
    InvalidLastLink(String),
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic)]
pub struct Page<'a> {
    pub feed: Feed<'a>,
    pub id: u64,
    pub country: app_store_access::country::Country,
    pub current_page: usize,
    pub last_page: Option<usize>,
    pub sort_order: SortOrder,
}

impl<'de> serde::de::Deserialize<'de> for Page<'_> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let page = InternalPage::deserialize(deserializer)?;
        let id = page.id().map_err(serde::de::Error::custom)?;
        let country = page.country().map_err(serde::de::Error::custom)?;
        let current_page = page.current_page().map_err(serde::de::Error::custom)?;
        let last_page = page.last_page().map_err(serde::de::Error::custom)?;
        let sort_order = page.sort_order().map_err(serde::de::Error::custom)?;

        Ok(Self {
            feed: page.feed,
            id,
            country,
            current_page,
            last_page,
            sort_order,
        })
    }
}

static SELF_URL_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(
        r"^https://itunes.apple.com/([a-z]{2})/rss/customerreviews/page=(\d+)/id=(\d+)/sortby=([a-zA-z]+)/json$"
    )
    .unwrap()
});

static LAST_URL_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(
        r"^https://itunes.apple.com/(?:[a-z]{2})/rss/customerreviews/page=(\d+)/id=(?:\d+)/",
    )
    .unwrap()
});

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct InternalPage<'a> {
    pub feed: Feed<'a>,
}

impl InternalPage<'_> {
    fn id(&self) -> Result<u64, Error> {
        let self_link = self.self_link()?;
        let self_link_href = &self_link.attributes.href;

        SELF_URL_RE
            .captures(self_link_href)
            .and_then(|captures| captures.get(3))
            .and_then(|m| m.as_str().parse().ok())
            .ok_or_else(|| Error::InvalidSelfLink(self_link_href.to_string()))
    }

    fn country(&self) -> Result<app_store_access::country::Country, Error> {
        let self_link = self.self_link()?;
        let self_link_href = &self_link.attributes.href;

        SELF_URL_RE
            .captures(self_link_href)
            .and_then(|captures| captures.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .ok_or_else(|| Error::InvalidSelfLink(self_link_href.to_string()))
    }

    fn current_page(&self) -> Result<usize, Error> {
        let self_link = self.self_link()?;
        let self_link_href = &self_link.attributes.href;

        SELF_URL_RE
            .captures(self_link_href)
            .and_then(|captures| captures.get(2))
            .and_then(|m| m.as_str().parse().ok())
            .ok_or_else(|| Error::InvalidSelfLink(self_link_href.to_string()))
    }

    fn sort_order(&self) -> Result<SortOrder, Error> {
        let self_link = self.self_link()?;
        let self_link_href = &self_link.attributes.href;

        SELF_URL_RE
            .captures(self_link_href)
            .and_then(|captures| captures.get(4))
            .and_then(|m| m.as_str().parse().ok())
            .ok_or_else(|| Error::InvalidSelfLink(self_link_href.to_string()))
    }

    fn last_page(&self) -> Result<Option<usize>, Error> {
        let last_link = self.last_link()?;
        let last_link_href = &last_link.attributes.href;

        if last_link_href.is_empty() {
            Ok(None)
        } else {
            LAST_URL_RE
                .captures(last_link_href)
                .and_then(|captures| captures.get(1))
                .and_then(|m| m.as_str().parse().ok())
                .ok_or_else(|| Error::InvalidLastLink(last_link_href.to_string()))
                .map(Some)
        }
    }

    fn self_link(&self) -> Result<&Link<'_>, Error> {
        self.feed
            .link
            .iter()
            .find(|link| link.attributes.rel == LinkRel::SelfLink)
            .ok_or(Error::MissingSelfLink)
    }

    fn last_link(&self) -> Result<&Link<'_>, Error> {
        self.feed
            .link
            .iter()
            .find(|link| link.attributes.rel == LinkRel::Last)
            .ok_or(Error::MissingLastLink)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Feed<'a> {
    pub id: TextLabel<'a>,
    pub icon: TextLabel<'a>,
    pub author: Author<'a>,
    pub entry: Option<EntryList<'a>>,
    pub link: Vec<Link<'a>>,
    pub title: TextLabel<'a>,
    pub rights: TextLabel<'a>,
    pub updated: TimestampLabel,
}

impl<'a> Feed<'a> {
    #[must_use]
    pub fn entries(&self) -> Vec<Entry<'a>> {
        self.entry
            .as_ref()
            .map(EntryList::entries)
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum EntryList<'a> {
    Singleton(Entry<'a>),
    Multi(Vec<Entry<'a>>),
}

impl<'a> EntryList<'a> {
    #[must_use]
    pub fn entries(&self) -> Vec<Entry<'a>> {
        match self {
            Self::Singleton(entry) => vec![entry.clone()],
            Self::Multi(entries) => entries.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TextLabel<'a> {
    pub label: Cow<'a, str>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct U8Label {
    #[serde(with = "integer_str")]
    pub label: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct U64Label {
    #[serde(with = "integer_str")]
    pub label: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TimestampLabel {
    pub label: DateTime<Utc>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypedLabel<'a> {
    pub label: Cow<'a, str>,
    pub attributes: TypeAttributes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeAttributes {
    #[serde(rename = "type")]
    label_type: LabelType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum LabelType {
    #[serde(rename = "text")]
    Text,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Author<'a> {
    pub name: TextLabel<'a>,
    pub uri: TextLabel<'a>,
    pub label: Option<Cow<'a, str>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Entry<'a> {
    pub id: U64Label,
    pub author: Author<'a>,
    pub updated: TimestampLabel,
    pub title: TextLabel<'a>,
    pub link: Link<'a>,
    #[serde(rename = "im:contentType")]
    pub content_type: ContentType,
    pub content: TypedLabel<'a>,
    #[serde(rename = "im:version")]
    pub version: TextLabel<'a>,
    #[serde(rename = "im:rating")]
    pub rating: U8Label,
    #[serde(rename = "im:voteCount")]
    pub vote_count: U64Label,
    #[serde(rename = "im:voteSum")]
    pub vote_sum: U64Label,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContentType {
    pub attributes: ContentTypeAttributes,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContentTypeAttributes {
    pub term: ContentTypeAttributesTerm,
    pub label: ContentTypeAttributesLabel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum ContentTypeAttributesTerm {
    Application,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum ContentTypeAttributesLabel {
    Application,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Link<'a> {
    pub attributes: LinkAttributes<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkAttributes<'a> {
    pub rel: LinkRel,
    pub href: Cow<'a, str>,
    #[serde(rename = "type")]
    pub link_type: Option<LinkType>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum LinkType {
    #[serde(rename = "text/html")]
    TextHtml,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum LinkRel {
    #[serde(rename = "related")]
    Related,
    #[serde(rename = "alternate")]
    Alternate,
    #[serde(rename = "self")]
    SelfLink,
    #[serde(rename = "first")]
    First,
    #[serde(rename = "last")]
    Last,
    #[serde(rename = "previous")]
    Previous,
    #[serde(rename = "next")]
    Next,
}
