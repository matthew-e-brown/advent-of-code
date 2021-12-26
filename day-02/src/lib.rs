use std::cmp::min;

pub struct PresentBox {
    pub side_a: usize,
    pub side_b: usize,
    pub side_c: usize,
}

pub fn boxes_from_strings(strings: &Vec<String>) -> Result<Vec<PresentBox>, &'static str> {
    let mut boxes = Vec::new();

    for string in strings {
        let sides = string.split("x").map(|s| {
            let num: usize = s.parse().or(Err("All boxes' dimensions should be made of numbers."))?;
            Ok(num)
        }).collect::<Result<Vec<usize>, &'static str>>()?;

        if sides.len() != 3 {
            return Err("All boxes should have three dimensions.");
        }

        boxes.push(PresentBox {
            side_a: *sides.get(0).unwrap(),
            side_b: *sides.get(1).unwrap(),
            side_c: *sides.get(2).unwrap(),
        });
    }

    Ok(boxes)
}

pub fn run_1(presents: &Vec<PresentBox>) -> usize {

    let mut running_total = 0;

    for gift in presents {

        let face_1 = gift.side_a * gift.side_b;
        let face_2 = gift.side_b * gift.side_c;
        let face_3 = gift.side_c * gift.side_a;

        let extra = min(face_1, min(face_2, face_3));

        running_total += 2 * face_1 + 2 * face_2 + 2 * face_3 + extra;

    }

    running_total
}


pub fn run_2(presents: &Vec<PresentBox>) -> usize {

    let mut running_total = 0;

    for gift in presents {
        let perimeter_1 = (gift.side_a + gift.side_b) * 2;
        let perimeter_2 = (gift.side_b + gift.side_c) * 2;
        let perimeter_3 = (gift.side_a + gift.side_c) * 2;

        let ribbon = gift.side_a * gift.side_b * gift.side_c;

        running_total += ribbon + min(perimeter_1, min(perimeter_2, perimeter_3));
    }

    running_total
}



#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    fn slice_to_owned(slices: Vec<&str>) -> Vec<String> {
        slices.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test_case(vec![ "2x3x4" ],   58; "case 1")]
    #[test_case(vec![ "1x1x10" ],  43; "case 2")]
    fn part_1(boxes: Vec<&str>, result: usize) {
        let boxes = slice_to_owned(boxes);
        let boxes = boxes_from_strings(&boxes).unwrap();
        assert_eq!(run_1(&boxes), result);
    }


    #[test_case(vec![ "2x3x4" ],   34; "case 1")]
    #[test_case(vec![ "1x1x10" ],  14; "case 2")]
    fn part_2(boxes: Vec<&str>, result: usize) {
        let boxes = slice_to_owned(boxes);
        let boxes = boxes_from_strings(&boxes).unwrap();
        assert_eq!(run_2(&boxes), result);
    }

}