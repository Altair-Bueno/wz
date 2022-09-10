use wz_core::Counter;
/// A data structure that acts like a linked list
///
/// Sheath's main purpose is allowing **cheap dynamic dispatch** for counters.
/// Each linked trait object should be allocated on the same memory pool to
/// maximize cache hits.
///
///
/// ```text
/// Sheath<ByteCounter> -> Sheath<LineCounter> -> ()
/// ```
pub struct Sheath<'bump, C, S> {
    counter: C,
    next: &'bump mut dyn Counter<S>,
}

impl<'bump, C, S> Sheath<'bump, C, S> {
    /// Creates a new Sheath object with a reference to the next counter
    pub fn new(counter: C, next: &'bump mut dyn Counter<S>) -> Self {
        Self { counter, next }
    }
}
impl<'bump, C, S> Counter<S> for Sheath<'bump, C, S>
where
    C: Counter<S>,
{
    fn count(&mut self, input: &[u8]) {
        self.counter.count(input);
        self.next.count(input);
    }

    fn output(&self, stats: &mut S) {
        self.counter.output(stats);
        self.next.output(stats);
    }
}
