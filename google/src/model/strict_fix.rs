use super::review::criterion::CriterionType;
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, bounded_static_derive_more::ToStatic)]
pub enum Fix<'a> {
    UnknownReviewCriterionType(Cow<'a, str>),
}

#[must_use]
pub fn review_page_fixes(page: &super::review::Page) -> Vec<Fix<'_>> {
    let mut fixes = vec![];

    for review in &page.reviews {
        if let Some(criteria) = &review.criteria {
            for (criterion, _) in criteria {
                if let CriterionType::Other(value) = criterion {
                    fixes.push(Fix::UnknownReviewCriterionType(value.into()));
                }
            }
        }
    }

    fixes
}
