use std::str::FromStr;

fn main() {
    let input = aoc_utils::puzzle_input();
    let presents = input
        .lines()
        .map(|line| line.parse::<Present>().expect("puzzle input should be valid"));

    let mut total_paper = 0;
    let mut total_ribbon = 0;
    for present in presents {
        total_paper += present.paper_needed();
        total_ribbon += present.ribbon_needed();
    }

    println!("Total amount of wrapping paper needed (part 1): {total_paper}");
    println!("Total amount of ribbon needed (part 2): {total_ribbon}");
}

#[derive(Debug, Clone, Copy)]
struct Present {
    l: u64,
    w: u64,
    h: u64,
}

impl Present {
    pub fn paper_needed(&self) -> u64 {
        // Present's total surface area, plus area of smallest side.
        let area1 = self.l * self.w;
        let area2 = self.w * self.h;
        let area3 = self.h * self.l;
        let smallest = area1.min(area2).min(area3);
        2 * (area1 + area2 + area3) + smallest
    }

    pub fn ribbon_needed(&self) -> u64 {
        // Perimeter of smallest face, plus total volume
        let peri1 = 2 * (self.l + self.w);
        let peri2 = 2 * (self.w + self.h);
        let peri3 = 2 * (self.h + self.l);
        let volume = self.l * self.w * self.h;
        peri1.min(peri2).min(peri3) + volume
    }
}

impl FromStr for Present {
    type Err = &'static str;

    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const INVALID_NUM: &str = "present contained an invalid integer";
        let mut bits = s.split('x');
        let l = bits.next().ok_or("cannot parse present from empty string")?.parse().map_err(|_| INVALID_NUM)?;
        let w = bits.next().ok_or("present had too few 'x' characters")?.parse().map_err(|_| INVALID_NUM)?;
        let h = bits.next().ok_or("present had too few 'x' characters")?.parse().map_err(|_| INVALID_NUM)?;
        if bits.count() == 0 {
            Ok(Present { l, w, h })
        } else {
            Err("present has more than 3 dimensions")
        }
    }
}
