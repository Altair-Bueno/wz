use wz_core::Counter;
use wz_fmt::Stats;

use crate::sheath::Sheath;

#[derive(Debug, Clone)]
pub struct Options {
    pub lines: bool,
    pub characters: bool,
    pub words: bool,
    pub bytes: bool,
    pub newline: u8,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            characters: Default::default(),
            words: Default::default(),
            bytes: Default::default(),
            newline: b'\n',
        }
    }
}

pub trait Builder {
    #[allow(clippy::mut_from_ref)]
    fn build<'bump>(&self, bump: &'bump bumpalo::Bump) -> &'bump mut dyn Counter<Stats>;
}

#[derive(Debug, Clone, Default)]
pub struct BuilderUtf8(Options);

impl From<Options> for BuilderUtf8 {
    fn from(options: Options) -> Self {
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

        counter
    }
}
/*
#[derive(Debug, Clone, Default)]
pub struct BuilderUtf16(Options);

impl From<Options> for BuilderUtf16 {
    fn from(options: Options) -> Self {
        Self(options)
    }
}

impl Builder for BuilderUtf16 {
    fn build<'bump>(&self, bump: &'bump bumpalo::Bump) -> &'bump mut dyn Counter<Stats> {
        let Self(options) = self;

        let mut counter = bump.alloc(()) as _;

        if options.bytes {
            let sheath = Sheath::new(wz_utf8::Bytes::default(), counter);
            counter = bump.alloc(sheath) as _;
        }
        // if options.lines {
        //     let sheath = Sheath::new(wz_utf8::Lines::with_linebreak(options.newline), counter);
        //     counter = bump.alloc(sheath) as _;
        // }
        // if options.words {
        //     let sheath = Sheath::new(wz_utf8::Words::default(), counter);
        //     counter = bump.alloc(sheath) as _;
        // }
        // if options.characters {
        //     let sheath = Sheath::new(wz_utf8::Chars::default(), counter);
        //     counter = bump.alloc(sheath) as _;
        // }

        counter
    }
}
 */
