use bounded_static::{IntoBoundedStatic, ToBoundedStatic};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct DeviceMap<V>(BTreeMap<DeviceName, V>);

impl DeviceMap<Vec<super::Artwork<'_>>> {
    pub fn urls(&self) -> impl Iterator<Item = &str> {
        self.0
            .values()
            .flat_map(|artworks| artworks.iter().map(|artwork| artwork.url.as_ref()))
    }
}

impl DeviceMap<Vec<super::Image<'_>>> {
    pub fn urls(&self) -> impl Iterator<Item = &str> {
        self.0
            .values()
            .flat_map(|images| images.iter().map(|image| image.url.as_ref()))
    }
}

impl<V: IntoBoundedStatic> IntoBoundedStatic for DeviceMap<V> {
    type Static = DeviceMap<V::Static>;

    fn into_static(self) -> Self::Static {
        DeviceMap(
            self.0
                .into_iter()
                .map(|(key, value)| (key, value.into_static()))
                .collect(),
        )
    }
}

impl<V: ToBoundedStatic> ToBoundedStatic for DeviceMap<V> {
    type Static = DeviceMap<V::Static>;

    fn to_static(&self) -> Self::Static {
        DeviceMap(
            self.0
                .iter()
                .map(|(key, value)| (key.clone(), value.to_static()))
                .collect(),
        )
    }
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "strict", derive(Copy))]
#[serde(deny_unknown_fields)]
pub enum DeviceName {
    #[serde(rename = "appleTV")]
    AppleTv,
    #[serde(rename = "appleVisionPro")]
    AppleVisionPro,
    #[serde(rename = "appleWatch")]
    AppleWatch,
    #[serde(rename = "appleWatch_2018")]
    AppleWatch2018,
    #[serde(rename = "appleWatch_2021")]
    AppleWatch2021,
    #[serde(rename = "appleWatch_2022")]
    AppleWatch2022,
    #[serde(rename = "appleWatch_2024")]
    AppleWatch2024,
    #[serde(rename = "ipad")]
    Ipad,
    #[serde(rename = "ipadPro")]
    IpadPro,
    #[serde(rename = "ipadPro_2018")]
    IpadPro2018,
    #[serde(rename = "ipad_10_5")]
    Ipad105,
    #[serde(rename = "ipad_11")]
    Ipad11,
    #[serde(rename = "iphone")]
    Iphone,
    #[serde(rename = "iphone5")]
    Iphone5,
    #[serde(rename = "iphone6")]
    Iphone6,
    #[serde(rename = "iphone6+")]
    Iphone6Plus,
    #[serde(rename = "iphone_5_8")]
    Iphone58,
    #[serde(rename = "iphone_6_5")]
    Iphone65,
    #[serde(rename = "iphone_d73")]
    IphoneD73,
    #[serde(rename = "iphone_d74")]
    IphoneD74,
    #[serde(rename = "mac")]
    Mac,
    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "strict", derive(Copy))]
pub enum Device {
    #[serde(rename = "AppleTV4-AppleTV4")]
    AppleTv4AppleTv4,
    #[serde(rename = "AppleTV4KSecondGen-AppleTV4KSecondGen")]
    AppleTv4KSecondGenAppleTv4KSecondGen,
    #[serde(rename = "AppleTV4KThirdGen-AppleTV4KThirdGen")]
    AppleTv4KThirdGenAppleTv4KThirdGen,
    #[serde(rename = "AppleTV62-AppleTV5")]
    AppleTv62AppleTv5,
    #[serde(rename = "AppleVisionPro-AppleVisionPro")]
    AppleVisionProAppleVisionPro,
    #[serde(rename = "iPadWifi-iPadWifi")]
    IPadWifiIPadWifi,
    #[serde(rename = "iPad2Wifi-iPad2Wifi")]
    IPad2WifiIPad2Wifi,
    #[serde(rename = "iPad3G-iPad3G")]
    IPad3GIPad3G,
    #[serde(rename = "iPad23G-iPad23G")]
    IPad23GIPad23G,
    #[serde(rename = "iPhone3GS-iPhone-3GS")]
    IPhone3GsIPhone3Gs,
    #[serde(rename = "iPhone3G-iPhone-3G")]
    IPhone3GIPhone3G,
    #[serde(rename = "iPhone4-iPhone4")]
    IPhone4IPhone4,
    #[serde(rename = "iPhone4S-iPhone4S")]
    IPhone4SIPhone4S,
    #[serde(rename = "iPadThirdGen-iPadThirdGen")]
    IPadThirdGenIPadThirdGen,
    #[serde(rename = "iPadThirdGen4G-iPadThirdGen4G")]
    IPadThirdGen4GIPadThirdGen4G,
    #[serde(rename = "iPhone5-iPhone5")]
    IPhone5IPhone5,
    #[serde(rename = "iPodTouchThirdGen-iPodTouchThirdGen")]
    IPodTouchThirdGenIPodTouchThirdGen,
    #[serde(rename = "iPodTouchFourthGen-iPodTouchFourthGen")]
    IPodTouchFourthGenIPodTouchFourthGen,
    #[serde(rename = "iPodTouchFifthGen-iPodTouchFifthGen")]
    IPodTouchFifthGenIPodTouchFifthGen,
    #[serde(rename = "iPadFourthGen-iPadFourthGen")]
    IPadFourthGenIPadFourthGen,
    #[serde(rename = "iPadFourthGen4G-iPadFourthGen4G")]
    IPadFourthGen4GIPadFourthGen4G,
    #[serde(rename = "iPadMini-iPadMini")]
    IPadMiniIPadMini,
    #[serde(rename = "iPadMini4G-iPadMini4G")]
    IPadMini4GIPadMini4G,
    #[serde(rename = "iPhone5c-iPhone5c")]
    IPhone5cIPhone5c,
    #[serde(rename = "iPhone5s-iPhone5s")]
    IPhone5sIPhone5s,
    #[serde(rename = "iPadAir-iPadAir")]
    IPadAirIPadAir,
    #[serde(rename = "iPadAirCellular-iPadAirCellular")]
    IPadAirCellularIPadAirCellular,
    #[serde(rename = "iPadMiniRetina-iPadMiniRetina")]
    IPadMiniRetinaIPadMiniRetina,
    #[serde(rename = "iPadMiniRetinaCellular-iPadMiniRetinaCellular")]
    IPadMiniRetinaCellularIPadMiniRetinaCellular,
    #[serde(rename = "iPhone6-iPhone6")]
    IPhone6IPhone6,
    #[serde(rename = "iPhone6Plus-iPhone6Plus")]
    IPhone6PlusIPhone6Plus,
    #[serde(rename = "iPadAir2-iPadAir2")]
    IPadAir2IPadAir2,
    #[serde(rename = "iPadAir2Cellular-iPadAir2Cellular")]
    IPadAir2CellularIPadAir2Cellular,
    #[serde(rename = "iPadMini3-iPadMini3")]
    IPadMini3IPadMini3,
    #[serde(rename = "iPadMini3Cellular-iPadMini3Cellular")]
    IPadMini3CellularIPadMini3Cellular,
    #[serde(rename = "iPodTouchSixthGen-iPodTouchSixthGen")]
    IPodTouchSixthGenIPodTouchSixthGen,
    #[serde(rename = "iPhone6s-iPhone6s")]
    IPhone6sIPhone6s,
    #[serde(rename = "iPhone6sPlus-iPhone6sPlus")]
    IPhone6sPlusIPhone6sPlus,
    #[serde(rename = "iPadMini4-iPadMini4")]
    IPadMini4IPadMini4,
    #[serde(rename = "iPadMini4Cellular-iPadMini4Cellular")]
    IPadMini4CellularIPadMini4Cellular,
    #[serde(rename = "iPadPro-iPadPro")]
    IPadProIPadPro,
    #[serde(rename = "iPadProCellular-iPadProCellular")]
    IPadProCellularIPadProCellular,
    #[serde(rename = "iPadPro97-iPadPro97")]
    IPadPro97IPadPro97,
    #[serde(rename = "iPadPro97Cellular-iPadPro97Cellular")]
    IPadPro97CellularIPadPro97Cellular,
    #[serde(rename = "iPhoneSE-iPhoneSE")]
    IPhoneSEIPhoneSE,
    #[serde(rename = "iPhone7-iPhone7")]
    IPhone7IPhone7,
    #[serde(rename = "iPhone7Plus-iPhone7Plus")]
    IPhone7PlusIPhone7Plus,
    #[serde(rename = "iPad611-iPad611")]
    IPad611IPad611,
    #[serde(rename = "iPad612-iPad612")]
    IPad612IPad612,
    #[serde(rename = "iPad71-iPad71")]
    IPad71IPad71,
    #[serde(rename = "iPad72-iPad72")]
    IPad72IPad72,
    #[serde(rename = "iPad73-iPad73")]
    IPad73IPad73,
    #[serde(rename = "iPad74-iPad74")]
    IPad74IPad74,
    #[serde(rename = "iPhone8-iPhone8")]
    IPhone8IPhone8,
    #[serde(rename = "iPhone8Plus-iPhone8Plus")]
    IPhone8PlusIPhone8Plus,
    #[serde(rename = "iPhoneX-iPhoneX")]
    IPhoneXIPhoneX,
    #[serde(rename = "iPad75-iPad75")]
    IPad75IPad75,
    #[serde(rename = "iPad76-iPad76")]
    IPad76IPad76,
    #[serde(rename = "iPhoneXS-iPhoneXS")]
    IPhoneXSIPhoneXS,
    #[serde(rename = "iPhoneXSMax-iPhoneXSMax")]
    IPhoneXSMaxIPhoneXSMax,
    #[serde(rename = "iPhoneXR-iPhoneXR")]
    IPhoneXRIPhoneXR,
    #[serde(rename = "iPad812-iPad812")]
    IPad812IPad812,
    #[serde(rename = "iPad834-iPad834")]
    IPad834IPad834,
    #[serde(rename = "iPad856-iPad856")]
    IPad856IPad856,
    #[serde(rename = "iPad878-iPad878")]
    IPad878IPad878,
    #[serde(rename = "iPadMini5-iPadMini5")]
    IPadMini5IPadMini5,
    #[serde(rename = "iPadMini5Cellular-iPadMini5Cellular")]
    IPadMini5CellularIPadMini5Cellular,
    #[serde(rename = "iPadAir3-iPadAir3")]
    IPadAir3IPadAir3,
    #[serde(rename = "iPadAir3Cellular-iPadAir3Cellular")]
    IPadAir3CellularIPadAir3Cellular,
    #[serde(rename = "iPodTouchSeventhGen-iPodTouchSeventhGen")]
    IPodTouchSeventhGenIPodTouchSeventhGen,
    #[serde(rename = "iPhone11-iPhone11")]
    IPhone11IPhone11,
    #[serde(rename = "iPhone11Pro-iPhone11Pro")]
    IPhone11ProIPhone11Pro,
    #[serde(rename = "iPadSeventhGen-iPadSeventhGen")]
    IPadSeventhGenIPadSeventhGen,
    #[serde(rename = "iPadSeventhGenCellular-iPadSeventhGenCellular")]
    IPadSeventhGenCellularIPadSeventhGenCellular,
    #[serde(rename = "iPhone11ProMax-iPhone11ProMax")]
    IPhone11ProMaxIPhone11ProMax,
    #[serde(rename = "iPhoneSESecondGen-iPhoneSESecondGen")]
    IPhoneSESecondGenIPhoneSESecondGen,
    #[serde(rename = "iPadProSecondGen-iPadProSecondGen")]
    IPadProSecondGenIPadProSecondGen,
    #[serde(rename = "iPadProSecondGenCellular-iPadProSecondGenCellular")]
    IPadProSecondGenCellularIPadProSecondGenCellular,
    #[serde(rename = "iPadProFourthGen-iPadProFourthGen")]
    IPadProFourthGenIPadProFourthGen,
    #[serde(rename = "iPadProFourthGenCellular-iPadProFourthGenCellular")]
    IPadProFourthGenCellularIPadProFourthGenCellular,
    #[serde(rename = "iPhone12Mini-iPhone12Mini")]
    IPhone12MiniIPhone12Mini,
    #[serde(rename = "iPhone12-iPhone12")]
    IPhone12IPhone12,
    #[serde(rename = "iPhone12Pro-iPhone12Pro")]
    IPhone12ProIPhone12Pro,
    #[serde(rename = "iPhone12ProMax-iPhone12ProMax")]
    IPhone12ProMaxIPhone12ProMax,
    #[serde(rename = "iPadAir4-iPadAir4")]
    IPadAir4IPadAir4,
    #[serde(rename = "iPadAir4Cellular-iPadAir4Cellular")]
    IPadAir4CellularIPadAir4Cellular,
    #[serde(rename = "iPadEighthGen-iPadEighthGen")]
    IPadEighthGenIPadEighthGen,
    #[serde(rename = "iPadEighthGenCellular-iPadEighthGenCellular")]
    IPadEighthGenCellularIPadEighthGenCellular,
    #[serde(rename = "iPadProThirdGen-iPadProThirdGen")]
    IPadProThirdGenIPadProThirdGen,
    #[serde(rename = "iPadProThirdGenCellular-iPadProThirdGenCellular")]
    IPadProThirdGenCellularIPadProThirdGenCellular,
    #[serde(rename = "iPadProFifthGen-iPadProFifthGen")]
    IPadProFifthGenIPadProFifthGen,
    #[serde(rename = "iPadProFifthGenCellular-iPadProFifthGenCellular")]
    IPadProFifthGenCellularIPadProFifthGenCellular,
    #[serde(rename = "iPhone13Pro-iPhone13Pro")]
    IPhone13ProIPhone13Pro,
    #[serde(rename = "iPhone13ProMax-iPhone13ProMax")]
    IPhone13ProMaxIPhone13ProMax,
    #[serde(rename = "iPhone13Mini-iPhone13Mini")]
    IPhone13MiniIPhone13Mini,
    #[serde(rename = "iPhone13-iPhone13")]
    IPhone13IPhone13,
    #[serde(rename = "iPadMiniSixthGen-iPadMiniSixthGen")]
    IPadMiniSixthGenIPadMiniSixthGen,
    #[serde(rename = "iPadMiniSixthGenCellular-iPadMiniSixthGenCellular")]
    IPadMiniSixthGenCellularIPadMiniSixthGenCellular,
    #[serde(rename = "iPadNinthGen-iPadNinthGen")]
    IPadNinthGenIPadNinthGen,
    #[serde(rename = "iPadNinthGenCellular-iPadNinthGenCellular")]
    IPadNinthGenCellularIPadNinthGenCellular,
    #[serde(rename = "iPhoneSEThirdGen-iPhoneSEThirdGen")]
    IPhoneSEThirdGenIPhoneSEThirdGen,
    #[serde(rename = "iPadAirFifthGen-iPadAirFifthGen")]
    IPadAirFifthGenIPadAirFifthGen,
    #[serde(rename = "iPadAirFifthGenCellular-iPadAirFifthGenCellular")]
    IPadAirFifthGenCellularIPadAirFifthGenCellular,
    #[serde(rename = "iPhone14-iPhone14")]
    IPhone14IPhone14,
    #[serde(rename = "iPhone14Plus-iPhone14Plus")]
    IPhone14PlusIPhone14Plus,
    #[serde(rename = "iPhone14Pro-iPhone14Pro")]
    IPhone14ProIPhone14Pro,
    #[serde(rename = "iPhone14ProMax-iPhone14ProMax")]
    IPhone14ProMaxIPhone14ProMax,
    #[serde(rename = "iPadTenthGen-iPadTenthGen")]
    IPadTenthGenIPadTenthGen,
    #[serde(rename = "iPadTenthGenCellular-iPadTenthGenCellular")]
    IPadTenthGenCellularIPadTenthGenCellular,
    #[serde(rename = "iPadPro11FourthGen-iPadPro11FourthGen")]
    IPadPro11FourthGenIPadPro11FourthGen,
    #[serde(rename = "iPadPro11FourthGenCellular-iPadPro11FourthGenCellular")]
    IPadPro11FourthGenCellularIPadPro11FourthGenCellular,
    #[serde(rename = "iPadProSixthGen-iPadProSixthGen")]
    IPadProSixthGenIPadProSixthGen,
    #[serde(rename = "iPadProSixthGenCellular-iPadProSixthGenCellular")]
    IPadProSixthGenCellularIPadProSixthGenCellular,
    #[serde(rename = "iPhone15-iPhone15")]
    IPhone15IPhone15,
    #[serde(rename = "iPhone15Plus-iPhone15Plus")]
    IPhone15PlusIPhone15Plus,
    #[serde(rename = "iPhone15Pro-iPhone15Pro")]
    IPhone15ProIPhone15Pro,
    #[serde(rename = "iPhone15ProMax-iPhone15ProMax")]
    IPhone15ProMaxIPhone15ProMax,
    #[serde(rename = "iPadAir11M2-iPadAir11M2")]
    IPadAir11M2IPadAir11M2,
    #[serde(rename = "iPadAir11M2Cellular-iPadAir11M2Cellular")]
    IPadAir11M2CellularIPadAir11M2Cellular,
    #[serde(rename = "iPadAir13M2-iPadAir13M2")]
    IPadAir13M2IPadAir13M2,
    #[serde(rename = "iPadAir13M2Cellular-iPadAir13M2Cellular")]
    IPadAir13M2CellularIPadAir13M2Cellular,
    #[serde(rename = "iPadPro11M4-iPadPro11M4")]
    IPadPro11M4IPadPro11M4,
    #[serde(rename = "iPadPro11M4Cellular-iPadPro11M4Cellular")]
    IPadPro11M4CellularIPadPro11M4Cellular,
    #[serde(rename = "iPadPro13M4-iPadPro13M4")]
    IPadPro13M4IPadPro13M4,
    #[serde(rename = "iPadPro13M4Cellular-iPadPro13M4Cellular")]
    IPadPro13M4CellularIPadPro13M4Cellular,
    #[serde(rename = "iPhone16-iPhone16")]
    IPhone16IPhone16,
    #[serde(rename = "iPhone16Plus-iPhone16Plus")]
    IPhone16PlusIPhone16Plus,
    #[serde(rename = "iPhone16Pro-iPhone16Pro")]
    IPhone16ProIPhone16Pro,
    #[serde(rename = "iPhone16ProMax-iPhone16ProMax")]
    IPhone16ProMaxIPhone16ProMax,
    #[serde(rename = "iPhone17-iPhone17")]
    IPhone17IPhone17,
    #[serde(rename = "iPhone17Pro-iPhone17Pro")]
    IPhone17ProIPhone17Pro,
    #[serde(rename = "iPhone17ProMax-iPhone17ProMax")]
    IPhone17ProMaxIPhone17ProMax,
    #[serde(rename = "iPhoneAir-iPhoneAir")]
    IPhoneAirIPhoneAir,
    #[serde(rename = "iPadMiniA17Pro-iPadMiniA17Pro")]
    IPadMiniA17ProIPadMiniA17Pro,
    #[serde(rename = "iPadMiniA17ProCellular-iPadMiniA17ProCellular")]
    IPadMiniA17ProCellularIPadMiniA17ProCellular,
    #[serde(rename = "iPhone16e-iPhone16e")]
    IPhone16eIPhone16e,
    #[serde(rename = "iPadA16-iPadA16")]
    IPadA16IPadA16,
    #[serde(rename = "iPadA16Cellular-iPadA16Cellular")]
    IPadA16CellularIPadA16Cellular,
    #[serde(rename = "iPadAir11M3-iPadAir11M3")]
    IPadAir11M3IPadAir11M3,
    #[serde(rename = "iPadAir11M3Cellular-iPadAir11M3Cellular")]
    IPadAir11M3CellularIPadAir11M3Cellular,
    #[serde(rename = "iPadAir13M3-iPadAir13M3")]
    IPadAir13M3IPadAir13M3,
    #[serde(rename = "iPadAir13M3Cellular-iPadAir13M3Cellular")]
    IPadAir13M3CellularIPadAir13M3Cellular,
    #[serde(rename = "MacDesktop-MacDesktop")]
    MacDesktopMacDesktop,
    #[serde(rename = "Watch10-Watch10")]
    Watch10Watch10,
    #[serde(rename = "Watch10Cellular-Watch10Cellular")]
    Watch10CellularWatch10Cellular,
    #[serde(rename = "Watch4-Watch4")]
    Watch4Watch4,
    #[serde(rename = "Watch5-Watch5")]
    Watch5Watch5,
    #[serde(rename = "Watch6-Watch6")]
    Watch6Watch6,
    #[serde(rename = "Watch6Cellular-Watch6Cellular")]
    Watch6CellularWatch6Cellular,
    #[serde(rename = "Watch7-Watch7")]
    Watch7Watch7,
    #[serde(rename = "Watch7Cellular-Watch7Cellular")]
    Watch7CellularWatch7Cellular,
    #[serde(rename = "Watch8-Watch8")]
    Watch8Watch8,
    #[serde(rename = "Watch8Cellular-Watch8Cellular")]
    Watch8CellularWatch8Cellular,
    #[serde(rename = "Watch9-Watch9")]
    Watch9Watch9,
    #[serde(rename = "Watch9Cellular-Watch9Cellular")]
    Watch9CellularWatch9Cellular,
    #[serde(rename = "Watch11-Watch11")]
    Watch11Watch11,
    #[serde(rename = "Watch11Cellular-Watch11Cellular")]
    Watch11CellularWatch11Cellular,
    #[serde(rename = "WatchSE-WatchSE")]
    WatchSeWatchSe,
    #[serde(rename = "WatchSECellular-WatchSECellular")]
    WatchSeCellularWatchSeCellular,
    #[serde(rename = "WatchSESecondGen-WatchSESecondGen")]
    WatchSeSecondGenWatchSeSecondGen,
    #[serde(rename = "WatchSESecondGenCellular-WatchSESecondGenCellular")]
    WatchSeSecondGenCellularWatchSeSecondGenCellular,
    #[serde(rename = "WatchSE3-WatchSE3")]
    WatchSe3WatchSe3,
    #[serde(rename = "WatchSE3Cellular-WatchSE3Cellular")]
    WatchSe3CellularWatchSe3Cellular,
    #[serde(rename = "WatchUltra-WatchUltra")]
    WatchUltraWatchUltra,
    #[serde(rename = "WatchUltra2-WatchUltra2")]
    WatchUltra2WatchUltra2,
    #[serde(rename = "WatchUltra3-WatchUltra3")]
    WatchUltra3WatchUltra3,
    #[serde(rename = "iPodTouchSecondGen-iPod-touch-with-mic")]
    IPodTouchSecondGenIPodTouchWithMic,
    #[serde(rename = "iPhoneFirstGen-iPhone")]
    IPhoneFirstGenIPhone,
    #[serde(rename = "iPodTouchFirstGen-iPod-touch")]
    IPodTouchFirstGenIPodTouch,
    #[serde(rename = "iPadPro11M5-iPadPro11M5")]
    IPadPro11M5IPadPro11M5,
    #[serde(rename = "iPadPro13M5-iPadPro13M5")]
    IPadPro13M5IPadPro13M5,
    #[serde(rename = "iPadPro11M5Cellular-iPadPro11M5Cellular")]
    IPadPro11M5CellularIPadPro11M5Cellular,
    #[serde(rename = "iPadPro13M5Cellular-iPadPro13M5Cellular")]
    IPadPro13M5CellularIPadPro13M5Cellular,
    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Other(String),
}
