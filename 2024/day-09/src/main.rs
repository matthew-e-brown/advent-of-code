use day_09::*;

fn main() {
    let input = aoc_utils::puzzle_input();
    let (map, largest_id) = parse_input(&input);

    let mut map1 = map.clone();
    let mut map2 = map;
    part1(&mut map1);
    part2(&mut map2, largest_id);

    println!("Compacted checksum (part 1): {}", checksum(&map1));
    println!("Defragged checksum (part 2): {}", checksum(&map2));
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

fn part2(map: &mut [Option<u32>], largest_id: u32) {
    let map = trim_nones(map);
    if map.len() == 0 {
        return;
    }

    let mut i = 0;
    let mut j = map.len() - 1;
    let mut id = largest_id;
    loop {
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

        if i >= j {
            break;
        }

        // Decrement to next file.
        if id > 0 {
            id -= 1;
        } else {
            break;
        }
    }
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
