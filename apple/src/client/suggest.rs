use crate::request::markets::MarketCode;

const SUGGEST_URL: &str = "https://search.itunes.apple.com/WebObjects/MZSearchHints.woa/wa/hints?clientApplication=Software&term=";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP client error")]
    Reqwest(#[from] reqwest::Error),
    #[error("PList deserialization error")]
    Plist(#[from] plist::Error),
}

impl app_store_access::client::SuggestionClient for super::Client {
    type Error = Error;

    fn lookup_suggestions(
        &self,
        term: &str,
        country: app_store_access::country::Country,
        _lang: app_store_access::language::Language,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Self::Error>> + Send {
        let url = format!("{SUGGEST_URL}{}", urlencoding::encode(term));
        let store_id: MarketCode = country.into();

        async move {
            let response = self
                .underlying
                .get(&url)
                .header("X-Apple-Store-Front", format!("{store_id},29"))
                .send()
                .await?;

            let bytes = response.bytes().await?;

            let hints = plist::from_bytes::<crate::model::suggest::HintList>(&bytes)?;

            Ok(hints
                .hints
                .into_iter()
                .map(|hint| hint.term.into_owned())
                .collect())
        }
    }
}
