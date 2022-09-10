use wz_core::Counter;
use wz_fmt::Stats;

use crate::sheath::Sheath;

/// A filter used by builders to construct dynamic counters (aka [`Sheath`])
///
/// [`Sheath`]: crate::sheath::Sheath
#[derive(Debug, Clone)]
pub struct Filter {
    pub lines: bool,
    pub characters: bool,
    pub words: bool,
    pub bytes: bool,
    pub newline: u8,
    pub max_line_length: bool,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            characters: Default::default(),
            words: Default::default(),
            bytes: Default::default(),
            newline: b'\n',
            max_line_length: Default::default(),
        }
    }
}

/// Defines the common interface for a [`Sheath`] builder
///
/// [`Sheath`]: crate::sheath::Sheath
pub trait Builder {
    #[allow(clippy::mut_from_ref)]
    fn build<'bump>(&self, bump: &'bump bumpalo::Bump) -> &'bump mut dyn Counter<Stats>;
}

/// A builder capable of creating UTF8 counters
#[derive(Debug, Clone, Default)]
pub struct BuilderUtf8(Filter);

impl From<Filter> for BuilderUtf8 {
    fn from(options: Filter) -> Self {
        Self(options)
    }
}

impl Builder for BuilderUtf8 {
    fn build<'bump>(&self, bump: &'bump bumpalo::Bump) -> &'bump mut dyn Counter<Stats> {
        let Self(options) = self;

        let mut counter = bump.alloc(()) as _;

        if options.bytes {
            let sheath = Sheath::new(wz_utf8::Bytes::default(), counter);
            counter = bump.alloc(sheath) as _;
        }
        if options.lines {
            let sheath = Sheath::new(wz_utf8::Lines::with_linebreak(options.newline), counter);
            counter = bump.alloc(sheath) as _;
        }
        if options.words {
            let sheath = Sheath::new(wz_utf8::Words::default(), counter);
            counter = bump.alloc(sheath) as _;
        }
        if options.characters {
            let sheath = Sheath::new(wz_utf8::Chars::default(), counter);
            counter = bump.alloc(sheath) as _;
        }

        if options.max_line_length {
            let sheath = Sheath::new(
                wz_utf8::MaxLineLength::with_linebreak(options.newline),
                counter,
            );
            counter = bump.alloc(sheath) as _;
        }
        counter
    }
}
