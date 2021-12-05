use regex::Regex;

#[derive(Debug, Clone)]
pub struct Reindeer {
    name: String,
    speed: usize,
    running_time: usize,
    resting_time: usize,
}


impl Reindeer {

    pub fn new_from_list(list: &Vec<String>) -> Result<Vec<Self>, String> {
        let re = Regex::new(r"^(\w+).+?(\d+) km/s.+?(\d+) seconds.+?(\d+) seconds\.?$").unwrap();

        let mut result = Vec::new();

        for line in list {
            let caps = re.captures(&line).ok_or(format!("Malformed line: {}", line))?;

            let name = caps.get(1).unwrap().as_str().to_owned();
            let speed = caps.get(2).unwrap().as_str().parse().unwrap();
            let running_time = caps.get(3).unwrap().as_str().parse().unwrap();
            let resting_time = caps.get(4).unwrap().as_str().parse().unwrap();

            result.push(Self { name, speed, running_time, resting_time });
        }

        Ok(result)
    }

}



fn let_them_fly(reindeer: &Reindeer, time: usize) -> usize {
    let mut resting = false;
    let mut current_resting_time = 0;
    let mut current_running_time = 0;
    let mut total_distance = 0;

    for _ in 0..time {
        if !resting {
            current_running_time += 1;

            // Check if we need to start resting on the next second
            if current_running_time >= reindeer.running_time {
                resting = true;
                current_resting_time = 0;
            }

            total_distance += reindeer.speed;
        } else {
            current_resting_time += 1;

            // Check if we can wake up on the next second
            if current_resting_time >= reindeer.resting_time {
                resting = false;
                current_running_time = 0;
            }
        }
    }

    total_distance
}


pub fn run_1(reindeer: &Vec<Reindeer>) -> Result<(&String, usize), &'static str> {
    reindeer
        .iter()
        .map(|r| (&r.name, let_them_fly(&r, 2503)) )
        .max_by(|r1, r2| r1.1.cmp(&r2.1))
        .ok_or("Empty vector.")
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let reindeer = vec![
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.".to_owned(),
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.".to_owned()
        ];
        let reindeer = Reindeer::new_from_list(&reindeer).unwrap();

        assert_eq!(let_them_fly(&reindeer[0], 1000), 1120);
        assert_eq!(let_them_fly(&reindeer[1], 1000), 1056);
    }

}