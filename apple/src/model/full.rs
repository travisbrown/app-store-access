use bounded_static_derive_more::ToStatic;
use chrono::{DateTime, Utc};
use serde_field_attributes::{integer_str, integer_str_array, optional_integer_str_array};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Page<'a> {
    #[serde(rename = "storePlatformData")]
    pub store_platform_data: StorePlatformData<'a>,
    #[serde(rename = "pageData")]
    pub page_data: PageData<'a>,
    pub properties: Properties<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct StorePlatformData<'a> {
    #[serde(rename = "product-dv")]
    pub product_dv: super::StorePlatformResults<ProductDvResult<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProductDvResult<'a> {
    #[serde(flatten)]
    pub common: super::AppCommon<'a>,
    #[serde(rename = "artistId", with = "integer_str")]
    pub artist_id: u64,
    pub artwork: super::Artwork<'a>,
    #[serde(rename = "circularArtwork")]
    pub circular_artwork: Option<super::Artwork<'a>>,
    #[serde(rename = "isAppleWatchSupported")]
    pub is_apple_watch_supported: Option<bool>,
    #[serde(rename = "isPreorder")]
    pub is_preorder: Option<bool>,
    #[serde(rename = "usesLocationBackgroundMode")]
    pub uses_location_background_mode: Option<bool>,
    #[serde(rename = "appBundleAdamIds")]
    pub app_bundle_adam_ids: Option<Vec<String>>,
    pub description: Description<'a>,
    #[serde(rename = "editorialArtwork")]
    pub editorial_artwork: EditorialArtwork<'a>,
    #[serde(rename = "familyShareEnabledDate")]
    pub family_share_enabled_date: DateTime<Utc>,
    #[serde(rename = "fileSizeByDevice")]
    pub file_size_by_device: Option<HashMap<String, usize>>,
    #[serde(with = "integer_str")]
    pub id: u64,
    #[serde(rename = "promotionalText")]
    pub promotional_text: Option<Cow<'a, str>>,
    #[serde(rename = "softwareInfo")]
    pub software_info: SoftwareInfo<'a>,
    #[serde(rename = "screenshotsByType")]
    pub screenshots_by_type: super::devices::DeviceMap<Vec<super::Artwork<'a>>>,
    #[serde(rename = "messagesScreenshots")]
    pub messages_screenshots: Option<super::devices::DeviceMap<Vec<super::Artwork<'a>>>>,
    #[serde(rename = "tellAFriendMessageContentsUrl")]
    pub tell_a_friend_message_contents_url: Cow<'a, str>,
    #[serde(rename = "ovalArtwork")]
    pub oval_artwork: Option<super::Artwork<'a>>,
    #[serde(rename = "circularIconArtwork")]
    pub circular_icon_artwork: Option<super::Artwork<'a>>,
    #[serde(rename = "editorialVideo")]
    pub editorial_video: Option<EditorialVideo<'a>>,
    #[serde(rename = "videoPreviewByType")]
    pub video_preview_by_type: super::devices::DeviceMap<Video<'a>>,
    pub uber: Option<Uber<'a>>,
    #[serde(rename = "appCount")]
    pub app_count: Option<usize>,
    // TODO: Model this.
    pub children: Option<serde_json::Value>,
    #[serde(rename = "childrenIds", with = "optional_integer_str_array", default)]
    pub children_ids: Option<Vec<u64>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Video<'a> {
    #[serde(rename = "previewFrame")]
    pub preview_frame: super::Artwork<'a>,
    pub video: Cow<'a, str>,
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
    pub master_art: Option<super::Artwork<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoftwareInfo<'a> {
    pub seller: Cow<'a, str>,
    #[serde(rename = "eulaUrl")]
    pub eula_url: Option<Cow<'a, str>>,
    #[serde(rename = "privacyPolicyTextUrl")]
    pub privacy_policy_text_url: Option<Cow<'a, str>>,
    #[serde(rename = "privacyPolicyUrl")]
    pub privacy_policy_url: Option<Cow<'a, str>>,
    #[serde(rename = "supportUrl")]
    pub support_url: Option<Cow<'a, str>>,
    #[serde(rename = "websiteUrl")]
    pub website_url: Option<Cow<'a, str>>,
    #[serde(rename = "languagesDisplayString")]
    pub languages_display_string: Option<Cow<'a, str>>,
    #[serde(rename = "requirementsString")]
    pub requirements_string: Cow<'a, str>,
    #[serde(rename = "hasInAppPurchases")]
    pub has_in_app_purchases: Option<bool>,
    #[serde(rename = "isGameCenter")]
    pub is_game_center: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EditorialArtwork<'a> {
    #[serde(rename = "bannerUber")]
    pub banner_uber: Option<super::Artwork<'a>>,
    #[serde(rename = "brandLogo")]
    pub brand_logo: Option<super::Artwork<'a>>,
    #[serde(rename = "contentIconTrimmed")]
    pub content_icon_trimmed: Option<super::Artwork<'a>>,
    #[serde(rename = "contentIconTrimmedMonochrome")]
    pub content_icon_trimmed_monochrome: Option<super::Artwork<'a>>,
    #[serde(rename = "emailFeature")]
    pub email_feature: Option<super::Artwork<'a>>,
    #[serde(rename = "storeFlowcase")]
    pub store_flowcase: Option<super::Artwork<'a>>,
    #[serde(rename = "subscriptionHero")]
    pub subscription_hero: Option<super::Artwork<'a>>,
    #[serde(rename = "universalAStatic16x9")]
    pub universal_a_static_16x9: Option<super::Artwork<'a>>,
    #[serde(rename = "brickStatic16x9")]
    pub brick_static_16x9: Option<super::Artwork<'a>>,
    #[serde(rename = "storyCardStatic16x9")]
    pub story_card_static_16x9: Option<super::Artwork<'a>>,
    #[serde(rename = "storyCardStatic3x4")]
    pub story_card_static_3x4: Option<super::Artwork<'a>>,
    #[serde(rename = "storyDetailStatic3x4")]
    pub story_detail_static_3x4: Option<super::Artwork<'a>>,
    #[serde(rename = "storyCenteredStatic16x9")]
    pub story_centered_static_16x9: Option<super::Artwork<'a>>,
    #[serde(rename = "storySearchStatic16x9")]
    pub story_search_static_16x9: Option<super::Artwork<'a>>,
    #[serde(rename = "productUberStatic16x9")]
    pub product_uber_static_16x9: Option<super::Artwork<'a>>,
    #[serde(rename = "heroStatic16x9")]
    pub hero_static_16x9: Option<super::Artwork<'a>>,
    #[serde(rename = "largeBreakoutStatic16x9")]
    pub large_breakout_static_16x9: Option<super::Artwork<'a>>,
    #[serde(rename = "originalFlowcaseBrick")]
    pub original_flowcase_brick: Option<super::Artwork<'a>>,
    #[serde(rename = "contentLogo")]
    pub content_logo: Option<super::Artwork<'a>>,
    #[serde(rename = "contentLogoTrimmed")]
    pub content_logo_trimmed: Option<super::Artwork<'a>>,
    #[serde(rename = "fullscreenBackground")]
    pub fullscreen_background: Option<super::Artwork<'a>>,
    #[serde(rename = "crossoverCard")]
    pub crossover_card: Option<super::Artwork<'a>>,
    #[serde(rename = "splashFullScreen")]
    pub splash_full_screen: Option<super::Artwork<'a>>,
    #[serde(rename = "dayCard")]
    pub day_card: Option<super::Artwork<'a>>,
    #[serde(rename = "mediaCard")]
    pub media_card: Option<super::Artwork<'a>>,
    #[serde(rename = "generalCard")]
    pub general_card: Option<super::Artwork<'a>>,
    #[serde(rename = "subscriptionCover")]
    pub subscription_cover: Option<super::Artwork<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EditorialVideo<'a> {
    #[serde(rename = "gameHeroMotion16x9")]
    pub game_hero_motion_16x9: Option<EditorialVideoItem<'a>>,
    #[serde(rename = "productUberMotion16x9")]
    pub product_uber_motion_16x9: Option<EditorialVideoItem<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct EditorialVideoItem<'a> {
    #[serde(rename = "previewFrame")]
    pub preview_frame: super::Artwork<'a>,
    pub video: Cow<'a, str>,
    #[serde(rename = "videoFile")]
    pub video_file: Option<Vec<super::VideoFile<'a>>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Description<'a> {
    pub standard: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, tag = "componentName")]
pub enum PageData<'a> {
    #[serde(rename = "software_page")]
    SoftwarePage(SoftwarePage<'a>),
    #[serde(rename = "unsupported_product_page")]
    UnsupportedProductPage {
        id: u64,
        #[serde(rename = "sf6ResourceImagePath")]
        sf6_resource_image_path: Cow<'a, str>,
    },
}

impl PageData<'_> {
    #[must_use]
    pub const fn id(&self) -> u64 {
        match self {
            Self::SoftwarePage(software_page) => software_page.id,
            Self::UnsupportedProductPage { id, .. } => *id,
        }
    }

    #[must_use]
    pub fn rating_and_advisories(&self) -> Option<Vec<super::ContentRatingAdvisory>> {
        match self {
            Self::SoftwarePage(software_page) => {
                software_page.rating_and_advisories.advisories.clone()
            }
            Self::UnsupportedProductPage { .. } => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoftwarePage<'a> {
    #[serde(with = "integer_str")]
    pub id: u64,
    #[serde(rename = "metricsBase")]
    pub metrics_base: super::metrics::MetricsBase<'a>,
    pub metrics: super::metrics::Metrics<MetricsFields>,
    #[serde(rename = "isFatBinary")]
    pub is_fat_binary: Option<u8>,
    #[serde(rename = "topApps")]
    pub top_apps: TopApps<'a>,
    #[serde(rename = "rating-and-advisories")]
    pub rating_and_advisories: RatingAndAdvisories<'a>,
    #[serde(rename = "versionHistory")]
    pub version_history: Vec<super::version::HistoryEntry<'a>>,
    #[serde(rename = "addOns")]
    pub add_ons: Option<Vec<AddOn<'a>>>,
    #[serde(
        rename = "customersAlsoBoughtApps",
        with = "optional_integer_str_array",
        default
    )]
    pub customers_also_bought_apps: Option<Vec<u64>>,
    #[serde(
        rename = "moreByThisDeveloper",
        with = "optional_integer_str_array",
        default
    )]
    pub more_by_this_developer: Option<Vec<u64>>,
    #[serde(rename = "sellerLabel")]
    pub seller_label: SellerLabel,
    #[serde(rename = "appRatingsLearnMoreUrl")]
    pub app_ratings_learn_more_url: Cow<'a, str>,
    #[serde(rename = "customerReviewsUrl")]
    pub customer_reviews_url: Cow<'a, str>,
    #[serde(rename = "sf6ResourceImagePath")]
    pub sf6_resource_image_path: Cow<'a, str>,
    pub uber: Option<PageUber<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum ComponentName {
    #[serde(rename = "software_page")]
    SoftwarePage,
    #[serde(rename = "unsupported_product_page")]
    UnsupportedProductPage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub enum SellerLabel {
    Seller,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct MetricsFields {
    #[serde(rename = "isUber")]
    pub is_uber: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Properties<'a> {
    #[serde(rename = "revNum")]
    pub rev_num: Cow<'a, str>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TopApps<'a> {
    pub iphone: Option<TopAppsList<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct TopAppsList<'a> {
    pub title: Cow<'a, str>,
    #[serde(with = "integer_str_array")]
    pub ids: Vec<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RatingAndAdvisories<'a> {
    pub advisories: Option<Vec<super::ContentRatingAdvisory>>,
    #[serde(rename = "rating-text")]
    pub rating_text: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AddOn<'a> {
    pub name: Cow<'a, str>,
    pub price: Cow<'a, str>,
    #[serde(rename = "offerType")]
    pub offer_type: OfferType,
    #[serde(rename = "buyParams")]
    pub buy_params: Cow<'a, str>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct OfferType {
    #[serde(rename = "offerType")]
    pub offer_type: super::offer::OfferType,
    #[serde(rename = "accompaniesNoun")]
    pub accompanies_noun: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, ToStatic, serde::Deserialize, serde::Serialize)]
// TODO: Re-add this.
#[serde(deny_unknown_fields)]
pub struct PageUber<'a> {
    #[serde(rename = "titleTextColor")]
    pub title_text_color: Option<Cow<'a, str>>,
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<Cow<'a, str>>,
    #[serde(rename = "primaryTextColor")]
    pub primary_text_color: Option<Cow<'a, str>>,
    #[serde(rename = "masterArt")]
    pub master_art: Option<Vec<super::Image<'a>>>,
}
