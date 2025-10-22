use crate::request::params::developer::DeveloperId;
use app_store_access::model::ImageType;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppMetadata {
    pub app_id: String,
    pub developer_id: DeveloperId,
}

impl AppMetadata {
    #[must_use]
    pub const fn new(app_id: String, developer_id: DeveloperId) -> Self {
        Self {
            app_id,
            developer_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct App {
    pub app_id: String,
    pub developer: super::developer::Developer,
    pub released: Option<DateTime<Utc>>,
    pub price: Option<super::Price>,
    pub title: String,
    pub description: String,
    pub summary: String,
    pub genre: Genre,
    pub content_rating: super::ContentRating,
    pub images: Images,
}

impl App {
    #[must_use]
    pub fn metadata(&self) -> AppMetadata {
        AppMetadata::new(self.app_id.clone(), self.developer.id.clone())
    }
}

impl<'de> serde::de::Deserialize<'de> for App {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal = super::full::App::deserialize(deserializer)?;
        // TODO: Could avoid some allocations here.
        let developer = internal.developer().map_err(|developer_id| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&developer_id),
                &"Google developer ID",
            )
        })?;
        let price = internal.price();

        Ok(Self {
            app_id: internal.id,
            developer,
            released: internal
                .released
                .map(|released| {
                    i64::try_from(released.timestamp_s)
                        .ok()
                        .and_then(|timestamp_s| DateTime::from_timestamp(timestamp_s, 0))
                        .ok_or_else(|| {
                            serde::de::Error::invalid_value(
                                serde::de::Unexpected::Unsigned(released.timestamp_s),
                                &"epoch timestamp second",
                            )
                        })
                })
                .map_or(Ok(None), |value| value.map(Some))?,
            price,
            title: internal.title,
            description: internal.description,
            summary: internal.summary,
            genre: Genre {
                id: internal.genre.id,
                name: internal.genre.name,
            },
            content_rating: super::ContentRating::new(
                internal.content_rating.name,
                internal.content_rating.description.map(|(_, value)| value),
            ),
            images: Images {
                icon: internal.icon,
                header: internal.header_image,
                screenshots: internal.screenshots,
            },
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Images {
    pub icon: String,
    pub header: String,
    pub screenshots: Vec<String>,
}

impl Images {
    pub fn urls(&self) -> impl Iterator<Item = (&str, ImageType)> {
        std::iter::once((self.icon.as_str(), ImageType::Icon))
            .chain(std::iter::once((self.header.as_str(), ImageType::Header)))
            .chain(
                self.screenshots
                    .iter()
                    .map(|url| (url.as_str(), ImageType::Screenshot)),
            )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Genre {
    pub id: String,
    pub name: String,
}
