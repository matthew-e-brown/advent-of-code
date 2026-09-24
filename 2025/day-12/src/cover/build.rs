use super::raw::build as raw;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Constraint<C> {
    pub label: C,
    pub count: usize,
    pub optional: bool,
}

impl<C> Constraint<C> {
    /// Creates a [`raw`] version of this constraint.
    pub fn raw(self) -> raw::Constraint {
        raw::Constraint {
            count: self.count,
            optional: self.optional,
        }
    }
}

#[allow(dead_code)]
impl<C> Constraint<C> {
    pub const fn required(label: C) -> Self {
        Constraint { label, count: 1, optional: false }
    }

    pub const fn optional(label: C) -> Self {
        Constraint { label, count: 1, optional: true }
    }

    pub const fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    pub const fn to_required(mut self) -> Self {
        self.optional = false;
        self
    }

    pub const fn to_optional(mut self) -> Self {
        self.optional = true;
        self
    }
}

impl<C: Clone> Constraint<C> {
    /// Returns an iterator that repeats this criterion specification multiple times.
    pub fn repeat(self, n: usize) -> std::iter::RepeatN<Self> {
        std::iter::repeat_n(self, n)
    }
}

impl<C: Default> Default for Constraint<C> {
    fn default() -> Self {
        Self::required(C::default()).with_count(1)
    }
}
