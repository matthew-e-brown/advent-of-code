use std::cmp::Ordering;
use std::collections::VecDeque;

use crate::graph::PageGraph;

mod graph;


fn main() {
    let input = aoc_utils::puzzle_input();
    let mut lines = input.lines();

    let rule_lines = lines.by_ref().take_while(|line| line.trim().len() > 0);
    let graph = PageGraph::from_input(rule_lines);

    let mut sorted_mid_sums = 0;
    let mut unsorted_mid_sums = 0;

    for line in lines {
        let mut pages = parse_nums(line);
        if is_ordered(&graph, &pages) {
            sorted_mid_sums += pages[pages.len() / 2];
        } else {
            // This has gotta be the most inefficient way to possibly do this...

            // Subset only the pages that matter, then use that subgraph to sort the pages:
            let subset = graph.subset(&pages);
            pages.sort_by(|a, b| {
                // If there exists a path from `a->b`, then `a` must come before `b`.
                if bfs(&subset, a, b) {
                    Ordering::Less
                } else if bfs(&subset, b, a) {
                    // Again... probably not that efficient to do a whole BFS twice. Oh well.
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            unsorted_mid_sums += pages[pages.len() / 2];
        }
    }

    println!("Sum of already-sorted middle elements (part 1): {}", sorted_mid_sums);
    println!("Sum of freshly-sorted middle elements (part 2): {}", unsorted_mid_sums);
}


/// Parses a comma-separated list string of page numbers into a vector.
fn parse_nums<S: AsRef<str>>(line: S) -> Vec<u32> {
    line.as_ref()
        .split(',')
        .map(|n| n.parse::<u32>().expect("invalid page number"))
        .collect::<Vec<_>>()
}


// NB: This can be done just as easily (and more intuitively) by iterating forwards through the list. The difference
// is that doing it this way means it can be done using just `outgoing` edges, which, depending on how part2 goes, may
// remove the need for an
fn is_ordered(graph: &PageGraph, pages: &[u32]) -> bool {
    // Iterate backwards over the slice, flagging the pages that depend on those we've seen.
    let mut flagged = Vec::new();
    for &page_num in pages.into_iter().rev() {
        // Check if this page appears in the list of flagged dependants, then one of its dependencies must have been
        // found earlier in the loop (i.e., *later* in the list, since we're looping backwards).
        if flagged.binary_search(&page_num).is_ok() {
            return false;
        }

        // Otherwise, if this page has dependants, add them to the list for next time.
        if let Some(page) = graph.get(&page_num) {
            for &dep in page.outgoing() {
                // Keep the list sorted as we insert; only insert new pages.
                if let Err(i) = flagged.binary_search(&dep) {
                    flagged.insert(i, dep);
                }
            }
        }
    }

    // Found nothing amiss, so it's in order! :)
    true
}


/// Searches the given path to see if a path exists from `src` to `dst`.
fn bfs(graph: &PageGraph, src: &u32, dst: &u32) -> bool {
    let Some(src) = graph.get(&src) else { return false };

    let mut queue = VecDeque::new();
    let mut enqueued = Vec::new();

    queue.push_back(src);
    enqueued.push(src.num());

    while !queue.is_empty() {
        let page = queue.pop_front().unwrap(); // unwrap: we know the queue is non-empty
        if page.num() == *dst {
            return true;
        } else {
            for &num in page.outgoing() {
                if let Err(i) = enqueued.binary_search(&num) {
                    let Some(page) = graph.get(&num) else { continue };
                    queue.push_back(page);
                    enqueued.insert(i, num);
                }
            }
        }
    }

    false
}
