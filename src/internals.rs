use std::fmt;
use std::iter::Peekable;


#[derive(Debug, Copy, Clone)]
pub struct Word<'a>(WordInner<'a>);

#[derive(Debug, Copy, Clone)]
enum WordInner<'a> {
    LowerCase(&'a str),
    MixedCase(&'a str),
    Capitalized(&'a str),
    UpperCase(&'a str),
}

#[derive(Debug, Clone)]
pub enum CaseValue<'a> {
    Words(Vec<Word<'a>>),
    Joined(&'a str),
}

impl<'a> Word<'a> {
    /// It is up to the caller to verify that the input is in the correct case
    pub fn lower_case(input: &'a str) -> Word<'a> {
        let word = Word(WordInner::LowerCase(input));
        debug_assert!(word.is_valid());
        word
    }

    /// It is up to the caller to verify that the input is in the correct case
    pub fn upper_case(input: &'a str) -> Word<'a> {
        let word = Word(WordInner::UpperCase(input));
        debug_assert!(word.is_valid());
        word
    }

    /// It is up to the caller to verify that the input is in the correct case
    pub fn mixed_case(input: &'a str) -> Word<'a> {
        let word = Word(WordInner::MixedCase(input));
        debug_assert!(word.is_valid());
        word
    }

    /// It is up to the caller to verify that the input is in the correct case
    pub fn capitalized(input: &'a str) -> Word<'a> {
        let word = Word(WordInner::Capitalized(input));
        debug_assert!(word.is_valid());
        word
    }

    pub fn write_lowercase<W: fmt::Write>(&self, f: &mut W) -> Result<(), fmt::Error> {
        debug_assert!(self.is_valid());
        match &self.0 {
            WordInner::LowerCase(word) => f.write_str(word),
            WordInner::MixedCase(word) | WordInner::UpperCase(word) => write!(f, "{}", word.to_lowercase()),
            WordInner::Capitalized(word) => {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    write!(f, "{}", first.to_lowercase())?;
                    let rest = unsafe { word.get_unchecked(first.len_utf8()..) };
                    f.write_str(&rest)?;
                }
                Ok(())
            }
        }
    }

    pub fn write_capitalized<W: fmt::Write>(&self, f: &mut W) -> Result<(), fmt::Error> {
        debug_assert!(self.is_valid());
        match &self.0 {
            WordInner::LowerCase(word) => {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    write!(f, "{}", first.to_uppercase())?;
                    let rest = unsafe { word.get_unchecked(first.len_utf8()..) };
                    f.write_str(&rest)?;
                }
                Ok(())
            }
            WordInner::MixedCase(word) | WordInner::UpperCase(word) => {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    write!(f, "{}", first.to_uppercase())?;
                    let rest = unsafe { word.get_unchecked(first.len_utf8()..) };
                    f.write_str(&rest.to_lowercase())?;
                }
                Ok(())
            }
            WordInner::Capitalized(word) => f.write_str(word),
        }
    }

    /// It is assumed that a Word is constructed with a string already in the correct case.
    /// However, this method is useful in debugging.
    pub fn is_valid(&self) -> bool {
        match &self.0 {
            WordInner::LowerCase(word) => word.chars().all(|c| !c.is_uppercase()),
            WordInner::MixedCase(_word) => true,
            WordInner::UpperCase(word) => word.chars().all(|c| !c.is_lowercase()),
            WordInner::Capitalized(word) => {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    if first.is_lowercase() {
                        false
                    } else {
                        chars.all(|c| !c.is_uppercase())
                    }
                } else {
                    true
                }
            }
        }
    }
}

pub trait Case<'a>: Sized {
    fn from_cased_words(words: Vec<Word<'a>>) -> Self;

    fn to_cased_words(self) -> Vec<Word<'a>>;
}

#[inline]
pub(crate) fn write_lower_delimited<'w, W, I>(
    words: I,
    buf: &mut W,
    sep: char,
) -> Result<(), fmt::Error>
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

#[inline]
pub(crate) fn write_pascal_case<'w, W, I>(words: I, buf: &mut W) -> Result<(), fmt::Error>
where
    W: fmt::Write,
    I: IntoIterator<Item = &'w Word<'w>>,
{
    for word in words {
        word.write_capitalized(buf)?;
    }
    Ok(())
}

struct UpperCaseSplitIter<'a, C: Iterator> {
    source: &'a str,
    word_start: usize,
    chars: Peekable<C>,
}

impl<'a, C: Iterator<Item = (usize, char)>> Iterator for UpperCaseSplitIter<'a, C> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        while self.chars.next().is_some() {
            match self.chars.peek() {
                None => {
                    let word = unsafe { self.source.get_unchecked(self.word_start..) };
                    return Some(word);
                }
                Some(&(n, c)) if c.is_uppercase() => {
                    let word = unsafe { self.source.get_unchecked(self.word_start..n) };
                    self.word_start = n;
                    return Some(word);
                }
                _ => {},
            }
        }
        None
    }
}

#[inline]
pub(crate) fn split_words_on_uppercase(source: &str) -> impl Iterator<Item = &str> {
    UpperCaseSplitIter {
        source,
        chars: source.char_indices().peekable(),
        word_start: 0,
    }
}

pub(crate) fn is_lower_case_delimited(source: &str, delim: char) -> bool {
    let mut delim_allowed = false;
    for ch in source.chars() {
        if ch == delim {
            if delim_allowed {
                delim_allowed = false;
            } else {
                return false;   
            }
        // Note: is_uppercase() is not equivalient to !is_lowercase(), which would return false for
        // writing systems that do not have a notion of case (e.g. Kanji)
        } else if !ch.is_alphanumeric() || ch.is_uppercase() {
            return false;
        } else {
            delim_allowed = true;
        }
    }

    if let Some(ch) = source.chars().next_back() {
        ch != delim
    } else {
        true
    }
}

struct CompareBuf<'a> {
    expected: &'a str,
    position: usize,
}

impl<'a> std::fmt::Write for CompareBuf<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        if &self.expected.as_bytes()[self.position..s.len()] == s.as_bytes() {
            self.position += s.len();
            Ok(())
        } else {
            Err(std::fmt::Error)
        }
    }
}

/// Check if the `Display` formatting for a value matches the expected value, without an additional allocation
pub(crate) fn display_eq<D: fmt::Display>(expected: &str, display: D) -> bool {
    use std::fmt::Write;
    let mut buf = CompareBuf {
        expected,
        position: 0,
    };
    write!(buf, "{}", display).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_words_on_uppercase_one_word_lower() {
        let words: Vec<_> = split_words_on_uppercase("abc").collect();
        assert_eq!(vec!["abc"], words);
    }

    #[test]
    fn test_split_words_on_uppercase_two_words_lower() {
        let words: Vec<_> = split_words_on_uppercase("abcDef").collect();
        assert_eq!(vec!["abc", "Def"], words);
    }
    #[test]
    fn test_split_words_on_uppercase_two_words_upper() {
        let words: Vec<_> = split_words_on_uppercase("AbcDef").collect();
        assert_eq!(vec!["Abc", "Def"], words);
    }
    #[test]
    fn test_split_words_on_uppercase_consecutive_uppers() {
        let words: Vec<_> = split_words_on_uppercase("ABCdef").collect();
        assert_eq!(vec!["A", "B", "Cdef"], words);
    }

    #[test]
    fn test_split_words_on_uppercase_single_letter_lower() {
        let words: Vec<_> = split_words_on_uppercase("a").collect();
        assert_eq!(vec!["a"], words);
    }

    #[test]
    fn test_split_words_on_uppercase_single_letter_upper() {
        let words: Vec<_> = split_words_on_uppercase("A").collect();
        assert_eq!(vec!["A"], words);
    }

    #[test]
    fn test_split_words_on_uppercase_kanji() {
        let words: Vec<_> = split_words_on_uppercase("こんにちは").collect();
        assert_eq!(vec!["こんにちは"], words);
    }

    #[test]
    fn test_split_words_on_uppercase_kanji_mixed_upper() {
        let words: Vec<_> = split_words_on_uppercase("こAんAにAちAはA").collect();
        assert_eq!(vec!["こ","Aん","Aに","Aち","Aは", "A"], words);
    }

    #[test]
    fn test_is_lower_case_delimited_lower_one_word() {
        assert!(is_lower_case_delimited("hello", '+'));
    }

    #[test]
    fn test_is_lower_case_delimited_lower_matching_delimiter() {
        assert!(is_lower_case_delimited("hello+bye", '+'));
        assert!(is_lower_case_delimited("hello_bye", '_'));
    }

    #[test]
    fn test_is_lower_case_delimited_lower_wrong_delimiter() {
        assert!(!is_lower_case_delimited("hello+bye", '-'));
    }

    #[test]
    fn test_is_lower_case_delimited_lower_trailing_delimiter() {
        assert!(!is_lower_case_delimited("hello+", '+'));
    }

    #[test]
    fn test_is_lower_case_delimited_lower_leading_delimiter() {
        assert!(!is_lower_case_delimited("+hello", '+'));
    }

    #[test]
    fn test_is_lower_case_delimited_upper() {
        assert!(!is_lower_case_delimited("HELLO", '-'));
        assert!(!is_lower_case_delimited("HELLO-BYE", '-'));
    }

    #[test]
    fn test_is_lower_case_delimited_repeated_delimiter() {
        assert!(!is_lower_case_delimited("HELLO__THERE", '_'));
    }

    fn upper_case() -> Word<'static> { Word::upper_case("HELLO") }
    fn lower_case() -> Word<'static> { Word::lower_case("hello") }
    fn capitalized() -> Word<'static> { Word::capitalized("Hello") }
    fn mixed_case1() -> Word<'static> { Word::mixed_case("hEllO") }
    fn mixed_case2() -> Word<'static> { Word::mixed_case("HeLLo") }

    #[test]
    fn test_word_write_capitalized() {
        for word in &[upper_case(), lower_case(), mixed_case1(), mixed_case2(), capitalized()] {
            let mut output = String::new();
            assert!(word.write_capitalized(&mut output).is_ok());
            assert_eq!("Hello", &output);
        }
    }

    #[test]
    fn test_word_write_lower_case() {
        for word in &[upper_case(), lower_case(), mixed_case1(), mixed_case2(), capitalized()] {
            let mut output = String::new();
            assert!(word.write_lowercase(&mut output).is_ok());
            assert_eq!("hello", &output);
        }
    }

    #[test]
    fn test_write_lower_delimited() {
        let words = vec![upper_case(), lower_case(), mixed_case1(), mixed_case2(), capitalized()];
        let mut output = String::new();
        assert!(write_lower_delimited(words.iter(), &mut output, '_').is_ok());
        assert_eq!("hello_hello_hello_hello_hello", &output);
    }

    #[test]
    fn test_write_pascal_case() {
        let words = vec![upper_case(), lower_case(), mixed_case1(), mixed_case2(), capitalized()];
        let mut output = String::new();
        assert!(write_pascal_case(words.iter(), &mut output).is_ok());
        assert_eq!("HelloHelloHelloHelloHello", &output);
    }

    #[test]
    fn test_display_eq() {
        assert!(display_eq("1", 1));
        assert!(display_eq("hi", "hi"));
        assert!(!display_eq("0", 1));
        assert!(!display_eq("HI", "hi"));
    }
}
