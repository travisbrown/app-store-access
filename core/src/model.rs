#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ImageType {
    Icon,
    Header,
    Screenshot,
}

impl ImageType {
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Icon => 0,
            Self::Header => 1,
            Self::Screenshot => 2,
        }
    }
}
