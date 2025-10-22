use crate::request::params::review::SortOrder;
use regex::Regex;
use std::borrow::Cow;
use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Unknown {
    Generic(Generic),
    Review(Review),
}

impl FromStr for Unknown {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("qnKhOb") {
            s.parse().ok().map(Self::Generic)
        } else {
            s.parse().ok().map(Self::Review)
        }
        .ok_or_else(|| s.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Generic {
    pub number: usize,
    pub token: String,
}

impl Generic {
    #[must_use]
    pub const fn new(number: usize, token: String) -> Self {
        Self { number, token }
    }
}

impl Display for Generic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "f.req=%5B%5B%5B%22qnKhOb%22%2C%22%5B%5Bnull%2C%5B%5B10%2C%5B10%2C{}%5D%5D%2Ctrue%2Cnull%2C%5B96%2C27%2C4%2C8%2C57%2C30%2C110%2C79%2C11%2C16%2C49%2C1%2C3%2C9%2C12%2C104%2C55%2C56%2C51%2C10%2C34%2C77%5D%5D%2Cnull%2C%5C%22{}%5C%22%5D%5D%22%2Cnull%2C%22generic%22%5D%5D%5D",
            self.number, self.token
        )
    }
}

impl FromStr for Generic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static GENERIC_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"^f\.req=\[\[\["qnKhOb","\[\[null,\[\[10,\[10,(\d+)\]\],true,null,\[96,27,4,8,57,30,110,79,11,16,49,1,3,9,12,104,55,56,51,10,34,77\]\],null,\\"([^\]]+)\\"\]\]",null,"generic"\]\]\]$"#).unwrap()
        });

        let decoded = urlencoding::decode(s).map_err(|_| s.to_string())?;

        GENERIC_RE
            .captures(&decoded)
            .and_then(|captures| {
                captures
                    .get(1)
                    .zip(captures.get(2))
                    .and_then(|(number_match, token_match)| {
                        number_match
                            .as_str()
                            .parse::<usize>()
                            .ok()
                            .map(|number| Self::new(number, token_match.as_str().to_string()))
                    })
            })
            .ok_or_else(|| s.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    pub app_id: String,
    pub sort_order: SortOrder,
    pub number: usize,
    pub token: Option<String>,
}

impl Review {
    #[must_use]
    pub const fn new(
        app_id: String,
        sort_order: SortOrder,
        number: usize,
        token: Option<String>,
    ) -> Self {
        Self {
            app_id,
            sort_order,
            number,
            token,
        }
    }
}

impl Display for Review {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = self.token.as_ref().map_or(Cow::Borrowed("null"), |token| {
            Cow::Owned(format!("%5C%22{token}%5C%22"))
        });

        write!(
            f,
            "f.req=%5B%5B%5B%22UsvDTd%22%2C%22%5Bnull%2Cnull%2C%5B2%2C{}%2C%5B{}%2Cnull%2C{token}%5D%2Cnull%2C%5B%5D%5D%2C%5B%5C%22{}%5C%22%2C7%5D%5D%22%2Cnull%2C%22generic%22%5D%5D%5D",
            self.sort_order, self.number, self.app_id
        )
    }
}

impl FromStr for Review {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REVIEW_RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"^f\.req=\[\[\["UsvDTd","\[null,null,\[2,(\d+),\[(\d+),null,([^\]]+)\],null,\[\]\],\[\\"([^\]]+)\\",7\]\]",null,"generic"\]\]\]$"#).unwrap()
        });

        let decoded = urlencoding::decode(s).map_err(|_| s.to_string())?;

        REVIEW_RE
            .captures(&decoded)
            .and_then(|captures| {
                captures
                    .get(1)
                    .zip(captures.get(2))
                    .zip(captures.get(3))
                    .zip(captures.get(4))
                    .and_then(
                        |(((sort_order_match, number_match), token_match), app_id_match)| {
                            sort_order_match
                                .as_str()
                                .parse::<u8>()
                                .ok()
                                .and_then(|sort_order_code| {
                                    SortOrder::try_from(sort_order_code).ok()
                                })
                                .zip(number_match.as_str().parse::<usize>().ok())
                                .zip(match token_match.as_str() {
                                    "null" => Some(None),
                                    other
                                        if other.starts_with(r#"\""#)
                                            && other.ends_with(r#"\""#) =>
                                    {
                                        Some(Some(other[2..other.len() - 2].to_string()))
                                    }
                                    _ => None,
                                })
                                .map(|((sort_order, number), token)| {
                                    Self::new(
                                        app_id_match.as_str().to_string(),
                                        sort_order,
                                        number,
                                        token,
                                    )
                                })
                        },
                    )
            })
            .ok_or_else(|| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::request::params::review::SortOrder;

    #[test]
    fn parse_generic_body() {
        let token = "YpUBSj8KPQgKELWezOMNEKrasekHEMeU080HEKqIuooPEO7AwKkFEMO8ursLEPi_tcEDEKGqpu8LELWWmoMCELeS7ElQCmoYZ29VRERJbHprV3NwT1Ezb041dkRuUT09sgIyChcKEzU3MDAzMTM2MTg3ODYxNzc3MDUQCBIXCAESEzU3MDAzMTM2MTg3ODYxNzc3MDWwEgA";
        let value = "f.req=%5B%5B%5B%22qnKhOb%22%2C%22%5B%5Bnull%2C%5B%5B10%2C%5B10%2C100%5D%5D%2Ctrue%2Cnull%2C%5B96%2C27%2C4%2C8%2C57%2C30%2C110%2C79%2C11%2C16%2C49%2C1%2C3%2C9%2C12%2C104%2C55%2C56%2C51%2C10%2C34%2C77%5D%5D%2Cnull%2C%5C%22YpUBSj8KPQgKELWezOMNEKrasekHEMeU080HEKqIuooPEO7AwKkFEMO8ursLEPi_tcEDEKGqpu8LELWWmoMCELeS7ElQCmoYZ29VRERJbHprV3NwT1Ezb041dkRuUT09sgIyChcKEzU3MDAzMTM2MTg3ODYxNzc3MDUQCBIXCAESEzU3MDAzMTM2MTg3ODYxNzc3MDWwEgA%5C%22%5D%5D%22%2Cnull%2C%22generic%22%5D%5D%5D";

        assert_eq!(
            value.parse::<super::Generic>().unwrap(),
            super::Generic::new(100, token.to_string())
        );
    }

    #[test]
    fn parse_review_body() {
        let token = "CtgBIs8BATWeoiYP_njwK9kkPqUzCoLgzvoz0-akH_UdYtuCfSq8HbFukgg-DJSyu9NvH8KNE3H4hNMS-FSQHQzFFwtLNCGhH7LehB2DZly35kC9D3L7BIGS20qOCkYRONefFPhLm5eYse5_q0OVGDqTHjoO5PyBBkKFPR6y1NULaUagEASLoa7N5d9NI5Oivu3XvqD522tZf5NNcf8RcT8AOohcFgIagNkI0bxoNzagSD_qvszlBYCzKXscM-6HEYjA81ioZ-EmeMnIbEkSlzj0E2L0KOCk9MYG";
        let value = "f.req=%5B%5B%5B%22UsvDTd%22%2C%22%5Bnull%2Cnull%2C%5B2%2C2%2C%5B150%2Cnull%2C%5C%22CtgBIs8BATWeoiYP_njwK9kkPqUzCoLgzvoz0-akH_UdYtuCfSq8HbFukgg-DJSyu9NvH8KNE3H4hNMS-FSQHQzFFwtLNCGhH7LehB2DZly35kC9D3L7BIGS20qOCkYRONefFPhLm5eYse5_q0OVGDqTHjoO5PyBBkKFPR6y1NULaUagEASLoa7N5d9NI5Oivu3XvqD522tZf5NNcf8RcT8AOohcFgIagNkI0bxoNzagSD_qvszlBYCzKXscM-6HEYjA81ioZ-EmeMnIbEkSlzj0E2L0KOCk9MYG%5C%22%5D%2Cnull%2C%5B%5D%5D%2C%5B%5C%22com.google.android.apps.translate%5C%22%2C7%5D%5D%22%2Cnull%2C%22generic%22%5D%5D%5D";

        assert_eq!(
            value.parse::<super::Review>().unwrap(),
            super::Review::new(
                "com.google.android.apps.translate".to_string(),
                SortOrder::Newest,
                150,
                Some(token.to_string())
            )
        );
    }
}
