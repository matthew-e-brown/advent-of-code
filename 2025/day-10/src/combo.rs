//! Iterators for generating **combinations with replacement**
//!
//! This module provides the [`Combinations`] iterator, which yields all possible size-`n` combinations of a given set
//! of options, with replacement.
//!
//! A _combination_ is a permutation where order does not matter: `ABC` is the same as `ACB`. Furthermore, a
//! _combination with replacement_ is one where each possible option may be repeated. For example, given the options
//! A, B, and C, and asked to for combinations of size 6, you might see `AAAAAA`, `AABBCC`, or `ABBCCC`.
//!
//! # `Iterator` implementation
//!
//! Due to a limitation, the [`Combinations`] struct does **not** actually implement the [`Iterator`] trait. See its
//! docs for more details.
//!
//! # Derivation
//!
//! > This section was originally a plain-text comment at the top of this file where I derived the logic for the
//! > counting pattern for generating these combinations. I knew it must be possible without tons of extra allocation,
//! > since I was capable of generating the sequence in my head just by counting. I just had to figure it out.
//! >
//! > I've left the comment below written basically exactly as I wrote it into the file the first time, just with
//! > updated formatting (mostly to make it shorter and to play nicer with being part of a doc-comment).
//!
//! Let's work through a few examples first:
//!
//! ```text
//! ┌───────────────────────────────────────┬───────────────────┬───────────────────────────┐
//! │ 012, length 3                         │ 012, Length 4     │ 012, for some length r    │
//! ├───────────────────────────────────────┼───────────────────┼───────────────────────────┤
//! │ - 000, 3 zero, 0 one, 0 two (300)     │ - 0000 (400)      │ - (r,0,0)                 │
//! │ - 001, 2 zero, 1 one, 0 two (210)     │ - 0001 (310)      │ - (r-1,1,0)               │
//! │ - 002, 2 zero, 0 one, 1 two (201)     │ - 0002 (301)      │ - (r-1,0,1)               │
//! │ - 011, 1 zero, 2 one, 0 two (120)     │ - 0011 (220)      │ - (r-2,2,0)               │
//! │ - 012, 1 zero, 1 one, 1 two (111)     │ - 0012 (211)      │ - (r-2,1,1)               │
//! │ - 022, 1 zero, 0 one, 2 two (102)     │ - 0022 (202)      │ - (r-2,0,2)               │
//! │ - 111, 0 zero, 3 one, 0 two (030)     │ - 0111 (130)      │ - ...                     │
//! │ - 112, 0 zero, 2 one, 1 two (021)     │ - 0112 (121)      │ - (r-i,i,0)               │
//! │ - 122, 0 zero, 1 one, 2 two (012)     │ - 0122 (112)      │ - (r-i,i-1,1)             │
//! │ - 222, 0 zero, 0 one, 3 two (003)     │ - 0222 (103)      │ - (r-i,i-2,2)             │
//! │                                       │ - 1111 (040)      │ - (r-i,i-3,3)             │
//! │                                       │ - 1112 (031)      │ - ...                     │
//! │                                       │ - 1122 (022)      │ - (r-i,i-i,i)             │
//! │                                       │ - 1222 (013)      │                           │
//! │                                       │ - 2222 (004)      │                           │
//! ├───────────────────────────────────────┴───────────────────┴───────────────────────────┤
//! │ 0123, length 3. Try and find the pattern: we exhaust all combos with n of the first   │
//! │ one, then with n-1 of the first, and so on.                                           │
//! ├───────────────────────────────────────────────────────────────────────────────────────┤
//! │ - 000 (3000) with 3 zeroes                                                            │
//! │ - 001 (2100) with 2 zeroes...                                                         │
//! │ - 002 (2010)                                                                          │
//! │ - 003 (2001)                                                                          │
//! │ - 011 (1200) with 1 zeroes...                                                         │
//! │ - 012 (1110)                                                                          │
//! │ - 013 (1101)                                                                          │
//! │ - 022 (1020)                                                                          │
//! │ - 023 (1011)                                                                          │
//! │ - 033 (1002)                                                                          │
//! │ - 111 (0300) with 0 zeroes..., with 3 ones                                            │
//! │ - 112 (0210)     with 2 ones, with 1 twos                                             │
//! │ - 113 (0201)         with 0 twos                                                      │
//! │ - 122 (0120)     with 1 ones, with 2 twos                                             │
//! │ - 123 (0111)         with 1 twos                                                      │
//! │ - 133 (0102)         with 0 twos                                                      │
//! │ - 222 (0030)     with 0 ones, with 3 twos                                             │
//! │ - 223 (0021)         with 2 twos                                                      │
//! │ - 233 (0012)         with 1 twos                                                      │
//! │ - 333 (0003)     with 0 twos                                                          │
//! ├───────────────────────────────────────────────────────────────────────────────────────┤
//! │ ABCD, length 59                                                                       │
//! ├───────────────────────────────────────────────────────────────────────────────────────┤
//! │ - (59 A, 0 B, 0 C, 0 D)                                                               │
//! │ - (58 A, 1 B, 0 C, 0 D)                                                               │
//! │ - (58 A, 0 B, 1 C, 0 D)                                                               │
//! │ - (58 A, 0 B, 0 C, 1 D)                                                               │
//! │ - (57 A, 2 B, 0 C, 0 D)                                                               │
//! │ - (57 A, 1 B, 1 C, 0 D)                                                               │
//! │ - (57 A, 1 B, 0 C, 1 D)                                                               │
//! │ - ...                                                                                 │
//! │ -                                                                                     │
//! └───────────────────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! This process is recursive:
//!
//! - return n of the first option.
//! - return return n-1 of the first option, followed by all the possible ways to pick 1 from the remaining 3 options.
//! - once all the n-1-of-the-first-option combos are done, step down to n-2. Follow n-2 A's with all possible ways to pick
//!   2 from the next 3 options. And so on.
//!
//! ```no_run
//! IterA {
//!     n: 59, // Total amount needed to be yielded by itself and its children.
//!     k: 57, // How many this particular iterator will yield.
//!     IterB {
//!         n: 2, // This one only needs to iterate 2 amongst itself and all its children: (59 - 57, parent.n - parent.k).
//!         k: 1, // This particular one will return 1 on this stage.
//!         IterC {
//!             n: 1, // (2 - 1) total.
//!             k: 1, // This one will yield 1.
//!             IterD {
//!                 n: 0, // This one has nothing to yield.
//!                 k: 0,
//!                 None
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! The example above would be the state for 57×A, 1×B, 1×C, 0×D. In this structure, each child iterator would get
//! replaced once it was exhausted, unless the parent is also exhausted. For example, after IterC yields its C, k is
//! decremented to zero. The next time we need a combo, it would replace D with a fresh iterator whose n value is
//! (n-k)=(1-0). Then, IterC with an n of 1 is only exhausted once its last child is exhausted.
//!
//! We could store these iterators in-line, instead of recursively:
//!
//! ```no_run
//! [
//!     Iter { n: 59, k: 25 }, // We will yield 25 A's; the rest are handled by children.
//!     Iter { n: 34, k: 18 }, // This and all children need to yield 34 items. This one will do 18.
//!     Iter { n: 16, k:  0 }, // We've already gone through all combos with 16 Cs, 15 Cs, 14 Cs..., so it's all up to:
//!     Iter { n: 16, k: 16 }, // This one, which will yield the final 16 elements.
//! ]
//! ```
//!
//! The question now is... how does D know that it's done, and doesn't need to decrement its k to 15? The answer:
//! because it's the last one in the chain. Once `IterD` yields 16... the next time it's polled, it needs to return
//! None. Let's consider the top case of the recursion instead of the bottom, for a second.
//!
//! At the very start, we'd have:
//!
//! ```no_run
//! [
//!     IterA { n: 59, k: 59 },
//!     None,
//!     None,
//!     None,
//! ]
//! ```
//!
//! When we first poll the iterator, we would expect to be given (59, 0, 0, 0). Then, the next time we poll, `IterA`
//! ticks down to `IterA { n: 59, k: 58 }`. Since B is None, it gets created with `IterB { n: 1, k: 1 }`. We poll it
//! immediately, and so it yields 1. Next poll, `IterA` does not decrement k, since `IterB` is not yet finished. `IterB`
//! however does decrement k. It yields zero, and then defers to its children to return (1, 0).
//!
//! So, working through this, we actually *don't* want B,C,D to start as None. Really, `IterA` needs all 3 children
//! there to produce (0, 0, 0) one time. Maybe the recursive approach would be easier to implement...
//!
//! ```no_run
//! [
//!     IterA { n: 59, k: 59 },
//!     IterB { n:  0, k:  0 },
//!     IterC { n:  0, k:  0 },
//!     IterD { n:  0, k:  0 },
//! ]
//! ```
//!
//! 1.  Poll: A yields 59, then defers to (B, C, D). B yields 0, then defers to (C, D). In total, we get (59,0,0,0).
//! 2.  Advance:
//!     - We work from the end. D wants to decrement k, but it's already zero. That means D is exhausted, so its
//!       parent should decrement instead.
//!     - C tries to decrement, but it is also already exhausted. So it tells B to decrement.
//!     - Finally, since B is also already exhausted, it tells A to decrement.
//!     - When A decrements k to 58, it needs to re-generate all its children with fresh values.
//!       - A creates `IterB { n: 1, k: 1 }`.
//!       - The new B creates `IterC { n: 0, k: 0 }`.
//!       - The new C creates `IterD { n: 0, k: 0 }`.
//!
//! 2.  Poll: A yields k=58, then defers to B. B yields k=1, then defers to C. Both C and D yield 0.
//! 3.  Advance:
//!     - D is told to decrement, but it is exhausted. So it tells C to decrement.
//!     - C decrements, but it is also exhausted. It tells B to decrement.
//!     - B decrements k from 1→0: `IterB { n: 1, k: 0 }`. When it does so, needs to regenerate all children:
//!       - B creates a new `IterC { n: (B.n-B.k=1), k: (C.n=1) }`.
//!       - C creates a new `IterD { n: (C.n-C.k=0), k: (D.n=0) }`.
//!
//! 4.  Poll: A yields k=58 again, then defers to B. B yields k=0, then defers to C. C yields k=1, then defers to D: 0.
//! 5.  Advance:
//!     - D decrements, but is exhausted, so tells C to decrement.
//!     - C decrements k to 0: `IterC { n: 1, k: 0 }`, then creates new children:
//!       - C creates a new `IterD { n: (C.n-C.k=1), k: 1 }`.
//!
//! And so on. This is the core logic we need to make the combinations iterator.
//!
//! Two things to note:
//!
//! 1.  We don't actually need to store `n` at each level. Instead, we can derive it based on the desired total and the
//!     sum of all `k` leading up to each spot. Now that we've boiled the problem down just to the core logic, we can
//!     represent the entire counter structure as just integers. Though, for very large sets of combinations, it may be
//!     preferable to store `n` within the level, so that scanning through the `0..i` chunk of the array is not required
//!     on each decrement.
//! 2.  We would like to be able to check whether or not the iterator is exhausted before we actually return a value.
//!     Otherwise, we'd need to copy the counters into a temporary buffer, then modify the counters for next time. So,
//!     we'll pre-seed the initial counter with a `k` value *one above* `n`. Then, when the first poll decrements, IterD
//!     will roll up to IterA, which will decrement `k` from 60 to 59. Then, when it regenerates its children, they'll
//!     all correctly be given a value of zero.
//!
//! With that out of the way, we can get started!

use std::mem::MaybeUninit;

/// An iterator over all possible combinations of a given set of elements, selected with replacement `r` at a time.
///
/// See module documentation for how _combinations with replacement_ are defined for this context.
///
/// # `Iterator` implementation
///
/// Even though this struct's primary purpose is to be an iterator, it does not actually implement the
/// [`std::iter::Iterator`] trait. This is because the basic `Iterator` trait does not allow the associated [`Item`]
/// type to be bound by the lifetime of the `&mut self` borrow given to [`Iterator::next`]. In a perfect world, struct
/// would implement `Iterator` with an `Item` type of either `&'_ [&'a T]` or `std::iter::Copied<std::slice::Iter<'_,
/// &'a T>>`, but that's not currently possible.
///
/// Instead, this struct provides [its own `next` method][Self::next] that mirrors `Iterator::next`'s behaviour and
/// includes the additional lifetime constraint.
///
/// [`Item`]: Iterator::Item
#[derive(Debug, Clone)]
pub struct Combinations<'a, T> {
    /// Which options we're going to pick from each iteration.
    options: &'a [T],
    /// The counter which generates selection sequences from [`Self::options`].
    counter: Counter,
    /// We need somewhere to actually yield references out of; this buffer is created once at initialization and re-used
    /// across polls.
    buf: Box<[MaybeUninit<&'a T>]>,
}

impl<'a, T> Combinations<'a, T> {
    /// Creates a new iterator over all possible combinations (order-agnostic) of the given options, selected `r` at a
    /// time, selected with replacement.
    pub fn new(options: &'a [T], r: usize) -> Self {
        // - `0 choose r` is zero for any possible `r`; there are no ways to choose anything from zero options.
        // - `n choose r` is also zero whenever `n < r`; there are no ways to select more than `n` things from `n`
        //   options.
        let n = options.len();
        let buf = if n == 0 || n < r {
            // In either case, we don't panic. Instead, use a zero-sized vec to avoid even allocating in the first
            // place, since the first `.next` call will return `None` (see the bottom of `Counter::advance`).
            Box::new_uninit_slice(0)
        } else {
            Box::new_uninit_slice(r)
        };

        Self {
            options,
            counter: Counter::new(n, r),
            buf,
        }
    }

    pub fn next(&mut self) -> Option<&'_ [&'a T]> {
        let counts = self.counter.next()?;

        // Fill the buffer with our choices:
        let mut start = 0;
        for (choice, &count) in counts.iter().enumerate() {
            let choice = &self.options[choice];
            for i in start..start + count {
                // SAFETY: It's fine to replace the old values
                self.buf[i].write(choice);
            }
            start += count;
        }

        // Now we need to yield a reference to our initialized buffer.

        assert!(start == self.buf.len(), "ComboCounter did not fill entire buffer with choices");

        let ptr = self.buf.as_ptr().cast::<&'a T>();
        let len = self.buf.len();

        // SAFETY:
        // - Our entire `Box<[MaybeUninit<&'a T>]>` has just been initialized; we asserted it so.
        // - `ptr` and `len` are both derived from an already valid, from-safe-code slice, so any slice-related safety
        //   requirements are already met.
        // - We bound the new slice by the '_ lifetime: the lifetime of `&mut self`. That means that `.next()` or any
        //   other `self.*` methods cannot be called again until after this slice is gone.
        let buf = unsafe { std::slice::from_raw_parts::<'_, &'a T>(ptr, len) };

        Some(buf)
    }
}

/// The "sub-iterator" that handles selecting [combinations] based on the algorithm derived in the module documentation.
///
/// [combinations]: Combinations
#[derive(Debug, Clone)]
struct Counter {
    /// The total number of choices each combination will be made of.
    choose: usize,
    /// Counter state for all the options.
    counters: Box<[usize]>,
}

impl Counter {
    /// Creates a new counter that yields the "shape" of all distinct "`n choose r`" combinations with replacement.
    pub fn new(n: usize, r: usize) -> Self {
        if n == 0 || n < r {
            // Same edge-case as in `Combinations::new`. Use a zero-sized vec to avoid even allocating in the first
            // place, since the first `.next` call will return `None`.
            Self {
                choose: r,
                counters: vec![0; 0].into_boxed_slice(),
            }
        } else {
            // We need one counter for each of the possible choices. But the first thing we want to yield is `choose` of
            // the first option, and we're going to decrement before that; so the first counter needs to get pre-seeded.
            let mut counters = vec![0; n].into_boxed_slice();
            counters[0] = r + 1;
            Self { choose: r, counters }
        }
    }

    /// Resets this counter to its initial state.
    pub fn reset(&mut self) {
        if self.counters.len() > 0 {
            self.counters[0] = self.choose + 1;
            self.counters[1..].fill(0);
        }
    }

    /// Returns the next item from this iterator.
    pub fn next(&mut self) -> Option<&[usize]> {
        if self.advance() { Some(&self.counters) } else { None }
    }

    /// Advances this iterator's counters forwards by one. Returns `false` if the iterator is exhausted.
    fn advance(&mut self) -> bool {
        // Work from the back:
        for i in (0..self.counters.len()).rev() {
            // Can the last one decrement?
            if self.counters[i] > 0 {
                // If so, decrement it. Then, reset all counters after this one.
                self.counters[i] -= 1;

                if i < self.counters.len() - 1 {
                    let j = i + 1;
                    // The starting value of the next counter is equal to the *total* overall number of choices we need,
                    // *minus* the current values of all counters before it. e.g., if we need 59 choices total, and the
                    // first three already have.
                    let k = self.choose - self.counters[0..j].iter().fold(0, |a, c| a + c);
                    self.counters[j] = k;
                    self.counters[j + 1..].fill(0);
                }

                return true;
            } else {
                // If we can't decrement, there are two cases:
                // 1. This is the first counter. In this case, we're completely finished; return false.
                // 2. The counter before us needs to decrement. We simply let the loop advance to the parent.
                if i == 0 {
                    return false;
                }
            }
        }

        // The only way we escape the for loop is if somehow *no* counters have a k > 0, but also if none of those
        // counters are counter #0. That is, we must have no counters, period (this iterator was created with zero
        // options). In this case, we are done immediately.
        false
    }
}
