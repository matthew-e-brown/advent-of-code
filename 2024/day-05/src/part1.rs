use crate::graph::PageGraph;
use crate::parse_nums;


pub fn main<I, S>(graph: &PageGraph, lines: I) -> u32
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut sum = 0;

    for line in lines {
        let pages = parse_nums(line.as_ref());
        if is_ordered(graph, &pages) {
            sum += pages[pages.len() / 2];
        }
    }

    sum
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
