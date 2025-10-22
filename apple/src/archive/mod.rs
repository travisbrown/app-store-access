use crate::request::Request;
use bounded_static::IntoBoundedStatic;
use scraper_trail::{
    archive::{Archiveable, entry::Field},
    exchange::Response,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Data<'a> {
    App(crate::model::full::Page<'a>),
    Search(super::model::search::Page<'a>),
    Lookup(super::model::lookup::LookupResultList<'a>),
    Reviews(super::model::reviews::Page<'a>),
}

impl bounded_static::IntoBoundedStatic for Data<'_> {
    type Static = Data<'static>;

    fn into_static(self) -> Self::Static {
        match self {
            Self::App(page) => Self::Static::App(page.into_static()),
            Self::Search(page) => Self::Static::Search(page.into_static()),
            Self::Lookup(list) => Self::Static::Lookup(list.into_static()),
            Self::Reviews(page) => Self::Static::Reviews(page.into_static()),
        }
    }
}

impl Archiveable for Data<'static> {
    type RequestParams = crate::request::Request<'static>;

    fn deserialize_response_field<'de, A: serde::de::MapAccess<'de>>(
        request_params: &Self::RequestParams,
        map: &mut A,
    ) -> Result<Option<(Field, Response<'de, Self>)>, A::Error> {
        match &request_params {
            Request::App { .. } => {
                let next =
                    map.next_entry::<Field, Response<'_, crate::model::full::Page<'de>>>()?;

                Ok(next.map(|(field, response)| {
                    (field, response.map(|data| Self::App(data.into_static())))
                }))
            }
            Request::Search { .. } => {
                let next =
                    map.next_entry::<Field, Response<'_, super::model::search::Page<'de>>>()?;

                Ok(next.map(|(field, response)| {
                    (field, response.map(|data| Self::Search(data.into_static())))
                }))
            }
            Request::LookupIds { .. } => {
                let next = map
                    .next_entry::<Field, Response<'_, super::model::lookup::LookupResultList<'de>>>(
                    )?;

                Ok(next.map(|(field, response)| {
                    (field, response.map(|data| Self::Lookup(data.into_static())))
                }))
            }
            Request::LookupBundleIds { .. } => {
                let next = map
                    .next_entry::<Field, Response<'_, super::model::lookup::LookupResultList<'de>>>(
                    )?;

                Ok(next.map(|(field, response)| {
                    (field, response.map(|data| Self::Lookup(data.into_static())))
                }))
            }
            Request::Reviews { .. } => {
                let next =
                    map.next_entry::<Field, Response<'_, super::model::reviews::Page<'de>>>()?;

                Ok(next.map(|(field, response)| {
                    (
                        field,
                        response.map(|data| Self::Reviews(data.into_static())),
                    )
                }))
            }
            Request::Ratings { .. } => {
                Err(serde::de::Error::custom("unsupported rating archive entry"))
            }
        }
    }
}
