# 2025 day 12 plan

We will attempt to solve today's problem using Knuth's _Algorithm X._ This
algorithm is designed to solve the **exact cover problem.** The problem at hand
is notably _not_ an exact cover problem, but it may be converted to one. First,
though, we will go over the basics of the algorithm.

## Preliminary: _Algorithm X_

_Algorithm X_ works by creating an incidence matrix **A** representing the
relationships between _choices_ and _constraints._ Each row is a choice, and
each column is a constraint. It is perhaps best explained given an example.

The example from Wikipedia is as follows: let the _universe_ be $U = \{1, 2, 3,
4, 5, 6, 7\}$ and a collection of sets $\mathcal{S} = \{A, B, C, D, E, F\}$,
where

- $A = \{1, 4, 7\}$;
- $B = \{1, 4\}$;
- $C = \{4, 5, 7\}$;
- $D = \{3, 5, 6\}$;
- $E = \{2, 3, 6, 7\}$; and
- $F = \{2, 7\}$.

Then the problem is to determine if there is a subset of $\mathcal{S}$ that may
be chosen such that their union **is** $U$ (hence: "exact cover"). In this case,
a valid solution is ${B, D, F}$, which is $\{ \{1, 4\}, \{3, 5, 6\}, \{2, 7\}
\}$. Finding this covering is NP-complete, but _Algorithm X_ provides a
moderately efficient means of searching possible combinations.

The problem given above is represented by the following matrix $A$:

|     | $1$ | $2$ | $3$ | $4$ | $5$ | $6$ | $7$ |
|-----|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| $A$ | $1$ |     |     | $1$ |     |     | $1$ |
| $B$ | $1$ |     |     | $1$ |     |     |     |
| $C$ |     |     |     | $1$ | $1$ |     | $1$ |
| $D$ |     |     | $1$ |     | $1$ | $1$ |     |
| $E$ |     | $1$ | $1$ |     |     | $1$ | $1$ |
| $F$ |     | $1$ |     |     |     |     | $1$ |

(the empty cells can also be represented as $0$s).

From here, the core of the algorithm is how the rows and the columns represent
the relationship of _choices_ and _constraints:_

- Each row represents a possible **choice.** In this case, selecting one of the
  sets of $\mathcal{S}$.
- The columns represent **constraints** (or perhaps **criteria** is a more
  intuitive word) of the problem that must be met. In this case, each of the
  values of the universe $U$ that are to be selected is a constraint that our
  final selection must meet.
- To solve the **exact** cover problem, each constraint must be met exactly
  once.
- When a choice is made (and added to a separate list representing our _current
  partial solution_), we remove that row from the matrix, since that choice
  cannot be made a second time.
- Then, when removing a row $r$ from the matrix, we look at all columns $c$ such
  that $A_{r,c}$ has a $1$; since selecting that row means those columns have
  had their constraints met, all other rows which also have a $1$ in that
  position may also be removed from consideration.
- When the matrix has no more columns, our current partial solution is a valid
  solution. If we run out of rows (choices) before columns, then this branch of
  the search does not work; so we re-add the columns and rows we just removed
  and backtrack up a level to keep searching.

The outcome of this formulation is that the algorithm takes the obvious approach
of simply enumerating all possibilities and makes it much more efficient by
creating a link between one choice and the other choices it eliminates.

Additionally, using an efficient representation for what is often in practice a
_very_ sparse matrix can make a big difference. In particular, Knuth uses this
algorithm as a showcase for his **Dancing Links** structure, wherein each $1$ is
stored as a node in a linked-list-esque structure. Each $1$ holds a link to the
$1$ which is next/previous in succession in all four directions. This makes it
very quick to scan across an entire row or column without having to hunt through
zeroes.

The actual algorithm works by first selecting a column deterministically
(according to some heuristic or other reasonable logic), _then_ by choosing a
row with a $1$ in that column. The choice of row can be backtracked (and is
therefore "non-deterministic", algorithmically speaking), but the choice of
column is not.

## Converting day 12's tiling puzzle into an exact cover problem

[TODO]
