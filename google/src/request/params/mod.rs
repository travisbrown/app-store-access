pub mod developer {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum DeveloperId {
        Numeric(u64),
        Name(String),
    }

    impl DeveloperId {
        pub fn parse_url_path(input: &str) -> Option<Self> {
            static PATH_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
                regex::Regex::new(r"^/(?:work|store)/apps/(dev|developer)\?id=(.+)$").unwrap()
            });

            PATH_RE
                .captures(input)
                .and_then(|captures| captures.get(1).zip(captures.get(2)))
                .and_then(|(endpoint_path_part_match, id_match)| {
                    match endpoint_path_part_match.as_str() {
                        "dev" => id_match
                            .as_str()
                            .parse::<u64>()
                            .ok()
                            .map(DeveloperId::Numeric),
                        "developer" => Some(Self::Name(
                            urlencoding::decode(id_match.as_str()).ok()?.to_string(),
                        )),
                        _ => None,
                    }
                })
        }
    }

    impl std::str::FromStr for DeveloperId {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            s.parse::<u64>().map_or_else(
                |_| Ok(Self::Name(s.to_string())),
                |id| Ok(Self::Numeric(id)),
            )
        }
    }

    impl std::fmt::Display for DeveloperId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Numeric(id) => id.fmt(f),
                Self::Name(name) => f.write_str(name),
            }
        }
    }

    impl DeveloperId {
        #[must_use]
        pub const fn endpoint_path_part(&self) -> &'static str {
            match self {
                Self::Numeric(_) => "dev",
                Self::Name(_) => "developer",
            }
        }

        #[must_use]
        pub const fn is_numeric(&self) -> bool {
            matches!(self, Self::Numeric(_))
        }
    }
}

pub mod review {
    #[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
    pub enum SortOrder {
        #[default]
        Newest,
        Rating,
        Helpfulness,
    }

    impl SortOrder {
        #[must_use]
        pub const fn code(&self) -> u8 {
            match self {
                Self::Newest => 2,
                Self::Rating => 3,
                Self::Helpfulness => 1,
            }
        }
    }

    impl TryFrom<u8> for SortOrder {
        type Error = u8;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                2 => Ok(Self::Newest),
                3 => Ok(Self::Rating),
                1 => Ok(Self::Helpfulness),
                other => Err(other),
            }
        }
    }

    impl std::fmt::Display for SortOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.code())
        }
    }
}

pub mod search {
    #[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
    pub enum PriceFilter {
        #[default]
        All,
        Free,
        Paid,
    }

    impl PriceFilter {
        #[must_use]
        pub const fn code(&self) -> u8 {
            match self {
                Self::All => 0,
                Self::Free => 1,
                Self::Paid => 2,
            }
        }
    }

    impl std::str::FromStr for PriceFilter {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.parse::<u8>() {
                Ok(0) => Ok(Self::All),
                Ok(1) => Ok(Self::Free),
                Ok(2) => Ok(Self::Paid),
                _ => Err(()),
            }
        }
    }

    impl std::fmt::Display for PriceFilter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.code())
        }
    }
}
