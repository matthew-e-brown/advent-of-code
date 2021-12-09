fn parse(string: &str) -> Result<Vec<u8>, &'static str> {
    string
        .split(",")
        .map(|s| {
            match s.parse::<u8>() {
                Ok(n) if n <= 8 => Ok(n),
                Ok(_) => Err("Fishy's lifespan must be between 0 and 8."),
                Err(_) => Err("Encountered malformed sequence."),
            }
        })
        .collect()
}


pub fn run(fishies: &str, days: usize) -> Result<usize, &'static str> {
    let fishies = parse(fishies)?;

    // Each index `n` in the array of spawn timers holds how many fish have that many days to go.  
    // Use two arrays and alternate between them for current and next day
    let mut timers_1 = [0usize; 9];
    let mut timers_2 = [0usize; 9];
    let mut which = true;

    for n in fishies {
        timers_1[n as usize] += 1;
    }

    #[cfg(test)]
    println!("curr => {:?}\nnext => {:?}\n", timers_1, timers_2);

    for _ in 0..days {
        let (current, next) = match which {
            true => (&mut timers_1, &mut timers_2),
            false => (&mut timers_2, &mut timers_1),
        };

        which = !which;

        // Zero out tomorrow
        next.iter_mut().for_each(|e| *e = 0);

        for n in 0..9 {
            if n == 0 {
                // All fish reset their timers
                next[6] += current[n];
                // And they also all give birth to new fish with timers at 8 days
                next[8] += current[n];
            } else {
                next[n - 1] += current[n];
            }
        }

        #[cfg(test)]
        println!(
            "{} => {:?}\n{} => {:?}\n",
            if which { "current" } else { "   next" }, timers_1,
            if which { "   next" } else { "current" }, timers_2,
        );
    }

    let last_day = match which {
        true => &timers_1,
        false => &timers_2,
    };

    #[cfg(test)]
    println!("{:?}", last_day);

    Ok(last_day.iter().sum())
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(run("3,4,3,1,2", 18).unwrap(), 26);
    }

    #[test]
    fn example_2() {
        assert_eq!(run("3,4,3,1,2", 80).unwrap(), 5934);
    }

    #[test]
    fn example_3() {
        assert_eq!(run("3,4,3,1,2", 256).unwrap(), 26_984_457_539)
    }

}