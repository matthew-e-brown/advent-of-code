use std::fmt::Display;
use std::collections::HashSet;

type ParseResult<T> = Result<T, String>;
pub const LINE_ERR: &'static str = "Encountered malformed line";


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dot {
    x: usize,
    y: usize,
}

pub enum Axis {
    X,
    Y,
}

pub struct Fold {
    axis: Axis,
    position: usize,
}

impl Fold {

    pub fn new<I>(data: I) -> ParseResult<Vec<Self>>
    where
        I: IntoIterator,
        I::Item: AsRef<str>
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


#[derive(Debug, Clone)]
pub struct Paper {
    dots: HashSet<Dot>,
}

impl Paper {

    pub fn new<I>(data: I) -> ParseResult<Self>
    where
        I: IntoIterator,
        I::Item: AsRef<str>
    {
        let mut dots = HashSet::new();

        for line in data {
            let line = line.as_ref();

            // Find the (x, y) pairs on this line
            let splits = line.split(",").collect::<Vec<_>>();
            if splits.len() != 2 {
                return Err(format!("{}: '{}'", LINE_ERR, line));
            }

            let x = splits[0].parse().or_else(|_| Err(format!("{}: '{}'", LINE_ERR, line)))?;
            let y = splits[1].parse().or_else(|_| Err(format!("{}: '{}'", LINE_ERR, line)))?;

            dots.insert(Dot { x, y });
        }

        Ok(Self { dots })
    }

    pub fn size(&self) -> (usize, usize) {
        self.dots.iter().fold((0, 0), |mut acc, cur| {
            if cur.x > acc.0 { acc.0 = cur.x; }
            if cur.y > acc.1 { acc.1 = cur.y; }
            acc
        })
    }

    pub fn count(&self) -> usize {
        self.dots.len()
    }

    pub fn fold(&mut self, fold: Fold) {
        let find_new_dot: fn(Dot, usize) -> Dot = match fold.axis {
            // If the fold is along the 'x' axis, 'y' is changing
            Axis::X => |dot, p| Dot {
                x: dot.x,
                y: if dot.y > p { dot.y - p } else { dot.y },
            },
            // Otherwise, 'x' is changing
            Axis::Y => |dot, p| Dot {
                x: if dot.x > p { dot.x - p } else { dot.x },
                y: dot.y,
            },
        };

        // Pull out all the dots, find their new positions, and stick them back in
        let dots = self.dots.drain().collect::<Vec<_>>();
        for dot in dots {
            self.dots.insert(find_new_dot(dot, fold.position));
        }
    }

}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = self.size();

        for y in 0..=height {
            for x in 0..=width {
                let is_point = self.dots.contains(&Dot { x, y });
                f.write_str(if is_point { "#" } else { "." })?;
            }

            if y <= height - 1 {
                f.write_str("\n")?;
            }
        }

        Ok(())
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
    fn parse_test() {
        let data = example_data();
        let results = parse(&data);

        assert!(results.is_ok());

        assert_eq!("\
            ...#..#..#.\n\
            ....#......\n\
            ...........\n\
            #..........\n\
            ...#....#.#\n\
            ...........\n\
            ...........\n\
            ...........\n\
            ...........\n\
            ...........\n\
            .#....#.##.\n\
            ....#......\n\
            ......#...#\n\
            #..........\n\
            #.#........".to_owned(),
            format!("{}", results.unwrap().0)
        );
    }

}