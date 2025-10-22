use crate::request::params::{developer::DeveloperId, review::SortOrder, search::PriceFilter};
use app_store_access::{country::Country, language::Language};
use chrono::{DateTime, Utc};
use scraper_trail::request::params::{Params, ParseError};
use std::{borrow::Cow, collections::HashMap};

const DOMAIN: &str = "play.google.com";
const BASE_URL: &str = "https://play.google.com";
const CONTENT_TYPE: &str = "application/x-www-form-urlencoded;charset=UTF-8";

pub mod body;
pub mod params;
pub mod url;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestData<'a> {
    Details {
        app_id: Cow<'a, str>,
    },
    Developer {
        developer_id: DeveloperId,
    },
    Search {
        query: Cow<'a, str>,
        price: Option<PriceFilter>,
    },
    Reviews {
        app_id: Cow<'a, str>,
        sort_order: SortOrder,
        number: usize,
        token: Option<Cow<'a, str>>,
    },
    Pagination {
        number: usize,
        token: Cow<'a, str>,
    },
}

impl bounded_static::IntoBoundedStatic for RequestData<'_> {
    type Static = RequestData<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::Details { app_id } => Self::Static::Details {
                app_id: app_id.into_static(),
            },
            Self::Developer { developer_id } => Self::Static::Developer { developer_id },
            Self::Search { query, price } => Self::Static::Search {
                query: query.into_static(),
                price,
            },
            Self::Reviews {
                app_id,
                sort_order,
                number,
                token,
            } => Self::Static::Reviews {
                app_id: app_id.into_static(),
                sort_order,
                number,
                token: token.into_static(),
            },
            Self::Pagination { number, token } => Self::Static::Pagination {
                number,
                token: token.into_static(),
            },
        }
    }
}

impl bounded_static::ToBoundedStatic for RequestData<'_> {
    type Static = RequestData<'static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Self::Details { app_id } => Self::Static::Details {
                app_id: app_id.to_static(),
            },
            Self::Developer { developer_id } => Self::Static::Developer {
                developer_id: developer_id.clone(),
            },
            Self::Search { query, price } => Self::Static::Search {
                query: query.to_static(),
                price: *price,
            },
            Self::Reviews {
                app_id,
                sort_order,
                number,
                token,
            } => Self::Static::Reviews {
                app_id: app_id.to_static(),
                sort_order: *sort_order,
                number: *number,
                token: token.to_static(),
            },
            Self::Pagination { number, token } => Self::Static::Pagination {
                number: *number,
                token: token.to_static(),
            },
        }
    }
}

#[derive(bounded_static_derive_more::ToStatic)]
pub struct Request<'a> {
    pub data: RequestData<'a>,
    pub language: Language,
    pub country: Country,
}

impl<'a> Request<'a> {
    pub fn details<S: Into<Cow<'a, str>>>(app_id: S, language: Language, country: Country) -> Self {
        Self {
            data: RequestData::Details {
                app_id: app_id.into(),
            },
            language,
            country,
        }
    }

    #[must_use]
    pub const fn developer(
        developer_id: DeveloperId,
        language: Language,
        country: Country,
    ) -> Self {
        Self {
            data: RequestData::Developer { developer_id },
            language,
            country,
        }
    }

    pub fn search<S: Into<Cow<'a, str>>>(
        query: S,
        price: Option<PriceFilter>,
        language: Language,
        country: Country,
    ) -> Self {
        Self {
            data: RequestData::Search {
                query: query.into(),
                price,
            },
            language,
            country,
        }
    }

    pub fn reviews<S: Into<Cow<'a, str>>>(
        app_id: S,
        sort_order: SortOrder,
        number: usize,
        token: Option<S>,
        language: Language,
        country: Country,
    ) -> Self {
        Self {
            data: RequestData::Reviews {
                app_id: app_id.into(),
                sort_order,
                number,
                token: token.map(std::convert::Into::into),
            },
            language,
            country,
        }
    }

    pub fn pagination<S: Into<Cow<'a, str>>>(
        number: usize,
        token: S,
        language: Language,
        country: Country,
    ) -> Self {
        Self {
            data: RequestData::Pagination {
                number,
                token: token.into(),
            },
            language,
            country,
        }
    }

    fn url(&self) -> String {
        match &self.data {
            RequestData::Details { app_id } => {
                format!(
                    "{BASE_URL}/store/apps/details?id={app_id}&hl={}&gl={}",
                    self.language, self.country
                )
            }
            RequestData::Developer { developer_id } => {
                format!(
                    "{BASE_URL}/store/apps/{}?id={developer_id}&hl={}&gl={}",
                    developer_id.endpoint_path_part(),
                    self.language,
                    self.country,
                )
            }
            RequestData::Search { query, price } => {
                format!(
                    "{BASE_URL}/work/search?q={query}&hl={}&gl={}&price={}",
                    self.language,
                    self.country,
                    price
                        .map(|price| price.code().to_string())
                        .unwrap_or_default()
                )
            }
            RequestData::Reviews { .. } | RequestData::Pagination { .. } => {
                url::Pagination::new(self.language, self.country).to_string()
            }
        }
    }

    fn body(&self) -> Option<String> {
        match &self.data {
            RequestData::Details { .. }
            | RequestData::Developer { .. }
            | RequestData::Search { .. } => None,
            RequestData::Reviews {
                app_id,
                sort_order,
                number,
                token,
            } => Some(
                crate::request::body::Review::new(
                    app_id.to_string(),
                    *sort_order,
                    *number,
                    token.as_deref().map(std::string::ToString::to_string),
                )
                .to_string(),
            ),
            RequestData::Pagination { number, token } => {
                Some(crate::request::body::Generic::new(*number, token.to_string()).to_string())
            }
        }
    }

    const fn url_error() -> ParseError {
        ParseError::InvalidUrl {
            expected: "Google Play URL",
        }
    }

    const fn body_error() -> ParseError {
        ParseError::InvalidBody {
            expected: "Google Play pagination request body",
        }
    }
}

impl<'a> Params for Request<'a> {
    fn build_request(
        &self,
        timestamp: Option<DateTime<Utc>>,
    ) -> scraper_trail::request::Request<'a> {
        let url = self.url();
        let body = self.body();
        let method = body.as_ref().map(|_| http::Method::POST);
        let headers = body
            .as_ref()
            .map(|_| [(reqwest::header::CONTENT_TYPE.as_str(), CONTENT_TYPE)]);

        scraper_trail::request::Request::new(url, timestamp, method, headers, body).unwrap()
    }

    fn parse_request(request: &scraper_trail::request::Request<'_>) -> Result<Self, ParseError> {
        let url = &request.url;
        let query_params = url.query_pairs().collect::<HashMap<_, _>>();

        let (language, country) = query_params
            .get("hl")
            .and_then(|hl| hl.parse().ok())
            .zip(query_params.get("gl").and_then(|hl| hl.parse().ok()))
            .ok_or(Self::url_error())?;

        if url.scheme() == "https" && url.domain() == Some(DOMAIN) {
            let data = match url.path() {
                "/store/apps/details" => RequestData::Details {
                    app_id: query_params
                        .get("id")
                        .ok_or(Self::url_error())?
                        .to_string()
                        .into(),
                },
                "/store/apps/dev" => RequestData::Developer {
                    developer_id: query_params
                        .get("id")
                        .and_then(|id| id.parse::<u64>().ok())
                        .map(DeveloperId::Numeric)
                        .ok_or(Self::url_error())?,
                },
                "/store/apps/developer" => RequestData::Developer {
                    developer_id: query_params
                        .get("id")
                        .map(|id| DeveloperId::Name(id.to_string()))
                        .ok_or(Self::url_error())?,
                },
                "/work/search" => RequestData::Search {
                    query: query_params
                        .get("q")
                        .ok_or(Self::url_error())?
                        .to_string()
                        .into(),
                    price: query_params
                        .get("price")
                        .and_then(|price| {
                            if price.is_empty() {
                                Some(None)
                            } else {
                                price.parse().ok().map(Some)
                            }
                        })
                        .ok_or(Self::url_error())?,
                },
                "/_/PlayStoreUi/data/batchexecute" => {
                    let body = request
                        .body
                        .as_ref()
                        .and_then(|body| body.parse::<body::Unknown>().ok())
                        .ok_or(Self::body_error())?;

                    match body {
                        body::Unknown::Review(review_body) => RequestData::Reviews {
                            app_id: review_body.app_id.into(),
                            sort_order: review_body.sort_order,
                            number: review_body.number,
                            token: review_body.token.map(std::convert::Into::into),
                        },
                        body::Unknown::Generic(generic_body) => RequestData::Pagination {
                            number: generic_body.number,
                            token: generic_body.token.into(),
                        },
                    }
                }
                _ => Err(Self::url_error())?,
            };

            Ok(Self {
                data,
                language,
                country,
            })
        } else {
            Err(Self::url_error())
        }
    }
}
