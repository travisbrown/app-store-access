use serde_field_attributes::integer_str;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("Invalid name")]
    InvalidName(String),
    #[error("Invalid ID")]
    InvalidId(u16),
}

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Genre {
    All = 0,
    Business = 6000,
    Weather = 6001,
    Utilities = 6002,
    Travel = 6003,
    Sports = 6004,
    SocialNetworking = 6005,
    Reference = 6006,
    Productivity = 6007,
    PhotoAndVideo = 6008,
    News = 6009,
    Navigation = 6010,
    Music = 6011,
    Lifestyle = 6012,
    HealthAndFitness = 6013,
    Games = 6014,
    Finance = 6015,
    Entertainment = 6016,
    Education = 6017,
    Books = 6018,
    Medical = 6020,
    MagazinesAndNewspapers = 6021,
    Catalogs = 6022,
    FoodAndDrink = 6023,
    Shopping = 6024,
    Stickers = 6025,
    DeveloperTools = 6026,
    GraphicsAndDesign = 6027,
    GamesAction = 7001,
    GamesAdventure = 7002,
    GamesCasual = 7003,
    GamesBoard = 7004,
    GamesCard = 7005,
    GamesCasino = 7006,
    GamesDice = 7007,
    GamesFamily = 7009,
    GamesMusic = 7011,
    GamesPuzzle = 7012,
    GamesRacing = 7013,
    GamesRolePlaying = 7014,
    GamesSimulation = 7015,
    GamesSports = 7016,
    GamesStrategy = 7017,
    GamesTrivia = 7018,
    GamesWord = 7019,
    StickersEmojiAndExpressions = 16001,
    StickersAnimalsAndNature = 16003,
    StickersArt = 16005,
    StickersCelebrations = 16006,
    StickersCelebrities = 16007,
    StickersComicsAndCartoons = 16008,
    StickersEatingAndDrinking = 16009,
    StickersGaming = 16010,
    StickersMoviesAndTv = 16014,
    StickersMusic = 16015,
    StickersPeople = 16017,
    StickersPlacesAndObjects = 16019,
    StickersSportsAndActivities = 16021,
    StickersKidsAndFamily = 16025,
    StickersFashion = 16026,
}

impl FromStr for Genre {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static FROM_NAME_MAP: LazyLock<HashMap<String, Genre>> = LazyLock::new(|| {
            GENRE_INFO
                .iter()
                .flat_map(|(genre, _, names)| {
                    names.iter().map(|name| ((*name).to_string(), *genre))
                })
                .collect()
        });

        FROM_NAME_MAP
            .get(s)
            .copied()
            .ok_or_else(|| Self::Err::InvalidName(s.to_string()))
    }
}

impl Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Genre {
    pub fn id(&self) -> u16 {
        static TO_ID_MAP: LazyLock<BTreeMap<Genre, u16>> = LazyLock::new(|| {
            GENRE_INFO
                .iter()
                .map(|(genre, id, _)| (*genre, *id))
                .collect()
        });

        TO_ID_MAP
            .get(self)
            .copied()
            .expect("Fix the genre list definition")
    }

    pub fn from_id(id: u16) -> Option<Self> {
        static FROM_ID_MAP: LazyLock<BTreeMap<u16, Genre>> = LazyLock::new(|| {
            GENRE_INFO
                .iter()
                .map(|(genre, id, _)| (*id, *genre))
                .collect()
        });

        FROM_ID_MAP.get(&id).copied()
    }

    pub fn as_str(&self) -> &'static str {
        static TO_NAME_MAP: LazyLock<BTreeMap<Genre, &str>> = LazyLock::new(|| {
            GENRE_INFO
                .iter()
                .map(|(genre, _, names)| (*genre, names[0]))
                .collect()
        });

        TO_NAME_MAP
            .get(self)
            .expect("Fix the genre list definition")
    }

    #[must_use]
    pub fn url(&self) -> String {
        format!("https://itunes.apple.com/us/genre/id{}", self.id())
    }

    pub fn is_valid_url(&self, url: &str) -> bool {
        static URL_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
            regex::Regex::new(r"^https\:\/\/itunes\.apple\.com\/([a-z]{2})\/genre\/id(\d+)$")
                .unwrap()
        });

        URL_RE
            .captures(url)
            .and_then(|captures| captures.get(2))
            .and_then(|m| m.as_str().parse::<u16>().ok())
            .is_some_and(|id| id == self.id())
    }

    pub fn values() -> impl Iterator<Item = (Self, u16, &'static str)> {
        GENRE_INFO
            .iter()
            .map(|(genre, code, names)| (*genre, *code, names[0]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct InternalGenre<'a> {
    #[serde(rename = "genreId", with = "integer_str")]
    genre_id: u16,
    #[serde(rename = "mediaType", with = "integer_str")]
    media_type: u8,
    name: Cow<'a, str>,
    url: Cow<'a, str>,
}

impl<'de> serde::de::Deserialize<'de> for Genre {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let internal = InternalGenre::deserialize(deserializer)?;

        if internal.media_type == 8 {
            match Self::from_id(internal.genre_id) {
                None => Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Unsigned(internal.genre_id.into()),
                    &"a genre ID",
                )),
                Some(genre) => {
                    #[cfg(feature = "strict")]
                    // TODO: fix the handling of "Sports" (which is ambigous) here.
                    if internal.name.parse::<Self>() != Ok(genre)
                        && internal.name != "Sports"
                        && internal.name != "Sport"
                        && internal.name != "Music"
                    {
                        Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&internal.name),
                            &genre.as_str(),
                        ))
                    } else {
                        let expected_url = genre.url();

                        if genre.is_valid_url(&internal.url) {
                            Ok(genre)
                        } else {
                            Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(&internal.url),
                                &expected_url.as_str(),
                            ))
                        }
                    }

                    #[cfg(not(feature = "strict"))]
                    Ok(genre)
                }
            }
        } else {
            Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Unsigned(internal.media_type.into()),
                &"8",
            ))
        }
    }
}

impl serde::ser::Serialize for Genre {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let internal = InternalGenre {
            genre_id: self.id(),
            media_type: 8,
            name: self.as_str().into(),
            url: self.url().into(),
        };

        internal.serialize(serializer)
    }
}

const GENRE_INFO: [(Genre, u16, &[&str]); 60] = [
    (Genre::All, 0, &["All"]),
    (
        Genre::Business,
        6000,
        &[
            "Business",
            "Wirtschaft",
            "Économie et entreprise",
            "Economía y empresa",
        ],
    ),
    (
        Genre::Weather,
        6001,
        &["Weather", "Wetter", "Météo", "Погода"],
    ),
    (
        Genre::Utilities,
        6002,
        &[
            "Utilities",
            "Dienst\u{ad}programme",
            "Utilitaires",
            "Utilidades",
            "Утилиты",
        ],
    ),
    (
        Genre::Travel,
        6003,
        &["Travel", "Reisen", "Voyages", "Viajes"],
    ),
    (Genre::Sports, 6004, &["Sports", "Sport", "Deportes"]),
    (
        Genre::SocialNetworking,
        6005,
        &[
            "Social Networking",
            "Soziale Netze",
            "Réseaux sociaux",
            "Redes sociales",
            "Социальные сети",
        ],
    ),
    (
        Genre::Reference,
        6006,
        &["Reference", "Nachschlagewerke", "Referencia", "Справочники"],
    ),
    (
        Genre::Productivity,
        6007,
        &[
            "Productivity",
            "Produktivität",
            "Productivité",
            "Productividad",
            "Производительность",
        ],
    ),
    (
        Genre::PhotoAndVideo,
        6008,
        &[
            "Photo & Video",
            "Foto und Video",
            "Photo et vidéo",
            "Foto y vídeo",
            "Фото и видео",
        ],
    ),
    (Genre::News, 6009, &["News", "Nachrichten", "Actualités"]),
    (Genre::Navigation, 6010, &["Navigation"]),
    (Genre::Music, 6011, &["Music", "Musik"]),
    (
        Genre::Lifestyle,
        6012,
        &["Lifestyle", "Style de vie", "Estilo de vida", "Образ жизни"],
    ),
    (
        Genre::HealthAndFitness,
        6013,
        &[
            "Health & Fitness",
            "Gesundheit und Fitness",
            "Forme et santé",
            "Salud y forma física",
        ],
    ),
    (
        Genre::Games,
        6014,
        &["Games", "Spiele", "Jeux", "Juegos", "Игры"],
    ),
    (Genre::Finance, 6015, &["Finance", "Finanzen", "Finanzas"]),
    (
        Genre::Entertainment,
        6016,
        &[
            "Entertainment",
            "Unterhaltung",
            "Divertissement",
            "Entretenimiento",
            "Развлечения",
        ],
    ),
    (
        Genre::Education,
        6017,
        &["Education", "Bildung", "Éducation", "Educación"],
    ),
    // "Book" seen at least once.
    (
        Genre::Books,
        6018,
        &["Books", "Book", "Bücher", "Livres", "Libros"],
    ),
    (
        Genre::Medical,
        6020,
        &["Medical", "Medizin", "Médecine", "Medicina"],
    ),
    (
        Genre::MagazinesAndNewspapers,
        6021,
        &[
            "Magazines & Newspapers",
            "Zeitungen und Zeitschriften",
            "Journaux et magazines",
            "Revistas y periódicos",
            "Газеты и журналы",
        ],
    ),
    (Genre::Catalogs, 6022, &["Catalogs"]),
    (
        Genre::FoodAndDrink,
        6023,
        &[
            "Food & Drink",
            "Essen und Trinken",
            "Cuisine et boissons",
            "Comida y bebida",
        ],
    ),
    (Genre::Shopping, 6024, &["Shopping", "Compras"]),
    (Genre::Stickers, 6025, &["Stickers", "Sticker"]),
    (
        Genre::DeveloperTools,
        6026,
        &[
            "Developer Tools",
            "Entwickler-Tools",
            "Outils de développement",
            "Para desarrolladores",
            "Разработчикам",
        ],
    ),
    (
        Genre::GraphicsAndDesign,
        6027,
        &[
            "Graphics & Design",
            "Grafik und Design",
            "Graphisme et design",
            "Diseño gráfico",
            "Графика и дизайн",
        ],
    ),
    (Genre::GamesAction, 7001, &["Action", "Acción"]),
    (
        Genre::GamesAdventure,
        7002,
        &[
            "Adventure",
            "Abenteuer",
            "Aventure",
            "Aventura",
            "Приключения",
        ],
    ),
    (
        Genre::GamesCasual,
        7003,
        &["Casual", "Parties rapides", "Recreativos"],
    ),
    (
        Genre::GamesBoard,
        7004,
        &["Board", "Brettspiele", "Jeux de société", "Juegos de mesa"],
    ),
    (
        Genre::GamesCard,
        7005,
        &["Card", "Karten", "Cartes", "Cartas"],
    ),
    (Genre::GamesCasino, 7006, &["Casino"]),
    (Genre::GamesDice, 7007, &["Dice"]),
    (
        Genre::GamesFamily,
        7009,
        &["Family", "Familie", "Famille", "Familiar"],
    ),
    (Genre::GamesMusic, 7011, &["Music"]),
    (Genre::GamesPuzzle, 7012, &["Puzzle", "Casse-tête", "Puzle"]),
    (
        Genre::GamesRacing,
        7013,
        &["Racing", "Rennsport", "Course", "Carreras"],
    ),
    (
        Genre::GamesRolePlaying,
        7014,
        // German-language responses include both "Role-Playing" and "Rollenspiel".
        &[
            "Role Playing",
            "Roleplaying",
            "Role-Playing",
            "Rollenspiel",
            "Jeux de rôle",
            "Juegos de rol",
        ],
    ),
    (
        Genre::GamesSimulation,
        7015,
        &["Simulation", "Simulación", "Симуляторы"],
    ),
    (Genre::GamesSports, 7016, &["Sports", "Deportes"]),
    (
        Genre::GamesStrategy,
        7017,
        &["Strategy", "Strategie", "Stratégie", "Estrategia"],
    ),
    (
        Genre::GamesTrivia,
        7018,
        &["Trivia", "Quiz- und Denkspiele", "Quiz"],
    ),
    (
        Genre::GamesWord,
        7019,
        &["Word", "Wortspiele", "Mots", "Palabras"],
    ),
    (
        Genre::StickersEmojiAndExpressions,
        16001,
        &["Emoji & Expressions", "Emojis und Emotionen"],
    ),
    (
        Genre::StickersAnimalsAndNature,
        16003,
        &["Animals & Nature", "Tiere und Natur"],
    ),
    (Genre::StickersArt, 16005, &["Art"]),
    (
        Genre::StickersCelebrations,
        16006,
        &["Celebrations", "Feierlichkeiten"],
    ),
    (Genre::StickersCelebrities, 16007, &["Celebrities", "Stars"]),
    (
        Genre::StickersComicsAndCartoons,
        16008,
        &["Comics & Cartoons", "Comics und Cartoons"],
    ),
    (
        Genre::StickersEatingAndDrinking,
        16009,
        &["Eating & Drinking"],
    ),
    (Genre::StickersGaming, 16010, &["Gaming"]),
    (Genre::StickersMoviesAndTv, 16014, &["Movies & TV"]),
    (Genre::StickersMusic, 16015, &["Music"]),
    (Genre::StickersPeople, 16017, &["People", "Leute"]),
    (
        Genre::StickersPlacesAndObjects,
        16019,
        &["Places & Objects"],
    ),
    (
        Genre::StickersSportsAndActivities,
        16021,
        &["Sports & Activities", "Sport und Aktivitäten"],
    ),
    (Genre::StickersKidsAndFamily, 16025, &["Kids & Family"]),
    (Genre::StickersFashion, 16026, &["Fashion"]),
];

pub mod from_id {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::model::genre::Genre;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<super::Genre, D::Error> {
        let id = u16::deserialize(deserializer)?;

        Genre::from_id(id).ok_or_else(|| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Unsigned(id.into()),
                &"a genre ID",
            )
        })
    }

    pub fn serialize<S: Serializer>(
        value: &super::Genre,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.id().serialize(serializer)
    }
}

pub mod from_name {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<super::Genre, D::Error> {
        let name: Cow<str> = Deserialize::deserialize(deserializer)?;

        name.parse().map_err(|_| {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&name), &"a genre name")
        })
    }

    pub fn serialize<S: Serializer>(
        value: &super::Genre,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.as_str().serialize(serializer)
    }
}

pub mod from_name_opt {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<super::Genre>, D::Error> {
        let name: Option<Cow<str>> = Deserialize::deserialize(deserializer)?;

        name.map(|name| {
            name.parse().map_err(|_| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&name),
                    &"an optional genre name",
                )
            })
        })
        .map_or(Ok(None), |name| name.map(Some))
    }

    pub fn serialize<S: Serializer>(
        value: &Option<super::Genre>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        value.map(|value| value.as_str()).serialize(serializer)
    }
}

pub mod from_names {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<super::Genre>, D::Error> {
        let names = Vec::<Cow<str>>::deserialize(deserializer)?;

        names
            .iter()
            .map(|name| {
                name.parse().map_err(|_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(name),
                        &"a genre name",
                    )
                })
            })
            .collect()
    }

    pub fn serialize<S: Serializer>(
        value: &[super::Genre],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let names = value.iter().map(super::Genre::as_str).collect::<Vec<_>>();

        names.serialize(serializer)
    }
}

pub mod from_id_strs {
    use crate::model::genre::Genre;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::borrow::Cow;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<super::Genre>, D::Error> {
        let id_strs = Vec::<Cow<str>>::deserialize(deserializer)?;

        id_strs
            .iter()
            .map(|id_str| {
                let id: u16 = id_str.parse().map_err(|_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(id_str),
                        &"a genre ID string",
                    )
                })?;

                Genre::from_id(id).ok_or_else(|| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(id.into()),
                        &"a genre ID string",
                    )
                })
            })
            .collect()
    }

    pub fn serialize<S: Serializer>(
        value: &[super::Genre],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let ids = value
            .iter()
            .map(|genre| genre.id().to_string())
            .collect::<Vec<_>>();

        ids.serialize(serializer)
    }
}
