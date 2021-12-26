pub const LINE_ERR: &'static str = "Encountered malformed line";

#[derive(Debug)]
pub struct Paper {
    /// Width of the paper
    w: usize,
    /// Height of the paper
    h: usize,
    /// Row-major 1d array representing if there is a dot at any given (x, y) point in the paper
    dots: Vec<bool>,
}


impl Paper {

    pub fn get(&self, pos: (usize, usize)) -> bool {
        let (x, y) = pos;
        self.dots[self.w * y + x]
    }

    pub fn get_mut(&mut self, pos: (usize, usize)) -> &mut bool {
        let (x, y) = pos;
        &mut self.dots[self.w * y + x]
    }

    fn set_point(&mut self, pos: (usize, usize), val: bool) {
        let (x, y) = pos;

        if x > self.w { self.w = x + 1; }
        if y > self.h { self.h = y + 1; }

        let req_len = self.w * self.h;
        if req_len > self.dots.len() {
            self.dots.resize(req_len, false);
        }

        *self.get_mut(pos) = val;
    }

    pub fn new<I>(data: I) -> Result<Self, String> where I: IntoIterator, I::Item: AsRef<str> {

        let mut output = Self { w: 0, h: 0, dots: Vec::new() };

        for line in data {
            let line = line.as_ref();

            let splits = line.split(",").collect::<Vec<_>>();
            if splits.len() != 2 {
                return Err(format!("{}: '{}'", LINE_ERR, line));
            }

            let x = splits[0].parse::<usize>().or_else(|_| Err(format!("{}: '{}'", LINE_ERR, line)))?;
            let y = splits[1].parse::<usize>().or_else(|_| Err(format!("{}: '{}'", LINE_ERR, line)))?;

            output.set_point((x, y), true);
        }


        #[cfg(test)]
        println!("Done paper parse");

        Ok(output)
    }

}


#[derive(Debug)]
pub enum Axis { X, Y }

#[derive(Debug)]
pub struct Fold {
    pub axis: Axis,
    pub position: usize,
}


impl Fold {

    pub fn new<I>(data: I) -> Result<Vec<Self>, String> where I: IntoIterator, I::Item: AsRef<str> {

        let mut folds = Vec::new();

        for line in data {
            let line = line.as_ref();

            // Find '=' to split at
            let e = line
                .chars()
                .position(|c| c == '=')
                .ok_or_else(|| format!("{}: '{}'", LINE_ERR, line))?;

            #[cfg(test)]
            println!("Found '=' at {}", e);

            let letter = line.chars().nth(e - 1);

            #[cfg(test)]
            println!("Letter be for 'e' = {:?}", letter);

            // Get the character before it
            let axis = match letter {
                Some(s) if s == 'x' => Axis::X,
                Some(s) if s == 'y' => Axis::Y,
                _ => { return Err(format!("{}: '{}'", LINE_ERR, line)); },
            };

            let position = line[e + 1..].parse().or_else(|_| Err(format!("{}: '{}'", LINE_ERR, line)))?;

            folds.push(Fold { axis, position });
        }

        Ok(folds)
    }

}


pub fn parse(data: &Vec<String>) -> Result<(Paper, Vec<Fold>), String> {

    let mut data = data.iter();

    // Take the first chunk, up until the first empty line.  
    // Take by_ref() so that the underlying `data` iterator is also advanced.
    let mut points = data.by_ref().take_while(|s| !s.trim().is_empty());

    let paper = Paper::new(&mut points)?;
    let folds = Fold::new(&mut data)?;

    Ok((paper, folds))
}


#[cfg(test)]
mod tests {

    use super::*;

    fn example_data() -> Vec<String> {
        vec![
            "6,10", "0,14", "9,10",  "0,3",  "10,4", "4,11",
             "6,0", "6,12",  "4,1", "0,13", "10,12",  "3,4",
             "3,0",  "8,4", "1,10", "2,14",  "8,10",  "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ].iter().map(|s| String::from(*s)).collect()
    }


    #[test]
    fn parse_text() {
        let data = example_data();
        let results = parse(&data);

        println!("{:?}", results);
        assert!(results.is_ok());
    }

}