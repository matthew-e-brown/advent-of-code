#[derive(Clone)]
pub struct Fishy {
    timer: u8,
}

impl Fishy {

    fn new_from(timer: u8) -> Self {
        Self { timer }
    }

    fn new() -> Self {
        Self::new_from(8)
    }

    fn tick(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            true
        } else {
            self.timer -= 1;
            false
        }
    }

}


pub fn school_from_string(string: &str) -> Result<Vec<Fishy>, &'static str> {
    string
        .split(",")
        .map(|s| {
            if let Ok(s) = s.parse() {
                Ok(Fishy::new_from(s))
            } else {
                Err("Encountered malformed sequence.")
            }
        })
        .collect()
}


pub fn run(fishies: &Vec<Fishy>, days: usize) -> usize {
    let mut fishies = fishies.clone();

    for _ in 0..days {

        let mut born_today = Vec::new();

        for fishy in fishies.iter_mut() {
            if fishy.tick() {
                born_today.push(Fishy::new());
            }
        }

        fishies.extend(born_today);

    }

    fishies.len()
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let start = school_from_string("3,4,3,1,2").unwrap();
        assert_eq!(run(&start, 18), 26);
    }
}