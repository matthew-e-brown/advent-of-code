## Step 1: Tidy up the builder impl

- [ ] Get rid of the `Header` step
- [ ] Convert `Body` into the main `MatrixBuilder` type
- [ ] That type then just has a `from_columns` that accepts an
      `IntoIterator<Item = Column>`
- [ ] Need a new error_kind for when columns is empty (should maybe also add one
      for rows). (or is a matrix with no columns a non-error, since it is
      technically instantly solved?)

See Git Stash for more details. Most if the groundwork was there, but I had so
many changed and modified files that it got daunting and I had to stash it away.

## Step 2: Unit tests

Write some simple unit tests to see if this whole thing works at all yet:

- [ ] The example from Wikipedia with the numbered sets
- [ ] The example from my notebook


## Step 3: Introduce generics

- [ ] Take the current `dlx.rs` and `dlx/builder.rs` and make those into a "raw"
      submodule
- [ ] Then build thin wrappers around them for `CoverProblem<C, R>` or something
      (see cover.rs in Git Stash). Can just hold a `raw::DLXMatrix` alongside an
      array of `col_names` and `row_names`.

Repeating the raw builder code and stuff around the raw stuff feels like a
shitty idea with lots of repetition... but there are only like two or three
methods involved here!

If we keep the `raw::DLXMatrix` API public, then we can use that as an excuse to
not feel as bad limiting `C` to be `Hash`... maybe?

- [ ] `ProblemBuilder<C, R>` can then have a `finish_raw()` method on it when
  `C=(),R=()`, that just returns the raw `DLXMatrix` for those who don't want
  generics.
