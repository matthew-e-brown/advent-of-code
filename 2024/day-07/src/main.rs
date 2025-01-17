use std::str::FromStr;

fn main() {
    let input = aoc_utils::puzzle_input();
    let input = input.lines().map(|line| line.parse::<Equation>().unwrap());

    let mut sum1 = 0;
    let mut sum2 = 0;
    for eq in input {
        let solutions = eq.find_solutions();
        if solutions.len() > 0 {
            sum2 += eq.value;

            // Part 1 didn't have the notion of concatenation, so we only want to count towards its total if
            let no_concat = solutions.into_iter().all(|sol| !sol.into_iter().any(|op| op == Op::Concat));
            if no_concat {
                sum1 += eq.value;
            }
        }
    }

    println!("Sum of solvable equations without concatenation (part 1): {sum1}");
    println!("Sum of solvable equations with concatenation (part 2): {sum2}");
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Concat,
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
                let mut mul_path = duplicate_vec(&path);
                let mut cat_path = path;
                add_path.push(Op::Add);
                mul_path.push(Op::Mul);
                cat_path.push(Op::Concat);
                recurse(x + r_terms[0], &r_terms[1..], goal, add_path, solutions);
                recurse(x * r_terms[0], &r_terms[1..], goal, mul_path, solutions);
                recurse(concat(x, r_terms[0]), &r_terms[1..], goal, cat_path, solutions);
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

/// Concatenates the base-10 digits of two numbers.
const fn concat(mut a: usize, b: usize) -> usize {
    // - Get number of digits in `b`.
    // - Shift `a` over by multiplying by that power of ten.
    // - Add `b` into the new zeroes on the right.
    let e = if b == 0 { 1 } else { b.ilog10() + 1 };
    a *= 10usize.pow(e);
    a += b;
    a
}
