#![allow(unused)]

use std::fmt::Display;
use std::str::FromStr;

/// A copy of the cephalopod's worksheet, with additional metadata associated to the column.
#[derive(Debug, Clone)]
pub struct Worksheet<'w> {
    /// The full input text, split by line.
    ///
    /// Does not include the final line of operators.
    raw_terms: Vec<&'w str>,

    /// Information about the columns in `raw_terms`.
    col_data: Vec<ColumnData>,
}

/// Information used by a [Worksheet] to index into its lines of input.
#[derive(Debug, Clone, Copy)]
struct ColumnData {
    /// Offset into the line where this column starts.
    start: usize,
    /// How wide this column is.
    width: u8,
    /// The operator at the bottom of this column.
    operator: Op,
}

/// An operator in a cephalopod problem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Mul,
}

impl<'w> Worksheet<'w> {
    pub fn from_input(input: &'w str) -> Result<Self, &'static str> {
        // Our parsing routines don't specifically depend on text being ASCII, but we do rely on every character being
        // exactly one byte in size. It's just easier this way.
        if !input.is_ascii() {
            return Err("worksheet input must be ascii-only text");
        }

        // First: read in the lines and count how many there are.
        // There must be at least 3: one line of operators plus 2 or more lines of terms.
        let mut lines = input.lines().collect::<Vec<&str>>();
        if lines.len() < 3 {
            return Err("too few lines: need at least 2 lines of terms + 1 line of operators");
        }

        // Next, figure out where the columns are split.
        let op_line = lines.pop().unwrap();
        let col_data = parse_operator_line(op_line)?;

        // That's all we really need to do to parse things, but it can't hurt to quickly double check that the columns
        // actually are the correct length and have whitespace between them:
        verify_line_offsets(&col_data, op_line)?;
        for &line in &lines {
            verify_line_offsets(&col_data, line)?;
        }

        Ok(Self { raw_terms: lines, col_data })
    }
}

/// Parses a series of columns which contain operators.
fn parse_operator_line(mut line: &str) -> Result<Vec<ColumnData>, &'static str> {
    // Trim off specifically any `\n` characters, but leave other whitespace (since ' ' are important for us).
    line = line.trim_end_matches('\n');

    let mut columns = Vec::new();
    let mut start = 0;
    while start < line.len() {
        // Each column should start with a valid operator. Each operator *should* only be one column, but it doesn't
        // hurt to allow for wider ones! :D
        //
        // We can figure out where the operator ends by looking for the next space character. If there are no more
        // spaces, the operator takes up the remaining chunk of the line.
        let op_end = line[start..].find(|c| c == ' ').map(|i| start + i).unwrap_or(line.len());
        let operator = line[start..op_end].parse::<Op>()?;

        // Next, we need to know how wide the column is. We can figure that out by looking for the start of the next
        // column; the first *non-space* character. If there are no more non-space characters, then this operator was
        // the last thing on the line.
        let next_start = line[op_end..].find(|c| c != ' ').map(|i| op_end + i).unwrap_or(line.len());

        // This column then ends one before the next one starts---unless this is the last column, in which case it ends
        // there.
        let col_width = if next_start == line.len() {
            next_start - start
        } else {
            next_start - start - 1
        };

        columns.push(ColumnData {
            operator,
            start,
            width: col_width
                .try_into()
                .map_err(|_| "columns wider than u8::MAX are not supported")?,
        });

        // Now we just want to actually tick forwards:
        start = next_start;
    }

    Ok(columns)
}

/// Ensures that every column is followed by a space.
fn verify_line_offsets(data: &[ColumnData], line: &str) -> Result<(), &'static str> {
    for &ColumnData { start, width, .. } in data {
        // The next column after this starts at start + width.
        let space = start + width as usize;
        if space < line.len() && &line[space..space + 1] != " " {
            return Err("columns are misaligned");
        }
    }
    Ok(())
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Op::Add => "+",
            Op::Mul => "*",
        })
    }
}

impl FromStr for Op {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err("encountered invalid operator"),
        }
    }
}
