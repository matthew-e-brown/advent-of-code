use aoc_utils::Grid;
use aoc_utils::clap::{self, ValueEnum};

/// Which character set should be used for drawing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Charset {
    /// Use a grid of 2x4-resolution characters from the Unicode "Braille Patterns" block (U+2800–U+28FF). This will
    /// result in the highest-resolution ("pixel" density) grid of all available options.
    Braille,
    /// Use a grid of 2x2-resolution "quadrants" from the Unicode "Block Elements" block (U+2580–U+259F), plus spaces
    /// (U+0020).
    Quadrants,
    /// Use a plain grid of space (U+0020) and "full block" (U+2588) characters. This can result in a very large grid;
    /// you may want to zoom your terminal out first.
    FullBlocks,
}

impl Charset {
    pub const fn x_step(&self) -> usize {
        match self {
            Charset::Braille => 2,
            Charset::Quadrants => 2,
            Charset::FullBlocks => 1,
        }
    }

    pub const fn y_step(&self) -> usize {
        match self {
            Charset::Braille => 4,
            Charset::Quadrants => 2,
            Charset::FullBlocks => 1,
        }
    }

    pub fn make_char(&self, counts: &Grid<u8>, x: usize, y: usize) -> char {
        match self {
            Charset::Braille => make_braille(counts, x, y),
            Charset::Quadrants => make_quadrant(counts, x, y),
            Charset::FullBlocks => make_full_block(counts, x, y),
        }
    }

    pub const fn grid_width_term(&self, grid_width: usize) -> u16 {
        let grid_width = grid_width as u16;
        let char_width = self.x_step() as u16;
        (grid_width / char_width) + (if grid_width % char_width > 0 { 1 } else { 0 })
    }

    pub const fn grid_height_term(&self, grid_height: usize) -> u16 {
        let grid_height = grid_height as u16;
        let char_height = self.y_step() as u16;
        (grid_height / char_height) + (if grid_height % char_height > 0 { 1 } else { 0 })
    }
}

fn make_braille(counts: &Grid<u8>, x: usize, y: usize) -> char {
    /// Extracts a position from the grid `($x_off, $y_off)` away from the starting `idx`, checks if it is greater than
    /// zero, and shifts that boolean 1/0 into the given bit position.
    macro_rules! bit {
        ($x_off:expr, $y_off:expr, $bit:expr) => {
            (counts.get((x + $x_off, y + $y_off)).is_some_and(|c| *c > 0) as u8) << const { $bit - 1 }
        };
    }

    // Bit positions come from:
    // https://en.wikipedia.org/w/index.php?title=Braille_Patterns&oldid=1280293752#Identifying,_naming_and_ordering
    #[rustfmt::skip]
    let b =
        bit!(0, 0, 1) | bit!(1, 0, 4) |
        bit!(0, 1, 2) | bit!(1, 1, 5) |
        bit!(0, 2, 3) | bit!(1, 2, 6) |
        bit!(0, 3, 7) | bit!(1, 3, 8) ;
    char::from_u32(0x2800 | b as u32).expect("U+2800 to U+28FF should be valid unicode")
}

fn make_quadrant(counts: &Grid<u8>, x: usize, y: usize) -> char {
    macro_rules! bit {
        ($x_off:expr, $y_off:expr, $bit:expr) => {
            (counts.get((x + $x_off, y + $y_off)).is_some_and(|c| *c > 0) as u8) << $bit
        };
    }

    let mask = bit!(0, 0, 3) | bit!(1, 0, 2) | bit!(0, 1, 1) | bit!(1, 1, 0);
    match mask {
        0b0000 => ' ', // U+0020 Space
        // Single
        0b1000 => '▘', // U+2598 Quadrant upper left
        0b0100 => '▝', // U+259D Quadrant upper right
        0b0010 => '▖', // U+2596 Quadrant lower left
        0b0001 => '▗', // U+2597 Quadrant lower right
        // Double
        0b1100 => '▀', // U+2580 Upper half block
        0b0011 => '▄', // U+2584 Lower half block
        0b1010 => '▌', // U+258C Left half block
        0b0101 => '▐', // U+2590 Right half block
        0b1001 => '▚', // U+259A Quadrant upper left and lower right
        0b0110 => '▞', // U+259E Quadrant upper right and lower left
        // Triple
        0b1011 => '▙', // U+2599 Quadrant upper left and lower left and lower right
        0b1110 => '▛', // U+259B Quadrant upper left and upper right and lower left
        0b1101 => '▜', // U+259C Quadrant upper left and upper right and lower right
        0b0111 => '▟', // U+259F Quadrant upper right and lower left and lower right
        // Other
        0b1111 => '█', // U+2588 Full block
        _ => unreachable!("bitmask is created from shifts of size 0 to 3"),
    }
}

fn make_full_block(counts: &Grid<u8>, x: usize, y: usize) -> char {
    if counts.get((x, y)).is_some_and(|c| *c > 0) {
        '█'
    } else {
        ' '
    }
}
