use crate::request::params::developer::DeveloperId;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Page {
    pub apps: Option<Vec<App>>,
    pub token: Option<String>,
}

impl From<SearchResult> for Page {
    fn from(value: SearchResult) -> Self {
        let token = value.token().map(std::string::ToString::to_string);

        Self {
            apps: value.inner.map(|inner| inner.apps),
            token,
        }
    }
}

impl From<SearchPaginationResult> for Page {
    fn from(value: SearchPaginationResult) -> Self {
        let token = value.token().map(std::string::ToString::to_string);

        Self {
            apps: Some(value.apps),
            token,
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
pub struct SearchResult {
    #[query(".data.[0].[1]")]
    //pub inner: serde_json::Value,
    pub inner: Option<SearchResultInner>,
}

/*#[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
pub struct SearchResult2 {
    #[query("data.[0].[1]?.[0].[0].[0].[]")]
    pub apps: Option<Vec<App>>,
    //#[query(".data.[0].[1].[0].[0].[0].[]")]
    //pub apps: Vec<serde_json::Value>,
    #[query("data.[0].[1]?.[0].[0].[7].[1]")]
    pub token: Option<String>,
}*/

impl SearchResult {
    #[must_use]
    pub fn token(&self) -> Option<&str> {
        self.inner.as_ref().and_then(|inner| inner.token())
    }
}

#[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
pub struct SearchResultInner {
    #[query(".[0].[0].[0].[]")]
    pub apps: Vec<App>,
    #[query(".[0].[0].[7]")]
    pub token_wrapper: TokenWrapper,
}

impl SearchResultInner {
    #[must_use]
    pub fn token(&self) -> Option<&str> {
        match &self.token_wrapper {
            TokenWrapper::Token(((), token)) => Some(token),
            TokenWrapper::Other(_) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
pub struct SearchPaginationResult {
    #[query(".[0].[0].[0].[]")]
    pub apps: Vec<App>,
    #[query(".[0].[0].[7]")]
    pub token_wrapper: TokenWrapper,
}

impl SearchPaginationResult {
    #[must_use]
    pub fn token(&self) -> Option<&str> {
        match &self.token_wrapper {
            TokenWrapper::Token(((), token)) => Some(token),
            TokenWrapper::Other(_) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum TokenWrapper {
    Token(((), String)),
    Other(serde::de::IgnoredAny),
}

#[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
pub struct App {
    #[query(".[12].[0]")]
    pub id: String,
    #[query(".[2]")]
    pub title: String,
    #[query(".[4].[1].[1].[1].[1]")]
    pub summary: String,
    #[query(".[4].[0].[0]")]
    pub developer: Developer,
    #[query(".[1].[1].[0].[3].[2]")]
    pub icon: String,
    //#[query(".[7].[0].[3].[2].[1].[0]")]
    //pub price: Option<Price>,
    #[query(".[7]")]
    pub price: Option<serde_json::Value>,
    #[query(".[6]")]
    pub score: Option<Score>,
}

impl App {
    #[must_use]
    pub fn developer_id(&self) -> Option<DeveloperId> {
        DeveloperId::parse_url_path(&self.developer.id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct Developer {
    #[query(".[1].[4].[2]")]
    pub id: String,
    #[query(".[0]")]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct Price {
    #[query(".[0].[3].[2].[1].[0].[1]")]
    pub currency: String,
    #[query(".[0].[3].[2].[1].[0].[0]")]
    pub value: u64,
}

#[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
pub struct Score {
    #[query(".[0].[2].[1].[1]")]
    pub value: f32,
    #[query(".[0].[2].[1].[0]")]
    pub text: String,
}
