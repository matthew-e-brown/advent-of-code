use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Reindeer {
    pub name: String,
    pub speed: usize,
    pub running_time: usize,
    pub resting_time: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Competitor<'a> {
    reindeer: &'a Reindeer,
    distance: usize,
    is_resting: bool,
    current_running_time: usize,
    current_resting_time: usize,
}


impl Reindeer {

    pub fn new_list(list: &Vec<String>) -> Result<Vec<Self>, String> {
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


impl<'a> Competitor<'a> {

    fn new(reindeer: &'a Reindeer) -> Self {
        Self {
            reindeer,
            is_resting: false,
            current_resting_time: 0,
            current_running_time: 0,
            distance: 0,
        }
    }

    fn new_list(reindeer: &'a Vec<Reindeer>) -> Result<Vec<Self>, &'static str> {
        if reindeer.len() < 1 {
            return Err("There needs to be at least one competitor!");
        }

        let result = reindeer
            .iter()
            .map(|r| Competitor::new(r))
            .collect::<Vec<_>>();

        Ok(result)
    }

    fn tick(&mut self) -> () {
        if !self.is_resting {
            self.current_running_time += 1;

            // Check if we need to start resting on the next second
            if self.current_running_time >= self.reindeer.running_time {
                self.is_resting = true;
                self.current_resting_time = 0;
            }

            self.distance += self.reindeer.speed;
        } else {
            self.current_resting_time += 1;

            // Check if we can wake up on the next second
            if self.current_resting_time >= self.reindeer.resting_time {
                self.is_resting = false;
                self.current_running_time = 0;
            }
        }
    }

}


pub fn run_1(reindeer: &Vec<Reindeer>, time: usize) -> Result<(&String, usize), &'static str> {

    let mut reindeer = Competitor::new_list(reindeer)?;

    for _second in 0..time {
        for r in reindeer.iter_mut() {
            r.tick();
        }
    }

    let winner = reindeer
        .iter()
        .max_by(|a, b| a.distance.cmp(&b.distance))
        .unwrap();

    Ok((&winner.reindeer.name, winner.distance))
}


pub fn run_2(reindeer: &Vec<Reindeer>, time: usize) -> Result<(&String, usize), &'static str> {

    let mut reindeer = Competitor::new_list(reindeer)?;
    let mut points = HashMap::new();

    for _second in 0..time {
        for r in reindeer.iter_mut() {
            r.tick();
        }

        // Can't use max_by because we need to handle the case where there's a tie

        // Find what the max distance is...
        let mut winning_distance = 0;
        for competitor in reindeer.iter() {
            if competitor.distance > winning_distance {
                winning_distance = competitor.distance;
            }
        }

        // ...and increment *all* the reindeer with that distance
        for competitor in reindeer.iter() {
            if competitor.distance == winning_distance {
                points
                    .entry(competitor.reindeer)
                    .and_modify(|p| *p += 1)
                    .or_insert(1usize);
            }
        }
    }

    let (winner, points) = points
        .iter()
        .max_by(|(_, &a), (_, &b)| a.cmp(&b))
        .unwrap();

    Ok((&winner.name, *points))
}


#[cfg(test)]
mod tests {

    use super::*;

    fn example_data() -> Vec<Reindeer> {
        Reindeer::new_list(&vec![
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.".to_owned(),
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.".to_owned()
        ]).unwrap()
    }

    #[test]
    fn example_1() {
        let reindeer = example_data();

        let mut reindeer = reindeer.iter()
            .map(|r| Competitor::new(r))
            .collect::<Vec<_>>();

        for _tick in 0..1000 {
            for r in reindeer.iter_mut() {
                r.tick();
            }
        }

        assert_eq!(reindeer[0].distance, 1120);
        assert_eq!(reindeer[1].distance, 1056);

    }

    #[test]
    fn example_2() {
        let reindeer = example_data();
        let (winner, points) = run_2(&reindeer, 1000).unwrap();

        assert_eq!(&winner[..], "Dancer");
        assert_eq!(points, 689);
    }

}