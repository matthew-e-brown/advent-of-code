# 2025, day 8: Sets of junction boxes

The crux of this puzzle is keeping track of a series of _[junction boxes]_
(points in 3D space) as they are connected together into _circuits._
Specifically, keeping track of which boxes are already connected to each other
as more and more connections are added.

Reading through [the problem], anyone in-the-know would be quick to realize that
this is meant to be a problem involving _[disjoint sets]._ However... I had
completely forgotten about disjoint sets. So I did it with a graph instead, and
manually kept track of which components were connected together.

[junction boxes]: https://en.wikipedia.org/wiki/Junction_box
[the problem]: https://adventofcode.com/2025/day/8
[disjoint sets]: https://en.wikipedia.org/wiki/Disjoint-set_data_structure
