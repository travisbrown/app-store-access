use crate::request::params::{developer::DeveloperId, review::SortOrder, search::PriceFilter};
use app_store_access::{country::Country, language::Language};
use reqwest::StatusCode;
use scraper_trail::{client::text_send, request::params::Params};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub mod suggest;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("HTTP client error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Scraper client error")]
    ScraperClient(#[from] scraper_trail::client::Error),
    #[error("Parse error")]
    Parse(#[from] crate::parse::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("Suggestion parsing error")]
    Suggest(#[from] suggest::Error),
}

#[derive(Clone)]
pub struct Client {
    underlying: reqwest::Client,
    output: Option<PathBuf>,
}

impl Client {
    pub fn new<P: AsRef<Path>>(
        output: Option<P>,
        cookies: Arc<reqwest::cookie::Jar>,
    ) -> Result<Self, Error> {
        let underlying = reqwest::ClientBuilder::new()
            .cookie_provider(cookies)
            .build()?;

        Ok(Self {
            underlying,
            output: output.map(|output| output.as_ref().to_path_buf()),
        })
    }

    pub async fn app(
        &self,
        app_id: &str,
        language: Language,
        country: Country,
    ) -> Result<Option<crate::model::app::App>, Error> {
        let request = crate::request::Request::details(app_id, language, country);

        match text_send(&self.underlying, request.build_request(None)).await {
            Ok(exchange) => {
                let html = scraper::Html::parse_document(&exchange.response.data);
                let json_3 = crate::parse::parse_ds_value::<Value>(&html, 3).ok();
                let json_5 = crate::parse::parse_ds_value::<Value>(&html, 5)?;
                let json_8 = crate::parse::parse_ds_value::<Value>(&html, 8).ok();
                let mut jsons = vec![json_5.clone()];

                if let Some(json_3) = json_3 {
                    jsons.push(json_3);
                }

                if let Some(json_8) = json_8 {
                    jsons.push(json_8);
                }

                let new_exchange = exchange.map(|_| serde_json::json!(jsons));

                if let Some(output) = &self.output {
                    new_exchange.save_file(output)?;
                }

                let app_result = serde_json::from_value(json_5)?;

                Ok(Some(app_result))
            }
            Err(scraper_trail::client::Error::UnexpectedStatus {
                status_code: StatusCode::NOT_FOUND,
                ..
            }) => Ok(None),
            Err(error) => Err(Error::from(error)),
        }
    }

    pub async fn developer(
        &self,
        developer: &DeveloperId,
        language: Language,
        country: Country,
        number: usize,
    ) -> Result<Option<Vec<crate::model::app::AppMetadata>>, Error> {
        use crate::model::developer::pagination::{
            InitialNameIdResponse, InitialNumericIdResponse, PaginatedNameIdResponse,
            PaginatedNumericIdResponse,
        };

        let request = crate::request::Request::developer(developer.clone(), language, country);

        match text_send(&self.underlying, request.build_request(None)).await {
            Ok(exchange) => {
                let html = scraper::Html::parse_document(&exchange.response.data);
                let json_3 = crate::parse::parse_ds_value::<Value>(&html, 3)?;
                let new_exchange = exchange.map(|_| serde_json::json!(vec![json_3.clone()]));

                if let Some(output) = &self.output {
                    new_exchange.save_file(output)?;
                }

                let page: crate::model::developer::PageResponse = if developer.is_numeric() {
                    serde_json::from_value::<InitialNumericIdResponse>(json_3)?.into()
                } else {
                    serde_json::from_value::<InitialNameIdResponse>(json_3)?.into()
                };

                let mut apps = page.apps;
                let mut token = page.token;

                while let Some(ref token_value) = token {
                    log::info!("Making developer pagination request via token: {token_value}");

                    let request =
                        crate::request::Request::pagination(number, token_value, language, country);

                    let exchange = scraper_trail::client::text_send(
                        &self.underlying,
                        request.build_request(None),
                    )
                    .await?;

                    let pagination_json = parse_pagination_data(&exchange.response.data)?;
                    let new_exchange = exchange.map(|_| pagination_json.clone());

                    if let Some(output) = &self.output {
                        new_exchange.save_file(output)?;
                    }

                    let page: crate::model::developer::PageResponse = if developer.is_numeric() {
                        serde_json::from_value::<PaginatedNumericIdResponse>(pagination_json)?
                            .into()
                    } else {
                        serde_json::from_value::<PaginatedNameIdResponse>(pagination_json)?.into()
                    };

                    apps.extend(page.apps);
                    token = page.token;
                }

                Ok(Some(apps))
            }
            Err(scraper_trail::client::Error::UnexpectedStatus {
                status_code: StatusCode::NOT_FOUND,
                ..
            }) => Ok(None),
            Err(error) => Err(Error::from(error)),
        }
    }

    pub async fn search(
        &self,
        query: &str,
        language: Language,
        country: Country,
        price: PriceFilter,
        number: usize,
    ) -> Result<Vec<crate::model::search::App>, Error> {
        let request = crate::request::Request::search(query, Some(price), language, country);
        let exchange = text_send(&self.underlying, request.build_request(None)).await?;

        let html = scraper::Html::parse_document(&exchange.response.data);
        let json = crate::parse::parse_ds_value::<Value>(&html, 1)?;

        let new_exchange = exchange.map(|_| serde_json::json!(vec![json.clone()]));

        if let Some(output) = &self.output {
            new_exchange.save_file(output)?;
        }

        let search_result: crate::model::search::SearchResult = serde_json::from_value(json)?;

        let mut apps = vec![];
        let mut token = search_result.token().map(str::to_string);

        if let Some(search_results) = search_result.inner {
            apps.extend(search_results.apps);
        }

        while let Some(ref token_value) = token {
            log::info!("Making search pagination request via token: {token_value}");

            let request =
                crate::request::Request::pagination(number, token_value, language, country);

            let exchange =
                scraper_trail::client::text_send(&self.underlying, request.build_request(None))
                    .await?;

            let pagination_json = parse_pagination_data(&exchange.response.data)?;
            let new_exchange = exchange.map(|_| pagination_json.clone());

            if let Some(output) = &self.output {
                new_exchange.save_file(output)?;
            }

            let search_pagination_result = serde_json::from_value::<
                crate::model::search::SearchPaginationResult,
            >(pagination_json)?;

            token = search_pagination_result.token().map(str::to_string);
            apps.extend(search_pagination_result.apps);
        }

        Ok(apps)
    }

    pub async fn reviews(
        &self,
        id: &str,
        language: Language,
        country: Country,
        sort_order: SortOrder,
        number: usize,
    ) -> Result<Vec<crate::model::review::Review>, Error> {
        let request =
            crate::request::Request::reviews(id, sort_order, number, None, language, country);

        let exchange =
            scraper_trail::client::text_send(&self.underlying, request.build_request(None)).await?;

        let pagination_json = parse_pagination_data(&exchange.response.data)?;
        let new_exchange = exchange.map(|_| serde_json::json![pagination_json.clone()]);

        if let Some(output) = &self.output {
            new_exchange.save_file(output)?;
        }

        let page: crate::model::review::PageResponse = serde_json::from_value(pagination_json)?;

        let mut reviews = vec![];
        let mut token = page.token;

        reviews.extend(page.reviews);

        while let Some(ref token_value) = token {
            log::info!("Making review pagination request via token: {token_value}");

            let request = crate::request::Request::reviews(
                id,
                sort_order,
                number,
                Some(token_value),
                language,
                country,
            );

            let exchange =
                scraper_trail::client::text_send(&self.underlying, request.build_request(None))
                    .await?;

            let pagination_json = parse_pagination_data(&exchange.response.data)?;
            let new_exchange = exchange.map(|_| pagination_json.clone());

            if let Some(output) = &self.output {
                new_exchange.save_file(output)?;
            }

            let page: crate::model::review::PageResponse = serde_json::from_value(pagination_json)?;

            token = page.token;

            reviews.extend(page.reviews);
        }

        Ok(reviews)
    }
}

fn parse_pagination_data(data: &str) -> Result<Value, serde_json::Error> {
    type PaginationJson = (
        (
            serde::de::IgnoredAny,
            serde::de::IgnoredAny,
            String,
            serde::de::IgnoredAny,
            serde::de::IgnoredAny,
            serde::de::IgnoredAny,
            serde::de::IgnoredAny,
        ),
        serde::de::IgnoredAny,
        serde::de::IgnoredAny,
    );

    let ((_, _, pagination_json, _, _, _, _), _, _) =
        serde_json::from_str::<PaginationJson>(&data[5..])?;

    serde_json::from_str(&pagination_json)
}
