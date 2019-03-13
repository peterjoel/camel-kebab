// TODO Other cases?
use std::fmt;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct KebabCase<'a>(CaseValue<'a>);
#[derive(Debug, Clone)]
pub struct SnakeCase<'a>(CaseValue<'a>);
#[derive(Debug, Clone)]
pub struct CamelCase<'a>(CaseValue<'a>);
#[derive(Debug, Clone)]
pub struct PascalCase<'a>(CaseValue<'a>);

#[derive(Debug, Copy, Clone)]
pub enum Word<'a> {
    LowerCase(&'a str),
    MixedCase(&'a str),
    Capitalized(&'a str),
    UpperCase(&'a str),
}

#[derive(Debug, Clone)]
enum CaseValue<'a> {
    Words(Vec<Word<'a>>),
    Joined(Cow<'a, str>),
}

impl<'a> Word<'a> {
    fn write_lowercase<W: fmt::Write>(&self, f: &mut W) -> Result<(), fmt::Error> {
        match &self {
            Word::LowerCase(word) => f.write_str(word),
            Word::MixedCase(word) | Word::UpperCase(word) => write!(f, "{}", word.to_lowercase()),
            Word::Capitalized(word) => {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    write!(f, "{}", first.to_lowercase())?;
                    let rest = unsafe {
                        word.get_unchecked(first.len_utf8()..)
                    };
                    f.write_str(&rest)?;
                }
                Ok(())
            }
        }
    }
    
    fn write_capitalized<W: fmt::Write>(&self, f: &mut W) -> Result<(), fmt::Error> {
        match &self {
            Word::LowerCase(word) => {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    write!(f, "{}", first.to_uppercase())?;
                    let rest = unsafe {
                        word.get_unchecked(first.len_utf8()..)
                    };
                    f.write_str(&rest)?;
                }
                Ok(())
            },
            Word::MixedCase(word) | Word::UpperCase(word) => {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    write!(f, "{}", first.to_uppercase())?;
                    let rest = unsafe {
                        word.get_unchecked(first.len_utf8()..)
                    };
                    f.write_str(&rest.to_lowercase())?;
                }
                Ok(())
            }
            Word::Capitalized(word) => f.write_str(word),
        }
    }
}

pub trait Case { 
    #[inline]
    fn to_kebab_case(&self) -> KebabCase<'_> {
        KebabCase(CaseValue::Words(self.to_words()))
    }

    #[inline]
    fn to_snake_case(&self) -> SnakeCase<'_> {
        SnakeCase(CaseValue::Words(self.to_words()))
    }

    #[inline]
    fn to_camel_case(&self) -> CamelCase<'_> {
        CamelCase(CaseValue::Words(self.to_words()))
    }

    #[inline]
    fn to_pascal_case(&self) -> PascalCase<'_> {
        PascalCase(CaseValue::Words(self.to_words()))
    }

    fn to_words(&self) -> Vec<Word<'_>>;

    fn is_case(source: &str) -> bool;
}



impl<'a> Case for KebabCase<'a> {
    #[inline]
    fn to_kebab_case(&self) -> KebabCase<'_> {
        self.clone()
    }

    fn to_words(&self) -> Vec<Word<'_>> {
        match &self.0 {
            CaseValue::Words(words) => words.clone(),
            CaseValue::Joined(string) => string.split('-').map(|w| Word::LowerCase(w)).collect(),
        }
    }

    #[inline]
    fn is_case(source: &str) -> bool {
        is_lower_case_delimited(source, '-')
    }
}

impl<'a> Case for SnakeCase<'a> {
    #[inline]
    fn to_snake_case(&self) -> SnakeCase<'_> {
        self.clone()
    }

    fn to_words(&self) -> Vec<Word<'_>> {
        match &self.0 {
            CaseValue::Words(words) => words.clone(),
            CaseValue::Joined(string) => string.split('_').map(|w| Word::LowerCase(w)).collect(),
        }
    }

    #[inline]
    fn is_case(source: &str) -> bool {
        is_lower_case_delimited(source, '-')
    }
}

impl<'a> Case for CamelCase<'a> {
    #[inline]
    fn to_camel_case(&self) -> CamelCase<'_> {
        self.clone()
    }

    fn to_words(&self) -> Vec<Word<'_>> {
        match &self.0 {
            CaseValue::Words(words) => words.clone(),
            CaseValue::Joined(string) => split_words_on_uppercase(&string).map(|w| Word::MixedCase(w)).collect(),
        }
    }

    #[inline]
    fn is_case(source: &str) -> bool {
        if let Some(first) = source.chars().next() {
            first.is_lowercase() && source.chars().all(|ch| ch.is_alphanumeric())
        } else {
            true
        }
    }
}

impl<'a> Case for PascalCase<'a> {
    #[inline]
    fn to_pascal_case(&self) -> PascalCase<'_> {
        self.clone()
    }

    fn to_words(&self) -> Vec<Word<'_>> {
        match &self.0 {
            CaseValue::Words(words) => words.clone(),
            CaseValue::Joined(string) => split_words_on_uppercase(&string).map(|w| Word::Capitalized(w)).collect(),
        }
    }

    #[inline]
    fn is_case(source: &str) -> bool {
           if let Some(first) = source.chars().next() {
            first.is_uppercase() && source.chars().all(|ch| ch.is_alphanumeric())
        } else {
            true
        }
    }
}

#[inline]
fn write_lower_delimited<'w, W, I>(words: I, buf: &mut W, sep: char) -> Result<(), fmt::Error>
where
    W: fmt::Write,
    I: IntoIterator<Item = &'w Word<'w>>,
{
    let mut iter = words.into_iter();
    if let Some(first_word) = iter.next() {
        first_word.write_lowercase(buf)?;
        for word in iter {
            buf.write_char(sep)?;   
            word.write_lowercase(buf)?;
        }
    }
    Ok(())
}

fn write_pascal_case<'w, W, I>(words: I, buf: &mut W) -> Result<(), fmt::Error>
where
    W: fmt::Write,
    I: IntoIterator<Item = &'w Word<'w>>,
{
    for word in words {
        word.write_capitalized(buf)?;
    }
    Ok(())
}

impl<'a> fmt::Display for KebabCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.0 {
            CaseValue::Joined(string) => write!(f, "{}", string),
            CaseValue::Words(words) => write_lower_delimited(words.iter(), f, '-'),
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

impl<'a> fmt::Display for PascalCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.0 {
            CaseValue::Joined(string) => string.fmt(f),
            CaseValue::Words(words) => write_pascal_case(words.iter(), f),
        }
    }
}

trait CaseExt {
    fn is_kebab_case(&self) -> bool;
    fn is_snake_case(&self) -> bool;
    fn is_camel_case(&self) -> bool;
    fn is_pascal_case(&self) -> bool; 
    fn as_kebab_case(&self) -> Option<KebabCase<'_>>;
    fn as_snake_case(&self) -> Option<SnakeCase<'_>>;
    fn as_camel_case(&self) -> Option<CamelCase<'_>>;
    fn as_pascal_case(&self) -> Option<PascalCase<'_>>;
    fn as_kebab_case_unchecked(&self) -> KebabCase<'_>;
    fn as_snake_case_unchecked(&self) -> SnakeCase<'_>;
    fn as_camel_case_unchecked(&self) -> CamelCase<'_>;
    fn as_pascal_case_unchecked(&self) -> PascalCase<'_>;   
}

struct UpperCaseSplitIter<'a, C> {
    source: &'a str,
    word_start: usize,
    chars: C,
}

impl<'a, C: Iterator<Item = (usize, char)>> Iterator for UpperCaseSplitIter<'a, C> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i, c)) = self.chars.next() {
            if c.is_uppercase() {
                let word = unsafe {
                    self.source.get_unchecked(self.word_start..i)
                };
                self.word_start = i;
                return Some(word);
            }
        }
        if self.word_start < self.source.len() - 1 {
            let word = unsafe {
                self.source.get_unchecked(self.word_start..)
            };
            self.word_start = self.source.len();
            Some(word)
        } else {
            None
        }
    }
}

#[inline]
fn split_words_on_uppercase(source: &str) -> impl Iterator<Item = &str> {
    UpperCaseSplitIter {
        source,
        chars: source.char_indices().skip(1),
        word_start: 0,
    }
}

fn is_lower_case_delimited(source: &str, delim: char) -> bool {
    let mut delim_allowed = false;
    for ch in source.chars() {
        if ch == delim {
            if delim_allowed {
                delim_allowed = false;
            } else {
                return false;
            }
        // TODO: Possibly reverse the logic on !is_lowercase() to is_uppercase(), to 
        // be more compatible with scripts that do not have case
        } else if !ch.is_alphanumeric() || !ch.is_lowercase() {
            return false;
        } else {
            delim_allowed = true;
        }
    }
    true
}

impl CaseExt for str {
    #[inline]
    fn is_kebab_case(&self) -> bool {
        is_lower_case_delimited(self, '-')
    }

    #[inline]
    fn is_snake_case(&self) -> bool {
        is_lower_case_delimited(self, '_')
    }

    #[inline]
    fn is_camel_case(&self) -> bool {
        if let Some(first) = self.chars().next() {
            first.is_lowercase() && self.chars().all(|ch| ch.is_alphanumeric())
        } else {
            true
        }
    }

    #[inline]
    fn is_pascal_case(&self) -> bool {
        if let Some(first) = self.chars().next() {
            first.is_uppercase() && self.chars().all(|ch| ch.is_alphanumeric())
        } else {
            true
        }
    }

    #[inline]
    fn as_kebab_case(&self) -> Option<KebabCase<'_>> {
        if self.is_kebab_case() {
            Some(self.as_kebab_case_unchecked())
        } else {
            None
        }
    }

    #[inline]
    fn as_kebab_case_unchecked(&self) -> KebabCase<'_> {
        KebabCase(CaseValue::Joined(Cow::Borrowed(self)))
    }

    #[inline]
    fn as_snake_case(&self) -> Option<SnakeCase<'_>> {
        if self.is_snake_case() {
            Some(self.as_snake_case_unchecked())
        } else {
            None
        }
    }

    #[inline]
    fn as_snake_case_unchecked(&self) -> SnakeCase<'_> {
        SnakeCase(CaseValue::Joined(Cow::Borrowed(self)))
    }

    #[inline]
    fn as_camel_case(&self) -> Option<CamelCase<'_>> {
        if self.is_camel_case() {
            Some(self.as_camel_case_unchecked())
        } else {
            None
        }
    }

    #[inline]
    fn as_camel_case_unchecked(&self) -> CamelCase<'_> {
        CamelCase(CaseValue::Joined(Cow::Borrowed(self)))
    }

    #[inline]
    fn as_pascal_case(&self) -> Option<PascalCase<'_>> {
        if self.is_pascal_case() {
            Some(self.as_pascal_case_unchecked())
        } else {
            None
        }
    }

    #[inline]
    fn as_pascal_case_unchecked(&self) -> PascalCase<'_> {
        PascalCase(CaseValue::Joined(Cow::Borrowed(self)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_to_snake() {
        let camel: CamelCase = "thisIsCamelCase".as_camel_case_unchecked();
        assert_eq!("this_is_camel_case", format!("{}", camel.to_snake_case()));
    }
}