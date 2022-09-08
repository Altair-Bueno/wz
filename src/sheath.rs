use wz_core::Counter;
// Works as a Linked list, but allocs on the same bump
pub struct Sheath<'bump, C, S> {
    counter: C,
    next: &'bump mut dyn Counter<S>,
}

impl<'bump, C, S> Sheath<'bump, C, S> {
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
