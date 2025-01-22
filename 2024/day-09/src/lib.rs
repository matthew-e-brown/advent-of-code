use std::ops::Range;

#[cfg(test)]
mod tests;

/// Parses puzzle input and returns the filesystem map and the largest ID in the file.
pub fn parse_input(input: &str) -> (Vec<Option<u32>>, u32) {
    let mut map = Vec::new();

    let mut id = 0u32;
    let mut file = true;
    for char in input.trim().chars() {
        let n = char.to_digit(10).expect("failed to parse digit to number");
        for _ in 0..n {
            map.push(file.then_some(id));
        }

        if file {
            id += 1;
        }

        file = !file;
    }

    (map, id.saturating_sub(1))
}

/// Computes the checksum of the filesystem map.
pub fn checksum(files: &[Option<u32>]) -> usize {
    let mut sum = 0;

    for (i, id) in files.into_iter().enumerate() {
        if let &Some(id) = id {
            sum += (id as usize) * i;
        }
    }

    sum
}

/// Trims the given slice to get rid of any `None` values on the end to handle the edge case of even-length puzzle
/// inputs.
pub fn trim_nones(arr: &mut [Option<u32>]) -> &mut [Option<u32>] {
    let num_none = arr.iter().rev().take_while(|id| id.is_none()).count();
    let trim_amt = arr.len() - num_none;
    &mut arr[..trim_amt]
}

/// Finds the index of the next [`None`] in the given array, starting from index `i` and looking forwards.
pub fn next_slot(map: &[Option<u32>], mut i: usize) -> Option<usize> {
    while map.get(i)?.is_some() {
        i += 1;
    }

    Some(i)
}

/// Finds the index of the next-last [`Some`] in the given array, starting from index `j` and looking backwards.
pub fn next_file(map: &[Option<u32>], mut j: usize) -> Option<usize> {
    while map.get(j)?.is_none() {
        j = j.checked_sub(1)?;
    }

    Some(j)
}

/// Scans through the given array starting at index `start` and looks for a slot (a region of `None`s) that is at least
/// as long as `width`; stops if one is not found before reaching index `limit`.
pub fn find_slot_range(map: &[Option<u32>], width: usize, mut start: usize, limit: usize) -> Option<Range<usize>> {
    let limit = limit.min(map.len());
    while start < limit {
        // Find the next single slot as we did in Part 1.
        start = next_slot(map, start)?;
        if start >= limit {
            break;
        }

        // Peek forwards to ensure it's wide enough.
        let mut k = start;
        while k < limit && map[k].is_none() {
            k += 1;
            if (start..k).len() >= width {
                return Some(start..k);
            }
        }

        // If we got through that loop without returning, then the slot we found was not long enough. Try again starting
        // at the end of the slot we just checked.
        start = k;
    }

    // If we get to the end of our `i < j` loop, then we did not find one in time.
    None
}

/// Scans through the given array, backwards, starting at index `start`, and looks for a file with the ID `id`. Its
/// region within the array is returned.
pub fn find_file_range(map: &[Option<u32>], id: u32, start: usize) -> Option<Range<usize>> {
    // First, find the end of the range:
    let mut x = start;
    while map[x] != Some(id) {
        x = x.checked_sub(1)?; // Break and return `None` if we don't find this ID anywhere
    }

    // `x` is the first Some(id) as we decrement downwards; exclusive range, so file ends at x+1.
    let fe = x + 1;

    // Now, decrement until we stop seeing Some(id):
    while map[x] == Some(id) {
        if x > 0 {
            x -= 1;
        } else {
            return Some(0..fe); // `x` hit zero before we stopped seeing Some(id).
        }
    }

    let fs = x + 1; // `x` is the first *non*-Some(id) that we found; the file starts one afterwards
    Some(fs..fe)
}
