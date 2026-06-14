mod input;

use input::Transform;

fn main() {
    let input = aoc_utils::puzzle_input();
    let (shapes, _regions) = input::parse_input(input).unwrap();

    // println!("{regions:?}");
    // println!("{shapes:#?}");

    println!("Shape 0 transformations:");
    for t in Transform::VARIANTS {
        println!("{t:?}:\n{:}\n", shapes[0].with_transform(t));
    }

    println!();

    // println!("\n0x.points == 4x.points? {:?}", shapes[4].points() == shapes[4].rotate_cw(4).points());

    println!("\nShape 1 flipped:");
    println!("None:\n{:}", shapes[1]);
    println!("Flipped:\n{:}", shapes[1].with_transform(Transform::ReflectV));

    println!(
        "\n2x flip.points == 0x.points? {:?}",
        shapes[1].points() == shapes[1].with_transform(Transform::ReflectV).with_transform(Transform::ReflectV).points(),
    );
}
