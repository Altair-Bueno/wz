#![no_std]

use wz_core::*;
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

#[derive(Clone, Debug)]
pub struct Lines {
    n: usize,
    line_break: u8,
}

impl Lines {
    pub fn with_linebreak(line_break: u8) -> Self {
        Self { n: 0, line_break }
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
