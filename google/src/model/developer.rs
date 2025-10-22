use crate::{model::app::AppMetadata, request::params::developer::DeveloperId};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Page {
    pub apps: Vec<AppMetadata>,
    pub token: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Developer {
    pub id: DeveloperId,
    pub name: String,
    pub email: String,
    pub website: Option<String>,
    pub address: Option<String>,
    pub legal_name: Option<String>,
    pub legal_email: Option<String>,
    pub legal_address: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageExchange {
    pub request: PageRequest,
    pub response: PageResponse,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageRequest {
    pub developer_id: DeveloperId,
    pub language: app_store_access::language::Language,
    pub country: app_store_access::country::Country,
    pub number: Option<usize>,
    pub token: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PageResponse {
    pub apps: Vec<AppMetadata>,
    pub token: Option<String>,
}

pub mod pagination {
    use crate::model::app::AppMetadata;

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct InitialNumericIdResponse(super::PageResponse);

    impl From<InitialNumericIdResponse> for super::PageResponse {
        fn from(value: InitialNumericIdResponse) -> Self {
            value.0
        }
    }

    impl From<InitialNumericIdResponse> for super::Page {
        fn from(value: InitialNumericIdResponse) -> Self {
            Self {
                apps: value.0.apps,
                token: value.0.token,
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for InitialNumericIdResponse {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let internal = super::internal::InitialNumericIdResponse::deserialize(deserializer)?;

            Ok(internal
                .outer
                .map(|outer| {
                    let data = super::internal::Data::from(outer.inner);

                    let developer_id =
                        super::internal::parse_developer_id_path(&data.developer_id)?;

                    let apps = data
                        .app_ids
                        .into_iter()
                        .map(|app_id| AppMetadata::new(app_id, developer_id.clone()))
                        .collect();

                    Ok(Self(super::PageResponse {
                        apps,
                        token: data.token,
                    }))
                })
                .map_or(Ok(None), |value| value.map(Some))?
                .unwrap_or_default())
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct InitialNameIdResponse(super::PageResponse);

    impl From<InitialNameIdResponse> for super::PageResponse {
        fn from(value: InitialNameIdResponse) -> Self {
            value.0
        }
    }

    impl From<InitialNameIdResponse> for super::Page {
        fn from(value: InitialNameIdResponse) -> Self {
            Self {
                apps: value.0.apps,
                token: value.0.token,
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for InitialNameIdResponse {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let internal = super::internal::InitialNameIdResponse::deserialize(deserializer)?;

            Ok(internal
                .outer
                .map(|outer| {
                    let data = super::internal::Data::from(outer.inner);

                    let developer_id = super::DeveloperId::Name(data.developer_id);

                    let apps = data
                        .app_ids
                        .into_iter()
                        .map(|app_id| AppMetadata::new(app_id, developer_id.clone()))
                        .collect();

                    Ok(Self(super::PageResponse {
                        apps,
                        token: data.token,
                    }))
                })
                .map_or(Ok(None), |value| value.map(Some))?
                .unwrap_or_default())
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct PaginatedNumericIdResponse(super::PageResponse);

    impl From<PaginatedNumericIdResponse> for super::PageResponse {
        fn from(value: PaginatedNumericIdResponse) -> Self {
            value.0
        }
    }

    impl From<PaginatedNumericIdResponse> for super::Page {
        fn from(value: PaginatedNumericIdResponse) -> Self {
            Self {
                apps: value.0.apps,
                token: value.0.token,
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for PaginatedNumericIdResponse {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let internal = super::internal::PaginatedNumericIdResponse::deserialize(deserializer)?;

            Ok(Self(super::PageResponse {
                apps: internal
                    .apps
                    .into_iter()
                    .map(|app| {
                        Ok(AppMetadata::new(
                            app.app_id,
                            super::internal::parse_developer_id_path(&app.developer_id_path)?,
                        ))
                    })
                    .collect::<Result<Vec<_>, D::Error>>()?,
                // TODO: Confirm that we will never see more results here.
                token: None,
            }))
        }
    }

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct PaginatedNameIdResponse(super::PageResponse);

    impl From<PaginatedNameIdResponse> for super::PageResponse {
        fn from(value: PaginatedNameIdResponse) -> Self {
            value.0
        }
    }

    impl From<PaginatedNameIdResponse> for super::Page {
        fn from(value: PaginatedNameIdResponse) -> Self {
            Self {
                apps: value.0.apps,
                token: value.0.token,
            }
        }
    }

    impl<'de> serde::Deserialize<'de> for PaginatedNameIdResponse {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let internal = super::internal::PaginatedNameIdResponse::deserialize(deserializer)?;

            Ok(Self(super::PageResponse {
                apps: internal
                    .apps
                    .into_iter()
                    .map(|app| {
                        Ok(AppMetadata::new(
                            app.app_id,
                            super::internal::parse_developer_id_path(&app.developer_id_path)?,
                        ))
                    })
                    .collect::<Result<Vec<_>, D::Error>>()?,
                // TODO: Confirm that we will never see more results here.
                token: None,
            }))
        }
    }
}

mod internal {
    use super::DeveloperId;

    pub fn parse_developer_id_path<E: serde::de::Error>(value: &str) -> Result<DeveloperId, E> {
        DeveloperId::parse_url_path(value).ok_or_else(|| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(value), &"developer ID path")
        })
    }

    pub struct Data {
        pub app_ids: Vec<String>,
        // May be a path (with either a numeric ID or a name ID) or a name ID string.
        pub developer_id: String,
        pub token: Option<String>,
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNumericIdResponse {
        #[query(".data.[0].[1]")]
        pub outer: Option<InitialNumericIdOuter>,
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNumericIdOuter {
        #[query(".[0].[21]")]
        pub inner: InitialNumericIdInner,
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    pub enum InitialNumericIdInner {
        Token((InitialNumericIdApps, InitialNumericIdTokenSection)),
        IdOnly((InitialNumericIdApps, InitialNumericIdIdOnlySection)),
    }

    impl From<InitialNumericIdInner> for Data {
        fn from(value: InitialNumericIdInner) -> Self {
            match value {
                InitialNumericIdInner::Token((apps, token_section)) => Self {
                    app_ids: apps.app_ids,
                    developer_id: token_section.developer_id_path,
                    token: Some(token_section.token),
                },
                InitialNumericIdInner::IdOnly((apps, id_only_section)) => Self {
                    app_ids: apps.app_ids,
                    developer_id: id_only_section.developer_id_path,
                    token: None,
                },
            }
        }
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNumericIdApps {
        #[query(".[].[0].[0]")]
        pub app_ids: Vec<String>,
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNumericIdTokenSection {
        #[query(".[2].[4].[2]")]
        pub developer_id_path: String,
        #[query(".[3].[1]")]
        pub token: String,
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNumericIdIdOnlySection {
        #[query(".[2].[4].[2]")]
        pub developer_id_path: String,
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNameIdResponse {
        #[query(".data.[0].[1]")]
        pub outer: Option<InitialNameIdOuter>,
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNameIdOuter {
        #[query(".[0].[22]")]
        pub inner: InitialNameIdInner,
    }

    #[derive(serde::Deserialize)]
    #[serde(untagged)]
    pub enum InitialNameIdInner {
        Token((InitialNameIdApps, InitialNameIdTokenSection)),
        IdOnly((InitialNameIdApps, InitialNameIdIdOnlySection)),
    }

    impl From<InitialNameIdInner> for Data {
        fn from(value: InitialNameIdInner) -> Self {
            match value {
                InitialNameIdInner::Token((apps, token_section)) => Self {
                    app_ids: apps.app_ids,
                    developer_id: token_section.developer_id.replace(' ', "+"),
                    token: Some(token_section.token),
                },
                InitialNameIdInner::IdOnly((apps, id_only_section)) => Self {
                    app_ids: apps.app_ids,
                    developer_id: id_only_section.developer_id.replace(' ', "+"),
                    token: None,
                },
            }
        }
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNameIdApps {
        #[query(".[].[0].[0].[0]")]
        pub app_ids: Vec<String>,
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNameIdTokenSection {
        #[query(".[0]")]
        pub developer_id: String,
        #[query(".[3].[1]")]
        pub token: String,
    }

    #[derive(serde_query::Deserialize)]
    pub struct InitialNameIdIdOnlySection {
        #[query(".[0]")]
        pub developer_id: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
    pub struct PaginatedNumericIdResponse {
        #[query(".[0].[6].[0]")]
        pub apps: Vec<PaginationResultsApp>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
    pub struct PaginatedNameIdResponse {
        #[query(".[0].[0].[0]")]
        pub apps: Vec<PaginationResultsApp>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
    pub struct PaginationResultsApp {
        #[query(".[4].[0].[0].[1].[4].[2]")]
        pub developer_id_path: String,
        #[query(".[12].[0]")]
        pub app_id: String,
    }
}
