use super::{content_rating::ContentRatingAdvisory, genre::Genre, lookup::LookupResult};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, bounded_static_derive_more::ToStatic)]
pub enum Fix<'a> {
    UnknownContentRatingAdvisory(Cow<'a, str>),
    UnknownGenreName {
        name: Cow<'a, str>,
        expected_id: u16,
    },
    InvalidGenreName {
        name: Cow<'a, str>,
        found_id: u16,
        expected_id: u16,
    },
}

pub fn app_fixes<'a>(app: &'a super::full::Page<'a>) -> Vec<Fix<'a>> {
    let mut fixes = vec![];

    if let Some(rating_and_advisories) = app.page_data.rating_and_advisories() {
        for content_rating_advisory in rating_and_advisories {
            if let ContentRatingAdvisory::Other(value) = content_rating_advisory {
                fixes.push(Fix::UnknownContentRatingAdvisory(value.into()));
            }
        }
    }

    fixes
}

pub fn lookup_fixes<'a>(list: &'a super::lookup::LookupResultList<'a>) -> Vec<Fix<'a>> {
    let mut fixes = vec![];

    for result in &list.results {
        match result {
            LookupResult::Artist(_artist) => {}
            LookupResult::Software(software) => {
                if let Some(advisories) = &software.advisories {
                    for content_rating_advisory in advisories {
                        if let ContentRatingAdvisory::Other(value) = content_rating_advisory {
                            fixes.push(Fix::UnknownContentRatingAdvisory(value.into()));
                        }
                    }
                }

                if let Some(fix) =
                    genre_fix(software.primary_genre_name.clone(), &software.primary_genre)
                {
                    fixes.push(fix);
                }

                for (genre, name) in software.genres.iter().zip(software.genres_from_name.iter()) {
                    if let Some(fix) = genre_fix(name.clone(), genre) {
                        fixes.push(fix);
                    }
                }
            }
        }
    }

    fixes
}

fn genre_fix<'a>(name: Cow<'a, str>, genre: &Genre) -> Option<Fix<'a>> {
    match name.parse::<Genre>() {
        Ok(genre) if genre.id() != genre.id() => Some(Fix::InvalidGenreName {
            name: name.clone(),
            found_id: genre.id(),
            expected_id: genre.id(),
        }),
        Err(super::genre::Error::InvalidName(name)) => Some(Fix::UnknownGenreName {
            name: name.into(),
            expected_id: genre.id(),
        }),
        _ => None,
    }
}
