#[macro_use]
mod macros;
mod internals;
mod camel;
pub use camel::CamelCase;
mod kebab;
pub use kebab::KebabCase;
mod snake;
pub use snake::SnakeCase;
mod pascal;
pub use pascal::PascalCase;

pub trait Case<'a>: internals::Case<'a> {
    #[inline]
    fn from_words<W>(words: W) -> Self 
    where
        W: IntoIterator<Item = &'a str>,
    {
        let words = words.into_iter()
            .map(|word| internals::Word::MixedCase(word))
            .collect();
        Self::from_cased_words(words)
    }

    fn str_is_case(source: &str) -> bool;

    #[inline]
    fn str_as_case(source: &'a str) -> Option<Self> {
        if Self::str_is_case(source) {
            Some(Self::str_as_case_unchecked(source))
        } else {
            None
        }
    }

    fn str_as_case_unchecked(source: &'a str) -> Self;
}

trait CaseExt {
    fn is_case<'a, C: Case<'a>>(&self) -> bool;
    fn as_case<'a, C: Case<'a>>(&'a self) -> Option<C>;
    fn as_case_unchecked<'a, C: Case<'a>>(&'a self) -> C;
}

impl CaseExt for str {
    #[inline]
    fn is_case<'a, C: Case<'a>>(&self) -> bool {
        C::str_is_case(self)
    }

    #[inline]
    fn as_case<'a, C: Case<'a>>(&'a self) -> Option<C> {
        C::str_as_case(self)
    }

    #[inline]
    fn as_case_unchecked<'a, C: Case<'a>>(&'a self) -> C {
        C::str_as_case_unchecked(self)
    }
}

impl_from!(CamelCase, KebabCase, PascalCase => SnakeCase);
impl_from!(KebabCase, PascalCase, SnakeCase => CamelCase);
impl_from!(PascalCase, SnakeCase, CamelCase => KebabCase);
impl_from!(SnakeCase, CamelCase, KebabCase => PascalCase);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_into() {
        let camel: CamelCase = "thisWasCamelCase".as_case_unchecked();
        let snake: SnakeCase = camel.into();
        assert_eq!("this_was_camel_case", format!("{}", snake));
    }

    #[test]
    fn test_camel_no_match() {
        assert_eq!(None, "ThisIsNotCamelCase".as_case::<CamelCase>());
    }
}
