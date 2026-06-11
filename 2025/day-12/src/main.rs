mod input;

fn main() {
    let input = aoc_utils::puzzle_input();
    let (shapes, _regions) = input::parse_input(input).unwrap();

    // println!("{regions:?}");
    // println!("{shapes:#?}");

    println!("Shape 4 rotated:");
    println!("0x:\n{:}", shapes[4]);
    println!("1x:\n{:}", shapes[4].rotate_cw(1));
    println!("2x:\n{:}", shapes[4].rotate_cw(2));
    println!("3x:\n{:}", shapes[4].rotate_cw(3));
    println!("4x:\n{:}", shapes[4].rotate_cw(4));

    println!("\n0x.points == 4x.points? {:?}", shapes[4].points() == shapes[4].rotate_cw(4).points());

    println!("Shape 1 flipped:");
    println!("None:       {:?}", shapes[1]);
    println!("Vertical:   {:?}", shapes[1].flip_vertical());
    println!("Horizontal: {:?}", shapes[1].flip_horizontal());

    println!(
        "\n2x flip.points == 0x.points? {:?} and {:?}",
        shapes[1].points() == shapes[1].flip_horizontal().flip_horizontal().points(),
        shapes[1].points() == shapes[1].flip_vertical().flip_vertical().points(),
    );
}
