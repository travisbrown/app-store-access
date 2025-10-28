use app_store_access::{
    country::{Country, country_code_uppercase},
    language::{RegionalLanguage, regional_language_code_lowercase},
};
use bounded_static_derive_more::ToStatic;
use num_rational::Ratio;
use serde_field_attributes::{integer_str, optional_ratio_u64, ratio_u64};
use std::borrow::Cow;
use std::collections::BTreeMap;

pub mod content_rating;
pub mod devices;
pub mod full;
pub mod genre;
pub mod lookup;
pub mod reviews;
#[cfg(not(feature = "strict"))]
pub mod strict_fix;
pub mod suggest;
pub mod version;

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum PageType {
    Search,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, rename_all = "UPPERCASE")]
pub enum Currency {
    Aed,
    Aud,
    Bgn,
    Brl,
    Cad,
    Chf,
    Clp,
    Cny,
    Cop,
    Czk,
    Dkk,
    Egp,
    Eur,
    Gbp,
    Hkd,
    Huf,
    Idr,
    Jpy,
    Krw,
    Kzt,
    Mxn,
    Myr,
    Ngn,
    Nok,
    Nzd,
    Pen,
    Pkr,
    Php,
    Pln,
    Qar,
    Ron,
    Rub,
    Sar,
    Sek,
    Sgd,
    Thb,
    Try,
    Twd,
    Tzs,
    Usd,
    Vnd,
    Zar,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
pub enum EntityType {
    Software,
    #[serde(rename = "mac-software")]
    MacSoftware,
    #[serde(rename = "iphoneAppBundle")]
    IphoneAppBundle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum Kind {
    #[serde(rename = "iosSoftware")]
    IosSoftware,
    #[serde(rename = "desktopApp")]
    DesktopApp,
    #[serde(rename = "macSoftware")]
    MacSoftware,
    #[serde(rename = "mobileSoftwareBundle")]
    MobileSoftwareBundle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum Feature {
    #[serde(rename = "iosUniversal")]
    IosUniversal,
    #[serde(rename = "gameCenter")]
    GameCenter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum ArtistType {
    #[serde(rename = "Software Artist")]
    SoftwareArtist,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AgeBand {
    #[serde(rename = "minAge")]
    pub min_age: usize,
    #[serde(rename = "maxAge")]
    pub max_age: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorePlatformResults<R> {
    #[serde(rename = "isAuthenticated")]
    pub is_authenticated: bool,
    pub meta: StorePlatformMeta,
    pub results: BTreeMap<u64, R>,
    pub version: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Image<'a> {
    pub url: Cow<'a, str>,
    pub height: u32,
    pub width: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Notes<'a> {
    pub name: Option<Cow<'a, str>>,
    pub short: Option<Cow<'a, str>>,
    pub standard: Option<Cow<'a, str>>,
    pub tagline: Option<Cow<'a, str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum DeviceFamily {
    #[serde(rename = "ipad")]
    Ipad,
    #[serde(rename = "iphone")]
    Iphone,
    #[serde(rename = "ipod")]
    Ipod,
    #[serde(rename = "mac")]
    Mac,
    #[serde(rename = "tvos")]
    Tvos,
    #[serde(rename = "watch")]
    Watch,
    #[serde(rename = "realityDevice")]
    RealityDevice,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Artwork<'a> {
    pub url: Cow<'a, str>,
    pub width: usize,
    pub height: usize,
    #[serde(rename = "bgColor")]
    pub bg_color: Option<Cow<'a, str>>,
    #[serde(rename = "textColor1")]
    pub text_color_1: Option<Cow<'a, str>>,
    #[serde(rename = "textColor2")]
    pub text_color_2: Option<Cow<'a, str>>,
    #[serde(rename = "textColor3")]
    pub text_color_3: Option<Cow<'a, str>>,
    #[serde(rename = "textColor4")]
    pub text_color_4: Option<Cow<'a, str>>,
    #[serde(rename = "hasAlpha")]
    pub has_alpha: Option<bool>,
    #[serde(rename = "hasP3")]
    pub has_p3: Option<bool>,
    #[serde(rename = "supportsLayeredImage")]
    pub supports_layered_image: Option<bool>,
    pub gradient: Option<Gradient>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Gradient {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct UserRating {
    #[serde(rename = "ratingCount")]
    pub rating_count: u32,
    #[serde(rename = "ratingCountCurrentVersion")]
    pub rating_count_current_version: Option<u32>,
    #[serde(with = "ratio_u64")]
    pub value: Ratio<u64>,
    #[serde(rename = "valueCurrentVersion", with = "optional_ratio_u64", default)]
    pub value_current_version: Option<Ratio<u64>>,
    #[serde(rename = "wasReset")]
    pub was_reset: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AppCommon<'a> {
    #[serde(rename = "artistName")]
    pub artist_name: Cow<'a, str>,
    #[serde(rename = "artistUrl")]
    pub artist_url: Cow<'a, str>,
    #[serde(rename = "bundleId")]
    pub bundle_id: Option<Cow<'a, str>>,
    #[serde(rename = "contentRatingsBySystem")]
    pub content_ratings_by_system: content_rating::ContentRatingsBySystem,
    pub copyright: Option<Cow<'a, str>>,
    #[serde(rename = "deviceFamilies")]
    pub device_families: Vec<DeviceFamily>,
    #[serde(rename = "designedForDeviceFamilies")]
    pub designed_for_device_families: Option<Vec<DeviceFamily>>,
    /// These should match the `name` values from the `genres` field.
    #[serde(rename = "genreNames")]
    pub genre_names: Vec<Cow<'a, str>>,
    pub genres: Vec<genre::Genre>,
    #[serde(rename = "hasInAppPurchases")]
    pub has_in_app_purchases: Option<bool>,
    #[serde(rename = "hasMessagesExtension")]
    pub has_messages_extension: Option<bool>,
    #[serde(rename = "hasPrimaryArtist")]
    pub has_primary_artist: bool,
    #[serde(rename = "isGameControllerSupported")]
    pub is_game_controller_supported: Option<bool>,
    #[serde(rename = "isHiddenFromSpringboard")]
    pub is_hidden_from_springboard: Option<bool>,
    #[serde(rename = "isFirstPartyHideableApp")]
    pub is_first_party_hideable_app: Option<bool>,
    #[serde(rename = "isSiriSupported")]
    pub is_siri_supported: Option<bool>,
    #[serde(rename = "isPurgeableLocalStorageSupported")]
    pub is_purgeable_local_storage_supported: Option<bool>,
    #[serde(rename = "is32bitOnly")]
    pub is_32bit_only: Option<bool>,
    #[serde(rename = "gracRegistrationNumber")]
    pub grac_registration_number: Option<Cow<'a, str>>,
    #[serde(rename = "itunesNotes")]
    pub itunes_notes: Option<Notes<'a>>,
    pub kind: Kind,
    #[serde(rename = "minimumOSVersion")]
    pub minimum_os_version: Cow<'a, str>,
    pub name: Cow<'a, str>,
    #[serde(rename = "nameRaw")]
    pub name_raw: Cow<'a, str>,
    #[serde(rename = "nameSortValue")]
    pub name_sort_value: Cow<'a, str>,
    pub offers: Vec<offer::Offer<'a>>,
    #[serde(rename = "releaseDate")]
    pub release_date: chrono::NaiveDate,
    #[serde(rename = "requiredCapabilities")]
    pub required_capabilities: Option<Cow<'a, str>>,
    #[serde(rename = "shortUrl")]
    pub short_url: Cow<'a, str>,
    pub subtitle: Option<Cow<'a, str>>,
    #[serde(rename = "supportsPassbook")]
    pub supports_passbook: Option<bool>,
    #[serde(rename = "templatizedArtwork")]
    pub templatized_artwork: Artwork<'a>,
    pub url: Cow<'a, str>,
    #[serde(rename = "userRating")]
    pub user_rating: UserRating,
    #[serde(rename = "iconArtwork")]
    pub icon_artwork: Option<Artwork<'a>>,
    #[serde(rename = "editorialBadgeInfo")]
    pub editorial_badge_info: Option<EditorialBadgeInfo<'a>>,
    #[serde(rename = "ageBand")]
    pub age_band: Option<AgeBand>,
    #[serde(rename = "regularPriceFormatted")]
    pub regular_price_formatted: Option<Cow<'a, str>>,
    // TODO: Model this.
    #[serde(rename = "screenshotsWithMetadataByType")]
    pub screenshots_with_metadata_by_type: Option<devices::DeviceMap<serde_json::Value>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum EditorialBadgeType {
    #[serde(rename = "editorialPriority")]
    EditorialPriority,
    #[serde(rename = "staffPick")]
    StaffPick,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EditorialBadgeInfo<'a> {
    #[serde(rename = "editorialBadgeType")]
    pub editorial_badge_type: EditorialBadgeType,
    #[serde(rename = "nameForDisplay")]
    pub name_for_display: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MultiPreviewVideo<'a> {
    #[serde(rename = "previewFrame")]
    pub preview_frame: Vec<Image<'a>>,
    pub video: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct VideoFile<'a> {
    #[serde(rename = "assetUrl")]
    pub asset_url: Cow<'a, str>,
    #[serde(rename = "durationMillis")]
    pub duration_millis: usize,
    pub height: usize,
    pub width: usize,
}

pub mod search {
    use bounded_static_derive_more::ToStatic;
    use serde_field_attributes::integer_str;
    use std::borrow::Cow;

    #[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Page<'a> {
        #[serde(rename = "metricsBase")]
        pub metrics_base: super::metrics::MetricsBase<'a>,
        #[serde(rename = "pageType")]
        pub page_type: super::PageType,
        #[serde(rename = "storePlatformData")]
        pub store_platform_data: StorePlatformData<'a>,
        pub term: Cow<'a, str>,
        pub bubbles: BubblesField,
        pub metrics: super::metrics::Metrics<MetricsFields<'a>>,
        #[serde(rename = "spellCorrection")]
        pub spell_correction: Option<Cow<'a, str>>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StorePlatformData<'a> {
        #[serde(rename = "native-search-lockup-search")]
        pub native_search_lockup_search:
            Option<super::StorePlatformResults<NativeSearchLockupSearchResult<'a>>>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct NativeSearchLockupSearchResult<'a> {
        #[serde(flatten)]
        pub common: super::AppCommon<'a>,
        #[serde(rename = "artistId")]
        pub artist_id: u64,
        pub artwork: Vec<super::Image<'a>>,
        #[serde(rename = "circularArtwork")]
        pub circular_artwork: Option<Vec<super::Image<'a>>>,
        #[serde(rename = "circularIconArtwork")]
        pub circular_icon_artwork: Option<Vec<super::Image<'a>>>,
        #[serde(rename = "contentRating")]
        pub content_rating: super::content_rating::ContentRating<super::content_rating::AppleName>,
        pub id: u64,
        #[serde(rename = "is32bitOnly")]
        pub is_32_bit_only: bool,
        #[serde(rename = "isAppleWatchSupported")]
        pub is_apple_watch_supported: bool,
        #[serde(rename = "isPreorder")]
        pub is_preorder: bool,
        #[serde(rename = "isSiriSupported")]
        pub is_siri_supported: bool,
        #[serde(rename = "requiresGameController")]
        pub requires_game_controller: bool,
        #[serde(rename = "messagesScreenshots")]
        pub messages_screenshots: super::devices::DeviceMap<Vec<Vec<super::Image<'a>>>>,
        #[serde(rename = "screenshotsByType")]
        pub screenshots_by_type: super::devices::DeviceMap<Vec<Vec<super::Image<'a>>>>,
        #[serde(rename = "ovalArtwork")]
        pub oval_artwork: Option<Vec<super::Image<'a>>>,
        #[serde(rename = "editorialVideo")]
        pub editorial_video: Option<EditorialVideo<'a>>,
        #[serde(rename = "videoPreviewByType")]
        pub video_preview_by_type: super::devices::DeviceMap<super::MultiPreviewVideo<'a>>,
        pub uber: Option<Uber<'a>>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Uber<'a> {
        pub name: Option<Cow<'a, str>>,
        pub description: Option<Cow<'a, str>>,
        #[serde(rename = "titleTextColor")]
        pub title_text_color: Option<Cow<'a, str>>,
        #[serde(rename = "titleTextColorOnBlack")]
        pub title_text_color_on_black: Option<Cow<'a, str>>,
        #[serde(rename = "backgroundColor")]
        pub background_color: Option<Cow<'a, str>>,
        #[serde(rename = "headerTextColor")]
        pub header_text_color: Option<Cow<'a, str>>,
        #[serde(rename = "primaryTextColor")]
        pub primary_text_color: Option<Cow<'a, str>>,
        #[serde(rename = "primaryTextColorOnBlack")]
        pub primary_text_color_on_black: Option<Cow<'a, str>>,
        #[serde(rename = "masterArt")]
        pub master_art: Option<Vec<super::Image<'a>>>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct EditorialVideo<'a> {
        #[serde(rename = "gameHeroMotion16x9")]
        pub game_hero_motion_16x9: EditorialVideoItem<'a>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct EditorialVideoItem<'a> {
        #[serde(rename = "previewFrame")]
        pub preview_frame: Vec<super::Image<'a>>,
        pub video: Cow<'a, str>,
        #[serde(rename = "videoFile")]
        pub video_file: Vec<super::VideoFile<'a>>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields, untagged)]
    pub enum BubblesField {
        Empty(Vec<()>),
        Bubbles((Bubbles,)),
    }

    impl BubblesField {
        #[must_use]
        pub fn bubbles(self) -> Option<Bubbles> {
            match self {
                Self::Bubbles((bubble,)) => Some(bubble),
                Self::Empty(_) => None,
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Bubbles {
        pub results: Vec<BubblesResult>,
        pub name: super::EntityType,
        #[serde(rename = "totalCount")]
        pub total_count: u32,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BubblesResult {
        #[serde(rename = "type")]
        pub bubbles_result_type: u32,
        #[serde(with = "integer_str")]
        pub id: u64,
        pub entity: super::EntityType,
    }

    #[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct MetricsFields<'a> {
        #[serde(rename = "searchTerm")]
        pub search_term: Cow<'a, str>,
    }
}

pub mod metrics {
    use bounded_static_derive_more::ToStatic;
    use serde_field_attributes::integer_str;
    use std::borrow::Cow;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Metrics<F> {
        pub config: Config,
        pub fields: F,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Config {}

    #[derive(Clone, Eq, Debug, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct MetricsBase<'a> {
        #[serde(rename = "pageType")]
        pub page_type: PageType,
        #[serde(rename = "pageId")]
        pub page_id: Cow<'a, str>,
        #[serde(rename = "pageDetails")]
        pub page_details: Cow<'a, str>,
        // TODO: Confirm / validate that this should always be "{page_type}_{page_id}"?
        page: Cow<'a, str>,
        #[serde(rename = "serverInstance", with = "integer_str")]
        pub server_instance: u32,
        #[serde(rename = "storeFrontHeader")]
        pub store_front_header: Cow<'a, str>,
        #[serde(with = "integer_str")]
        pub language: u32,
        #[serde(rename = "storeFront", with = "integer_str")]
        pub store_front: u32,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub enum PageType {
        Search,
        Software,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorePlatformMeta {
    pub language: StorePlatformMetaLanguage,
    #[serde(rename = "storefront")]
    pub store_front: StorePlatformMetaStoreFront,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorePlatformMetaLanguage {
    #[serde(with = "regional_language_code_lowercase")]
    pub tag: RegionalLanguage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorePlatformMetaStoreFront {
    #[serde(with = "country_code_uppercase")]
    pub cc: Country,
    #[serde(with = "integer_str")]
    pub id: u32,
}

pub mod offer {
    use bounded_static_derive_more::ToStatic;
    use chrono::NaiveDate;
    use serde_field_attributes::ratio_u64;
    use std::borrow::Cow;

    #[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Offer<'a> {
        #[serde(rename = "actionText")]
        pub action_text: ActionText,
        pub assets: Vec<Asset>,
        #[serde(rename = "buyParams")]
        pub buy_params: Cow<'a, str>,
        #[serde(with = "ratio_u64")]
        pub price: num_rational::Ratio<u64>,
        #[serde(rename = "priceFormatted")]
        pub price_formatted: Cow<'a, str>,
        #[serde(rename = "type")]
        pub offer_type: OfferType,
        pub version: Option<Version<'a>>,
        #[serde(rename = "expectedReleaseDate")]
        pub expected_release_date: Option<NaiveDate>,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct ActionText {
        pub downloaded: DownloadedText,
        pub downloading: DownloadingText,
        pub long: LongText,
        pub medium: MediumText,
        pub short: ShortText,
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[cfg_attr(feature = "strict", derive(Copy))]
    #[serde(deny_unknown_fields)]
    pub enum DownloadedText {
        #[serde(
            rename = "INSTALLED",
            alias = "Installed",
            alias = "INSTALLIERT",
            alias = "УСТАНОВЛЕНО"
        )]
        Installed,
        #[serde(rename = "DOWNLOADED")]
        Downloaded,
        #[cfg(not(feature = "strict"))]
        #[serde(untagged)]
        Other(String),
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[cfg_attr(feature = "strict", derive(Copy))]
    #[serde(deny_unknown_fields)]
    pub enum DownloadingText {
        #[serde(
            rename = "INSTALLING",
            alias = "Installing",
            alias = "WIRD INSTALLIERT",
            alias = "УСТАНОВКА"
        )]
        Installing,
        #[serde(rename = "DOWNLOADING")]
        Downloading,
        #[cfg(not(feature = "strict"))]
        #[serde(untagged)]
        Other(String),
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[cfg_attr(feature = "strict", derive(Copy))]
    #[serde(deny_unknown_fields)]
    pub enum OfferType {
        #[serde(rename = "buy")]
        Buy,
        #[serde(rename = "get")]
        Get,
        #[serde(rename = "preorder")]
        Preorder,
        #[cfg(not(feature = "strict"))]
        #[serde(untagged)]
        Other(String),
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[cfg_attr(feature = "strict", derive(Copy))]
    #[serde(deny_unknown_fields)]
    pub enum LongText {
        #[serde(
            rename = "BUY APP",
            alias = "Buy",
            alias = "APP KAUFEN",
            alias = "КУПИТЬ ПРИЛОЖЕНИЕ"
        )]
        BuyApp,
        #[serde(
            rename = "GET APP",
            alias = "Get",
            alias = "APP LADEN",
            alias = "ЗАГРУЗИТЬ"
        )]
        GetApp,
        #[serde(rename = "GET BUNDLE")]
        GetBundle,
        #[cfg(not(feature = "strict"))]
        #[serde(untagged)]
        Other(String),
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[cfg_attr(feature = "strict", derive(Copy))]
    #[serde(deny_unknown_fields)]
    pub enum MediumText {
        #[serde(alias = "Kaufen", alias = "Купить")]
        Buy,
        #[serde(alias = "Laden", alias = "Загрузить")]
        Get,
        #[cfg(not(feature = "strict"))]
        #[serde(untagged)]
        Other(String),
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[cfg_attr(feature = "strict", derive(Copy))]
    #[serde(deny_unknown_fields)]
    pub enum ShortText {
        #[serde(rename = "BUY", alias = "Buy", alias = "KAUFEN", alias = "КУПИТЬ")]
        Buy,
        #[serde(rename = "GET", alias = "Get", alias = "LADEN", alias = "ЗАГРУЗИТЬ")]
        Get,
        #[cfg(not(feature = "strict"))]
        #[serde(untagged)]
        Other(String),
    }

    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Asset {
        pub flavor: super::Kind,
        pub size: u64,
    }

    #[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Version<'a> {
        pub display: Cow<'a, str>,
        #[serde(rename = "externalId")]
        pub external_id: u32,
    }
}
