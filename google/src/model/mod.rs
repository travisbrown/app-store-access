pub mod app;
pub mod developer;
pub mod full;
pub mod review;
pub mod search;
#[cfg(not(feature = "strict"))]
pub mod strict_fix;

#[derive(Clone, Debug, PartialEq)]
pub struct Price {
    pub value: f32,
    pub currency: String,
}

impl Price {
    #[must_use]
    pub const fn new(value: f32, currency: String) -> Self {
        Self { value, currency }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentRating {
    pub name: String,
    pub description: Option<String>,
}

impl ContentRating {
    #[must_use]
    pub const fn new(name: String, description: Option<String>) -> Self {
        Self { name, description }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Category {
    pub name: String,
    pub id: Option<String>,
    pub code: String,
    pub url: String,
}

impl Category {
    #[must_use]
    pub const fn new(name: String, id: Option<String>, code: String, url: String) -> Self {
        Self {
            name,
            id,
            code,
            url,
        }
    }
}
