use crate::request::params::review::SortOrder;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub mod criterion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Page {
    pub reviews: Vec<Review>,
    pub token: Option<String>,
}

impl From<PageResponse> for Page {
    fn from(value: PageResponse) -> Self {
        Self {
            reviews: value.reviews,
            token: value.token,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageExchange {
    pub request: PageRequest,
    pub response: PageResponse,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageRequest {
    pub app_id: String,
    pub language: app_store_access::language::Language,
    pub country: app_store_access::country::Country,
    pub sort_order: SortOrder,
    pub number: usize,
    pub token: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageResponse {
    pub reviews: Vec<Review>,
    pub token: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub id: Option<u128>,
    pub display_name: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    pub id: Uuid,
    pub user: User,
    pub score: u8,
    pub timestamp: DateTime<Utc>,
    pub text: Option<String>,
    pub thumbs_up: Option<usize>,
    pub version: Option<String>,
    pub reply: Option<Reply>,
    pub criteria: Option<Vec<(criterion::CriterionType, u32)>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reply {
    pub display_name: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

impl<'de> serde::de::Deserialize<'de> for PageResponse {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal_page = internal::ReviewsPage::deserialize(deserializer)?;
        let (reviews, token) = internal_page.reviews_and_token();

        Ok(Self {
            reviews: reviews
                .into_iter()
                .map(internal::Review::into_review::<D::Error>)
                .collect::<Result<Vec<_>, D::Error>>()?,
            token,
        })
    }
}

mod internal {
    use chrono::{DateTime, Utc};
    use uuid::Uuid;

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
    #[serde(untagged)]
    pub enum ReviewsPage {
        WithToken(ReviewsPageWithToken),
        WithoutToken(ReviewsPageWithoutToken),
        Empty(),
    }

    impl ReviewsPage {
        pub fn reviews_and_token(self) -> (Vec<Review>, Option<String>) {
            match self {
                Self::WithToken(ReviewsPageWithToken { reviews, token }) => (reviews, Some(token)),
                Self::WithoutToken(ReviewsPageWithoutToken { reviews }) => (reviews, None),
                Self::Empty() => (vec![], None),
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde_query::Deserialize)]
    pub struct ReviewsPageWithoutToken {
        #[query(".[0].[]")]
        pub reviews: Vec<Review>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde_query::Deserialize)]
    pub struct ReviewsPageWithToken {
        #[query(".[0].[]")]
        pub reviews: Vec<Review>,
        #[query(".[1].[1]")]
        pub token: String,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde_query::Deserialize)]
    pub struct Review {
        #[query(".[0]")]
        pub id: String,
        #[query(".[1].[0]")]
        pub display_name: String,
        #[query(".[9].[0]")]
        pub user_id: Option<String>,
        #[query(".[1].[1].[3].[2]")]
        pub avatar_url: String,
        #[query(".[2]")]
        pub score: u8,
        #[query(".[5].[0]")]
        pub timestamp_s: u64,
        #[query(".[4]")]
        pub text: Option<String>,
        #[query(".[6]")]
        pub thumbs_up: Option<usize>,
        #[query(".[10]")]
        pub version: Option<String>,
        #[query(".[7]")]
        pub reply_section: Option<ReplySection>,
        #[query(".[12]")]
        pub criteria_section: Option<CriteriaSection>,
    }

    impl Review {
        pub fn into_review<E: serde::de::Error>(self) -> Result<super::Review, E> {
            Ok(super::Review {
                id: Uuid::parse_str(&self.id).map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Str(&self.id), &"UUID")
                })?,
                user: super::User {
                    id: self
                        .user_id
                        .map(|user_id_str| {
                            user_id_str.parse().map_err(|_| {
                                serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&user_id_str),
                                    &"Google user ID",
                                )
                            })
                        })
                        .map_or(Ok(None), |value| value.map(Some))?,
                    display_name: self.display_name,
                    avatar_url: self.avatar_url,
                },
                score: self.score,
                timestamp: timestamp_s_to_timestamp(self.timestamp_s)?,
                text: self.text,
                thumbs_up: self.thumbs_up,
                version: self.version,
                reply: self
                    .reply_section
                    .map(|reply_section| {
                        timestamp_s_to_timestamp(reply_section.timestamp_s).map(|timestamp| {
                            super::Reply {
                                display_name: reply_section.display_name,
                                text: reply_section.text,
                                timestamp,
                            }
                        })
                    })
                    .map_or(Ok(None), |value| value.map(Some))?,
                criteria: self.criteria_section.map(|criteria_section| {
                    criteria_section
                        .criteria
                        .into_iter()
                        .map(|criterion| (criterion.criterion_type(), criterion.rating()))
                        .collect()
                }),
            })
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde_query::Deserialize)]
    pub struct ReplySection {
        #[query(".[0]")]
        pub display_name: String,
        #[query(".[1]")]
        pub text: String,
        #[query(".[2].[0]")]
        pub timestamp_s: u64,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde_query::Deserialize)]
    pub struct CriteriaSection {
        #[query(".[0].[]")]
        pub criteria: Vec<Criterion>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
    #[serde(untagged)]
    pub enum Criterion {
        Two(crate::model::review::criterion::CriterionType, (u32,)),
        Three(
            crate::model::review::criterion::CriterionType,
            Option<()>,
            (u32,),
        ),
    }

    impl Criterion {
        const fn criterion_type(&self) -> crate::model::review::criterion::CriterionType {
            match self {
                Self::Two(criterion_type, ..) | Self::Three(criterion_type, ..) => *criterion_type,
            }
        }

        const fn rating(&self) -> u32 {
            match self {
                Self::Two(.., (rating,)) | Self::Three(.., (rating,)) => *rating,
            }
        }
    }

    fn timestamp_s_to_timestamp<E: serde::de::Error>(timestamp_s: u64) -> Result<DateTime<Utc>, E> {
        i64::try_from(timestamp_s)
            .ok()
            .and_then(|timestamp_s| DateTime::from_timestamp(timestamp_s, 0))
            .ok_or_else(|| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Unsigned(timestamp_s),
                    &"epoch second",
                )
            })
    }
}
