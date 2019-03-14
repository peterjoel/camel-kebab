use crate::internals::{is_lower_case_delimited, self, write_lower_delimited, CaseValue, Word};
use crate::Case;
use std::fmt;

#[derive(Debug, Clone)]
pub struct KebabCase<'a>(pub(crate) CaseValue<'a>);

impl<'a> Case<'a> for KebabCase<'a> {
    #[inline]
    fn str_is_case(source: &str) -> bool {
        is_lower_case_delimited(source, '-')
    }

    #[inline]
    fn str_as_case_unchecked(source: &'a str) -> Self {
        KebabCase(CaseValue::Joined(source))
    }
}

impl<'a> internals::Case<'a> for KebabCase<'a> {
    #[inline]
    fn from_cased_words(words: Vec<Word<'a>>) -> Self {
        KebabCase(CaseValue::Words(words))
    }

    #[inline]
    fn to_cased_words(self) -> Vec<Word<'a>> {
        match self.0 {
            CaseValue::Words(words) => words,
            CaseValue::Joined(string) => string.split('-').map(|w| Word::LowerCase(w)).collect(),
        }
    }
}

impl<'a> fmt::Display for KebabCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.0 {
            CaseValue::Joined(string) => write!(f, "{}", string),
            CaseValue::Words(words) => write_lower_delimited(words.iter(), f, '-'),
        }
    }
}

impl_str_ext! {
    trait: KebabCaseExt,
    case: KebabCase,
    as: as_kebab_case,
    as_unchecked: as_kebab_case_unchecked,
    is: is_kebab_case
}

impl_eq!(KebabCase);
