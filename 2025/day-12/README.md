# 2025 day 12

- [Overall plan](#overall-plan)
  - [Preliminary: _Algorithm X_](#preliminary-algorithm-x)
  - [Converting day 12's tiling puzzle into an exact cover problem](#converting-day-12s-tiling-puzzle-into-an-exact-cover-problem)
    - [Pentominoes](#pentominoes)
    - [TODO: Continue write-up](#todo-continue-write-up)
- [Derivations](#derivations)
  - [Transformations of the pieces](#transformations-of-the-pieces)
    - [Rotations](#rotations)
    - [Reflections](#reflections)
    - [Conclusion](#conclusion)

<style>
.scroll {
  overflow-x: auto;
}

pre.scroll {
  white-space: pre !important;
}
</style>

## Overall plan

We will attempt to solve today's problem using Knuth's _Algorithm X._ This
algorithm is designed to solve the **exact cover problem.** The problem at hand
is notably _not_ an exact cover problem, but it may be converted to one. First,
though, we will go over the basics of the algorithm.

### Preliminary: _Algorithm X_

_Algorithm X_ works by creating an incidence matrix representing the
relationships between _choices_ and _constraints._ Each row is a choice, and
each column is a constraint. It is perhaps best explained given an example.

<!--
  Note:

  Writing braces in Markdown (and having it appear properly in a GitHub readme)
  requires using this funky '$``$' syntax.

  - https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/writing-mathematical-expressions#writing-inline-expressions
  - https://github.com/orgs/community/discussions/16993#discussioncomment-5848894
  - https://github.com/microsoft/vscode/issues/208430#issuecomment-2762344110

  Unfortunately, the Markdown All-in-One extension doesn't support this syntax,
  causing them to appear as curly quotes in the preview. We can turn off
  Markdown All-in-One's Math support and fallback to VS Code's, which uses
  MathJax instead of KaTeX, and does support the backtick syntax... but it also
  doesn't syntax-highlight properly when math blocks break across lines. So we
  have to pick one or the other.

  Boowomp :(
-->

The example [from Wikipedia][wiki-algo-x] is as follows: let the _universe_ be
$`U = \{1,\,2,\,3,\,4,\,5,\,6,\,7\}`$ and a collection of sets
$`\mathcal{S} = \{A,\,B,\,C,\,D,\,E,\,F\}`$, where

- $`A = \{1,\,4,\,7\}`$;
- $`B = \{1,\,4\}`$;
- $`C = \{4,\,5,\,7\}`$;
- $`D = \{3,\,5,\,6\}`$;
- $`E = \{2,\,3,\,6,\, 7\}`$; and
- $`F = \{2,\,7\}`$.

Then the problem is to determine if there is a subset of $\mathcal{S}$ that may
be chosen such that their union **is** $U$ (hence: "exact cover"). In this case,
a valid solution is $`\{B,\,D,\,F\}`$, which is
$`\{\{1,\,4\},\{3,\,5,\,6\},\{2,\,7\}\}`$. Finding this covering is NP-complete,
but _Algorithm X_ provides a moderately efficient means of searching possible
combinations.

The problem given above is represented by the following matrix $A$:

|       |  $1$  |  $2$  |  $3$  |  $4$  |  $5$  |  $6$  |  $7$  |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
|  $A$  |  $1$  |       |       |  $1$  |       |       |  $1$  |
|  $B$  |  $1$  |       |       |  $1$  |       |       |       |
|  $C$  |       |       |       |  $1$  |  $1$  |       |  $1$  |
|  $D$  |       |       |  $1$  |       |  $1$  |  $1$  |       |
|  $E$  |       |  $1$  |  $1$  |       |       |  $1$  |  $1$  |
|  $F$  |       |  $1$  |       |       |       |       |  $1$  |

(the non-$1$ cells could also be represented as zeroes, instead of as empty
cells).

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


### Converting day 12's tiling puzzle into an exact cover problem

The method for converting today's puzzle into an exact cover problem is, at its
simplest, the same as the pentomino problem outline by Knuth [in his original
paper][knuth-paper] on Dancing Links and Algorithm X. Our method will be based
on that version, but with significant changes made to accommodate our specific
problem.

#### Pentominoes

The original pentomino problem is slightly different from ours. In that problem,
the goal is to **completely fill** a board by using each of the 12 pentominoes
**exactly once**. For that problem:

- Every single tile on the board to be covered constraint/criteria, and gets its
  own column. So, for a 6&times;10 board, that's 60 columns.
- Every single possible way to place a pentomino gets a single row. All the
  tiles it would fill get a $1$ placed in their particular column.
- Each of the pentominoes gets its own column, since each one needs to be used
  exactly once. All rows would have a $1$ in exactly one of these 12 columns.

Obviously, this results in **a lot** of rows. The 5&times;1 pentomino (called
"$I$") can be placed in 6 different places in each row when laid down
horizontally, and in two different places per column when standing upright (for
a 6-tall, 10-wide board). That means that the $I$ pentomino alone would have
$6\times5 + 2\times10 = 50$ rows in our matrix. Each different pentomino would
have a different number of rows depending on how many unique orientations they
have and how many positions they can be placed in within the bounds of the
board.

#### TODO: Continue write-up


<!-- ----------------------------------------------------------------------- -->
<!-- ----------------------------------------------------------------------- -->
<!-- ----------------------------------------------------------------------- -->

[wiki-algo-x]: https://en.wikipedia.org/w/index.php?title=Knuth%27s_Algorithm_X&oldid=1267470602
[medium-algo-x]: https://medium.com/@lamasalah32/solving-exact-cover-problems-using-algorithm-x-b6410f6255e1
[knuth-paper]: https://arxiv.org/pdf/cs.DS/0011047


## Derivations

### Transformations of the pieces

We are permitted to rotate or reflect the present shapes as we see fit to
attempt to fit them into each region. I had heard that there were only **eight**
unique ways to orient a square like this, but I wanted to make sure myself.

Here, I worked out, by hand, all the rotations of a 3&times;3 square, including
after reflections. Each orientation is given a number. It appears that, yes,
there are indeed only eight!

<pre id="transform-table" class="scroll" style="max-width: max-content;">
           +-----------------------------------------------+
           |                   Reflection                  |
+----------+-----------+-----------+-----------+-----------+
| Rotation |   None    |    Hor.   |    Ver.   |   H.,V.   |
+==========+===========+===========+===========+===========+
|          |  123      |  321      |  789      |  987      |
|     0°   |  456      |  654      |  456      |  654      |
|          |  789 (1)  |  987 (5)  |  123 (7)  |  321 (3)  |
+----------+-----------+-----------+-----------+-----------+
|          |  741      |  963      |  147      |  369      |
|    90°   |  852      |  852      |  258      |  258      |
|          |  963 (2)  |  741 (6)  |  369 (8)  |  147 (4)  |
+----------+-----------+-----------+-----------+-----------+
|          |  987      |  789      |  321      |  123      |
|   180°   |  654      |  456      |  654      |  456      |
|          |  321 (3)  |  123 (7)  |  987 (5)  |  789 (1)  |
+----------+-----------+-----------+-----------+-----------+
|          |  369      |  147      |  963      |  741      |
|   270°   |  258      |  258      |  852      |  852      |
|          |  147 (4)  |  369 (8)  |  741 (6)  |  963 (2)  |
+----------+-----------+-----------+-----------+-----------+
|          |  123      |  321      |  789      |  987      |
|   360°   |  456      |  654      |  456      |  654      |
|          |  789 (1)  |  987 (5)  |  123 (7)  |  321 (3)  |
+----------+-----------+-----------+-----------+-----------+
</pre>

> [!TIP] Note
>
> With some smarter googling, I eventually found the proper name for this. These
> orientations are the _[dihedral group of order 8][dihedral-matrices],_ which is
> the symmetry group for a square. This is even though our pieces may be
> rectangular, and we don't necessarily care about the number of ways those
> rectangles may be symmetrical; the "square" being transformed is the coordinate
> space that the pieces sit inside of.
>
> [dihedral-matrices]: https://en.wikipedia.org/w/index.php?title=Dihedral_group&oldid=1347455413#Matrix_representation

#### Rotations

Of course, we can represent these transformations as matrices. Our shapes are
represented as a list of $(x,y)$ tile positions. Assuming we treat them as
column vectors, a 90-degree (counterclockwise) rotation in two dimensions would
look like:

$$
R = \begin{bmatrix} 0 & -1 \\ 1 & 0 \end{bmatrix}.
$$

We need to be careful, though. This $R$ assumes that the shape is centered at
the origin, but our pieces' positions are relative to the top-left corner, which
is mirrored from if it appeared on the standard Cartesian plane:

```
Shape:      Becomes:
###         (0,0), (1,0), (2,0),
#..    ==>  (0,1),
##.         (0,2), (1,2).
```

That means this rotation matrix will actually result in a clockwise rotation,
not counterclockwise&mdash;that's okay, for our purposes, though. A bigger
problem is that applying $R$ to this piece would result in some of our
coordinates becoming negative and falling off the grid we're using.

To address this, note that the piece would rotate from the top-right quadrant of
the Cartesian plane onto its side in the top-left plane. To get the piece back
into the top-right plane, we need to shift it to the right by its height (which
has now become its width), minus one.

<pre class="scroll">
Piece with w = 3, h = 3.
      ^                     ^                     ^      
      |                     |                     |      
      ###      R(90)      #..       T(2,0)        #..    
      #..       ==>       #.#         ==>         #.#    
<-----##.--->         <---###----->         <-----###--->
      |                     |                     |      
      v                     v                     v      

(# and . tiles are centered on their (x, y) positions)
</pre>

To apply translations like this, we'll need to use homogeneous coordinates. The
final matrix to rotate one of _our_ pieces, with a height of $\mathtt{h}$,
clockwise by 90&deg;, is:

<div class="scroll">

$$
\begin{aligned}
  r_1
    &=
      \begin{bmatrix}
        1 & 0 & \mathtt{h} - 1 \\
        0 & 1 & 0 \\
        0 & 0 & 1 \\
      \end{bmatrix}
      \begin{bmatrix}
        0 & -1 & 0 \\
        1 &  0 & 0 \\
        0 &  0 & 1 \\
      \end{bmatrix}
    \\[2em]
    &=
      \begin{bmatrix}
        0 & -1 & \mathtt{h} - 1 \\
        1 & 0 & 0 \\
        0 & 0 & 1 \\
      \end{bmatrix}.
\end{aligned}
$$

</div>

Applying this to an $(x, y)$ column vector gives us

<div class="scroll">

$$
\begin{aligned}
  &
    \begin{bmatrix}
      0 & -1 & \mathtt{h} - 1 \\
      1 & 0 & 0 \\
      0 & 0 & 1 \\
    \end{bmatrix}
    \begin{bmatrix} x \\ y \\ 1 \end{bmatrix}
  \\[2em]
  =&
    \begin{bmatrix}
      \mathtt{h} - y - 1 \\ x \\ 1
    \end{bmatrix}.
\end{aligned}
$$

</div>

This is a nice and simple expression that we can apply to our list of $(x,\,y)$
points:

$$
R_1(x,\,y) = \left(\mathtt{h} - y - 1,\; x\right).
$$

This is the formula we'll want to apply to each of the points in our list in
order to rotate it.

Of course, to rotate by 180&deg;, we can simply apply this transformation
multiple times. If we want to do it in one fell swoop, however, we can just
square that $r_1$ matrix, right? _Not quite!_ That $r_1$ matrix only references
the shape's height; but if we want to support rectangular pieces in our
implementation (have I mentioned I'm supporting rectangular pieces in my
implementation? We're supporting rectangular pieces in our implementation.),
then we'll need to ensure that the second rotation in the sequence shifts
backwards by the original piece's _width_ instead.

<pre class="scroll">
Piece with w = 4, h = 2:

 (scroll →)

       ^                       ^                       ^                       ^                       ^       
       |                       |         h-1           |                       |         w-1           |       
       |                      ##          ↓            ##                      |          ↓            |       
       |         R(90)        #.        T(1,0)         #.        R(90)         |        T(3,0)         |       
       .###       ==>         ##          ==>          ##         ==>       #.##          ==>          #.##    
<------##.#--->         <-----.#------>         <------.#----->         <---###.------>         <------###.--->
       |                       |                       |                       |                       |       
       v                       v                       v                       v                       v       
</pre>

Now, we could create our 180-degree $r_2$ matrix by composing together these
steps: rotate by 90, translate by height (minus one), rotate by 90 again, then
translate by width (minus one). But a more elegant way to do it would probably
be to simply rotate by 90 twice, then shift right _and_ up by the shape's width
and height at the same time.

<div class="scroll">

$$
\begin{aligned}
  &&
    r_2
    &=
      \begin{bmatrix}
        1 & 0 & \mathtt{w} - 1 \\
        0 & 1 & \mathtt{h} - 1 \\
        0 & 0 & 1 \\
      \end{bmatrix}
      \left(
        \begin{bmatrix}
          0 & -1 & 0 \\
          1 &  0 & 0 \\
          0 &  0 & 1 \\
        \end{bmatrix}
      \right)^2
  \\[2em]
  &&
    &=
      \begin{bmatrix}
        -1 &  0 & \mathtt{w} - 1 \\
         0 & -1 & \mathtt{h} - 1 \\
         0 &  0 & 1 \\
      \end{bmatrix};
  \\[2em]
  \Longrightarrow
  &&
    r_2 \begin{bmatrix} x \\ y \\ 1 \end{bmatrix}
    &=
      \begin{bmatrix}
        -1 &  0 & \mathtt{w} - 1 \\
         0 & -1 & \mathtt{h} - 1 \\
         0 &  0 & 1 \\
      \end{bmatrix}
      \begin{bmatrix} x \\ y \\ 1 \end{bmatrix}
  \\[2em]
  &&
    &=
      \begin{bmatrix}
        \mathtt{w} - x - 1 \\
        \mathtt{h} - y - 1 \\
        1 \\
      \end{bmatrix}.
\end{aligned}
$$

</div>

This gives us the following formula to apply to our $(x, y)$ points:

$$
R_2(x,\,y) = \left(\mathtt{w} - x - 1,\;\mathtt{h} - y - 1\right).
$$

Determining $R_3$ is done in much the same fashion; compose the original $R$
with itself twice (for a total of three instances), which puts our shape
sideways in the bottom-right quadrant, and then shift it upwards by its width.
This gives us the following matrix and formula:

$$
r_3
  =
    \begin{bmatrix}
       0 & 1 & 0 \\
      -1 & 0 & \mathtt{w} -1 \\
       0 & 0 & 1
    \end{bmatrix},
  \\[1em]
R_3(x,\,y)
  = \left(y,\; \mathtt{w} - x - 1\right).
$$

#### Reflections

Together with the identity transformation, these three rotation formulae, $R_1$,
$R_2$, and $R_3$, give us four of the eight possible orientations for our
present shapes. For the remaining four, we simply need to compose one additional
transformation on each of them: a horizontal or vertical reflection. It doesn't
matter which; as can be seen in the original table, either one will yield the
four remaining configurations, just in a different order.

Arbitrarily, we'll use a horizontal reflection. Deriving the transformation
matrix, $s_0$[^1], is simple: simply negate $x$, then shift to the right by the
shape's width (minus one).

> [!NOTE]
>
> We are following the naming scheme given by [the Wikipedia article on the
> dihedral group][dihedral-matrices], even though we are introducing the extra
> shifting factors of $\mathtt{w} - 1$ and $\mathtt{h} - 1$.

$$
\begin{aligned}
  s_0
  &=
    \begin{bmatrix}
      1 & 0 & \mathtt{w} - 1 \\
      0 & 1 & 0 \\
      0 & 0 & 1 \\
    \end{bmatrix}
    \begin{bmatrix}
      -1 & 0 & 0 \\
       0 & 1 & 0 \\
       0 & 0 & 1 \\
    \end{bmatrix}
  \\[2em]
  &=
    \begin{bmatrix}
      -1 & 0 & \mathtt{w} - 1 \\
       0 & 1 & 0 \\
       0 & 0 & 1 \\
    \end{bmatrix}.
\end{aligned}
$$

Of course, this gives us the first of the four new formulae:

$$
S_0(x,\,y) = \left(\mathtt{w} - x - 1,\; y\right).
$$

Then, composing each of our $r_1$, $r_2$, and $r_3$ matrices on the left-hand
side of $s_0$ should give us the remaining three.

<div class="scroll">

$$
\begin{aligned}
  s_1 = r_1 s_0
  &=
    \begin{bmatrix}
       0 & -1 & \mathtt{h} - 1 \\
      -1 &  0 & \mathtt{w} - 1 \\
       0 &  0 & 1 \\
    \end{bmatrix}
    \thickspace
  &&\Longrightarrow&
    \thickspace
  S_1(x,\,y)
  &=
    \left(\mathtt{h} - y - 1,\; \mathtt{w} - x - 1\right),

  \\[3em]

  s_2 = r_2 s_0
  &=
    \begin{bmatrix}
      1 &  0 & 0 \\
      0 & -1 & \mathtt{h} - 1 \\
      0 &  0 & 1 \\
    \end{bmatrix}
    \thickspace
  &&\Longrightarrow&
    \thickspace
  S_2(x,\,y)
  &=
    \left(x,\; \mathtt{h} - y - 1\right),

  \\[3em]

  s_3 = r_3 s_0
  &=
    \begin{bmatrix}
      0 & 1 & 0 \\
      1 & 0 & 0 \\
      0 & 0 & 1 \\
    \end{bmatrix}
    \thickspace
  &&\Longrightarrow&
    \thickspace
  S_3(x,\,y)
  &=
    \left(y,\; x\right).
\end{aligned}
$$

</div>

#### Conclusion

Finally, the transformations to go directly from the initial position into the
transformed positions are:

$$
\begin{aligned}
  R_0(x,\,y) &= (x,\;y), \\
  R_1(x,\,y) &= (\mathtt{h}-y-1,\;x), \\
  R_2(x,\,y) &= (\mathtt{w}-x-1,\;\mathtt{h}-y-1), \\
  R_3(x,\,y) &= (y,\;\mathtt{w}-x-1), \\[1ex]
  S_0(x,\,y) &= (\mathtt{w}-x-1,\;y), \\
  S_1(x,\,y) &= (\mathtt{h}-y-1,\;\mathtt{w}-x-1), \\
  S_2(x,\,y) &= (x,\;\mathtt{h}-y-1), \\
  S_3(x,\,y) &= (y,\;x).
\end{aligned}
$$

1.  $R_0$ is the identity transformation.
2.  $R_1$ is a 90-degree rotation.
3.  $R_2$ is a 180-degree rotation.
4.  $R_3$ is a 270-degree rotation.
5.  $S_0$ is a horizontal reflection.
6.  $S_1$ is a horizontal reflection followed by a 90-degree rotation
    (equivalent to a vertical reflection followed by a 270&deg; rotation).
7.  $S_2$ is a horizontal reflection followed by a 180-degree rotation
    (equivalent to a single vertical reflection).
8.  $S_3$ is a horizontal reflection followed by a 270-degree rotation
    (equivalent to a vertical reflection followed by a 90&deg; rotation, and
    also to a diagonal reflection).

The last thing we need to consider is again the fact that we will be storing the
width and heights of these shapes separately from their lists of points. So, any
transformation that results in the shape on its side must also include a swap of
$\mathtt{w}$ and $\mathtt{h}$. These are $R_1$, $R_3$, $S_1$, and $S_3$.
