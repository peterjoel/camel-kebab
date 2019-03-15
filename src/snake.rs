use crate::internals::{is_lower_case_delimited, self, write_lower_delimited, CaseValue, Word};
use crate::Case;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SnakeCase<'a>(pub(crate) CaseValue<'a>);

impl<'a> Case<'a> for SnakeCase<'a> {
    #[inline]
    fn str_is_case(source: &str) -> bool {
        is_lower_case_delimited(source, '-')
    }

    #[inline]
    fn str_as_case_unchecked(source: &'a str) -> Self {
        SnakeCase(CaseValue::Joined(source))
    }
}

impl<'a> internals::Case<'a> for SnakeCase<'a> {
    #[inline]
    fn from_cased_words(words: Vec<Word<'a>>) -> Self {
        SnakeCase(CaseValue::Words(words))
    }

    #[inline]
    fn to_cased_words(self) -> Vec<Word<'a>> {
        match self.0 {
            CaseValue::Words(words) => words,
            CaseValue::Joined(string) => string.split('_').map(|w| Word::lower_case(w)).collect(),
        }
    }
}

impl<'a> fmt::Display for SnakeCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.0 {
            CaseValue::Joined(string) => string.fmt(f),
            CaseValue::Words(words) => write_lower_delimited(words.iter(), f, '_'),
        }
    }
}

impl_str_ext! {
    trait: SnakeCaseExt,
    case: SnakeCase,
    as: as_snake_case,
    as_unchecked: as_snake_case_unchecked,
    is: is_snake_case
}

impl_eq!(SnakeCase);
