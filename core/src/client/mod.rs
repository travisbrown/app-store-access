use crate::{country::Country, language::Language};

pub trait SuggestionClient {
    type Error;

    fn lookup_suggestions(
        &self,
        query: &str,
        country: Country,
        language: Language,
    ) -> impl std::future::Future<Output = Result<Vec<String>, Self::Error>> + Send;
}
