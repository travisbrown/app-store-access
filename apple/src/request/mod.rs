use app_store_access::{country::Country, language::Language};
use chrono::{DateTime, Utc};
use regex::Regex;
use scraper_trail::request::params::{Params, ParseError};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::LazyLock;

pub mod headers;
pub mod markets;
pub mod params;

const APP_URL: &str = "https://itunes.apple.com/us/app/app/id";
const SEARCH_URL: &str = "https://search.itunes.apple.com/WebObjects/MZStore.woa/wa/search?clientApplication=Software&media=software&term=";
const LOOKUP_URL: &str = "https://itunes.apple.com/lookup";

static REVIEWS_URL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^https://itunes.apple.com/(\w{2})/rss/customerreviews/page=(\d+)/id=(\d+)/sortby=([^/]+)/json$").unwrap()
});

static RATINGS_URL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^https://itunes.apple.com/(\w{2})/customer-reviews/id(\d+)?displayable-kind=11$")
        .unwrap()
});

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Request<'a> {
    App {
        id: u64,
        country: Country,
    },
    Search {
        query: Cow<'a, str>,
        country: Country,
        language: Language,
    },
    LookupIds {
        ids: Vec<u64>,
        country: Country,
        language: Language,
    },
    LookupBundleIds {
        bundle_ids: Vec<Cow<'a, str>>,
        country: Country,
        language: Language,
    },
    Reviews {
        id: u64,
        country: Country,
        sort: params::review::SortOrder,
        page: usize,
    },
    Ratings {
        id: u64,
        country: Country,
    },
}

impl<'a> Request<'a> {
    #[must_use]
    pub const fn app(id: u64, country: Country) -> Self {
        Self::App { id, country }
    }

    pub fn search<S: Into<Cow<'a, str>>>(query: S, country: Country, language: Language) -> Self {
        Self::Search {
            query: query.into(),
            country,
            language,
        }
    }

    pub fn lookup_ids<I: IntoIterator<Item = u64>>(
        ids: I,
        country: Country,
        language: Language,
    ) -> Self {
        Self::LookupIds {
            ids: ids.into_iter().collect(),
            country,
            language,
        }
    }

    pub fn lookup_bundle_ids<S: Into<Cow<'a, str>>, I: IntoIterator<Item = S>>(
        bundle_ids: I,
        country: Country,
        language: Language,
    ) -> Self {
        Self::LookupBundleIds {
            bundle_ids: bundle_ids
                .into_iter()
                .map(std::convert::Into::into)
                .collect(),
            country,
            language,
        }
    }

    #[must_use]
    pub const fn reviews(
        id: u64,
        country: Country,
        sort: params::review::SortOrder,
        page: usize,
    ) -> Self {
        Self::Reviews {
            id,
            country,
            sort,
            page,
        }
    }

    #[must_use]
    pub const fn ratings(id: u64, country: Country) -> Self {
        Self::Ratings { id, country }
    }

    fn url(&self) -> String {
        match self {
            Self::App { id, .. } => format!("{APP_URL}{id}"),
            Self::Search { query, .. } => {
                format!("{SEARCH_URL}{}", urlencoding::encode(query))
            }
            Self::LookupIds {
                ids,
                country,
                language,
            } => {
                format!(
                    "{LOOKUP_URL}?id={}&country={country}&entity=software&lang={language}",
                    ids.iter()
                        .map(|id| (*id).to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
            Self::LookupBundleIds {
                bundle_ids,
                country,
                language,
            } => {
                format!(
                    "{LOOKUP_URL}?bundleId={}&country={country}&entity=software&lang={language}",
                    bundle_ids
                        .iter()
                        .map(|id| (*id).to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
            Self::Reviews {
                id,
                country,
                sort,
                page,
            } => {
                format!(
                    "https://itunes.apple.com/{country}/rss/customerreviews/page={page}/id={id}/sortby={sort}/json"
                )
            }
            Self::Ratings { id, country } => {
                format!(
                    "https://itunes.apple.com/{country}/customer-reviews/id{id}?displayable-kind=11"
                )
            }
        }
    }

    fn headers(&self) -> headers::RequestHeaders {
        match self {
            Self::App { country, .. } => {
                let store_id: markets::MarketCode = (*country).into();
                let store_front =
                    crate::request::headers::AppleStoreFrontHeader::new(store_id.0, 29, None);

                crate::request::headers::RequestHeaders::new(Some(store_front), None)
            }
            Self::Search {
                country, language, ..
            } => {
                let store_id: markets::MarketCode = (*country).into();
                let store_front = crate::request::headers::AppleStoreFrontHeader::new(
                    store_id.0,
                    24,
                    Some("t:native".to_string()),
                );

                crate::request::headers::RequestHeaders::new(
                    Some(store_front),
                    Some(language.to_string()),
                )
            }
            Self::LookupIds { .. } | Self::LookupBundleIds { .. } | Self::Reviews { .. } => {
                crate::request::headers::RequestHeaders::default()
            }
            Self::Ratings { country, .. } => {
                let store_id: markets::MarketCode = (*country).into();
                let store_front =
                    crate::request::headers::AppleStoreFrontHeader::new(store_id.0, 12, None);

                crate::request::headers::RequestHeaders::new(Some(store_front), None)
            }
        }
    }

    const fn url_error() -> ParseError {
        ParseError::InvalidUrl {
            expected: "valid Apple URL",
        }
    }

    const fn header_error() -> ParseError {
        ParseError::Other {
            message: "valid Apple store front header",
        }
    }
}

impl bounded_static::IntoBoundedStatic for Request<'_> {
    type Static = Request<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::App { id, country } => Self::Static::App { id, country },
            Self::Search {
                query,
                country,
                language,
            } => Self::Static::Search {
                query: query.into_static(),
                country,
                language,
            },
            Self::LookupIds {
                ids,
                country,
                language,
            } => Self::Static::LookupIds {
                ids,
                country,
                language,
            },
            Self::LookupBundleIds {
                bundle_ids,
                country,
                language,
            } => Self::Static::LookupBundleIds {
                bundle_ids: bundle_ids.into_static(),
                country,
                language,
            },
            Self::Reviews {
                id,
                country,
                sort,
                page,
            } => Self::Static::Reviews {
                id,
                country,
                sort,
                page,
            },
            Self::Ratings { id, country } => Self::Static::Ratings { id, country },
        }
    }
}

impl bounded_static::ToBoundedStatic for Request<'_> {
    type Static = Request<'static>;

    fn to_static(&self) -> Self::Static {
        match self {
            Self::App { id, country } => Self::Static::App {
                id: *id,
                country: *country,
            },
            Self::Search {
                query,
                country,
                language,
            } => Self::Static::Search {
                query: query.to_static(),
                country: *country,
                language: *language,
            },
            Self::LookupIds {
                ids,
                country,
                language,
            } => Self::Static::LookupIds {
                ids: ids.clone(),
                country: *country,
                language: *language,
            },
            Self::LookupBundleIds {
                bundle_ids,
                country,
                language,
            } => Self::Static::LookupBundleIds {
                bundle_ids: bundle_ids.to_static(),
                country: *country,
                language: *language,
            },
            Self::Reviews {
                id,
                country,
                sort,
                page,
            } => Self::Static::Reviews {
                id: *id,
                country: *country,
                sort: *sort,
                page: *page,
            },
            Self::Ratings { id, country } => Self::Static::Ratings {
                id: *id,
                country: *country,
            },
        }
    }
}

impl<'a> Params for Request<'a> {
    fn build_request(
        &self,
        timestamp: Option<DateTime<Utc>>,
    ) -> scraper_trail::request::Request<'a> {
        let url = self.url();
        let headers = indexmap::IndexMap::from(&self.headers());
        let headers = if headers.is_empty() {
            None
        } else {
            Some(headers)
        };

        scraper_trail::request::Request::new::<_, _, _, _, String>(
            url, timestamp, None, headers, None,
        )
        .unwrap()
    }

    fn parse_request(request: &scraper_trail::request::Request<'_>) -> Result<Self, ParseError> {
        let url = &request.url;
        let apple_store_front_header = request
            .headers
            .get(headers::APPLE_STORE_FRONT_HEADER_NAME)
            .map(|header| {
                header
                    .parse::<headers::AppleStoreFrontHeader>()
                    .map_err(|_| Self::header_error())
            })
            .map_or(Ok(None), |value| value.map(Some))?;

        let country: Option<Country> =
            apple_store_front_header.map(|header| markets::MarketCode(header.store_id).into());

        let accept_language_header = request
            .headers
            .get(headers::ACCEPT_LANGUAGE_HEADER_NAME)
            .map(|header| header.parse::<Language>().map_err(|_| Self::header_error()))
            .map_or(Ok(None), |value| value.map(Some))?;

        match url.as_str().strip_prefix(APP_URL) {
            Some(id_str) => {
                let id = id_str.parse().map_err(|_| Self::url_error())?;

                Ok(Self::App {
                    id,
                    country: country.ok_or_else(Self::header_error)?,
                })
            }
            None => match url.as_str().strip_prefix(SEARCH_URL) {
                Some(query_str) => Ok(Self::Search {
                    query: query_str.to_string().into(),
                    country: country.ok_or_else(Self::header_error)?,
                    language: accept_language_header.ok_or_else(Self::header_error)?,
                }),
                None => match url.as_str().strip_prefix(LOOKUP_URL) {
                    Some(remainder) if remainder.starts_with("?id=") => {
                        let query_params = url.query_pairs().collect::<HashMap<_, _>>();

                        let ids = query_params
                            .get("id")
                            .and_then(|ids| ids.split(',').map(|id| id.parse().ok()).collect())
                            .ok_or_else(Self::url_error)?;

                        Ok(Self::LookupIds {
                            ids,
                            country: query_params
                                .get("country")
                                .and_then(|country| country.parse().ok())
                                .ok_or_else(Self::header_error)?,
                            language: query_params
                                .get("lang")
                                .and_then(|language| language.parse().ok())
                                .ok_or_else(Self::header_error)?,
                        })
                    }
                    Some(remainder) if remainder.starts_with("?bundleId=") => {
                        let query_params = url.query_pairs().collect::<HashMap<_, _>>();

                        let bundle_ids = query_params
                            .get("bundleId")
                            .map(|ids| ids.split(',').map(|id| Cow::from(id.to_string())).collect())
                            .ok_or_else(Self::url_error)?;

                        Ok(Self::LookupBundleIds {
                            bundle_ids,
                            country: query_params
                                .get("country")
                                .and_then(|country| country.parse().ok())
                                .ok_or_else(Self::header_error)?,
                            language: query_params
                                .get("lang")
                                .and_then(|language| language.parse().ok())
                                .ok_or_else(Self::header_error)?,
                        })
                    }
                    Some(_) => Err(Self::url_error()),
                    None => match REVIEWS_URL_RE.captures(url.as_str()) {
                        Some(captures) => {
                            let country = captures
                                .get(1)
                                .and_then(|country_code| country_code.as_str().parse().ok())
                                .ok_or_else(Self::url_error)?;

                            let page = captures
                                .get(2)
                                .and_then(|page| page.as_str().parse().ok())
                                .ok_or_else(Self::url_error)?;

                            let id = captures
                                .get(3)
                                .and_then(|id| id.as_str().parse().ok())
                                .ok_or_else(Self::url_error)?;

                            let sort = captures
                                .get(4)
                                .and_then(|sort| sort.as_str().parse().ok())
                                .ok_or_else(Self::url_error)?;

                            Ok(Self::Reviews {
                                id,
                                country,
                                sort,
                                page,
                            })
                        }
                        None => match RATINGS_URL_RE.captures(url.as_str()) {
                            Some(captures) => {
                                let country = captures
                                    .get(1)
                                    .and_then(|country_code| country_code.as_str().parse().ok())
                                    .ok_or_else(Self::url_error)?;

                                let id = captures
                                    .get(2)
                                    .and_then(|id| id.as_str().parse().ok())
                                    .ok_or_else(Self::url_error)?;

                                Ok(Self::Ratings { id, country })
                            }
                            None => Err(Self::url_error()),
                        },
                    },
                },
            },
        }
    }
}
