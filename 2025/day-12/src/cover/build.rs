/// Specification for a column during construction of a DLX Matrix.
///
/// Each column/criterion has a "cover count" associated with it. This count determines how many times rows containing
/// the column may be selected as part of a solution. Once a column has been covered by `n` columns, it and all rows
/// which contain it are removed from consideration for the rest of the search.
///
/// - A required column with count `n` **must** be covered exactly `n` times before its associated cover problem is
///   considered solved.
/// - An optional column with count `n` may be covered **at most** `n` times.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Constraint {
    Required(usize),
    Optional(usize),
}

#[allow(dead_code)]
impl Constraint {
    pub const fn count(&self) -> usize {
        let (Constraint::Required(count) | Constraint::Optional(count)) = *self;
        count
    }

    pub const fn is_optional(&self) -> bool {
        match self {
            Constraint::Required(_) => false,
            Constraint::Optional(_) => true,
        }
    }

    pub const fn is_required(&self) -> bool {
        match self {
            Constraint::Required(_) => true,
            Constraint::Optional(_) => false,
        }
    }

    pub const fn with_count(self, cover_count: usize) -> Self {
        match self {
            Constraint::Required(_) => Constraint::Required(cover_count),
            Constraint::Optional(_) => Constraint::Optional(cover_count),
        }
    }

    pub const fn to_optional(self) -> Self {
        Constraint::Optional(self.count())
    }

    pub const fn to_required(self) -> Self {
        Constraint::Required(self.count())
    }

    /// Returns an iterator that repeats this criterion specification multiple times.
    pub fn repeat(self, n: usize) -> std::iter::RepeatN<Self> {
        std::iter::repeat_n(self, n)
    }
}

impl Default for Constraint {
    fn default() -> Self {
        Self::Required(1)
    }
}
