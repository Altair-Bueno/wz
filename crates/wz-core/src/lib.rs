//! Core traits for [wz]
//!
//! [wz]: https://crates.io/crates/wz

#![no_std]

/// Abstraction for [wz]'s stateful counters
///
/// This trait's identity is the unit (`()`) type
///
/// [wz]: https://crates.io/crates/wz
pub trait Counter<T> {
    fn count(&mut self, input: &[u8]);
    fn output(&self, collector: &mut T);
}

// Identity
impl<T> Counter<T> for () {
    fn count(&mut self, _: &[u8]) {}
    fn output(&self, _: &mut T) {}
}

/// Generates a collector trait with a given name
///
/// ```
/// // gen_collector_trait!(FooCollector);
/// // generates the following trait
/// pub trait FooCollector {
///    fn collect(&mut self, count: usize);
/// }
/// ```
#[macro_export]
macro_rules! gen_collector_trait {
    ( $($name:tt), * ) => {
        $(
            pub trait $name {
                fn collect(&mut self, count: usize);
            }
        )*
    };
}

/// Implements a collector trait for usize
///
/// ```
/// pub trait FooCollector {
///    fn collect(&mut self, count: usize);
/// }
/// //  impl_collector_usize(FooCollector);
/// // generates the following impl block
/// impl FooCollector for usize {
///     fn collect(&mut self, count: usize) {
///         *self = count;
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_collector_usize {
    ( $($name:ty), *) => {
        $(
            impl $name for usize {
                fn collect(&mut self, count: usize) {
                    *self = count;
                }
            }
        )*
    };
}

gen_collector_trait!(
    BytesCollector,
    CharsCollector,
    LinesCollector,
    WordsCollector,
    MaxLineLengthCollector
);

impl_collector_usize!(
    BytesCollector,
    CharsCollector,
    LinesCollector,
    WordsCollector,
    MaxLineLengthCollector
);
