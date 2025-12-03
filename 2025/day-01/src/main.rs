fn main() {
    let input = aoc_utils::puzzle_input();
    let lines = input.lines();

    let mut dial: isize = 50;
    let mut password1: usize = 0;
    let mut password2: usize = 0;

    for line in lines {
        let rot = parse_rotation(line);

        // r / 100 is the number of full revolutions this rotation would cause on its own (signed).
        // r % 100 is the effect of r "within" the modulus; -110 will have the same effect as -10.
        let rot_div = rot / 100;
        let rot_rem = rot % 100;

        // Then, if dial grows to be beyond 100 or below 0, that's one more revolution.
        // We can check for that simply by seeing if r == r mod 100.
        let tmp = dial + rot_rem;
        let rem = tmp.rem_euclid(100);

        // We need to be careful with checking our mod 100, though:
        // - landing on zero counts, but:
        //   - if we land on 100 and mod down to zero, r != r mod 100.
        //   - if we land on 0 and don't need to mod, r == r mod 100.
        // - if we *start* at zero, then r != r mod 100, but it *doesn't* count as a full revolution.
        // So, our final condition is `(landed_on_zero || wrapped_around && !started_zero)`.
        // Plus the number of revolutions given by r / 100.
        let p1 = (rem == 0) as usize;
        let p2 = (rem == 0 || (rem != tmp && dial != 0)) as usize + rot_div.abs() as usize;

        // Man, I needed so much debugging for this one... what a doozy for day 1!
        if aoc_utils::verbosity() > 0 {
            print!("Dial {dial:2}");

            if rot_rem != rot {
                print!(" + {rot:5} (effect {rot_rem:3})");
            } else {
                print!(" + {rot:5}         {rot_rem:3} ", rot_rem = "");
            }

            print!(" => {rem:2}. Full revs: {rot_div:2}.", rot_div = rot_div.abs());

            if p1 > 0 {
                print!(" P1 = {:4}.", password1 + p1);
            } else {
                print!("           ");
            }

            if p2 > 0 {
                print!(" P2 = {:4}.", password2 + p2);
            } else {
                print!("           ");
            }

            println!();
        }

        password1 += p1;
        password2 += p2;
        dial = rem;
    }

    println!("Password (part 1): {password1}");
    println!("Password (part 2): {password2}");

    if aoc_utils::verbosity() >= 2 {
        // Brute fucking force:
        let mut dial = 50isize;
        let mut pass2 = 0usize;
        for line in input.lines() {
            let rot = parse_rotation(line);
            let dir = rot.signum(); // -1, 0, +1

            if aoc_utils::verbosity() >= 3 {
                print!("Dial {dial:2} + {rot:5} (effect {:3}) => ", rot % 100);
            }

            for _ in 0..rot.abs() {
                dial = (dial + dir).rem_euclid(100);
                if dial == 0 {
                    pass2 += 1;
                }
            }

            if aoc_utils::verbosity() >= 3 {
                println!("{dial:2}. P2 = {pass2:4}.");
            }
        }

        println!("Password (part 2, bazooka method): {pass2}");
    }
}

fn parse_rotation(line: &str) -> isize {
    assert!(line.len() > 0 && line.is_char_boundary(1), "invalid puzzle input");
    let amt = line[1..].parse::<isize>().expect("invalid puzzle input");
    match line.as_bytes()[0] {
        b'L' => -amt,
        b'R' => amt,
        _ => panic!("invalid puzzle input"),
    }
}
