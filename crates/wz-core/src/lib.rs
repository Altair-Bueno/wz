#![no_std]
pub trait Counter<T> {
    fn count(&mut self, input: &[u8]);
    fn output(&self, collector: &mut T);
}

// Unit type
impl<T> Counter<T> for () {
    fn count(&mut self, _: &[u8]) {}

    fn output(&self, _: &mut T) {}
}

pub trait BytesCollector {
    fn collect(&mut self, count: usize);
}
pub trait CharsCollector {
    fn collect(&mut self, count: usize);
}
pub trait LinesCollector {
    fn collect(&mut self, count: usize);
}

pub trait WordsCollector {
    fn collect(&mut self, count: usize);
}

// impl BytesCollector for usize {
//     fn collect(&mut self, count: usize) {
//         *self = count;
//     }
// }

macro_rules! impl_collector_usize {
    ( $($x:ty), *) => {
        $(
            impl $x for usize {
                fn collect(&mut self, count: usize) {
                    *self = count;
                }
            }
        )*
    };
}

impl_collector_usize!(
    BytesCollector,
    CharsCollector,
    LinesCollector,
    WordsCollector
);
