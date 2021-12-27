use std::fmt::Display;

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

    pub fn get(&self, pos: (usize, usize)) -> Option<&bool> {
        let (x, y) = pos;
        self.dots.get(self.w * y + x)
    }

    pub fn get_mut(&mut self, pos: (usize, usize)) -> Option<&mut bool> {
        let (x, y) = pos;
        self.dots.get_mut(self.w * y + x)
    }

    fn set_point(&mut self, pos: (usize, usize), val: bool) {
        let (x, y) = pos;

        if x >= self.w { self.w = x + 1; }
        if y >= self.h { self.h = y + 1; }

        let req_len = self.w * self.h;
        if req_len > self.dots.len() {
            self.dots.resize(req_len, false);
        }

        // Can unwrap because we just checked for overflow errors
        *self.get_mut(pos).unwrap() = val;
    }

    /// Same as [set_point], but checks if there is already a value at that location and ORs the two
    fn add_point(&mut self, pos: (usize, usize), val: bool) {
        match self.get(pos) {
            Some(&existing) => self.set_point(pos, val || existing),
            None => self.add_point(pos, val),
        }
    }

    pub fn new<I>(data: I) -> Result<Self, String>
    where
        I: IntoIterator, I::Item: AsRef<str>
    {

        let mut output = Self { w: 0, h: 0, dots: Vec::new() };

        for line in data {
            let line = line.as_ref();

            let splits = line.split(",").collect::<Vec<_>>();
            if splits.len() != 2 {
                return Err(format!("{}: '{}'", LINE_ERR, line));
            }

            let x = splits[0].parse().or_else(|_| Err(format!("{}: '{}'", LINE_ERR, line)))?;
            let y = splits[1].parse().or_else(|_| Err(format!("{}: '{}'", LINE_ERR, line)))?;

            output.set_point((x, y), true);
        }

        Ok(output)
    }


    pub fn fold(&mut self, fold: &Fold) -> Result<(), String> {

        let invalid =  match fold.axis {
            Axis::X => fold.position >= self.w,
            Axis::Y => fold.position >= self.h,
        };

        if invalid {
            return Err(format!(
                "Can not fold along {} = {}, because paper is not large enough ({} x {})",
                fold.axis, fold.position, self.w, self.h
            ));
        }

        // ----

        // We create a new Paper to copy the points into
        let new_self: Paper;
        // Finds a point's mirror point
        let find_new_pos: fn((usize, usize), usize) -> (usize, usize);
        // Determines if this point needs to be mirrored while copying
        let is_after_fold: fn((usize, usize), usize) -> bool;

        match fold.axis {
            Axis::X => {
                let (new_w, new_h) = (fold.position - 1, self.h);
                new_self = Self { w: new_w, h: new_h, dots: Vec::new() };

                find_new_pos = |p, l| (p.0 - l, p.1);
                is_after_fold = |p, l| p.0 >= l;
            },
            Axis::Y => {
                let (new_w, new_h) = (self.w, fold.position - 1);
                new_self = Self { w: new_w, h: new_h, dots: Vec::new() };

                find_new_pos = |p, l| (p.0, p.1 - l);
                is_after_fold = |p, l| p.1 >= l;
            },
        }


        for x in 0..self.w {
            for y in 0..self.h {
                let pos = (x, y);
                let val = *self.get(pos).unwrap();

                // This dot is before the fold-line, does not need to be mirrored; it can simply be copied over
                if !is_after_fold((x, y), fold.position) {
                    self.add_point(pos, val);
                } else {
                    let new_pos = find_new_pos(pos, fold.position);
                    self.add_point(new_pos, val);
                }
            }
        }

        *self = new_self;
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.dots.iter().fold(0, |acc, &cur| {
            acc + if cur { 1 } else { 0 }
        })
    }

}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                let point = *self.get((x, y)).unwrap();
                let dot = if point { "#" } else { "." };
                f.write_str(dot)?;
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}


#[derive(Debug)]
pub enum Axis {
    X,
    Y,
}

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Axis::X => f.write_str("x"),
            Axis::Y => f.write_str("y"),
        }
    }
}



#[derive(Debug)]
pub struct Fold {
    pub axis: Axis,
    pub position: usize,
}

impl Fold {

    pub fn new<I>(data: I) -> Result<Vec<Self>, String>
    where
        I: IntoIterator, I::Item: AsRef<str>
    {

        let mut folds = Vec::new();

        for line in data {
            let line = line.as_ref();

            // Find '=' to split at
            let e = line
                .chars()
                .position(|c| c == '=')
                .ok_or_else(|| format!("{}: '{}'", LINE_ERR, line))?;

            let letter = line.chars().nth(e - 1);

            // Get the character before it
            let axis = match letter {
                Some(s) if s == 'x' => Axis::X,
                Some(s) if s == 'y' => Axis::Y,
                _ => { return Err(format!("{}: '{}'", LINE_ERR, line)); },
            };

            let position = line[(e + 1)..].parse().or_else(|_| Err(format!("{}: '{}'", LINE_ERR, line)))?;

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

        assert!(results.is_ok());

        let (mut paper, folds) = results.unwrap();

        println!("{}", paper);
        println!("{:?}", folds);

        assert_eq!(paper.count(), 18);

        paper.fold(&folds[0]).unwrap();
        assert_eq!(paper.count(), 17);
    }

    #[test]
    fn example() {
        let data = example_data();
        let (mut paper, folds) = parse(&data).unwrap();

        for fold in folds.iter() {
            paper.fold(fold).unwrap();
        }

        assert_eq!(paper.count(), 16);
    }

}