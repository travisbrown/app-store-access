use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::sync::LazyLock;

use app_store_access::country::Country;

pub struct MarketCode(pub u32);

impl MarketCode {
    #[must_use]
    pub fn known_countries() -> Vec<Country> {
        let mut countries = MARKETS_VALUES
            .iter()
            .map(|(_, code)| Self(*code).into())
            .collect::<Vec<Country>>();

        countries.sort();
        countries
    }
}

impl Default for MarketCode {
    fn default() -> Self {
        Self(143441)
    }
}

impl From<Country> for MarketCode {
    fn from(value: Country) -> Self {
        MARKETS
            .get(&value.as_str().to_uppercase())
            .map(|market_code| Self(*market_code))
            .unwrap_or_default()
    }
}

impl From<MarketCode> for Country {
    fn from(value: MarketCode) -> Self {
        COUNTRIES
            .get(&value.0)
            .and_then(|country_code| country_code.to_lowercase().parse().ok())
            .unwrap()
    }
}

impl Display for MarketCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

static MARKETS: LazyLock<HashMap<String, u32>> = LazyLock::new(|| {
    MARKETS_VALUES
        .iter()
        .map(|(code, value)| ((*code).to_string(), *value))
        .collect()
});

static COUNTRIES: LazyLock<BTreeMap<u32, String>> = LazyLock::new(|| {
    MARKETS_VALUES
        .iter()
        .map(|(code, value)| (*value, (*code).to_string()))
        .collect()
});

pub const MARKETS_VALUES: [(&str, u32); 110] = [
    ("US", 143441),
    ("FR", 143442),
    ("DE", 143443),
    ("GB", 143444),
    ("AT", 143445),
    ("BE", 143446),
    ("FI", 143447),
    ("GR", 143448),
    ("IE", 143449),
    ("IT", 143450),
    ("LU", 143451),
    ("NL", 143452),
    ("PT", 143453),
    ("ES", 143454),
    ("CA", 143455),
    ("SE", 143456),
    ("NO", 143457),
    ("DK", 143458),
    ("CH", 143459),
    ("AU", 143460),
    ("NZ", 143461),
    ("JP", 143462),
    ("HK", 143463),
    ("SG", 143464),
    ("CN", 143465),
    ("KR", 143466),
    ("IN", 143467),
    ("MX", 143468),
    ("RU", 143469),
    ("TW", 143470),
    ("VN", 143471),
    ("ZA", 143472),
    ("MY", 143473),
    ("PH", 143474),
    ("TH", 143475),
    ("ID", 143476),
    ("PK", 143477),
    ("PL", 143478),
    ("SA", 143479),
    ("TR", 143480),
    ("AE", 143481),
    ("HU", 143482),
    ("CL", 143483),
    ("NP", 143484),
    ("PA", 143485),
    ("LK", 143486),
    ("RO", 143487),
    ("CZ", 143489),
    ("HR", 143494),
    ("CR", 143495),
    ("SK", 143496),
    ("LB", 143497),
    ("QA", 143498),
    ("SI", 143499),
    ("CO", 143501),
    ("VE", 143502),
    ("BR", 143503),
    ("GT", 143504),
    ("AR", 143505),
    ("SV", 143506),
    ("PE", 143507),
    ("EC", 143509),
    ("HN", 143510),
    ("JM", 143511),
    ("NI", 143512),
    ("PY", 143513),
    ("UY", 143514),
    ("MO", 143515),
    ("EG", 143516),
    ("EE", 143518),
    ("LV", 143519),
    ("LT", 143520),
    ("MT", 143521),
    ("AM", 143524),
    ("BW", 143525),
    ("BG", 143526),
    ("JO", 143528),
    ("KE", 143529),
    ("MK", 143530),
    ("MG", 143531),
    ("ML", 143532),
    ("MU", 143533),
    ("NE", 143534),
    ("SN", 143535),
    ("TN", 143536),
    ("UG", 143537),
    ("AI", 143538),
    ("BB", 143541),
    ("BM", 143542),
    ("VG", 143543),
    ("KY", 143544),
    ("DM", 143545),
    ("GD", 143546),
    ("MS", 143547),
    ("BZ", 143555),
    ("BO", 143556),
    ("CY", 143557),
    ("IS", 143558),
    ("BH", 143559),
    ("BN", 143560),
    ("NG", 143561),
    ("OM", 143562),
    ("DZ", 143563),
    ("AO", 143564),
    ("BY", 143565),
    ("UZ", 143566),
    ("AZ", 143568),
    ("YE", 143571),
    ("TZ", 143572),
    ("GH", 143573),
];

#[cfg(test)]
mod tests {
    use crate::request::markets::{MARKETS_VALUES, MarketCode};
    use app_store_access::country::Country;

    #[test]
    fn country_from_market_code() {
        for (_, code) in MARKETS_VALUES {
            let code = MarketCode(code);
            let _country: Country = code.into();
        }
    }
}
