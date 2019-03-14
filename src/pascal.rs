use crate::internals::{split_words_on_uppercase, write_pascal_case, Case, CaseValue, Word};
use std::fmt;

#[derive(Debug, Clone)]
pub struct PascalCase<'a>(pub(crate) CaseValue<'a>);

impl<'a> Case<'a> for PascalCase<'a> {
    #[inline]
    fn str_is_case(source: &str) -> bool {
        if let Some(first) = source.chars().next() {
            first.is_uppercase() && source.chars().all(|ch| ch.is_alphanumeric())
        } else {
            true
        }
    }

    #[inline]
    fn str_as_case_unchecked(source: &'a str) -> Self {
        PascalCase(CaseValue::Joined(source))
    }

    #[inline]
    fn from_words(words: Vec<Word<'a>>) -> Self {
        PascalCase(CaseValue::Words(words))
    }

    #[inline]
    fn to_words(self) -> Vec<Word<'a>> {
        match self.0 {
            CaseValue::Words(words) => words,
            CaseValue::Joined(string) => split_words_on_uppercase(string)
                .map(|w| Word::Capitalized(w))
                .collect(),
        }
    }
}

impl<'a> fmt::Display for PascalCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.0 {
            CaseValue::Joined(string) => string.fmt(f),
            CaseValue::Words(words) => write_pascal_case(words.iter(), f),
        }
    }
}

impl_str_ext! {
    trait: PascalCaseExt,
    case: PascalCase,
    as: as_pascal_case,
    as_unchecked: as_pascal_case_unchecked,
    is: is_pascal_case
}

impl_eq!(PascalCase);
