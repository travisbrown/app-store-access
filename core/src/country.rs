use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Other([u8; 2]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Country {
    Austria,
    Australia,
    Belgium,
    Canada,
    China,
    Denmark,
    Finland,
    Germany,
    GreatBritain,
    Greece,
    HongKong,
    India,
    Indonesia,
    Ireland,
    Italy,
    Japan,
    Luxembourg,
    Malaysia,
    Mexico,
    Netherlands,
    NewZealand,
    Norway,
    Pakistan,
    Philippines,
    Poland,
    Portugal,
    Russia,
    SaudiArabia,
    Singapore,
    SouthAfrica,
    SouthKorea,
    Spain,
    Sweden,
    Switzerland,
    Taiwan,
    Thailand,
    Turkey,
    UnitedArabEmirates,
    UnitedKingdom,
    UnitedStates,
    Vietnam,
    Other(Other),
}

const KNOWN_COUNTRIES: [(Country, &str); 40] = [
    (Country::UnitedArabEmirates, "ae"),
    (Country::Austria, "at"),
    (Country::Australia, "au"),
    (Country::Belgium, "be"),
    (Country::Canada, "ca"),
    (Country::Switzerland, "ch"),
    (Country::China, "cn"),
    (Country::Germany, "de"),
    (Country::Denmark, "dk"),
    (Country::Spain, "es"),
    (Country::Finland, "fi"),
    (Country::GreatBritain, "gb"),
    (Country::Greece, "gr"),
    (Country::HongKong, "hk"),
    (Country::Indonesia, "id"),
    (Country::Ireland, "ie"),
    (Country::India, "in"),
    (Country::Italy, "it"),
    (Country::Japan, "jp"),
    (Country::SouthKorea, "kr"),
    (Country::Luxembourg, "lu"),
    (Country::Mexico, "mx"),
    (Country::Malaysia, "my"),
    (Country::Netherlands, "nl"),
    (Country::Norway, "no"),
    (Country::NewZealand, "nz"),
    (Country::Philippines, "ph"),
    (Country::Pakistan, "pk"),
    (Country::Poland, "pl"),
    (Country::Portugal, "pt"),
    (Country::Russia, "ru"),
    (Country::SaudiArabia, "sa"),
    (Country::Sweden, "se"),
    (Country::Singapore, "sg"),
    (Country::Thailand, "th"),
    (Country::Turkey, "tr"),
    (Country::Taiwan, "tw"),
    (Country::UnitedKingdom, "uk"),
    (Country::UnitedStates, "us"),
    (Country::SouthAfrica, "za"),
];

impl Country {
    pub fn as_str(&self) -> &str {
        static AS_STR: LazyLock<HashMap<Country, &'static str>> = LazyLock::new(|| {
            KNOWN_COUNTRIES
                .map(|(country, code)| (country, code))
                .iter()
                .copied()
                .collect()
        });

        match self {
            // Safe because we control the `Other` constructor.
            Self::Other(Other(chars)) => std::str::from_utf8(chars).unwrap(),
            // Safe because we country the `KNOWN_COUNTRIES` definition.
            other => AS_STR.get(other).expect("Fix KNOWN_COUNTRIES definition"),
        }
    }
}

impl Ord for Country {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for Country {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Country {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static FROM_STR: LazyLock<BTreeMap<&'static str, Country>> = LazyLock::new(|| {
            KNOWN_COUNTRIES
                .map(|(country, code)| (code, country))
                .iter()
                .copied()
                .collect()
        });

        FROM_STR
            .get(s)
            .copied()
            .or_else(|| {
                <[u8; 2]>::try_from(s.as_bytes())
                    .ok()
                    .filter(|bytes| {
                        bytes[0].is_ascii_alphabetic() && bytes[1].is_ascii_alphabetic()
                    })
                    .map(|bytes| Self::Other(Other(bytes)))
            })
            .ok_or_else(|| s.to_string())
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

pub mod country_code {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    const EXPECTED: &str = "a country code";

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<super::Country, D::Error> {
        let code: Cow<str> = Deserialize::deserialize(deserializer)?;

        if code.chars().all(|c| c.is_ascii_lowercase()) {
            code.parse().map_err(|_| {
                serde::de::Error::invalid_value(serde::de::Unexpected::Str(&code), &EXPECTED)
            })
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&code),
                &EXPECTED,
            ))
        }
    }

    pub fn serialize<S: Serializer>(
        value: &super::Country,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.as_str().serialize(serializer)
    }
}

pub mod country_code_uppercase {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    const EXPECTED: &str = "a country code";

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<super::Country, D::Error> {
        let code: Cow<str> = Deserialize::deserialize(deserializer)?;

        if code.chars().all(|c| c.is_ascii_uppercase()) {
            code.to_ascii_lowercase().parse().map_err(|_| {
                serde::de::Error::invalid_value(serde::de::Unexpected::Str(&code), &EXPECTED)
            })
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&code),
                &EXPECTED,
            ))
        }
    }

    pub fn serialize<S: Serializer>(
        value: &super::Country,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.as_str().to_ascii_uppercase().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_countries_order() {
        let countries = super::KNOWN_COUNTRIES
            .iter()
            .map(|(country, _)| *country)
            .collect::<Vec<_>>();

        let mut sorted = countries.clone();
        sorted.sort();

        assert_eq!(countries, sorted);
    }

    #[test]
    fn round_trip_countries_order() {
        let countries = super::KNOWN_COUNTRIES
            .iter()
            .map(|(country, _)| *country)
            .collect::<Vec<_>>();

        let codes = countries
            .iter()
            .map(|country| country.as_str())
            .collect::<Vec<_>>();

        let parsed = codes
            .iter()
            .map(|code| code.parse())
            .collect::<Result<Vec<super::Country>, _>>()
            .unwrap();

        assert_eq!(countries, parsed);
    }
}
