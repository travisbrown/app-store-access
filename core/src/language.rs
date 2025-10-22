use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

/// A pair of a language and a country.
///
/// By default the expected string representation has the language in lowercase and the country in uppercase.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RegionalLanguage {
    pub language: Language,
    pub country: super::country::Country,
}

impl RegionalLanguage {
    #[must_use]
    pub const fn new(language: Language, country: super::country::Country) -> Self {
        Self { language, country }
    }

    pub fn parse_lowercase(s: &str) -> Result<Self, String> {
        let mut parts = s.split('-');
        let (language_str, country_str) = parts
            .next()
            .zip(parts.next())
            .ok_or_else(|| s.to_string())?;

        if parts.next().is_none() {
            Ok(Self::new(
                language_str.parse()?,
                country_str.to_ascii_uppercase().parse()?,
            ))
        } else {
            Err(s.to_string())
        }
    }
}

impl FromStr for RegionalLanguage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let (language_str, country_str) = parts
            .next()
            .zip(parts.next())
            .ok_or_else(|| s.to_string())?;

        if parts.next().is_none() {
            Ok(Self::new(language_str.parse()?, country_str.parse()?))
        } else {
            Err(s.to_string())
        }
    }
}

impl Display for RegionalLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{}",
            self.language,
            self.country.as_str().to_ascii_uppercase()
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Other([u8; 2]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Language {
    Abkhazian,
    Afrikaans,
    Akan,
    Albanian,
    Amharic,
    Arabic,
    Aragonese,
    Armenian,
    Assamese,
    Azerbaijani,
    Basque,
    Belarusian,
    Bengali,
    Bosnian,
    Breton,
    Bulgarian,
    Burmese,
    Catalan,
    Chamorro,
    Chichewa,
    Chinese,
    Corsican,
    Croatian,
    Czech,
    Danish,
    Dhivehi,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Finnish,
    French,
    Gaelic,
    Galician,
    Ganda,
    Georgian,
    German,
    Greek,
    Gujarati,
    Haitian,
    Hausa,
    Hebrew,
    Hindi,
    Hungarian,
    Icelandic,
    Igbo,
    Indonesian,
    Interlingua,
    Irish,
    Italian,
    Japanese,
    Javanese,
    Kalaallisut,
    Kannada,
    Kazakh,
    Khmer,
    Kikuyu,
    Kinyarwanda,
    Korean,
    Kurdish,
    Kyrgyz,
    Lao,
    Latin,
    Latvian,
    Lithuanian,
    Luxembourgish,
    Macedonian,
    Malagasy,
    Malay,
    Malayalam,
    Maltese,
    Maori,
    Marathi,
    Mongolian,
    Nepali,
    NorthernSami,
    NorwegianBokmal,
    NorwegianNynorsk,
    Occitan,
    Oriya,
    Oromo,
    Pashto,
    Persian,
    Polish,
    Portuguese,
    Punjabi,
    Romansh,
    Romanian,
    Russian,
    Samoan,
    Serbian,
    Shona,
    Sindhi,
    Sinhala,
    Slovak,
    Slovenian,
    Somali,
    SouthernSotho,
    Spanish,
    Sundanese,
    Swahili,
    Swedish,
    Tagalog,
    Tajik,
    Tamil,
    Tatar,
    Telugu,
    Thai,
    Tibetan,
    Tigrinya,
    Turkish,
    Turkmen,
    Ukrainian,
    Urdu,
    Uyghur,
    Uzbek,
    Vietnamese,
    Welsh,
    Xhosa,
    Yiddish,
    Yoruba,
    Zulu,
    Other(Other),
}

pub const KNOWN_LANGUAGES: [(Language, &str); 122] = [
    (Language::Abkhazian, "ab"),
    (Language::Afrikaans, "af"),
    (Language::Akan, "ak"),
    (Language::Amharic, "am"),
    (Language::Aragonese, "an"),
    (Language::Arabic, "ar"),
    (Language::Assamese, "as"),
    (Language::Azerbaijani, "az"),
    (Language::Belarusian, "be"),
    (Language::Bulgarian, "bg"),
    (Language::Bengali, "bn"),
    (Language::Tibetan, "bo"),
    (Language::Breton, "br"),
    (Language::Bosnian, "bs"),
    (Language::Catalan, "ca"),
    (Language::Chamorro, "ch"),
    (Language::Corsican, "co"),
    (Language::Czech, "cs"),
    (Language::Welsh, "cy"),
    (Language::Danish, "da"),
    (Language::German, "de"),
    (Language::Dhivehi, "dv"),
    (Language::Greek, "el"),
    (Language::English, "en"),
    (Language::Esperanto, "eo"),
    (Language::Spanish, "es"),
    (Language::Estonian, "et"),
    (Language::Basque, "eu"),
    (Language::Persian, "fa"),
    (Language::Finnish, "fi"),
    (Language::French, "fr"),
    (Language::Irish, "ga"),
    (Language::Gaelic, "gd"),
    (Language::Galician, "gl"),
    (Language::Gujarati, "gu"),
    (Language::Hausa, "ha"),
    (Language::Hebrew, "he"),
    (Language::Hindi, "hi"),
    (Language::Croatian, "hr"),
    (Language::Haitian, "ht"),
    (Language::Hungarian, "hu"),
    (Language::Armenian, "hy"),
    (Language::Interlingua, "ia"),
    (Language::Indonesian, "id"),
    (Language::Igbo, "ig"),
    (Language::Icelandic, "is"),
    (Language::Italian, "it"),
    (Language::Japanese, "ja"),
    (Language::Javanese, "jv"),
    (Language::Georgian, "ka"),
    (Language::Kikuyu, "ki"),
    (Language::Kazakh, "kk"),
    (Language::Kalaallisut, "kl"),
    (Language::Khmer, "km"),
    (Language::Kannada, "kn"),
    (Language::Korean, "ko"),
    (Language::Kurdish, "ku"),
    (Language::Kyrgyz, "ky"),
    (Language::Latin, "la"),
    (Language::Luxembourgish, "lb"),
    (Language::Ganda, "lg"),
    (Language::Lao, "lo"),
    (Language::Lithuanian, "lt"),
    (Language::Latvian, "lv"),
    (Language::Malagasy, "mg"),
    (Language::Maori, "mi"),
    (Language::Macedonian, "mk"),
    (Language::Malayalam, "ml"),
    (Language::Mongolian, "mn"),
    (Language::Marathi, "mr"),
    (Language::Malay, "ms"),
    (Language::Maltese, "mt"),
    (Language::Burmese, "my"),
    (Language::NorwegianBokmal, "nb"),
    (Language::Nepali, "ne"),
    (Language::Dutch, "nl"),
    (Language::NorwegianNynorsk, "nn"),
    (Language::Chichewa, "ny"),
    (Language::Occitan, "oc"),
    (Language::Oromo, "om"),
    (Language::Oriya, "or"),
    (Language::Punjabi, "pa"),
    (Language::Polish, "pl"),
    (Language::Pashto, "ps"),
    (Language::Portuguese, "pt"),
    (Language::Romansh, "rm"),
    (Language::Romanian, "ro"),
    (Language::Russian, "ru"),
    (Language::Kinyarwanda, "rw"),
    (Language::Sindhi, "sd"),
    (Language::NorthernSami, "se"),
    (Language::Sinhala, "si"),
    (Language::Slovak, "sk"),
    (Language::Slovenian, "sl"),
    (Language::Samoan, "sm"),
    (Language::Shona, "sn"),
    (Language::Somali, "so"),
    (Language::Albanian, "sq"),
    (Language::Serbian, "sr"),
    (Language::SouthernSotho, "st"),
    (Language::Sundanese, "su"),
    (Language::Swedish, "sv"),
    (Language::Swahili, "sw"),
    (Language::Tamil, "ta"),
    (Language::Telugu, "te"),
    (Language::Tajik, "tg"),
    (Language::Thai, "th"),
    (Language::Tigrinya, "ti"),
    (Language::Turkmen, "tk"),
    (Language::Tagalog, "tl"),
    (Language::Turkish, "tr"),
    (Language::Tatar, "tt"),
    (Language::Uyghur, "ug"),
    (Language::Ukrainian, "uk"),
    (Language::Urdu, "ur"),
    (Language::Uzbek, "uz"),
    (Language::Vietnamese, "vi"),
    (Language::Xhosa, "xh"),
    (Language::Yiddish, "yi"),
    (Language::Yoruba, "yo"),
    (Language::Chinese, "zh"),
    (Language::Zulu, "zu"),
];

impl Language {
    pub fn as_str(&self) -> &str {
        static AS_STR: LazyLock<HashMap<Language, &'static str>> = LazyLock::new(|| {
            KNOWN_LANGUAGES
                .map(|(language, code)| (language, code))
                .iter()
                .copied()
                .collect()
        });

        match self {
            // Safe because we control the `Other` constructor.
            Self::Other(Other(chars)) => std::str::from_utf8(chars).unwrap(),
            // Safe because we country the `KNOWN_LANGS` definition.
            other => AS_STR.get(other).expect("Fix KNOWN_LANGUAGES definition"),
        }
    }
}

impl Ord for Language {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for Language {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static FROM_STR: LazyLock<BTreeMap<&'static str, Language>> = LazyLock::new(|| {
            KNOWN_LANGUAGES
                .map(|(language, code)| (code, language))
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

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

pub mod language_code {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    const EXPECTED: &str = "a language code";

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<super::Language, D::Error> {
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
        value: &super::Language,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.as_str().serialize(serializer)
    }
}

pub mod language_code_uppercase {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    const EXPECTED: &str = "a language code";

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<super::Language, D::Error> {
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
        value: &super::Language,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.as_str().to_ascii_uppercase().serialize(serializer)
    }
}

pub mod language_code_uppercase_array {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    const EXPECTED: &str = "an array of language codes";

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<super::Language>, D::Error> {
        let codes: Vec<Cow<str>> = Deserialize::deserialize(deserializer)?;

        codes
            .iter()
            .map(|code| {
                if code.chars().all(|c| c.is_ascii_uppercase()) {
                    code.to_ascii_lowercase().parse().map_err(|_| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Str(code), &EXPECTED)
                    })
                } else {
                    Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(code),
                        &EXPECTED,
                    ))
                }
            })
            .collect()
    }

    pub fn serialize<S: Serializer>(
        value: &[super::Language],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value
            .iter()
            .map(|language| language.as_str().to_ascii_uppercase())
            .collect::<Vec<_>>()
            .serialize(serializer)
    }
}

pub mod regional_language_code_lowercase {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    const EXPECTED: &str = "a fully-lowercase regional language code";

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<super::RegionalLanguage, D::Error> {
        let code: Cow<str> = Deserialize::deserialize(deserializer)?;

        code.parse().map_err(|_| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&code), &EXPECTED)
        })
    }

    pub fn serialize<S: Serializer>(
        value: &super::RegionalLanguage,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.to_string().to_ascii_lowercase().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn known_countries_order() {
        let langs = super::KNOWN_LANGUAGES
            .iter()
            .map(|(language, _)| *language)
            .collect::<Vec<_>>();

        let mut sorted = langs.clone();
        sorted.sort();

        assert_eq!(langs, sorted);
    }

    #[test]
    fn round_trip_countries_order() {
        let langs = super::KNOWN_LANGUAGES
            .iter()
            .map(|(language, _)| *language)
            .collect::<Vec<_>>();

        let codes = langs
            .iter()
            .map(|country| country.as_str())
            .collect::<Vec<_>>();

        let parsed = codes
            .iter()
            .map(|code| code.parse())
            .collect::<Result<Vec<super::Language>, _>>()
            .unwrap();

        assert_eq!(langs, parsed);
    }
}
