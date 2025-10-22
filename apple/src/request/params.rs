pub mod review {
    #[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
    pub enum SortOrder {
        #[default]
        Recent,
        Helpful,
    }

    impl SortOrder {
        #[must_use]
        pub const fn as_str(&self) -> &'static str {
            match self {
                Self::Recent => "mostRecent",
                Self::Helpful => "mostHelpful",
            }
        }
    }

    impl std::fmt::Display for SortOrder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_str())
        }
    }

    impl std::str::FromStr for SortOrder {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "mostRecent" => Ok(Self::Recent),
                "mostHelpful" => Ok(Self::Helpful),
                other => Err(other.to_string()),
            }
        }
    }
}
