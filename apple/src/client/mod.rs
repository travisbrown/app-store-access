use crate::request::params::review::SortOrder;
use app_store_access::{country::Country, language::Language};
use bounded_static_derive_more::ToStatic;
use scraper_trail::client::{json_send, text_send};
use scraper_trail::request::params::Params;
use serde_json::Value;
use std::borrow::Cow;
use std::path::PathBuf;

pub mod suggest;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP client error")]
    Reqwest(#[from] reqwest::Error),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("Scraper client error")]
    ScraperClient(#[from] scraper_trail::client::Error),
    #[error("Header value error")]
    RequestHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, ToStatic)]
pub enum AppId<'a> {
    Track(u64),
    Bundle(Cow<'a, str>),
}

impl From<u64> for AppId<'static> {
    fn from(value: u64) -> Self {
        Self::Track(value)
    }
}

impl<'a> From<&'a str> for AppId<'a> {
    fn from(value: &'a str) -> Self {
        Self::Bundle(value.into())
    }
}

#[derive(Clone, Default)]
pub struct Client {
    underlying: reqwest::Client,
    output: Option<PathBuf>,
}

impl Client {
    #[must_use]
    pub fn new(output: Option<PathBuf>) -> Self {
        Self {
            underlying: reqwest::Client::default(),
            output,
        }
    }

    fn is_not_found_body(body: &str) -> bool {
        body.starts_with("<?xml")
            && body.contains("<key>dialogId</key><string>itemNotAvailable</string>")
    }

    pub async fn app(
        &self,
        id: u64,
        country: Country,
    ) -> Result<Option<crate::model::full::Page<'_>>, Error> {
        let request = crate::request::Request::app(id, country);
        let exchange = text_send(&self.underlying, request.build_request(None)).await?;

        if Self::is_not_found_body(&exchange.response.data) {
            Ok(None)
        } else {
            let json: Value = serde_json::from_str(&exchange.response.data)?;

            let new_exchange = exchange.map(|_| json.clone());

            if let Some(output) = &self.output {
                new_exchange.save_file(output)?;
            }

            Ok(serde_json::from_value(json)?)
        }
    }

    pub async fn search(
        &self,
        query: &str,
        country: Country,
        language: Language,
    ) -> Result<super::model::search::Page<'_>, Error> {
        let request = crate::request::Request::search(query, country, language);
        let exchange = json_send(&self.underlying, request.build_request(None)).await?;

        if let Some(output) = &self.output {
            exchange.save_file(output)?;
        }

        Ok(serde_json::from_value(exchange.response.data)?)
    }

    pub async fn lookup_ids(
        &self,
        ids: &[u64],
        country: Country,
        language: Language,
    ) -> Result<super::model::lookup::LookupResultList<'_>, Error> {
        let request = crate::request::Request::lookup_ids(ids.iter().copied(), country, language);
        let exchange = json_send(&self.underlying, request.build_request(None)).await?;

        if let Some(output) = &self.output {
            exchange.save_file(output)?;
        }

        Ok(serde_json::from_value(exchange.response.data)?)
    }

    pub async fn lookup_bundle_ids(
        &self,
        ids: &[&str],
        country: Country,
        language: Language,
    ) -> Result<super::model::lookup::LookupResultList<'_>, Error> {
        let request =
            crate::request::Request::lookup_bundle_ids(ids.iter().copied(), country, language);
        let exchange = json_send(&self.underlying, request.build_request(None)).await?;

        if let Some(output) = &self.output {
            exchange.save_file(output)?;
        }

        Ok(serde_json::from_value(exchange.response.data)?)
    }

    pub async fn reviews(
        &self,
        id: u64,
        country: Country,
        sort: SortOrder,
        page: usize,
    ) -> Result<Vec<super::model::reviews::Page<'_>>, Error> {
        let request = crate::request::Request::reviews(id, country, sort, page);
        let exchange = json_send(&self.underlying, request.build_request(None)).await?;

        if let Some(output) = &self.output {
            exchange.save_file(output)?;
        }

        let first =
            serde_json::from_value::<super::model::reviews::Page<'_>>(exchange.response.data)?;

        let last_page = first.last_page;

        let mut pages = vec![];
        pages.push(first);

        if let Some(last_page) = last_page {
            for next in 2..=last_page {
                let request = crate::request::Request::reviews(id, country, sort, next);
                let exchange = json_send(&self.underlying, request.build_request(None)).await?;

                if let Some(output) = &self.output {
                    exchange.save_file(output)?;
                }

                let page = serde_json::from_value::<super::model::reviews::Page<'_>>(
                    exchange.response.data,
                )?;

                pages.push(page);
            }
        }

        Ok(pages)
    }

    // TODO: Parse ratings HTML.
    pub async fn ratings_html(&self, id: u64, country: Country) -> Result<String, Error> {
        let request = crate::request::Request::ratings(id, country);
        let exchange = text_send(&self.underlying, request.build_request(None)).await?;

        Ok(exchange.response.data)
    }
}
