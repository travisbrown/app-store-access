use crate::request::params::developer::DeveloperId;
use app_store_access::{country::Country, language::Language};
use regex::Regex;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

const BASE_URL: &str = "https://play.google.com";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Developer {
    pub developer_id: DeveloperId,
    pub language: Language,
    pub country: Country,
}

impl Developer {
    #[must_use]
    pub const fn new(developer_id: DeveloperId, language: Language, country: Country) -> Self {
        Self {
            developer_id,
            language,
            country,
        }
    }
}

impl Display for Developer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/store/apps/{}?id={}&hl={}&gl={}",
            BASE_URL,
            self.developer_id.endpoint_path_part(),
            self.developer_id,
            self.language,
            self.country,
        )
    }
}

impl FromStr for Developer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static DEVELOPER_URL_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^https://play.google.com/store/apps/(dev|developer)\?id=([^&]+)&hl=([a-z]{2})&gl=([a-z]{2})").unwrap()
        });

        DEVELOPER_URL_RE
            .captures(s)
            .and_then(|captures| {
                captures
                    .get(1)
                    .zip(captures.get(2))
                    .zip(captures.get(3))
                    .zip(captures.get(4))
            })
            .and_then(
                |(((path_part_match, id_match), language_match), country_match)| {
                    let developer_id = match path_part_match.as_str() {
                        "dev" => id_match
                            .as_str()
                            .parse::<u64>()
                            .ok()
                            .map(DeveloperId::Numeric),
                        "developer" => Some(DeveloperId::Name(id_match.as_str().to_string())),
                        _ => None,
                    }?;
                    let language = language_match.as_str().parse().ok()?;
                    let country = country_match.as_str().parse().ok()?;

                    Some(Self {
                        developer_id,
                        language,
                        country,
                    })
                },
            )
            .ok_or_else(|| s.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pagination {
    pub language: Language,
    pub country: Country,
}

impl Pagination {
    #[must_use]
    pub const fn new(language: Language, country: Country) -> Self {
        Self { language, country }
    }
}

impl Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{BASE_URL}/_/PlayStoreUi/data/batchexecute?rpcids=qnKhOb&f.sid=-697906427155521722&bl=boq_playuiserver_20190903.08_p0&hl={}&gl={}&authuser&soc-app=121&soc-platform=1&soc-device=1&_reqid=1065213",
            self.language, self.country
        )
    }
}

impl FromStr for Pagination {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static LANGUAGE_AND_COUNTRY_RE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"hl=([a-z]{2}).*gl=([a-z]{2})").unwrap());

        LANGUAGE_AND_COUNTRY_RE
            .captures(s)
            .and_then(|captures| captures.get(1).zip(captures.get(2)))
            .and_then(|(language_match, country_match)| {
                language_match
                    .as_str()
                    .parse()
                    .ok()
                    .zip(country_match.as_str().parse().ok())
            })
            .map(|(language, country)| Self { language, country })
            .ok_or_else(|| s.to_string())
    }
}
