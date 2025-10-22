use indexmap::IndexMap;
use reqwest::header::{HeaderMap, InvalidHeaderValue};
use std::fmt::Display;
use std::str::FromStr;

pub(super) const APPLE_STORE_FRONT_HEADER_NAME: &str = "X-Apple-Store-Front";
pub(super) const ACCEPT_LANGUAGE_HEADER_NAME: &str = "Accept-Language";

pub struct AppleStoreFrontHeader {
    pub store_id: u32,
    pub code: u8,
    pub attribute: Option<String>,
}

impl AppleStoreFrontHeader {
    #[must_use]
    pub const fn new(store_id: u32, code: u8, attribute: Option<String>) -> Self {
        Self {
            store_id,
            code,
            attribute,
        }
    }
}

impl FromStr for AppleStoreFrontHeader {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let with_attribute = s.split(' ').collect::<Vec<_>>();

        if with_attribute.is_empty() || with_attribute.len() > 2 {
            Err(s.to_string())
        } else {
            let parts = with_attribute[0].split(',').collect::<Vec<_>>();

            if parts.len() == 2 {
                let (store_id, code) = parts[0]
                    .parse::<u32>()
                    .ok()
                    .zip(parts[1].parse::<u8>().ok())
                    .ok_or_else(|| s.to_string())?;

                Ok(Self {
                    store_id,
                    code,
                    attribute: with_attribute
                        .get(1)
                        .map(|attribute| (*attribute).to_string()),
                })
            } else {
                Err(s.to_string())
            }
        }
    }
}

impl Display for AppleStoreFrontHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.store_id, self.code)?;

        self.attribute
            .as_ref()
            .map_or(Ok(()), |value| write!(f, " {value}"))
    }
}

#[derive(Default)]
pub struct RequestHeaders {
    pub apple_store_front: Option<AppleStoreFrontHeader>,
    pub accept_language: Option<String>,
}

impl RequestHeaders {
    #[must_use]
    pub const fn new(
        apple_store_front: Option<AppleStoreFrontHeader>,
        accept_language: Option<String>,
    ) -> Self {
        Self {
            apple_store_front,
            accept_language,
        }
    }
}

impl From<&RequestHeaders> for IndexMap<String, String> {
    fn from(value: &RequestHeaders) -> Self {
        let mut map = Self::new();

        if let Some(apple_store_front) = &value.apple_store_front {
            map.insert(
                APPLE_STORE_FRONT_HEADER_NAME.to_string(),
                apple_store_front.to_string(),
            );
        }

        if let Some(accept_language) = &value.accept_language {
            map.insert(
                ACCEPT_LANGUAGE_HEADER_NAME.to_string(),
                accept_language.clone(),
            );
        }

        map
    }
}

impl TryFrom<&RequestHeaders> for HeaderMap {
    type Error = InvalidHeaderValue;

    fn try_from(value: &RequestHeaders) -> Result<Self, Self::Error> {
        let mut map = Self::new();

        if let Some(apple_store_front) = &value.apple_store_front {
            map.insert(
                APPLE_STORE_FRONT_HEADER_NAME,
                apple_store_front.to_string().try_into()?,
            );
        }

        if let Some(accept_language) = &value.accept_language {
            map.insert(ACCEPT_LANGUAGE_HEADER_NAME, accept_language.try_into()?);
        }

        Ok(map)
    }
}
