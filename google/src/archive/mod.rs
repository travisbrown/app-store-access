use crate::request::{RequestData, params::developer::DeveloperId};
use scraper_trail::{
    archive::{Archiveable, entry::Field},
    exchange::Response,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Detail(crate::model::app::App),
    DeveloperInitial(crate::model::developer::Page),
    DeveloperPagination(DeveloperId, crate::model::developer::Page),
    Search(crate::model::search::Page),
    Reviews(crate::model::review::Page),
}

impl bounded_static::IntoBoundedStatic for Data {
    type Static = Self;

    fn into_static(self) -> Self::Static {
        self
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(untagged)]
enum Pagination {
    Search(crate::model::search::SearchPaginationResult),
    NumericIdDeveloper(crate::model::developer::pagination::PaginatedNumericIdResponse),
    NameIdDeveloper(crate::model::developer::pagination::PaginatedNameIdResponse),
}

impl Archiveable for Data {
    type RequestParams = crate::request::Request<'static>;

    fn deserialize_response_field<'de, A: serde::de::MapAccess<'de>>(
        request_params: &Self::RequestParams,
        map: &mut A,
    ) -> Result<Option<(Field, Response<'de, Self>)>, A::Error> {
        match &request_params.data {
            RequestData::Details { .. } => {
                let next = map.next_entry::<Field, Response<'_, crate::model::full::AppData>>()?;

                Ok(next
                    .map(|(field, response)| (field, response.map(|data| Self::Detail(data.ds5)))))
            }
            RequestData::Developer { developer_id } => {
                if developer_id.is_numeric() {
                    let next = map.next_entry::<Field, Response<(
                        crate::model::developer::pagination::InitialNumericIdResponse,
                    )>>()?;

                    Ok(next.map(|(field, response)| {
                        (
                            field,
                            response.map(|data| Self::DeveloperInitial(data.0.into())),
                        )
                    }))
                } else {
                    let next = map.next_entry::<Field, Response<(
                        crate::model::developer::pagination::InitialNameIdResponse,
                    )>>()?;

                    Ok(next.map(|(field, response)| {
                        (
                            field,
                            response.map(|data| Self::DeveloperInitial(data.0.into())),
                        )
                    }))
                }
            }
            RequestData::Search { .. } => {
                let next =
                    map.next_entry::<Field, Response<(crate::model::search::SearchResult,)>>()?;

                Ok(next.map(|(field, response)| {
                    (field, response.map(|data| Self::Search(data.0.into())))
                }))
            }
            RequestData::Reviews { .. } => {
                let next =
                    map.next_entry::<Field, Response<crate::model::review::PageResponse>>()?;

                Ok(next.map(|(field, response)| {
                    (field, response.map(|data| Self::Reviews(data.into())))
                }))
            }
            RequestData::Pagination { .. } => {
                let next = map.next_entry::<Field, Response<Pagination>>()?;

                next.map(|(field, response)| {
                    response
                        .and_then(|data| {
                            Ok(match data {
                                Pagination::NumericIdDeveloper(response) => {
                                    let (developer_id, page) =
                                        paginated_developer_page_exchange(response)?;

                                    Self::DeveloperPagination(developer_id, page)
                                }
                                Pagination::NameIdDeveloper(response) => {
                                    let (developer_id, page) =
                                        paginated_developer_page_exchange(response)?;

                                    Self::DeveloperPagination(developer_id, page)
                                }
                                Pagination::Search(search) => Self::Search(search.into()),
                            })
                        })
                        .map(|response| (field, response))
                })
                .map_or(Ok(None), |value| value.map(Some))
            }
        }
    }
}

fn paginated_developer_page_exchange<
    P: Into<crate::model::developer::Page>,
    E: serde::de::Error,
>(
    data: P,
) -> Result<(DeveloperId, crate::model::developer::Page), E> {
    let page = data.into();
    let mut developer_ids = page
        .apps
        .iter()
        .map(|app| app.developer_id.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter();

    developer_ids.next().map_or_else(
        || {
            Err(serde::de::Error::custom(
                "no developer IDs in developer response",
            ))
        },
        |developer_id| {
            if developer_ids.next().is_none() {
                Ok((developer_id, page))
            } else {
                Err(serde::de::Error::custom(
                    "multiple developer IDs in developer response",
                ))
            }
        },
    )
}
