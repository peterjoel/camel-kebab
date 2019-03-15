use crate::internals::{self, split_words_on_uppercase, write_pascal_case, CaseValue, Word};
use crate::Case;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CamelCase<'a>(pub(crate) CaseValue<'a>);

impl<'a> Case<'a> for CamelCase<'a> {
    #[inline]
    fn str_is_case(source: &str) -> bool {
        if let Some(first) = source.chars().next() {
            first.is_lowercase() && source.chars().all(|ch| ch.is_alphanumeric())
        } else {
            true
        }
    }

    #[inline]
    fn str_as_case_unchecked(source: &'a str) -> Self {
        CamelCase(CaseValue::Joined(source))
    }
}

impl<'a> internals::Case<'a> for CamelCase<'a> {
    #[inline]
    fn from_cased_words(words: Vec<Word<'a>>) -> Self {
        CamelCase(CaseValue::Words(words))
    }

    #[inline]
    fn to_cased_words(self) -> Vec<Word<'a>> {
        match self.0 {
            CaseValue::Words(words) => words,
            CaseValue::Joined(string) => {
                let mut words = split_words_on_uppercase(&string);
                let mut vec = Vec::new();
                if let Some(first) = words.next() {
                    vec.push(Word::lower_case(first));
                    vec.extend(words.map(|w| Word::capitalized(w)));
                }
                vec
            }
        }
    }
}

impl<'a> fmt::Display for CamelCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.0 {
            CaseValue::Joined(string) => string.fmt(f),
            CaseValue::Words(words) => {
                let mut words = words.iter();
                if let Some(first_word) = words.next() {
                    first_word.write_lowercase(f)?;
                    write_pascal_case(words, f)?;
                }
                Ok(())
            }
        }
    }
}

impl_str_ext! {
    trait: CamelCaseExt,
    case: CamelCase,
    as: as_camel_case,
    as_unchecked: as_camel_case_unchecked,
    is: is_camel_case
}

impl_eq!(CamelCase);
