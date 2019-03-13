// TODO Nothing is being lower-cased!
// TODO Other cases?
use std::fmt;
use std::borrow::Cow;

#[derive(Debug)]
pub struct KebabCase<'a>(CaseValue<'a>);
#[derive(Debug)]
pub struct SnakeCase<'a>(CaseValue<'a>);
#[derive(Debug)]
pub struct CamelCase<'a>(CaseValue<'a>);
#[derive(Debug)]
pub struct PascalCase<'a>(CaseValue<'a>);

// enum Word<'a> {
//     LowerCase(&'a str),
//     MixedCase(&'a str),
//     Capitalized(&'a str),
//     UpperCase(&'a str),
// }

// impl<'a> Word<'a> {
//     fn write_lowercase<W: fmt::Write>(&self, f: &mut W) -> Result<(), fmt::Error> {
//         match &self {
//             Word::LowerCase(word) => f.write_str(word),
//             Word::MixedCase(word) | Word::UpperCase(word) => write!(f, "{}", word.to_lowercase()),
//             Word::Capitalized(word) => {
//                 let mut chars = word.chars();
//                 if let Some(first) = chars.next() {
//                     write!(f, "{}", first.to_lowercase())?;
//                     let rest = unsafe {
//                         word.get_unchecked(first.len_utf8()..)
//                     };
//                     f.write_str(&rest)?;
//                 }
//                 Ok(())
//             }
//         }
//     }
    
//     fn write_capitalized<W: fmt::Write>(&self, f: &mut W) -> Result<(), fmt::Error> {
//         match &self {
//             Word::LowerCase(word) => {
//                 let mut chars = word.chars();
//                 if let Some(first) = chars.next() {
//                     write!(f, "{}", first.to_uppercase())?;
//                     let rest = unsafe {
//                         word.get_unchecked(first.len_utf8()..)
//                     };
//                     f.write_str(&rest)?;
//                 }
//                 Ok(())
//             },
//             Word::MixedCase(word) | Word::UpperCase(word) => {
//                 let mut chars = word.chars();
//                 if let Some(first) = chars.next() {
//                     write!(f, "{}", first.to_uppercase())?;
//                     let rest = unsafe {
//                         word.get_unchecked(first.len_utf8()..)
//                     };
//                     f.write_str(&rest.to_lowercase())?;
//                 }
//                 Ok(())
//             }
//             Word::Capitalized(word) => f.write_str(word),
//         }
//     }
// }

pub trait ToWords {
    // TODO change this Vec to a non-allocating iterator
    fn to_words(&self) -> Vec<&str>;
}

pub trait Case { 
    fn to_kebab_case(&self) -> KebabCase<'_>;

    fn to_snake_case(&self) -> SnakeCase<'_>;

    fn to_camel_case(&self) -> CamelCase<'_> ;

    fn to_pascal_case(&self) -> PascalCase<'_>;
}

impl<T> Case for T where T: ToWords {
    fn to_kebab_case(&self) -> KebabCase<'_> {
        KebabCase(CaseValue::MixedCaseWords(self.to_words()))
    }

    fn to_snake_case(&self) -> SnakeCase<'_> {
        SnakeCase(CaseValue::MixedCaseWords(self.to_words()))
    }

    fn to_camel_case(&self) -> CamelCase<'_> {
        CamelCase(CaseValue::MixedCaseWords(self.to_words()))
    }

    fn to_pascal_case(&self) -> PascalCase<'_> {
        PascalCase(CaseValue::MixedCaseWords(self.to_words()))
    } 
}

impl<'a> ToWords for KebabCase<'a> {
    fn to_words(&self) -> Vec<&str> {
        match &self.0 {
            CaseValue::MixedCaseWords(words) | CaseValue::LowerCaseWords(words) => words.clone(),
            CaseValue::Joined(string) => {
                string.split('-').collect()
            }
        }
    }
}

impl<'a> ToWords for SnakeCase<'a> {
    fn to_words(&self) -> Vec<&str> {
        match &self.0 {
            CaseValue::MixedCaseWords(words) | CaseValue::LowerCaseWords(words) => words.clone(),
            CaseValue::Joined(string) => {
                string.split('_').collect()
            }
        }
    }
}

impl<'a> ToWords for CamelCase<'a> {
    fn to_words(&self) -> Vec<&str> {
        match &self.0 {
            CaseValue::MixedCaseWords(words) | CaseValue::LowerCaseWords(words) => words.clone(),
            CaseValue::Joined(string) => {
                split_words_on_uppercase(&string)
            }
        }
    }
}

impl<'a> ToWords for PascalCase<'a> {
    fn to_words(&self) -> Vec<&str> {
        match &self.0 {
            CaseValue::MixedCaseWords(words) | CaseValue::LowerCaseWords(words) => words.clone(),
            CaseValue::Joined(string) => {
                split_words_on_uppercase(&string)
            }
        }
    }
}

#[derive(Debug)]
// TODO capture statically if the string is already lower/upper cased
enum CaseValue<'a> {
    // TODO change this Vec to a non-allocating iterator
    MixedCaseWords(Vec<&'a str>),
    LowerCaseWords(Vec<&'a str>),
    Joined(Cow<'a, str>),
}

impl<'a> CaseValue<'a> {
    fn normalize_to(&mut self, value: String) {
        *self = CaseValue::Joined(value.into());
    }
}

#[inline]
fn concat_lower_delimited<'w, W, I>(words: I, buf: &mut W, sep: char) -> Result<(), fmt::Error>
where
    W: fmt::Write,
    I: IntoIterator<Item = &'w &'w str>,
{
    let mut iter = words.into_iter();
    if let Some(first_word) = iter.next() {
        buf.write_str(first_word)?;
        for word in iter {
            buf.write_char(sep)?;   
            buf.write_str(word)?;
        }
    }
    Ok(())
}

fn write_pascal_case<'a, W, I>(words: I, buf: &mut W) -> Result<(), fmt::Error>
where
    W: fmt::Write,
    I: IntoIterator<Item = &'a str> + 'a,
{
    let mut iter = words.into_iter();
    if let Some(first_word) = iter.next() {
        buf.write_str(first_word)?;
        for word in iter {
            if let Some(first_char) = word.chars().next() {
                write!(buf, "{}", first_char.to_uppercase())?;
                let other_chars = unsafe {
                    // safe because checking the first char's byte length prevents 
                    // accidentally breaking a character boundary
                    word.get_unchecked(first_char.len_utf8()..) 
                };
                buf.write_str(other_chars)?;
            }
        }
    }
    Ok(())
}

impl<'a> fmt::Display for KebabCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.0 {
            CaseValue::Joined(string) => write!(f, "{}", string),
            CaseValue::LowerCaseWords(words) => concat_lower_delimited(words.iter(), f, '-'),
            CaseValue::MixedCaseWords(words) => concat_lower_delimited(words.iter(), f, '-'),
        }
    }
}

impl<'a> fmt::Display for SnakeCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.0 {
            CaseValue::Joined(string) => string.fmt(f),
            CaseValue::LowerCaseWords(words) => concat_lower_delimited(words.iter(), f, '_'),
            CaseValue::MixedCaseWords(words) => concat_lower_delimited(words.iter(), f, '_'),
        }
    }
}

impl<'a> fmt::Display for CamelCase<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match &self.0 {
            CaseValue::Joined(string) => string.fmt(f),
            CaseValue::MixedCaseWords(words) | CaseValue::LowerCaseWords(words) => {
                let mut words = words.iter().cloned();
                if let Some(first_word) = words.next() {
                    f.write_str(first_word)?;
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
            CaseValue::MixedCaseWords(words) | CaseValue::LowerCaseWords(words) => write_pascal_case(words.iter().cloned(), f),
        }
    }
}

impl<'a> KebabCase<'a> {
    pub fn from_string(source: String) -> Option<Self> {
        if source.is_kebab_case() {
            Some(KebabCase(CaseValue::Joined(source.into())))
        } else {
            None
        }
    }

    pub unsafe fn from_string_unchecked(source: String) -> Self {
        KebabCase(CaseValue::Joined(source.into()))
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

// TODO make the result a non-allocating iterator
fn split_words_on_uppercase(source: &str) -> Vec<&str> {
    let mut words = Vec::with_capacity(source.len());
    let mut word_start = 0;
    for (i, c) in source.char_indices() {
        if c.is_uppercase() {
            if i > 0 {
                let word = unsafe {
                    source.get_unchecked(word_start..i)
                };
                words.push(word);
            }
            word_start = i;
        }
    }
    if word_start < source.len() - 1 {
        let word = unsafe {
            source.get_unchecked(word_start..)
        };
        words.push(word);
    }
    words.shrink_to_fit();
    words
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

    fn is_camel_case(&self) -> bool {
        if let Some(first) = self.chars().next() {
            first.is_lowercase() && self.chars().all(|ch| ch.is_alphanumeric())
        } else {
            true
        }
    }

    fn is_pascal_case(&self) -> bool {
        if let Some(first) = self.chars().next() {
            first.is_uppercase() && self.chars().all(|ch| ch.is_alphanumeric())
        } else {
            true
        }
    }

    fn as_kebab_case(&self) -> Option<KebabCase<'_>> {
        if self.is_kebab_case() {
            Some(self.as_kebab_case_unchecked())
        } else {
            None
        }
    }

    #[inline]
    fn as_kebab_case_unchecked(&self) -> KebabCase<'_> {
        KebabCase(CaseValue::Joined(self.into()))
    }

    fn as_snake_case(&self) -> Option<SnakeCase<'_>> {
        if self.is_snake_case() {
            Some(self.as_snake_case_unchecked())
        } else {
            None
        }
    }

    #[inline]
    fn as_snake_case_unchecked(&self) -> SnakeCase<'_> {
        SnakeCase(CaseValue::Joined(self.into()))
    }

    fn as_camel_case(&self) -> Option<CamelCase<'_>> {
        if self.is_camel_case() {
            Some(self.as_camel_case_unchecked())
        } else {
            None
        }
    }

    #[inline]
    fn as_camel_case_unchecked(&self) -> CamelCase<'_> {
        CamelCase(CaseValue::Joined(self.into()))
    }

    fn as_pascal_case(&self) -> Option<PascalCase<'_>> {
        if self.is_pascal_case() {
            Some(self.as_pascal_case_unchecked())
        } else {
            None
        }
    }

    #[inline]
    fn as_pascal_case_unchecked(&self) -> PascalCase<'_> {
        PascalCase(CaseValue::Joined(self.into()))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_to_snake() {
        let camel = "thisIsCamelCase".as_camel_case_unchecked();
        assert_eq!("this_is_camel_case", format!("{}", camel.to_snake_case()));
    }
}