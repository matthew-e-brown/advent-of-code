fn main() {
    let input = aoc_utils::puzzle_input();

    let mut map = parse_input(input.trim());
    let mut i = next_slot(&map, 0).unwrap();
    let mut j = next_file(&map, map.len() - 1).unwrap();

    loop {
        let file = map[j].take().unwrap();
        map[i] = Some(file);

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
    }

    println!("De-fragmented checksum (part 1): {}", checksum(&map));
}

fn next_slot<T>(arr: &[Option<T>], mut i: usize) -> Option<usize> {
    while arr.get(i)?.is_some() {
        i += 1;
    }

    Some(i)
}

fn next_file<T>(arr: &[Option<T>], mut i: usize) -> Option<usize> {
    while arr.get(i)?.is_none() {
        i = i.checked_sub(1)?;
    }

    Some(i)
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

fn parse_input(input: &str) -> Vec<Option<u32>> {
    let mut map = Vec::new();

    let mut id = 0u32;
    let mut file = true;
    for char in input.chars() {
        let n = char.to_digit(10).expect("failed to parse digit to number");
        for _ in 0..n {
            map.push(file.then_some(id));
        }

        if file {
            id += 1;
        }

        file = !file;
    }

    map
}
