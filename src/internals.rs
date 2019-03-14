use std::fmt;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Word<'a> {
    LowerCase(&'a str),
    MixedCase(&'a str),
    Capitalized(&'a str),
    UpperCase(&'a str),
}

#[derive(Debug, Clone)]
pub(crate) enum CaseValue<'a> {
    Words(Vec<Word<'a>>),
    Joined(&'a str),
}

impl<'a> Word<'a> {
    pub fn write_lowercase<W: fmt::Write>(&self, f: &mut W) -> Result<(), fmt::Error> {
        match &self {
            Word::LowerCase(word) => f.write_str(word),
            Word::MixedCase(word) | Word::UpperCase(word) => write!(f, "{}", word.to_lowercase()),
            Word::Capitalized(word) => {
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
        match &self {
            Word::LowerCase(word) => {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    write!(f, "{}", first.to_uppercase())?;
                    let rest = unsafe { word.get_unchecked(first.len_utf8()..) };
                    f.write_str(&rest)?;
                }
                Ok(())
            }
            Word::MixedCase(word) | Word::UpperCase(word) => {
                let mut chars = word.chars();
                if let Some(first) = chars.next() {
                    write!(f, "{}", first.to_uppercase())?;
                    let rest = unsafe { word.get_unchecked(first.len_utf8()..) };
                    f.write_str(&rest.to_lowercase())?;
                }
                Ok(())
            }
            Word::Capitalized(word) => f.write_str(word),
        }
    }
}

pub(crate) trait Case<'a>: Sized {
    fn from_words(words: Vec<Word<'a>>) -> Self;

    fn to_words(self) -> Vec<Word<'a>>;

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
                let word = unsafe { self.source.get_unchecked(self.word_start..i) };
                self.word_start = i;
                return Some(word);
            }
        }
        if self.word_start < self.source.len() - 1 {
            let word = unsafe { self.source.get_unchecked(self.word_start..) };
            self.word_start = self.source.len();
            Some(word)
        } else {
            None
        }
    }
}

#[inline]
pub(crate) fn split_words_on_uppercase(source: &str) -> impl Iterator<Item = &str> {
    UpperCaseSplitIter {
        source,
        chars: source.char_indices().skip(1),
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
    true
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

pub(crate) fn display_eq<D: fmt::Display>(expected: &str, display: D) -> bool {
    use std::fmt::Write;
    let mut buf = CompareBuf {
        expected,
        position: 0,
    };
    write!(buf, "{}", display).is_ok()
}
