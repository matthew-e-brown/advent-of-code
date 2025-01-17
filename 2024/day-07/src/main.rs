use std::str::FromStr;

fn main() {
    let input = aoc_utils::puzzle_input();
    let input = input.lines().map(|line| line.parse::<Equation>().unwrap());

    let mut sum = 0;
    for eq in input {
        let solutions = eq.find_solutions();
        if solutions.len() > 0 {
            sum += eq.value;
        }
    }

    println!("Sum of solvable equations (part 1): {sum}");
}

impl FromStr for Equation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = s.find(':').ok_or("equation is missing a ':'")?;
        let value = s[..c].parse::<usize>().or(Err("equation has invalid value"))?;
        let terms = s
            .get(c + 1..)
            .unwrap_or("")
            .split_whitespace()
            .map(|n| n.parse::<usize>().or(Err("equation has invalid term")))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Equation { value, terms })
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Debug)]
struct Equation {
    value: usize,
    terms: Vec<usize>,
}

/// Clones a vector ensuring it has the same capacity as the original.
#[inline]
fn duplicate_vec<T: Clone>(v: &Vec<T>) -> Vec<T> {
    let mut new = Vec::with_capacity(v.capacity());
    new.clone_from(&v);
    new
}

impl Equation {
    pub fn find_solutions(&self) -> Vec<Vec<Op>> {
        let Equation { value, terms } = self;

        fn recurse(x: usize, r_terms: &[usize], goal: usize, path: Vec<Op>, solutions: &mut Vec<Vec<Op>>) {
            if r_terms.len() == 0 {
                if x == goal {
                    // This path was successful!
                    solutions.push(path);
                }
            } else if x > goal {
                // If our number ever gets larger than the goal, we can instantly drop the rest of this branch, since
                // all subsequent additions or multiplications will only make it even larger.
                return;
            } else {
                let mut add_path = duplicate_vec(&path);
                let mut mul_path = path;
                add_path.push(Op::Add);
                mul_path.push(Op::Mul);
                recurse(x + r_terms[0], &r_terms[1..], goal, add_path, solutions);
                recurse(x * r_terms[0], &r_terms[1..], goal, mul_path, solutions);
            }
        }

        if terms.len() == 0 {
            return vec![];
        }

        let mut solution_paths = Vec::new();
        let base_path = Vec::with_capacity(terms.len());

        recurse(
            terms[0],
            terms.get(1..).unwrap_or_default(),
            *value,
            base_path,
            &mut solution_paths,
        );

        solution_paths
    }
}
