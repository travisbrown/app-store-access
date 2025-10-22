use app_store_access::language::{Language, language_code_uppercase_array};
use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc};
use serde_field_attributes::{integer_str, optional_ratio_u64, ratio_u64};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct LookupResultList<'a> {
    #[serde(rename = "resultCount")]
    pub result_count: usize,
    pub results: Vec<LookupResult<'a>>,
    #[serde(rename = "metricsBase")]
    pub metrics_base: Option<super::metrics::MetricsBase<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, tag = "wrapperType")]
pub enum LookupResult<'a> {
    #[serde(rename = "artist")]
    Artist(Artist<'a>),
    #[serde(rename = "software")]
    Software(Box<Software<'a>>),
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Artist<'a> {
    #[serde(rename = "artistId")]
    pub artist_id: u64,
    #[serde(rename = "artistType")]
    pub artist_type: super::ArtistType,
    #[serde(rename = "artistName")]
    pub artist_name: Cow<'a, str>,
    #[serde(rename = "artistLinkUrl")]
    pub artist_link_url: Cow<'a, str>,
    #[serde(rename = "primaryGenreId")]
    pub primary_genre_id: Option<u16>,
    #[serde(rename = "primaryGenreName")]
    pub primary_genre_name: Option<ArtistGenre>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum ArtistGenre {
    #[serde(rename = "App Store")]
    AppStore,
    Productivity,
    Games,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Software<'a> {
    #[serde(rename = "ipadScreenshotUrls")]
    pub ipad_screenshot_urls: Option<Vec<Cow<'a, str>>>,
    #[serde(rename = "appletvScreenshotUrls")]
    pub appletv_screenshot_urls: Option<Vec<Cow<'a, str>>>,
    #[serde(rename = "screenshotUrls")]
    pub screenshot_urls: Vec<Cow<'a, str>>,
    #[serde(rename = "artistViewUrl")]
    pub artist_view_url: Cow<'a, str>,
    #[serde(rename = "artworkUrl60")]
    pub artwork_url_60: Cow<'a, str>,
    #[serde(rename = "artworkUrl100")]
    pub artwork_url_100: Cow<'a, str>,
    #[serde(rename = "artworkUrl512")]
    pub artwork_url_51: Cow<'a, str>,
    #[serde(rename = "isGameCenterEnabled")]
    pub is_game_center_enabled: Option<bool>,
    #[serde(rename = "supportedDevices")]
    pub supported_devices: Option<Vec<super::devices::Device>>,
    pub advisories: Option<Vec<super::ContentRatingAdvisory>>,
    pub features: Option<Vec<super::Feature>>,
    pub kind: super::EntityType,
    #[serde(rename = "languageCodesISO2A", with = "language_code_uppercase_array")]
    pub language_codes_iso_2_a: Vec<Language>,
    #[serde(rename = "fileSizeBytes", with = "integer_str")]
    pub file_size_bytes: usize,
    #[serde(rename = "formattedPrice")]
    pub formatted_price: Option<Cow<'a, str>>,
    #[serde(rename = "trackContentRating")]
    pub track_content_rating: super::ContentRatingName,
    #[serde(rename = "trackCensoredName")]
    pub track_censored_name: Cow<'a, str>,
    #[serde(rename = "trackViewUrl")]
    pub track_view_url: Cow<'a, str>,
    #[serde(rename = "contentAdvisoryRating")]
    pub content_advisory_rating: super::ContentRatingName,
    #[serde(rename = "artistId")]
    pub artist_id: u32,
    #[serde(rename = "artistName")]
    pub artist_name: Cow<'a, str>,
    #[serde(rename = "isVppDeviceBasedLicensingEnabled")]
    pub is_vpp_device_based_licensing_enabled: bool,
    #[serde(rename = "sellerName")]
    pub seller_name: Cow<'a, str>,
    #[serde(rename = "sellerUrl")]
    pub seller_url: Option<Cow<'a, str>>,
    #[serde(rename = "bundleId")]
    pub bundle_id: Cow<'a, str>,
    #[serde(rename = "trackId")]
    pub track_id: u64,
    #[serde(rename = "currentVersionReleaseDate")]
    pub current_version_release_date: DateTime<Utc>,
    #[serde(rename = "trackName")]
    pub track_name: Cow<'a, str>,
    #[serde(rename = "releaseDate")]
    pub release_date: DateTime<Utc>,
    #[serde(rename = "releaseNotes")]
    pub release_notes: Option<Cow<'a, str>>,
    pub version: Cow<'a, str>,
    pub currency: super::Currency,
    pub description: Cow<'a, str>,
    #[serde(rename = "minimumOsVersion")]
    pub minimum_os_version: Cow<'a, str>,
    #[serde(rename = "primaryGenreId", with = "super::genre::from_id")]
    pub primary_genre: super::genre::Genre,
    #[serde(rename = "primaryGenreName", with = "super::genre::from_name")]
    pub primary_genre_name: super::genre::Genre,
    #[serde(rename = "genreIds", with = "super::genre::from_id_strs")]
    pub genres: Vec<super::genre::Genre>,
    #[serde(rename = "genres", with = "super::genre::from_names")]
    pub genres_from_name: Vec<super::genre::Genre>,
    #[serde(with = "optional_ratio_u64", default)]
    pub price: Option<num_rational::Ratio<u64>>,
    #[serde(rename = "userRatingCount")]
    pub user_rating_count: usize,
    #[serde(rename = "userRatingCountForCurrentVersion")]
    pub user_rating_count_for_current_version: usize,
    #[serde(rename = "averageUserRating", with = "ratio_u64")]
    pub average_user_rating: num_rational::Ratio<u64>,
    #[serde(rename = "averageUserRatingForCurrentVersion", with = "ratio_u64")]
    pub average_user_rating_for_current_version: num_rational::Ratio<u64>,
}
