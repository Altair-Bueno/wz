//! UTF-8 counters for [wz]
//!
//! [wz]: https://crates.io/crates/wz
#![no_std]
use wz_core::*;

/// Byte counter for UTF-8 encoded byte slices
///
/// ```
/// use wz_core::Counter;
/// use wz_utf8::Bytes;
///
/// let counter = Bytes::default();
/// ```
#[derive(Clone, Debug, Default)]
pub struct Bytes {
    n: usize,
}

impl<T> Counter<T> for Bytes
where
    T: BytesCollector,
{
    fn count(&mut self, bytes: &[u8]) {
        self.n += bytes.len();
    }

    fn output(&self, collector: &mut T) {
        collector.collect(self.n)
    }
}

/// Character counter for UTF-8 encoded byte slices
///
/// ```
/// use wz_core::Counter;
/// use wz_utf8::Chars;
///
/// let counter = Chars::default();
/// ```
#[derive(Clone, Debug, Default)]
pub struct Chars {
    n: usize,
}

impl<T> Counter<T> for Chars
where
    T: CharsCollector,
{
    fn count(&mut self, bytes: &[u8]) {
        self.n += bytecount::num_chars(bytes)
    }

    fn output(&self, count: &mut T) {
        count.collect(self.n);
    }
}

/// Line counter for UTF-8 encoded byte slices
///
/// ```
/// use wz_core::Counter;
/// use wz_utf8::Lines;
///
/// let counter = Lines::with_linebreak(b'\n');
/// ```
#[derive(Clone, Debug)]
pub struct Lines {
    n: usize,
    line_break: u8,
}

impl Lines {
    /// Creates a new Lines counter that counts `line_break` bytes
    pub fn with_linebreak(line_break: u8) -> Self {
        Self { n: 0, line_break }
    }
    /// Creates a new Lines counter that counts `\n` bytes
    pub fn line_feed() -> Self {
        Self::with_linebreak(b'\n')
    }
    /// Creates a new Lines counter that counts `\r` bytes
    pub fn carriage_return() -> Self {
        Self::with_linebreak(b'\r')
    }
}

impl<T> Counter<T> for Lines
where
    T: LinesCollector,
{
    fn count(&mut self, bytes: &[u8]) {
        self.n += bytecount::count(bytes, self.line_break)
    }

    fn output(&self, collector: &mut T) {
        collector.collect(self.n);
    }
}
/// Word counter for UTF-8 encoded byte slices
///
/// A word boundary is defined in `isspace(3)`
///
/// ```
/// use wz_core::Counter;
/// use wz_utf8::Words;
///
/// let counter = Words::default();
/// ```
#[derive(Clone, Debug, Default)]
pub struct Words {
    n: usize,
    on_word: bool,
}

impl<T> Counter<T> for Words
where
    T: WordsCollector,
{
    fn count(&mut self, bytes: &[u8]) {
        *self = bytes.iter().fold(self.clone(), |acc, next| {
            // matches!(x, 0x20 | 0x09 | 0x0A..=0x0D) == ISSPACE
            let on_word = !matches!(next, 0x20 | 0x09 | 0x0A..=0x0D);
            let n = acc.n + usize::from(acc.on_word && !on_word);
            Self { n, on_word }
        });
    }

    fn output(&self, counter: &mut T) {
        counter.collect(self.n + usize::from(self.on_word));
    }
}

/// Max line length counter for UTF-8 encoded byte slices
///
/// ```
/// use wz_core::Counter;
/// use wz_utf8::MaxLineLength;
///
/// let counter = MaxLineLength::with_linebreak(b'\n');
/// ```
#[derive(Clone, Debug)]
pub struct MaxLineLength {
    max: usize,
    character_counter: Chars,
    line_break: u8,
}

impl MaxLineLength {
    /// Creates a new MaxLineLength counter that looks for line_break bytes
    pub fn with_linebreak(line_break: u8) -> Self {
        Self {
            max: 0,
            line_break,
            character_counter: Default::default(),
        }
    }
    /// Creates a new MaxLineLength counter that looks  for '\n'
    pub fn line_feed() -> Self {
        Self::with_linebreak(b'\n')
    }
    /// Creates a new MaxLineLength counter that looks for '\r'
    pub fn carriage_return() -> Self {
        Self::with_linebreak(b'\r')
    }
}

impl<T> Counter<T> for MaxLineLength
where
    T: MaxLineLengthCollector,
{
    fn count(&mut self, input: &[u8]) {
        let mut index = 0;

        while let Some(offset_index) = memchr::memchr(self.line_break, &input[index..]) {
            Counter::<usize>::count(
                &mut self.character_counter,
                &input[index..offset_index + index],
            );
            let mut chars = 0;
            self.character_counter.output(&mut chars);
            index += offset_index + 1;
            self.max = core::cmp::max(self.max, chars);
            self.character_counter = Default::default();
        }
        Counter::<usize>::count(&mut self.character_counter, &input[index..]);
    }

    fn output(&self, collector: &mut T) {
        let mut chars = 0;
        self.character_counter.output(&mut chars);
        let count = core::cmp::max(self.max, chars);
        collector.collect(count)
    }
}
