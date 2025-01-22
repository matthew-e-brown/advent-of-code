use std::ops::Range;

fn main() {
    let input = aoc_utils::puzzle_input();

    let (map, largest_id) = parse_input(&input);
    println!("Map size is {}", map.len());

    let mut map1 = map.clone();
    let mut map2 = map;
    part1(&mut map1);
    part2(&mut map2, largest_id);

    println!("Compacted checksum (part 1): {}", checksum(&map1));
    println!("Defragged checksum (part 2): {}", checksum(&map2));
}

/// Parses puzzle input, returning the file block map, as well as the largest ID in the file.
fn parse_input(input: &str) -> (Vec<Option<u32>>, u32) {
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

fn checksum(files: &[Option<u32>]) -> usize {
    let mut sum = 0;

    for (i, id) in files.into_iter().enumerate() {
        if let &Some(id) = id {
            sum += (id as usize) * i;
        }
    }

    sum
}

#[allow(unused)] // For debugging, only works for digits 0-9
fn print_map(files: &[Option<u32>]) -> String {
    files
        .into_iter()
        .map(|f| match f {
            Some(id) => char::from_u32(id + '0' as u32).unwrap(),
            None => '.',
        })
        .collect()
}

/// Trims the given slice to get rid of any `None` values on the end.
fn trim_nones(arr: &mut [Option<u32>]) -> &mut [Option<u32>] {
    let num_none = arr.iter().rev().take_while(|id| id.is_none()).count();
    let trim_amt = arr.len() - num_none;
    &mut arr[..trim_amt]
}

fn part1(map: &mut [Option<u32>]) {
    let map = trim_nones(map);
    if map.len() == 0 {
        return;
    }

    let mut i = 0;
    let mut j = map.len() - 1;
    loop {
        match next_slot(&map, i) {
            Some(idx) => i = idx,
            None => break,
        }

        match next_file(&map, j) {
            Some(idx) => j = idx,
            None => break,
        }

        if i >= j {
            break;
        }

        let id = map[j].take().unwrap();
        map[i] = Some(id);
    }
}

fn next_slot(map: &[Option<u32>], mut i: usize) -> Option<usize> {
    while map.get(i)?.is_some() {
        i += 1;
    }

    Some(i)
}

fn next_file(map: &[Option<u32>], mut j: usize) -> Option<usize> {
    while map.get(j)?.is_none() {
        j = j.checked_sub(1)?;
    }

    Some(j)
}

// - 6381635886788 -- too large
// - 6381726813949 -- even larger
fn part2(map: &mut [Option<u32>], largest_id: u32) {
    let map = trim_nones(map);
    if map.len() == 0 {
        return;
    }

    let mut i = 0;
    let mut j = map.len() - 1;
    let mut id = largest_id;
    loop {
        // println!("i={i:2}, j={j:2} -- {} -- attempting to move {id}", print_map(&map));

        // Find the position of the next file, then try to find a spot for that file.
        let file_range = find_file_range(map, id, j).expect("puzzle input should always contain all IDs");
        j = file_range.start; // Subsequent moves will always take place to the left of here

        if let Some(slot_range) = find_slot_range(map, file_range.len(), i, j) {
            // If the slot we find starts right at our furthest-left-slot, then we know we're filling right up to the
            // left edge. So we can shrink our range for later searches.
            if slot_range.start == i {
                i += slot_range.len();
            }

            // Move the file into place:
            map[slot_range].fill(Some(id));
            map[file_range].fill(None);
        }

        // Decrement to next file.
        if id > 0 {
            id -= 1;
        } else {
            break;
        }
    }

    // println!("i={i:2}, j={j:2} -- {}", print_map(&map));
}

fn find_file_range(map: &[Option<u32>], id: u32, start: usize) -> Option<Range<usize>> {
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

fn find_slot_range(map: &[Option<u32>], width: usize, mut start: usize, limit: usize) -> Option<Range<usize>> {
    while start < limit && start < map.len() {
        // Find the next single slot as we did in Part 1.
        start = next_slot(map, start)?;

        // Peek forwards to ensure it's wide enough.
        let mut k = start;
        while k < map.len() && map[k].is_none() {
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
