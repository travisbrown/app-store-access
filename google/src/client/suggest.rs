use serde_json::Value;

const BASE_URL: &str = "https://play.google.com";
const URL: &str = "/_/PlayStoreUi/data/batchexecute?rpcids=IJ4APc&f.sid=-697906427155521722&bl=boq_playuiserver_20190903.08_p0&authuser&soc-app=121&soc-platform=1&soc-device=1&_reqid=1065213";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP client error")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("Too short")]
    TooShort(String),
    #[error("Invalid outer JSON")]
    InvalidOuterJson(Value),
    #[error("Invalid inner JSON")]
    InvalidInnerJson(Value),
}

impl app_store_access::client::SuggestionClient for super::Client {
    type Error = Error;

    fn lookup_suggestions(
        &self,
        query: &str,
        country: app_store_access::country::Country,
        lang: app_store_access::language::Language,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Self::Error>> + Send {
        let url = format!("{BASE_URL}{URL}&hl={lang}&gl={country}");

        async move {
            let response = self
            .underlying
            .post(url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded;charset=UTF-8",
            )
            .body(format!("f.req=%5B%5B%5B%22IJ4APc%22%2C%22%5B%5Bnull%2C%5B%5C%22${query}%5C%22%5D%2C%5B10%5D%2C%5B2%5D%2C4%5D%5D%22%5D%5D%5D"))
            .send()
            .await?;

            let body = response.text().await?;

            Ok(body.parse::<SuggestionList>()?.0)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SuggestionList(Vec<String>);

impl std::str::FromStr for SuggestionList {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 5 {
            let outer = serde_json::from_str::<Value>(&s[5..])?;

            let inner_str = outer
                .get(0)
                .and_then(|value| value.get(2))
                .and_then(|value| value.as_str())
                .ok_or_else(|| Error::InvalidOuterJson(outer.clone()))?;

            let inner = serde_json::from_str::<Value>(inner_str)?;

            let array = inner
                .get(0)
                .and_then(|value| value.get(0))
                .and_then(|value| value.as_array())
                .ok_or_else(|| Error::InvalidInnerJson(outer.clone()))?;

            let values = array
                .iter()
                .map(|value| {
                    value
                        .get(0)
                        .and_then(|value| value.as_str())
                        .map(std::string::ToString::to_string)
                        .ok_or_else(|| Error::InvalidInnerJson(outer.clone()))
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Self(values))
        } else {
            Err(Error::TooShort(s.to_string()))
        }
    }
}
