use crate::request::params::developer::DeveloperId;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AppData {
    pub ds5: super::app::App,
    pub ds3: Option<serde_json::Value>,
    pub ds8: Option<serde_json::Value>,
}

impl<'de> serde::de::Deserialize<'de> for AppData {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let app_dss = <Vec<AppDs> as serde::de::Deserialize<'de>>::deserialize(deserializer)?;

        let mut ds5: Option<super::app::App> = None;
        let mut ds3: Option<serde_json::Value> = None;
        let mut ds8: Option<serde_json::Value> = None;

        for app_ds in app_dss {
            match app_ds {
                AppDs::Ds5 { data } => {
                    ds5 = Some(data);
                }
                AppDs::Ds3 { data } => {
                    ds3 = Some(data);
                }
                AppDs::Ds8 { data } => {
                    ds8 = Some(data);
                }
            }
        }

        match ds5 {
            Some(app) => Ok(Self { ds5: app, ds3, ds8 }),
            None => Err(serde::de::Error::custom("missing object with ds:5 key")),
        }

        /*#[derive(Default)]
        struct AppDataVisitor {
            ds5: Option<App>,
            ds3: Option<serde_json::Value>,
            ds8: Option<serde_json::Value>,
        }

        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        pub enum Field {
            Key,
        }

        impl<'de> serde::de::Visitor<'de> for AppDataVisitor {
            type Value = Self;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("Google app detail response")
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                seq: A,
            ) -> Result<Self::Value, A::Error> {
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                map: A,
            ) -> Result<Self::Value, A::Error> {
                map.next_key()
            }
        }

        let result = deserializer.deserialize_seq(AppDataVisitor::default())?;

        match result.ds5 {
            Some(app) => Ok(Self {
                ds5: app,
                ds3: result.ds3,
                ds8: result.ds8,
            }),
            None => Err(serde::de::Error::custom("missing object with ds:5 key")),
        }*/
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
#[serde(tag = "key")]
pub(super) enum AppDs {
    #[serde(rename = "ds:5")]
    Ds5 {
        #[serde(flatten)]
        data: super::app::App,
    },
    #[serde(rename = "ds:3")]
    Ds3 {
        #[serde(flatten)]
        data: serde_json::Value,
    },
    #[serde(rename = "ds:8")]
    Ds8 {
        #[serde(flatten)]
        data: serde_json::Value,
    },
}

#[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
pub(super) struct App {
    #[query(".data.[1].[2].[77].[0]")]
    pub id: String,
    #[query(".data.[1].[2].[0].[0]")]
    pub title: String,
    #[query(".data.[1].[2].[72].[0].[1]")]
    pub description: String,
    #[query(".data.[1].[2].[73].[0].[1]")]
    pub summary: String,
    #[query(".data.[1].[2].[68]")]
    pub developer_metadata: DeveloperMetadata,
    #[query(".data.[1].[2].[69]")]
    pub developer_data: DeveloperData,
    #[query(".data.[1].[2].[57]")]
    pub purchase_section: Option<purchase_section::PurchaseSection>,
    //#[query(".data.[1].[2].[140]")]
    //version_section: Option<version_section::VersionSection>,
    #[query(".data.[1].[2].[95].[0].[3].[2]")]
    pub icon: String,
    #[query(".data.[1].[2].[96].[0].[3].[2]")]
    pub header_image: String,
    #[query(".data.[1].[2].[78].[0].[].[3].[2]")]
    pub screenshots: Vec<String>,
    #[query(".data.[1].[2].[9]")]
    pub content_rating: ContentRating,
    #[query(".data.[1].[2].[13].[0]")]
    pub installs: Option<String>,
    #[query(".data.[1].[2].[13].[1]")]
    pub min_installs: Option<u64>,
    #[query(".data.[1].[2].[13].[2]")]
    pub max_installs: u64,
    #[query(".data.[1].[2].[10]")]
    pub released: Option<ReleasedSection>,
    #[query(".data.[1].[2].[99].[0].[5].[2]")]
    pub privacy_policy: String,
    #[query(".data.[1].[2].[79].[0].[0]")]
    pub genre: Genre,
    //#[query(".data.[1].[2].[118].[0].[0]")]
    //#[query(".data.[1].[2].[118]")]
    //categories_section: Option<categories_section::CategoriesSection>,
    /*
    #[query(".data.[1].[2].[51].[0].[1]")]
    pub score: f32,
    #[query(".data.[1].[2].[51].[0].[0]")]
    pub score_text: String,
    #[query(".data.[1].[2].[51].[2].[1]")]
    pub ratings: u64,
    #[query(".data.[1].[2].[51].[3].[1]")]
    pub reviews: u64,
    #[query(".data.[1].[2].[51].[1]")]
    pub histogram: Histogram,
    */
    //#[query(".data.[1].[2].[18].[0]")]
    //pub available: u8,
    //#[query(".data.[1].[2].[99].[0].[5].[2]")]
    //pub privacy_policy: String,
    /*#[query(".data.[1].[2].[48].[0]")]
    pub ad_supported: String,*/
}

impl App {
    pub fn developer(&self) -> Result<super::developer::Developer, String> {
        let developer_id = DeveloperId::parse_url_path(&self.developer_metadata.id)
            .ok_or_else(|| self.developer_metadata.id.clone())?;

        let name = self.developer_metadata.name.clone();

        let (email, website, address, legal_name, legal_email, legal_address) =
            match &self.developer_data {
                DeveloperData::Full(full) => (
                    full.email.clone(),
                    full.website_wrapper
                        .as_ref()
                        .map(|value| value.value.clone()),
                    full.address_wrapper
                        .as_ref()
                        .map(|value| value.value.clone()),
                    full.legal_data.legal_name.clone(),
                    full.legal_data.legal_email.clone(),
                    full.legal_data.legal_address.clone(),
                ),
                DeveloperData::Short(short) => (
                    short.email.clone(),
                    short
                        .website_wrapper
                        .as_ref()
                        .map(|value| value.value.clone()),
                    short
                        .address_wrapper
                        .as_ref()
                        .map(|value| value.value.clone()),
                    None,
                    None,
                    None,
                ),
                DeveloperData::Shorter(shorter) => {
                    (shorter.email.clone(), None, None, None, None, None)
                }
            };

        Ok(super::developer::Developer {
            id: developer_id,
            name,
            email,
            website,
            address,
            legal_name,
            legal_email,
            legal_address,
        })
    }

    /*#[must_use]
    pub fn url(&self) -> Option<&str> {
        self.purchase_section
            .as_ref()
            .and_then(|purchase_section| purchase_section.url.as_ref())
            .map(std::string::String::as_str)
    }*/

    #[must_use]
    pub fn price(&self) -> Option<crate::model::Price> {
        self.purchase_section.as_ref().map(|purchase_section| {
            match &purchase_section.price_section {
                purchase_section::PriceSection::Both((price, _))
                | purchase_section::PriceSection::PriceOnly((price,)) => {
                    crate::model::Price::new(price.value, price.currency.clone())
                }
            }
        })
    }

    /*
    #[must_use]
    pub fn original_price(&self) -> Option<f32> {
        self.purchase_section.as_ref().and_then(|purchase_section| {
            match &purchase_section.price_section {
                purchase_section::PriceSection::Both((_, discount)) => {
                    Some(discount.original_price)
                }
                purchase_section::PriceSection::PriceOnly(_) => None,
            }
        })
    }

    #[must_use]
    pub fn primary_version(&self) -> Option<&str> {
        self.version_section
            .as_ref()
            .and_then(|version_section| version_section.primary.as_ref())
            .map(|(value,)| value.as_str())
    }

    #[must_use]
    pub fn secondary_version(&self) -> Option<&str> {
        self.version_section
            .as_ref()
            .and_then(|version_section| version_section.secondary.as_ref())
            .and_then(|value| match value {
                version_section::SecondaryVersion::Empty(_) => None,
                version_section::SecondaryVersion::Value((_, value)) => Some(value.as_str()),
            })
    }

    #[must_use]
    pub fn content_rating(&self) -> crate::model::ContentRating {
        crate::model::ContentRating::new(
            self.content_rating.name.clone(),
            self.content_rating
                .description
                .as_ref()
                .map(|(_, value)| value.clone()),
        )
    }

    #[must_use]
    pub const fn categories(&self) -> Vec<crate::model::Category> {
        /*self.categories_section
        .as_ref()
        .map(|categories_section| {
            categories_section
                .values
                .iter()
                .map(|value| {
                    crate::model::Category::new(
                        value.name.clone(),
                        value.id.clone(),
                        value.code.clone(),
                        value.url.clone(),
                    )
                })
                .collect()
        })
        /*categories_section::CategoriesSection::Value(values) => Some(
            values
                .iter()
                .map(|value| {
                    crate::model::Category::new(
                        value.name.clone(),
                        value.id.clone(),
                        value.code.clone(),
                        value.url.clone(),
                    )
                })
                .collect(),
        ),
        categories_section::CategoriesSection::Empty(_) => None,*/
        .unwrap_or_default()*/
        vec![]
    }*/
}

mod purchase_section {
    #[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
    pub struct PurchaseSection {
        #[query(".[0].[0].[0].[0].[6].[5].[2]")]
        pub url: Option<String>,
        #[query(".[0].[0].[0].[0].[1]")]
        pub price_section: PriceSection,
    }

    #[derive(Clone, Debug, PartialEq, serde::Deserialize)]
    #[serde(untagged)]
    pub enum PriceSection {
        Both((Price, Discount)),
        PriceOnly((Price,)),
    }

    #[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
    pub struct Price {
        #[query(".[0]")]
        pub value: f32,
        #[query(".[1]")]
        pub currency: String,
    }

    #[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
    pub struct Discount {
        #[query(".[0]")]
        pub original_price: f32,
    }
}

mod version_section {
    #[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
    pub struct VersionSection {
        #[query(".[0].[0]")]
        pub primary: Option<(String,)>,
        #[query(".[1].[0].[0]")]
        pub secondary: Option<SecondaryVersion>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
    #[serde(untagged)]
    pub enum SecondaryVersion {
        Empty((u8,)),
        Value((u8, String)),
    }
}

/*mod categories_section {
    /*#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
    #[serde(untagged)]
    pub enum CategoriesSection {
        Value(Vec<Category>),
        Empty(Vec<()>),
    }*/

    #[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
    pub struct CategoriesSection {
        #[query(".[].[0].[0]")]
        pub values: Vec<Category>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
    pub struct Category {
        #[query(".[0]")]
        pub name: String,
        #[query(".[2]")]
        pub id: Option<String>,
        #[query(".[3]")]
        pub code: String,
        #[query(".[1].[4].[2]")]
        pub url: String,
    }
}*/

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct Histogram {
    #[query(".[1].[1]")]
    pub rating_1: u64,
    #[query(".[2].[1]")]
    pub rating_2: u64,
    #[query(".[3].[1]")]
    pub rating_3: u64,
    #[query(".[4].[1]")]
    pub rating_4: u64,
    #[query(".[5].[1]")]
    pub rating_5: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct DeveloperMetadata {
    #[query(".[1].[4].[2]")]
    pub id: String,
    #[query(".[0]")]
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(untagged)]
pub enum DeveloperData {
    Full(FullDeveloperData),
    Short(ShortDeveloperData),
    Shorter(ShorterDeveloperData),
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct FullDeveloperData {
    #[query(".[1].[0]")]
    pub email: String,
    #[query(".[2]")]
    pub address_wrapper: Option<AddressWrapper>,
    #[query(".[0]")]
    pub website_wrapper: Option<WebsiteWrapper>,
    #[query(".[4]")]
    pub legal_data: DeveloperLegalData,
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct ShortDeveloperData {
    #[query(".[1].[0]")]
    pub email: String,
    #[query(".[2]")]
    pub address_wrapper: Option<AddressWrapper>,
    #[query(".[0]")]
    pub website_wrapper: Option<WebsiteWrapper>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct ShorterDeveloperData {
    #[query(".[1].[0]")]
    pub email: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct DeveloperLegalData {
    #[query(".[0]")]
    pub legal_name: Option<String>,
    #[query(".[1].[0]")]
    pub legal_email: Option<String>,
    #[query(".[2].[0]")]
    pub legal_address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct AddressWrapper {
    #[query(".[0]")]
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct WebsiteWrapper {
    #[query(".[5].[2]")]
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde_query::Deserialize)]
pub struct Genre {
    #[query(".[0]")]
    pub name: String,
    #[query(".[2]")]
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
pub(super) struct ContentRating {
    #[query(".[0]")]
    pub name: String,
    #[query(".[2]")]
    pub description: Option<(serde::de::IgnoredAny, String)>,
}

#[derive(Clone, Debug, PartialEq, serde_query::Deserialize)]
pub(super) struct ReleasedSection {
    #[query(".[0]")]
    pub text: String,
    #[query(".[1].[0]")]
    pub timestamp_s: u64,
}
